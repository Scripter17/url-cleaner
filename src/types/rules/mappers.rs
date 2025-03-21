//! The logic for how to modify a URL.

use std::str::Utf8Error;
use std::collections::HashSet;
use std::time::Duration;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// The part of a [`Rule`] that specifies how to modify a [`Url`] if the rule's condition passes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Mapper {

    // Testing.

    /// Does nothing.
    None,
    /// Always returns the error [`MapperError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`MapperError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its application to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    // Logic.

    /// If `condition` passes, apply `mapper`, otherwise apply `else_mapper`.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If either possible call to [`Mapper::apply`] returns an error, that error is returned.
    IfCondition {
        /// The [`Condition`] that decides if `mapper` or `else_mapper` is used.
        condition: Condition,
        /// The [`Self`] to use if `condition` passes.
        mapper: Box<Self>,
        /// The [`Self`] to use if `condition` fails.
        #[serde(default)]
        else_mapper: Option<Box<Self>>
    },
    /// Effectively a [`Self::IfCondition`] where each subsequent link is put inside the previous link's [`Self::IfCondition::else_mapper`].
    /// # Errors
    /// If a call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If a call to [`Mapper::apply`] returns an error, that error is returned.
    ConditionChain(Vec<ConditionChainLink>),
    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If one of the calls to [`Self::apply`] returns an error, the URL is left unchanged and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut job_state).unwrap_err();
    /// assert_eq!(job_state.url.domain(), Some("example.com"));
    /// ```
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// 
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the calls to [`Self::apply`] returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut job_state).unwrap_err();
    /// assert_eq!(job_state.url.domain(), Some("3.com"));
    /// ```
    AllNoRevert(Vec<Self>),
    /// If any of the calls to [`Self::apply`] returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// 
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.domain(), Some("6.com"));
    /// ```
    AllIgnoreError(Vec<Self>),
    /// Indexes `map` with the string returned by `part` and applies that mapper.
    /// # Errors
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    PartMap {
        /// The part to index `map` with.
        part: UrlPart,
        /// The map specifying which values should apply which mapper.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Indexes `map` with the string returned by `value` and applies that mapper.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    StringMap {
        /// The string to index `map` with.
        value: StringSource,
        /// The map specifying which strings should apply which mapper.
        #[serde(flatten)]
        map: Map<Self>
    },

    // Error handling.

    /// Ignores any error the call to [`Self::apply`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
    /// 
    /// If `try` does not return an error, `else` is not applied.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::None )}.apply(&mut job_state).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::Error)}.apply(&mut job_state).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::None )}.apply(&mut job_state).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::Error)}.apply(&mut job_state).unwrap_err();
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every call to [`Self::apply`] returns an error, returns the last error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Mapper::FirstNotError(vec![Mapper::SetHost("1.com".to_string()), Mapper::SetHost("2.com".to_string())]).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.domain(), Some("1.com"));
    /// Mapper::FirstNotError(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error                       ]).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.domain(), Some("3.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::SetHost("4.com".to_string())]).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.domain(), Some("4.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::Error                       ]).apply(&mut job_state).unwrap_err();
    /// assert_eq!(job_state.url.domain(), Some("4.com"));
    /// ```
    FirstNotError(Vec<Self>),

    // Query.

    /// Removes the URL's entire query.
    /// Useful for websites that only use the query for tracking.
    RemoveQuery,
    /// Removes a single query parameter with the specified name.
    ///
    /// Unlike [`Self::RemoveQueryParams`] and [`Self::AllowQueryParams`], this uses a [`StringSource`] to be a lot more versatile.
    RemoveQueryParam(StringSource),
    /// Removes all query parameters whose name exists in the specified [`std::collections::HashMap`].
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    /// # Examples
    /// ```
    /// # use std::collections::hash_set::HashSet;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state; url = "https://example.com?a=2&b=3&c=4&d=5";);
    /// 
    /// Mapper::RemoveQueryParams(["a".to_string()].into()).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.query(), Some("b=3&c=4&d=5"));
    /// Mapper::RemoveQueryParams(["b".to_string(), "c".to_string()].into()).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.query(), Some("d=5"));
    /// Mapper::RemoveQueryParams(["d".to_string()].into()).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.query(), None);
    /// ```
    RemoveQueryParams(HashSet<String>),
    /// Keeps only the query parameters whose name exists in the specified [`HashSet`].
    /// Useful for websites that keep changing their tracking parameters and you're sick of updating your rule set.
    /// # Examples
    /// ```
    /// # use std::collections::hash_set::HashSet;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state; url = "https://example.com?a=2&b=3";);
    /// 
    /// Mapper::AllowQueryParams(["a".to_string()].into()).apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.as_str(), "https://example.com/?a=2");
    /// ```
    AllowQueryParams(HashSet<String>),
    /// Removes all query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only the query parameters whose name matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    AllowQueryParamsMatching(StringMatcher),
    /// Replace the current URL with the value of the specified query parameter.
    /// Useful for websites for have a "are you sure you want to leave?" page with a URL like `https://example.com/outgoing?to=https://example.com`.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    /// 
    /// If the query parameter is found but its value cannot be parsed as a URL, returns the error [`MapperError::UrlParseError`].
    GetUrlFromQueryParam(String),
    /// Replace the current URL's path with the value of the specified query parameter.
    /// Useful for websites that have a "you must log in to see this page" page.
    /// # Errors
    /// If the specified query parameter cannot be found, returns the error [`MapperError::CannotFindQueryParam`].
    GetPathFromQueryParam(String),

    // Other parts.

    /// [`Url::set_host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, returns that error.
    SetHost(String),
    /// [`Url::join`].
    Join(StringSource),

    // Generic part handling.

    /// Sets the specified URL part to `to`.
    /// # Errors
    /// If the call to [`StringSource::get`] return's an error, that error is returned.
    /// 
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    SetPart {
        /// The name of the part to replace.
        part: UrlPart,
        /// The value to set the part to.
        value: StringSource
    },
    /// Modifies the specified part of the URL.
    ///
    /// If the part is [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error.
    /// 
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    ModifyPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// How exactly to modify the part.
        modification: StringModification
    },
    /// Copies the part specified by `from` to the part specified by `to`.
    /// # Errors
    /// If the part specified by `from` is [`None`] and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`UrlPartSetError::PartCannotBeNone`].
    CopyPart {
        /// The part to get the value from.
        from: UrlPart,
        /// The part to set to `from`'s value.
        to: UrlPart
    },
    /// Effectively [`Self::CopyPart`] then [`Self::SetPart`] `from` to [`None`].
    /// # Errors
    /// If the part specified by `from` is [`None`] and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`UrlPartSetError::PartCannotBeNone`].
    /// 
    /// If the call to `from`'s [`UrlPart::set`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state; url = "https://abc.example.com";);
    /// 
    /// Mapper::MovePart{from: UrlPart::Subdomain, to: UrlPart::BeforePathSegment(0)}.apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.as_str(), "https://example.com/abc/");
    /// 
    /// Mapper::MovePart{from: UrlPart::Scheme, to: UrlPart::BeforePathSegment(0)}.apply(&mut job_state).unwrap_err();
    /// assert_eq!(job_state.url.as_str(), "https://example.com/abc/");
    /// ```
    MovePart {
        /// The part to get the value from then set to [`None`].
        from: UrlPart,
        /// The part to set to `from`'s value.
        to: UrlPart
    },

    // Miscellaneous.

    /// Sends an HTTP GET request to the current URL and, if the website returns a status code between 300 and 399 (inclusive) (a "3xx" status code), sets the URL to the value found in the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header.
    /// Useful for link shorteners like `bit.ly` and `t.co`.
    /// 
    /// Please note that some websites (like `tinyurl.com` and `duckduckgo.com`) don't do redirects properly and therefore need to be fixed via more complex methods.
    /// If you know how to detect when a DDG search query has a bang that DDG will actually use (`"a !g"` doesn't redirect to google), please let me know as that would be immensely useful.
    /// 
    /// # Privacy
    /// 
    /// Please note that, by default, this mapper recursively expands short links. If a `t.co` link links to a `bit.ly` link, it'll return the page the `bit.ly` link links to.
    /// However, this means that this mapper will by default send an HTTP GET request to all pages pointed to even if they're not redirects.
    /// 
    /// The default config handles this by configuring [`Self::ExpandRedirect::http_client_config_diff`]'s [`HttpClientConfigDiff::redirect_policy`] to `Some(`[`RedirectPolicy::None`]`)`.
    /// And, because it's in a [`Rule::Repeat`], it still handles recursion up to 10 levels deep while preventing leaks to the last page.
    /// # Errors
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::read`] returns an error, that error is returned.")]
    /// 
    /// If the call to [`JobStateView::http_client`] returns an error, that error is returned.
    /// 
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    /// 
    /// (3xx status code) If the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header is not found, returns the error [`MapperError::HeaderNotFound`].
    /// 
    /// (3xx status code) If the call to [`reqwest::header::HeaderValue::to_str`] to get the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header returns an error, that error is returned.
    /// 
    /// (3xx status code) If the call to [`Url::parse`] to parse the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header returns an error, that error is returned.
    /// 
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::write`] returns an error, that error is returned.")]
    /// # Examples
    /// ```
    /// # use reqwest::header::HeaderMap;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state; url = "https://t.co/H8IF8DHSFL";);
    /// 
    /// Mapper::ExpandRedirect{headers: HeaderMap::default(), http_client_config_diff: None}.apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.as_str(), "https://www.eff.org/deeplinks/2024/01/eff-and-access-now-submission-un-expert-anti-lgbtq-repression");
    /// ```
    #[cfg(feature = "http")]
    ExpandRedirect {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "headermap")]
        headers: HeaderMap,
        /// Rules for how to create the HTTP client in addition to [`Params::http_client_config`] and [`CommonCallArgs::http_client_config_diff`].
        #[serde(default)]
        http_client_config_diff: Option<Box<HttpClientConfigDiff>>
    },
    /// Sets the the specified flag in [`JobScratchpad::flags`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MapperError::StringSourceIsNone`].
    SetScratchpadFlag {
        /// The name of the flag to set.
        name: StringSource,
        /// The value to set it to.
        value: bool
    },
    /// Sets the specified var in [`JobScratchpad::vars`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    SetScratchpadVar {
        /// The name of the variable to set.
        name: StringSource,
        /// The value to set the variable to.
        value: StringSource
    },
    /// Delete the current job's `name` string var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    DeleteScratchpadVar(StringSource),
    /// Applies a [`StringModification`] to the current job's `name` string var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    ModifyScratchpadVar {
        /// The name of the variable to set.
        name: StringSource,
        /// The modification to apply.
        modification: StringModification
    },
    /// Executes the contained [`Rule`].
    /// # Errors
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    Rule(Box<Rule>),
    /// Excites the contained [`Rules`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    Rules(Rules),
    /// Read from the cache using the current [`JobState::url`] as the [`CacheEntry::key`].
    /// 
    /// If an entry is found, sets the provided [`JobState::url`] to its value.
    /// 
    /// If an entry is not found, calls [`Mapper::apply`] and writes the new [`JobState::url`] to the cache.
    /// 
    /// Changes to [`JobState::scratchpad`] are not cached but the resulting URL still is.
    /// 
    /// That will hopefully change at some point.
    /// # Errors
    /// If the call to [`Cache::read`] returns an error, that error is returned.
    /// 
    /// If the call to [`Cache::read`] returns [`None`], returns the error [`MapperError::CachedUrlIsNone`].
    /// 
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    /// 
    /// If the call to [`Cache::write`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    CacheUrl {
        /// The category to cache in.
        category: StringSource,
        /// The [`Self`] to cache.
        mapper: Box<Self>
    },
    /// Retry `mapper` after `delay` at most `limit` times.
    /// 
    /// Note that if the call to [`Mapper::apply`] changes the job state (see [`Mapper::AllNoRevert`]), the job state is not reverted.
    Retry {
        /// The mapper to apply.
        mapper: Box<Self>,
        /// The duration to wait between tries.
        delay: Duration,
        /// The max number of tries.
        /// 
        /// Defaults to `10`.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::mappers`].
    Common(CommonCall),
    /// Uses a function pointer.
    /// 
    /// Cannot be serialized or deserialized.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    Custom(FnWrapper<fn(&mut JobState) -> Result<(), MapperError>>)
}

/// Individual links in the [`Mapper::ConditionChain`] chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct ConditionChainLink {
    /// The [`Condition`] to apply [`Self::mapper`] under.
    pub condition: Condition,
    /// The [`Mapper`] to apply if [`Self::condition`] is satisfied.
    pub mapper: Mapper
}

/// Serde helper function.
const fn get_10_u8() -> u8 {10}

/// An enum of all possible errors a [`Mapper`] can return.
#[derive(Debug, Error)]
pub enum MapperError {
    /// Returned when [`Mapper::Error`] is used.
    #[error("Mapper::Error was used.")]
    ExplicitError,
    /// Returned when the provided URL does not contain the requested query parameter.
    #[error("The provided URL does not contain the requested query parameter.")]
    CannotFindQueryParam,
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`reqwest::Error`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// Returned when a [`UrlPartSetError`] is encountered.
    #[error(transparent)]
    UrlPartSetError(#[from] UrlPartSetError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)]
    RuleError(Box<RuleError>),
    /// Returned when the requested header is not found.
    #[cfg(feature = "http")]
    #[error("The requested header was not found.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when both the `try` and `else` of a [`Mapper::TryElse`] both return errors.
    #[error("A `Mapper::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`Mapper::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`Mapper::TryElse::else`],
        else_error: Box<Self>
    },
    /// Returned when a [`JobState`] string var is [`None`].
    #[error("A JobState string var was none.")]
    ScratchpadVarIsNone,
    /// Returned when a [`ReadFromCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    /// Returned when a [`WriteToCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),
    /// Returned when the cached [`Url`] is [`None`].
    #[cfg(feature = "cache")]
    #[error("The cached URL was None.")]
    CachedUrlIsNone,
    /// Returned when the common [`Mapper`] is not found.
    #[error("The common Mapper was not found.")]
    CommonMapperNotFound,
    /// Returned when the mapper is not found.
    #[error("The mapper was not found.")]
    MapperNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Custom error.
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>),
    /// Returned when the requested part of a URL is [`None`].
    #[error("The requested part of the URL was None.")]
    UrlPartIsNone
}

impl From<RuleError> for MapperError {
    fn from(value: RuleError) -> Self {
        Self::RuleError(Box::new(value))
    }
}

impl Mapper {
    /// Applies the mapper to the provided URL.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    ///
    /// If an error occurs, `job_state` is effectively unmodified, though the mutable parts may be clones.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), MapperError> {
        debug!(Mapper::apply, self, job_state);
        match self {
            // Testing.

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let old_url = job_state.url.clone();
                let old_scratchpad = job_state.scratchpad.clone();
                let mapper_result=mapper.apply(job_state);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nMapper return value: {mapper_result:?}\nNew job state: {job_state:?}");
                mapper_result?;
            },

            // Logic.

            Self::IfCondition {condition, mapper, else_mapper} => if condition.satisfied_by(&job_state.to_view())? {
                mapper.apply(job_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(job_state)?;
            },
            Self::ConditionChain(chain) => for link in chain {
                if link.condition.satisfied_by(&job_state.to_view())? {
                    link.mapper.apply(job_state)?;
                    break;
                }
            },
            Self::All(mappers) => {
                let old_url = job_state.url.clone();
                let old_scratchpad = job_state.scratchpad.clone();
                for mapper in mappers {
                    match mapper.apply(job_state) {
                        Ok(_) => {},
                        Err(e) => {
                            *job_state.url = old_url;
                            *job_state.scratchpad = old_scratchpad;
                            return Err(e);
                        }
                    }
                }
            },
            Self::AllNoRevert(mappers) => {
                for mapper in mappers {
                    mapper.apply(job_state)?;
                }
            },
            Self::AllIgnoreError(mappers) => {
                for mapper in mappers {
                    let _=mapper.apply(job_state);
                }
            },
            Self::PartMap  {part , map} => if let Some(mapper) = map.get(part .get( job_state.url      ) ) {mapper.apply(job_state)?},
            Self::StringMap{value, map} => if let Some(mapper) = map.get(value.get(&job_state.to_view())?) {mapper.apply(job_state)?},

            // Error handling.

            Self::IgnoreError(mapper) => {let _=mapper.apply(job_state);},
            Self::TryElse{r#try, r#else} => r#try.apply(job_state).or_else(|try_error| r#else.apply(job_state).map_err(|else_error2| MapperError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error2)}))?,
            Self::FirstNotError(mappers) => {
                let mut result = Ok(());
                for mapper in mappers {
                    result = mapper.apply(job_state);
                    if result.is_ok() {break}
                }
                result?
            },

            // Query.

            Self::RemoveQuery => job_state.url.set_query(None),
            Self::RemoveQueryParam(name) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let job_state_view = job_state.to_view();
                let name = get_cow!(name, job_state_view, MapperError);
                let new_query = form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(x, _)| *x != name)).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParams(names) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(job_state.url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in job_state.url.query_pairs() {
                    if !matcher.satisfied_by(&name, &job_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query_len) = job_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in job_state.url.query_pairs() {
                    if matcher.satisfied_by(&name, &job_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                match job_state.url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_url)) => {*job_state.url=Url::parse(&new_url)?.into()},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match job_state.url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {#[expect(clippy::unnecessary_to_owned, reason = "False positive.")] job_state.url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => job_state.url.set_host(Some(new_host))?,
            Self::Join(with) => *job_state.url=job_state.url.join(get_str!(with, job_state, MapperError))?.into(),

            // Generic part handling.

            Self::SetPart{part, value} => part.set(job_state.url, value.get(&job_state.to_view())?.map(Cow::into_owned).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart{part, modification} => if let Some(mut temp) = part.get(job_state.url).map(|x| x.into_owned()) {
                modification.apply(&mut temp, &job_state.to_view())?;
                part.set(job_state.url, Some(&temp))?;
            }
            Self::CopyPart{from, to} => to.set(job_state.url, from.get(job_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart{from, to} => {
                let mut temp_url = job_state.url.clone();
                let temp_url_ref = &mut temp_url;
                to.set(temp_url_ref, from.get(temp_url_ref).map(|x| x.into_owned()).as_deref())?;
                from.set(&mut temp_url, None)?;
                *job_state.url = temp_url;
            },

            // Miscellaneous.

            #[cfg(feature = "http")]
            Self::ExpandRedirect {headers, http_client_config_diff} => {
                #[cfg(feature = "cache")]
                if job_state.params.read_cache {
                    if let Some(new_url) = job_state.cache.read("redirect", job_state.url.as_str())? {
                        *job_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let response = job_state.to_view().http_client(http_client_config_diff.as_deref())?.get(job_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    Url::parse(std::str::from_utf8(response.headers().get("location").ok_or(MapperError::HeaderNotFound)?.as_bytes())?)?
                } else {
                    response.url().clone()
                };
                #[cfg(feature = "cache")]
                if job_state.params.write_cache {
                    job_state.cache.write("redirect", job_state.url.as_str(), Some(new_url.as_str()))?;
                }
                *job_state.url=new_url.into();
            },

            Self::SetScratchpadFlag {name, value} => {
                let name = get_string!(name, job_state, MapperError);
                match value {
                    true  => job_state.scratchpad.flags.insert( name),
                    false => job_state.scratchpad.flags.remove(&name)
                };
            },
            Self::SetScratchpadVar {name, value} => {let _ = job_state.scratchpad.vars.insert(get_string!(name, job_state, MapperError).to_owned(), get_string!(value, job_state, MapperError).to_owned());},
            Self::DeleteScratchpadVar(name) => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let _ = job_state.scratchpad.vars.remove(&name);
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let mut temp = job_state.scratchpad.vars.get_mut(&name).ok_or(MapperError::ScratchpadVarIsNone)?.to_owned();
                modification.apply(&mut temp, &job_state.to_view())?;
                let _ = job_state.scratchpad.vars.insert(name, temp);
            },
            Self::Rule(rule) => {rule.apply(job_state)?;},
            Self::Rules(rules) => {rules.apply(job_state)?;},
            #[cfg(feature = "cache")]
            Self::CacheUrl {category, mapper} => {
                let category = get_string!(category, job_state, MapperError);
                if job_state.params.read_cache {
                    if let Some(new_url) = job_state.cache.read(&category, job_state.url.as_str())? {
                        *job_state.url = Url::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?.into();
                        return Ok(());
                    }
                }
                let old_url = job_state.url.clone();
                let old_vars = job_state.scratchpad.vars.clone();
                mapper.apply(job_state)?;
                if job_state.params.write_cache {
                    if let e @ Err(_) = job_state.cache.write(&category, old_url.as_str(), Some(job_state.url.as_str())) {
                        *job_state.url = old_url;
                        job_state.scratchpad.vars = old_vars;
                        e?;
                    }
                }
            },
            Self::Retry {mapper, delay, limit} => {
                for i in 0..*limit {
                    match mapper.apply(job_state) {
                        Ok(()) => return Ok(()),
                        #[allow(clippy::arithmetic_side_effects, reason = "`i` is never 255 and therefore never overflows.")]
                        e @ Err(_) if i+1==*limit => e?,
                        Err(_) => {std::thread::sleep(*delay);}
                    }
                }
            },
            Self::Common(common_call) => {
                job_state.commons.mappers.get(get_str!(common_call.name, job_state, MapperError)).ok_or(MapperError::CommonMapperNotFound)?.apply(&mut JobState {
                    common_args: Some(&common_call.args.make(&job_state.to_view())?),
                    url: job_state.url,
                    context: job_state.context,
                    params: job_state.params,
                    scratchpad: job_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: job_state.cache,
                    commons: job_state.commons,
                    jobs_context: job_state.jobs_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        };
        Ok(())
    }
}

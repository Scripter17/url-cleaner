//! The logic for how to modify a URL.

use std::str::Utf8Error;
use std::collections::hash_set::HashSet;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[cfg(all(feature = "http", not(target_family = "wasm")))]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// The part of a [`Rule`] that specifies how to modify a [`Url`] if the rule's condition passes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    /// 
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
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
        /// The [`Self`] to use if `conditionf` passes.
        mapper: Box<Self>,
        /// The [`Self`] to use if `confition` fails.
        #[serde(default)]
        else_mapper: Option<Box<Self>>
    },
    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left unchanged and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::All(vec![Mapper::SetHost("2.com".to_string()), Mapper::Error]).apply(&mut JobState::new(&mut url)).unwrap_err();
    /// assert_eq!(url.domain(), Some("www.example.com"));
    /// ```
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::AllNoRevert(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error, Mapper::SetHost("4.com".to_string())]).apply(&mut JobState::new(&mut url)).unwrap_err();
    /// assert_eq!(url.domain(), Some("3.com"));
    /// ```
    AllNoRevert(Vec<Self>),
    /// If any of the contained [`Self`]s returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::AllIgnoreError(vec![Mapper::SetHost("5.com".to_string()), Mapper::Error, Mapper::SetHost("6.com".to_string())]).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.domain(), Some("6.com"));
    /// ```
    AllIgnoreError(Vec<Self>),

    // Error handling.

    /// Ignores any error the contained [`Self`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
    /// If `try` does not return an error, `else` is not applied.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::None )}.apply(&mut JobState::new(&mut Url::parse("https://www.example.com").unwrap())).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::None ), r#else: Box::new(Mapper::Error)}.apply(&mut JobState::new(&mut Url::parse("https://www.example.com").unwrap())).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::None )}.apply(&mut JobState::new(&mut Url::parse("https://www.example.com").unwrap())).unwrap ();
    /// Mapper::TryElse {r#try: Box::new(Mapper::Error), r#else: Box::new(Mapper::Error)}.apply(&mut JobState::new(&mut Url::parse("https://www.example.com").unwrap())).unwrap_err();
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url=Url::parse("https://www.example.com").unwrap();
    /// Mapper::FirstNotError(vec![Mapper::SetHost("1.com".to_string()), Mapper::SetHost("2.com".to_string())]).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.domain(), Some("1.com"));
    /// Mapper::FirstNotError(vec![Mapper::SetHost("3.com".to_string()), Mapper::Error                       ]).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.domain(), Some("3.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::SetHost("4.com".to_string())]).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.domain(), Some("4.com"));
    /// Mapper::FirstNotError(vec![Mapper::Error                       , Mapper::Error                       ]).apply(&mut JobState::new(&mut url)).unwrap_err();
    /// assert_eq!(url.domain(), Some("4.com"));
    /// ```
    FirstNotError(Vec<Self>),

    // Query.

    /// Removes the URL's entire query.
    /// Useful for websites that only use the query for tracking.
    RemoveQuery,
    /// Removes all query parameters whose name exists in the specified [`std::collections::HashMap`].
    /// Useful for websites that append random stuff to shared URLs so the website knows your friend got that link from you.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.query(), Some("b=3&c=4&d=5"));
    /// Mapper::RemoveQueryParams(HashSet::from(["b".to_string(), "c".to_string()])).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.query(), Some("d=5"));
    /// Mapper::RemoveQueryParams(HashSet::from(["d".to_string()])).apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.query(), None);
    /// ```
    RemoveQueryParams(HashSet<String>),
    /// Keeps only the query parameters whose name exists in the specified [`HashSet`].
    /// Useful for websites that keep changing their tracking parameters and you're sick of updating your rule set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::hash_set::HashSet;
    /// let mut url=Url::parse("https://example.com?a=2&b=3&c=4&d=5").unwrap();
    /// Mapper::RemoveQueryParams(HashSet::from(["a".to_string()])).apply(&mut JobState::new(&mut url)).unwrap();
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
    /// If the call to [`Url::set_host`] reutrns an error, returns that error.
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
        value: Option<StringSource>
    },
    /// Modifies the specified part of the URL.
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error, that error is returned in a [`MapperError::StringModificationError`].
    /// 
    /// If the call to [`UrlPart::modify`] returns an error, that error is returned in a [`MapperError::UrlPartModifyError`].
    ModifyPart {
        /// The name of the part to modify.
        part: UrlPart,
        /// How exactly to modify the part.
        modification: StringModification
    },
    /// Copies the part specified by `from` to the part specified by `to`.
    /// # Errors
    /// If the part specified by `from` is None and the part specified by `to` cannot be `None` (see [`Mapper::SetPart`]), returns the error [`UrlPartSetError::PartCannotBeNone`].
    CopyPart {
        /// The part to get the value from.
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
    /// This mapper (and all HTTP stuff) only works on non-WASM targets.
    /// This is both because CORS makes this mapper useless and because `reqwest::blocking` does not work on WASM targets.
    /// See [reqwest#891](https://github.com/seanmonstar/reqwest/issues/891) and [reqwest#1068](https://github.com/seanmonstar/reqwest/issues/1068) for details.
    /// 
    /// # Privacy
    /// 
    /// Please note that, by default, this mapper recursively expands short links. If a `t.co` link links to a `bit.ly` link, it'll return the page the `bit.ly` link links to.
    /// However, this means that this mapper will by default send an HTTP GET request to all pages pointed to even if they're not shortlinks.
    /// 
    /// The default config handles this by configuring [`Self::ExpandShortLink::http_client_config_diff`]'s [`HttpClientConfigDiff::redirect_policy`] to `Some(`[`RedirectPolicy::None`]`)`.
    /// And, because it's in a [`Rule::RepeatUntilNonePass`], it still handles recursion up to 10 levels deep while protecting privacy.
    /// # Errors
    #[cfg_attr(feature = "cache", doc = "If the call to [`Params::get_redirect_from_cache`] returns an error, that error is returned.")]
    /// 
    /// If the call to [`Params::http_client`] returns an error, that error is returned.
    /// 
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    /// 
    /// (3xx status code) If the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header is not found, returns the error [`MapperError::HeaderNotFound`].
    /// 
    /// (3xx status code) If the call to [`reqwest::header::HeaderValue::to_str`] to get the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header returns an error, that error is returned.
    /// 
    /// (3xx status code) If the call to [`Url::parse`] to parse the [`Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location) header returns an error, that error is returned.
    /// 
    #[cfg_attr(feature = "cache", doc = "If the call to [`Params::write_redirect_to_cache`] returns an error, that error is returned.")]
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use reqwest::header::HeaderMap;
    /// let mut url = Url::parse("https://t.co/H8IF8DHSFL").unwrap();
    /// Mapper::ExpandShortLink{headers: HeaderMap::default(), http_client_config_diff: None}.apply(&mut JobState::new(&mut url)).unwrap();
    /// assert_eq!(url.as_str(), "https://www.eff.org/deeplinks/2024/01/eff-and-access-now-submission-un-expert-anti-lgbtq-repression");
    /// ```
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    ExpandShortLink {
        /// The headers to send alongside the param's default headers.
        #[serde(default, with = "headermap")]
        headers: HeaderMap,
        /// Rules for how to make the HTTP client.
        #[serde(default)]
        http_client_config_diff: Option<HttpClientConfigDiff>
    },
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`println`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    Println(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`print`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    Eprintln(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`eprintln`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    Print(StringSource),
    /// If [`StringSource::get`] returns `Ok(Some(x))`, [`eprint`]'s `x`.
    /// If it returns `Ok(None)`, doesn't print anything.
    /// Does not change the URL at all.
    /// # Errors
    /// If [`StringSource::get`] returns an error, that error is returned.
    Eprint(StringSource),
    /// Loads a config specified by `path` (or the default config if it's [`None`]) and applies it.
    /// # Errors
    /// If the call to [`Config::get_default_or_load`] returns an error, that error is returned.
    /// 
    /// If the call to [`Config::apply`] returns an error, that error is returned.
    ApplyConfig {
        /// The path of the config to load. Loads the default config if [`None`].
        path: Option<PathBuf>,
        /// How to modify the config's params.
        /// 
        /// Boxed because it's huge.
        params_diff: Box<ParamsDiff>
    },
    /// Sets the current job's `name` string var to `value`.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    SetJobStringVar {
        /// The name of the variable to set.
        name: StringSource,
        /// The value to set the variable to.
        value: StringSource
    },
    /// Delete the current job's `name` string var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    DeleteJobStringVar(StringSource),
    /// Applies a [`StringModification`] to the current job's `name` string var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    ModifyJobStringVar {
        /// The name of the variable to set.
        name: StringSource,
        /// The modification to apply.
        modification: StringModification
    }
}

/// An enum of all possible errors a [`Mapper`] can return.
#[derive(Debug, Error)]
pub enum MapperError {
    /// Returned when [`Mapper::Error`] is used.
    #[error("Mapper::Error was used.")]
    ExplicitError,
    /// Returned when a call to [`UrlPart::get`] returns `None` where it has to return `Some`.
    #[error("The provided URL does not have the requested part.")]
    UrlPartNotFound,
    /// Returned when the provided URL does not contain the requested query parameter.
    #[error("The provided URL does not contain the requested query parameter.")]
    CannotFindQueryParam,
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`reqwest::Error`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// Returned when a [`UrlPartModifyError`] is encountered.
    #[error(transparent)]
    UrlPartModifyError(#[from] UrlPartModifyError),
    /// Returned when a [`UrlPartSetError`] is encountered.
    #[error(transparent)]
    UrlPartSetError(#[from] UrlPartSetError),
    /// Returned when the provided URL does not have a path.
    #[error("The URL does not have a path.")]
    UrlDoesNotHaveAPath,
    /// Returned when a regex does not find any matches.
    #[error("A regex pattern did not find any matches.")]
    NoRegexMatchesFound,
    /// Returned when the requested variable is not found in [`Params::vars`].
    #[error("A variable was requested but not found at runtime.")]
    VarNotFound,
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
    /// Returned when a [`ReadCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadCacheError(#[from] ReadCacheError),
    /// Returned when a [`WriteCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteCacheError(#[from] WriteCacheError),
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
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error("The requested header was not found.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
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
    JobStateStringVarIsNone
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
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), MapperError> {
        #[cfg(feature = "debug")]
        println!("Mapper: {self:?}");
        match self {
            // Testing.

            Self::None => {},
            Self::Error => Err(MapperError::ExplicitError)?,
            Self::Debug(mapper) => {
                let mut old_url = job_state.url.clone();
                let old_job_state = JobState {
                    url: &mut old_url,
                    params: job_state.params,
                    string_vars: job_state.string_vars.clone()
                };
                let mapper_result=mapper.apply(job_state);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nOld job state: {old_job_state:?}\nMapper return value: {mapper_result:?}\nNew job state: {job_state:?}");
                mapper_result?;
            }

            // Logic.

            Self::IfCondition {condition, mapper, else_mapper} => if condition.satisfied_by(job_state)? {
                mapper.apply(job_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(job_state)?;
            },
            Self::All(mappers) => {
                let mut temp_url = job_state.url.clone();
                let mut temp_job_state = JobState {
                    url: &mut temp_url,
                    params: job_state.params,
                    string_vars: job_state.string_vars.clone()
                };
                for mapper in mappers {
                    mapper.apply(&mut temp_job_state)?;
                }
                job_state.string_vars = temp_job_state.string_vars;
                *job_state.url = temp_url;
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
            Self::RemoveQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(job_state.url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => {
                let new_query=form_urlencoded::Serializer::new(String::new()).extend_pairs(job_state.url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                job_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in job_state.url.query_pairs() {
                    if !matcher.satisfied_by(&name, job_state)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::AllowQueryParamsMatching(matcher) => {
                let mut new_query=form_urlencoded::Serializer::new(String::new());
                for (name, value) in job_state.url.query_pairs() {
                    if matcher.satisfied_by(&name, job_state)? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                job_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                match job_state.url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_url)) => {*job_state.url=Url::parse(&new_url)?},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },
            Self::GetPathFromQueryParam(name) => {
                match job_state.url.query_pairs().find(|(param_name, _)| param_name==name) {
                    Some((_, new_path)) => {#[allow(clippy::unnecessary_to_owned)] job_state.url.set_path(&new_path.into_owned());},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => job_state.url.set_host(Some(new_host))?,
            Self::Join(with) => *job_state.url=job_state.url.join(get_str!(with, job_state, MapperError))?,

            // Generic part handling.

            Self::SetPart{part, value} => part.set(job_state.url, get_option_string!(value, job_state).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart{part, modification} => part.modify(modification, job_state)?,
            Self::CopyPart{from, to} => to.set(job_state.url, from.get(job_state.url).map(|x| x.into_owned()).as_deref())?,

            // Miscellaneous.

            #[cfg(all(feature = "http", not(target_family = "wasm")))]
            Self::ExpandShortLink {headers, http_client_config_diff} => {
                #[cfg(feature = "cache-redirects")]
                if let Some(cached_result) = job_state.params.get_redirect_from_cache(job_state.url)? {
                    *job_state.url = cached_result;
                    return Ok(())
                }
                let response = job_state.params.http_client(http_client_config_diff.as_ref())?.get(job_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    Url::parse(response.headers().get("location").ok_or(MapperError::HeaderNotFound)?.to_str()?)?
                } else {
                    response.url().clone()
                };
                #[cfg(feature = "cache-redirects")]
                job_state.params.write_redirect_to_cache(job_state.url, &new_url)?;
                *job_state.url=new_url;
            },

            Self::Println (source) => if let Some(x) = source.get(job_state)? {println! ("{x}");},
            Self::Print   (source) => if let Some(x) = source.get(job_state)? {print!   ("{x}");},
            Self::Eprintln(source) => if let Some(x) = source.get(job_state)? {eprintln!("{x}");},
            Self::Eprint  (source) => if let Some(x) = source.get(job_state)? {eprint!  ("{x}");},
            Self::ApplyConfig {path, params_diff} => {
                let mut config = Config::get_default_or_load(path.as_deref())?.into_owned();
                params_diff.apply(&mut config.params);
                config.apply(job_state.url)?;
            },
            Self::SetJobStringVar {name, value} => {let _ = job_state.string_vars.insert(get_string!(name, job_state, MapperError).to_owned(), get_string!(value, job_state, MapperError).to_owned());},
            Self::DeleteJobStringVar(name) => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let _ = job_state.string_vars.remove(&name);
            },
            Self::ModifyJobStringVar {name, modification} => {
                let name = get_string!(name, job_state, MapperError).to_owned();
                let mut temp = job_state.string_vars.get_mut(&name).ok_or(MapperError::JobStateStringVarIsNone)?.to_owned();
                modification.apply(&mut temp, job_state)?;
                let _ = job_state.string_vars.insert(name, temp);
            }
        };
        Ok(())
    }
}

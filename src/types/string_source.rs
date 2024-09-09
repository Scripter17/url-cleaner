//! Provides [`StringSource`] which allows for getting strings from various parts of URL Cleaner's current state.

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;
use std::env::var;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Allows conditions and mappers to get strings from various sources without requiring different conditions and mappers for each source.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum StringSource {
    // Error handling/prevention.

    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// 
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// If the contained [`Self`] returns `None`, instead return `Some(Cow::Borrowed(""))`
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    NoneToEmptyString(Box<Self>),
    /// If [`Self::NoneTo::source`] returns `None`, instead return the value of [`Self::NoneTo::if_none`].
    /// 
    /// Please note that [`Self::NoneTo::if_none`] can still return [`None`] and does not return an error when it does so.
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    NoneTo {
        /// The [`Self`] to use by default.
        source: Box<Self>,
        /// The [`Self`] to use if [`Self::NoneTo::source`] returns [`None`].
        if_none: Box<Self>
    },

    // Logic.

    /// Joins a list of strings. Effectively a [`slice::join`].
    /// By default, `join` is `""` so the strings are concatenated.
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(
    ///     StringSource::Join {
    ///         sources: vec![
    ///             StringSource::String(".".to_string()),
    ///             StringSource::Part(UrlPart::NotSubdomain)
    ///         ],
    ///         join: "".to_string()
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Owned(".example.com".to_string()))
    /// );
    /// ```
    Join {
        /// The list of string sources to join.
        sources: Vec<Self>,
        /// The value to join `sources` with. Defaults to an empty string.
        #[serde(default, skip_serializing_if = "is_default")]
        join: String
    },
    /// If the flag specified by `flag` is set, return the result of `then`. Otherwise return the result of `r#else`.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// # use std::collections::HashSet;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = url_cleaner::types::Params { flags: vec!["abc".to_string()].into_iter().collect(), ..Default::default() };
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: Box::new("abc".into()),
    ///         then: "abc".into(),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Borrowed("abc"))
    /// );
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: Box::new("xyz".into()),
    ///         then: "xyz".into(),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Borrowed("example.com"))
    /// );
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: Box<Self>,
        /// If the flag is set, use this.
        then: Box<Self>,
        /// If the flag is not set, use this.
        r#else: Box<Self>
    },
    /// If the value of `source` matches `matcher`, returns the value of `then`, otheriwse returns tha velue of `else`.
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringMatcher::satisfied_by`] returns an erorr, that error is returned.
    IfSourceMatches {
        /// The [`Self`] to match on.
        source: Box<Self>,
        /// The matcher.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to return if the matcher passes.
        then: Box<Self>,
        /// The [`Self`] to return if thematcher fails.
        r#else: Box<Self>
    },
    /// If the value of `source` is [`None`], returns the value of `then`, otherwise returns the value of `else`.
    /// # Errors
    /// If any of the calls to [`StringSource::get`] return an error, that error is returned.
    IfSourceIsNone {
        /// The value to check the [`None`]ness of.
        source: Box<Self>,
        /// THe value to return if `source` is [`None`].
        then: Box<Self>,
        /// The value to return if `source` is not [`None`]
        r#else: Box<Self>
    },
    /// Gets the `Option<String>` from [`Self::Map::source`] then, if it exists in [`Self::Map::map`], gets its corresponding [`Self`]'s value.
    /// 
    /// The main benefit of this over [`StringModification::Map`] is this can handle [`None`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If string returned by [`Self::Map::source`] is not in the specified map, returns the error [`StringModificationError::StringNotInMap`].
    Map {
        /// The string to index the map with.
        source: Option<Box<Self>>,
        /// The map to map the string with.
        /// 
        /// God these docs need a total rewrite.
        map: HashMap<Option<String>, Self>,
        /// JSON doesn't allow `null`/[`None`] to be a key in objects.
        /// 
        /// If `source` returns [`None`], there's no [`None`] in `map`, and `if_null` is not [`None`], the [`Self`] in `if_null` is used.
        /// 
        /// Defaults to [`None`].
        #[serde(default)]
        if_null: Option<Box<Self>>,
        /// The [`Self`] to use if the string is not found in `map` and `if_null` isn't used.
        /// 
        /// Defaults to [`None`].
        #[serde(default)]
        r#else: Option<Box<Self>>
    },

    // Basic stuff.

    /// Just a string. The most common variant.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert_eq!(StringSource::String("abc".to_string()).get(&job_state).unwrap(), Some(Cow::Borrowed("abc")));
    /// ```
    String(String),
    /// Gets the specified URL part.
    /// # Errors
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Params::default();
    /// assert_eq!(StringSource::Part(UrlPart::Domain).get(&job_state).unwrap(), Some(Cow::Borrowed("example.com")));
    /// ```
    Part(UrlPart),
    /// Parses `source` as a URL and gets the specified part.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// 
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(
    ///     StringSource::ExtractPart {
    ///         source: "https://example.com".into(),
    ///         part: UrlPart::Scheme
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Borrowed("https"))
    /// );
    /// ```
    ExtractPart {
        /// The string to parse and extract `part` from.
        source: Box<Self>,
        /// The part to extract from `source`.
        part: UrlPart
    },
    /// Indexes [`JobState::common_vars`].
    /// # Errors
    /// If [`JobState::common_vars`] is [`None`], returns the error [`StringSourceError::NotInACommonContext`].
    CommonVar(Box<Self>),
    /// Gets the specified variable's value.
    /// 
    /// Returns [`None`] (NOT an error) if the variable is not set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// # use std::collections::HashMap;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = url_cleaner::types::Params { vars: vec![("abc".to_string(), "xyz".to_string())].into_iter().collect(), ..Default::default() };
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Params {vars: HashMap::from_iter([("abc".to_string(), "xyz".to_string())]), ..Params::default()};
    /// assert_eq!(StringSource::Var("abc".into()).get(&job_state).unwrap(), Some(Cow::Borrowed("xyz")));
    /// ```
    Var(Box<Self>),
    /// Gets the value of the specified [`JobState::vars`].
    /// 
    /// Returns [`None`] (NOT an error) if the string var is not set.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    JobVar(Box<Self>),
    /// Gets the value of the specified [`UrlContext::vars`]
    /// 
    /// Returns [`None`] (NOT an error) if the string var is not set.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    ContextVar(Box<Self>),
    /// Indexes into a [`Params::maps`] using `map` then indexes the returned [`HashMap`] with `key`.
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    MapKey {
        /// The map from [`Params::maps`] to index in.
        map: Box<Self>,
        /// The key to index the map with.
        key: Box<Self>
    },
    /// Gets a string with `source`, modifies it with `modification`, and returns the result.
    /// # Errors
    /// If the call to [`StringModification::apply`] errors, returns that error.
    Modified {
        /// The source to get the string from.
        source: Box<Self>,
        /// The modification to apply to the string.
        modification: Box<StringModification>
    },

    // External state.

    /// Gets the environment variable.
    /// 
    /// If the call to [`std::env::var`] returns the error [`std::env::VarError::NotPresent`], returns [`None`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`std::env::var`] returns the error [`std::env::VarError::NotUnicode`], returns the error [`StringSourceError::EnvVarIsNotUtf8`].
    EnvVar(Box<Self>),
    /// Sends an HTTP request and returns a string from the response determined by the specified [`ResponseHandler`].
    /// # Errors
    /// If the call to [`RequestConfig::response`] returns an error, that error is returned.
    #[cfg(feature = "advanced-requests")]
    HttpRequest(Box<RequestConfig>),
    /// Run a command and return its output.
    /// # Errors
    /// If the call to [`CommandConfig::output`] returns an error, that error is returned.
    #[cfg(feature = "commands")]
    CommandOutput(Box<CommandConfig>),
    /// Read from the cache.
    /// 
    /// If an entry is found, return its value.
    /// 
    /// If an entry is not found, calls [`StringSource::get`], writes its value to the cache, then reutrns it.
    /// 
    /// Please note that [`Self::Cache::category`] and [`Self::Cache::key`] should be chosen to make all possible collisions intentional.
    /// # Errors
    /// If the call to [`CacheHandler::read_from_cache`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`CacheHandler::write_to_cache`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    Cache {
        /// The category to cache in.
        category: Box<Self>,
        /// The key to cache with.
        key: Box<Self>,
        /// The [`Self`] to cache.
        source: Box<Self>
    },
    /// Extracts the substring of `source` found between the first `start` and the first subsequent `end`.
    /// 
    /// The same as [`StringModification::ExtractBetween`] but preserves borrowedness.
    /// 
    /// If `source` returns a [`Cow::Borrowed`], this will also return a [`Cow::Borrowed`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If any call to [`Self::get`] returns [`None`], returns the error [`StringSourceError::StringSourceIsNone`].
    /// 
    /// If `start` is not found in `source`, returns the error [`StringSourceError::ExtractBetweenStartNotFound`].
    /// 
    /// If `end` is not found in `source` after `start`, returns the error [`StringSourceError::ExtractBetweenEndNotFound`].
    ExtractBetween {
        /// The [`Self`] to get a substring from.
        source: Box<Self>,
        /// The [`Self`] to look for before the substring.
        start: Box<Self>,
        /// The [`Self`] to look for after the substring.
        end: Box<Self>
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::string_sources`].
    /// 
    /// Currently does not pass-in [`JobState::vars`] or preserve updates. This will eventually be changed.
    Common {
        /// The name of the [`Self`] to use.
        name: Box<Self>,
        /// The [`JobState::common_vars`] to pass.
        #[serde(default, skip_serializing_if = "is_default")]
        vars: HashMap<String, Self>
    }
}

impl FromStr for StringSource {
    type Err = Infallible;

    /// Returns a [`Self::String`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl From<&str> for StringSource {
    /// Returns a [`Self::String`].
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for StringSource {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Box<StringSource> {
    fn from(value: &str) -> Self {
        Box::new(value.into())
    }
}

impl From<String> for Box<StringSource> {
    fn from(value: String) -> Self {
        Box::new(value.into())
    }
}

crate::util::string_or_struct_magic!(StringSource);

/// The enum of all possible errors [`StringSource::get`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringSourceError {
    /// Returned when [`StringSource::Error`] is used.
    #[error("StringSource::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when [`reqwest::Error`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a requested HTTP response header is not found.
    #[cfg(feature = "http")]
    #[error("The HTTP request response did not contain the requested header.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    HeaderToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a regex does not find any matches.
    #[error("A regex pattern did not find any matches.")]
    #[cfg(feature = "regex")]
    NoRegexMatchesFound,
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`RequestConfigError`] is encountered.
    #[cfg(feature = "advanced-requests")]
    #[error(transparent)]
    RequestConfigError(#[from] RequestConfigError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[cfg(feature = "advanced-requests")]
    #[error(transparent)]
    ReponseHandlerError(#[from] ResponseHandlerError),
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(Box<CommandError>),
    /// Returned when the key is not in the map.
    #[error("The key was not in the map.")]
    KeyNotInMap,
    /// Returned when the provided string is not in the specified map.
    #[error("The provided string was not in the specified map.")]
    StringNotInMap,
    /// Returned when a [`ReadFromCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    /// Returned when a [`WriteToCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    /// Returned when the value of a requested environemnt variable is not UTF-8.
    #[error("The value of the requested environment variable was not UTF-8.")]
    EnvVarIsNotUtf8,
    /// Returned when the requested map is not found.
    #[error("The requested map was not found.")]
    MapNotFound,
    /// Returneed when [`StringSource::Common`] is used outside of a common context.
    #[error("Not in a common context.")]
    NotInACommonContext,
    /// Returned when the `start` of a [`StringSource::ExtractBetween`] is not found in the `source`.
    #[error("The `start` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenStartNotFound,
    /// Returned when the `start` of a [`StringSource::ExtractBetween`] is not found in the `source`.
    #[error("The `end` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenEndNotFound,
    /// Returned when the common [`StringSource`] is not found.
    #[error("The common StringSource was not found.")]
    CommonStringSourceNotFound,
}


#[cfg(feature = "commands")]
impl From<CommandError> for StringSourceError {
    fn from(value: CommandError) -> Self {
        Self::CommandError(Box::new(value))
    }
}

impl From<StringMatcherError> for StringSourceError {
    fn from(value: StringMatcherError) -> Self {
        Self::StringMatcherError(Box::new(value))
    }
}

impl StringSource {
    /// Gets the string from the source.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn get<'a>(&'a self, job_state: &'a JobState) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        debug!(StringSource::get, self, job_state);
        Ok(match self {
            Self::Error => Err(StringSourceError::ExplicitError)?,
            Self::Debug(source) => {
                let ret=source.get(job_state);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nJob state: {job_state:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneToEmptyString(source) => source.get(job_state)?.or(Some(Cow::Borrowed(""))),
            Self::NoneTo {source, if_none} => source.get(job_state).transpose().or_else(|| if_none.get(job_state).transpose()).transpose()?,



            // I love that [`Result`] and [`Option`] implement [`FromIterator`].
            // It's so silly but it works SO well.
            Self::Join {sources, join} => sources.iter().map(|source| source.get(job_state)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            Self::IfFlag {flag, then, r#else} => if job_state.params.flags.contains(&get_string!(flag, job_state, StringSourceError)) {then} else {r#else}.get(job_state)?,
            Self::IfSourceMatches {source, matcher, then, r#else} => {
                if matcher.satisfied_by(get_str!(source, job_state, StringSourceError), job_state)? {
                    then.get(job_state)?
                } else {
                    r#else.get(job_state)?
                }
            },
            Self::IfSourceIsNone {source, then, r#else} => {
                if source.get(job_state)?.is_none() {
                    then.get(job_state)?
                } else {
                    r#else.get(job_state)?
                }
            },
            Self::Map {source, map, if_null, r#else} => {
                let key = get_option_string!(source, job_state);
                match (key.is_none(), map.get(&key), if_null, r#else) {
                    (_   , Some(mapper), _           , _           ) => mapper,
                    (true, _           , Some(mapper), _           ) => mapper,
                    (_   , _           , _           , Some(mapper)) => mapper,
                    _ => Err(StringSourceError::StringNotInMap)?
                }.get(job_state)?
            },



            Self::String(string) => Some(Cow::Borrowed(string.as_str())),
            Self::Part(part) => part.get(job_state.url),
            Self::ExtractPart{source, part} => source.get(job_state)?.map(|url_str| Url::parse(&url_str)).transpose()?.and_then(|url| part.get(&url).map(|part_value| Cow::Owned(part_value.into_owned()))),
            Self::CommonVar(name) => job_state.common_vars.ok_or(StringSourceError::NotInACommonContext)?.get(get_str!(name, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::Var(key) => job_state.params.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::JobVar(key) => job_state.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::ContextVar(key) => job_state.context.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::MapKey {map, key} => job_state.params.maps.get(get_str!(map, job_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(get_str!(key, job_state, StringSourceError)).map(|x| Cow::Borrowed(&**x)),
            Self::Modified {source, modification} => {
                match source.as_ref().get(job_state)? {
                    Some(x) => {
                        let mut x = x.into_owned();
                        modification.apply(&mut x, job_state)?;
                        Some(Cow::Owned(x))
                    },
                    None => None
                }
            },

            // External state.

            Self::EnvVar(name) => {
                match var(get_str!(name, job_state, StringSourceError)) {
                    Ok(value) => Some(Cow::Owned(value)),
                    Err(std::env::VarError::NotPresent) => None,
                    Err(std::env::VarError::NotUnicode(_)) => Err(StringSourceError::EnvVarIsNotUtf8)?
                }
            },
            #[cfg(feature = "advanced-requests")]
            Self::HttpRequest(config) => Some(Cow::Owned(config.response(job_state)?)),
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => Some(Cow::Owned(command.output(job_state)?)),
            #[cfg(feature = "cache")]
            Self::Cache {category, key, source} => {
                let category = get_string!(category, job_state, StringSourceError);
                let key = get_string!(key, job_state, StringSourceError);
                if job_state.params.read_cache {
                    if let Some(ret) = job_state.cache_handler.read_from_cache(&category, &key)? {
                        return Ok(ret.map(Cow::Owned));
                    }
                }
                let ret = source.get(job_state)?;
                if job_state.params.write_cache {
                    job_state.cache_handler.write_to_cache(&category, &key, ret.as_deref())?;
                }
                ret
            },
            Self::ExtractBetween {source, start, end} => {
                Some(match source.get(job_state)?.ok_or(StringSourceError::StringSourceIsNone)? {
                    Cow::Borrowed(x) => Cow::Borrowed(x
                        .split_once(get_str!(start, job_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenStartNotFound)?
                        .1
                        .split_once(get_str!(end, job_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenEndNotFound)?
                        .0),
                    Cow::Owned(x) => Cow::Owned(x
                        .split_once(get_str!(start, job_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenStartNotFound)?
                        .1
                        .split_once(get_str!(end, job_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenEndNotFound)?
                        .0
                        .to_string())
                })
            },
            Self::Common {name, vars} => {
                let common_vars = vars.iter().map(|(k, v)| Ok::<_, StringSourceError>((k.clone(), get_string!(v, job_state, StringSourceError)))).collect::<Result<HashMap<_, _>, _>>()?;
                let mut temp_url = job_state.url.clone();
                job_state.commons.string_sources.get(get_str!(name, job_state, StringSourceError)).ok_or(StringSourceError::CommonStringSourceNotFound)?.get(&JobState {
                    url: &mut temp_url,
                    context: job_state.context,
                    params: job_state.params,
                    vars: Default::default(),
                    #[cfg(feature = "cache")]
                    cache_handler: job_state.cache_handler,
                    commons: job_state.commons,
                    common_vars: Some(&common_vars)
                })?.map(|x| Cow::Owned(x.into_owned()))
            }
        })
    }

    /// Internal method to make sure I don't accidetnally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used, reason = "Private API, but they should be replaced by [`Option::is_none_or`] in 1.82.")]
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        match self {
            Self::NoneToEmptyString(source) => source.is_suitable_for_release(),
            Self::NoneTo {source, if_none} => source.is_suitable_for_release() && if_none.is_suitable_for_release(),
            Self::Join {sources, ..} => sources.iter().all(|source| source.is_suitable_for_release()),
            Self::IfFlag {flag, then, r#else} => flag.is_suitable_for_release() && then.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::IfSourceMatches {source, matcher, then, r#else} => source.is_suitable_for_release() && matcher.is_suitable_for_release() && then.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::IfSourceIsNone {source, then, r#else} => source.is_suitable_for_release() && then.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::Map {source, map, if_null, r#else} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release()) && map.iter().all(|(_, source)| source.is_suitable_for_release()) && (if_null.is_none() || if_null.as_ref().unwrap().is_suitable_for_release()) && (r#else.is_none() || r#else.as_ref().unwrap().is_suitable_for_release()),
            Self::Part(part) => part.is_suitable_for_release(),
            Self::ExtractPart {source, part} => source.is_suitable_for_release() && part.is_suitable_for_release(),
            Self::CommonVar(name) => name.is_suitable_for_release(),
            Self::Var(name) => name.is_suitable_for_release(),
            Self::JobVar(name) => name.is_suitable_for_release(),
            Self::ContextVar(name) => name.is_suitable_for_release(),
            Self::MapKey {map, key} => map.is_suitable_for_release() && key.is_suitable_for_release(),
            Self::Modified {source, modification} => source.is_suitable_for_release() && modification.is_suitable_for_release(),
            Self::EnvVar(name) => name.is_suitable_for_release(),
            #[cfg(feature = "cache")] Self::Cache {category, key, source} => category.is_suitable_for_release() && key.is_suitable_for_release() && source.is_suitable_for_release(),
            Self::Debug(_) => false,
            #[cfg(feature = "commands")]
            Self::CommandOutput(_) => false,
            Self::Error | Self::String(_) => true,
            #[cfg(feature = "advanced-requests")]
            Self::HttpRequest(_) => true,
            Self::ExtractBetween {source, start, end} => source.is_suitable_for_release() && start.is_suitable_for_release() && end.is_suitable_for_release(),
            Self::Common {name, vars} => name.is_suitable_for_release() && vars.iter().all(|(_, v)| v.is_suitable_for_release())
        }
    }
}

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
    /// Just a string. The most common variant.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::borrow::Cow;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
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
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Params::default();
    /// assert_eq!(StringSource::Part(UrlPart::Domain).get(&job_state).unwrap(), Some(Cow::Borrowed("example.com")));
    /// ```
    Part(UrlPart),
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
    /// let params = url_cleaner::types::Params { vars: vec![("abc".to_string(), "xyz".to_string())].into_iter().collect(), ..Default::default() };
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// 
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Params {vars: HashMap::from_iter([("abc".to_string(), "xyz".to_string())]), ..Params::default()};
    /// assert_eq!(StringSource::Var("abc".to_string()).get(&job_state).unwrap(), Some(Cow::Borrowed("xyz")));
    /// ```
    Var(String),
    /// Gets the value of the specified [`JobState::vars`].
    /// 
    /// Returns [`None`] (NOT an error) if the string var is not set.
    JobVar(Box<Self>),
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
    /// let params = url_cleaner::types::Params { flags: vec!["abc".to_string()].into_iter().collect(), ..Default::default() };
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// 
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: "abc".to_string(),
    ///         then: Box::new(StringSource::String("abc".to_string())),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Borrowed("abc"))
    /// );
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: "xyz".to_string(),
    ///         then: Box::new(StringSource::String("xyz".to_string())),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state).unwrap(),
    ///     Some(Cow::Borrowed("example.com"))
    /// );
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: String,
        /// If the flag is set, use this.
        then: Box<Self>,
        /// If the flag is not set, use this.
        r#else: Box<Self>
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
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
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
    /// Parses `source` as a URL and gets the specified value.
    /// Useful when used with [`Self::HttpRequest`].
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
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// 
    /// assert_eq!(
    ///     StringSource::ExtractPart {
    ///         source: Box::new(StringSource::String("https://example.com".to_string())),
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
    /// Sends an HTTP request and returns a string from the response determined by the specified [`ResponseHandler`].
    /// # Errors
    /// If the call to [`RequestConfig::response`] returns an error, that error is returned.
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
    HttpRequest(Box<RequestConfig>),
    /// If the contained [`Self`] returns `None`, instead return `Some(Cow::Borrowed(""))`
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    NoneToEmptyString(Box<Self>),
    /// Run a command and return its output.
    /// # Errors
    /// If the call to [`CommandConfig::output`] returns an error, that error is returned.
    #[cfg(feature = "commands")]
    CommandOutput(Box<CommandConfig>),
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
        map: HashMap<Option<String>, Self>
    },
    /// Read from the cache.
    /// 
    /// If an entry is found, return its value.
    /// 
    /// If an entry is not found, calls [`StringSource::get`], writes its value to the cache, then reutrns it.
    /// 
    /// Please note that [`Self::Cache::key`] should be chosen to make all possible collisions intentional.
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
    /// Gets the environment variable.
    /// 
    /// If the call to [`std::env::var`] returns the error [`std::env::VarError::NotPresent`], returns [`None`].
    /// # Errors
    /// If the call to [`std::env::var`] returns the error [`std::env::VarError::NotUnicode`], returns the error [`StringSourceError::EnvVarIsNotUtf8`].
    EnvVar(Box<Self>)
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

crate::util::string_or_struct_magic!(StringSource);

/// The enum of all possible errors [`StringSource::get`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringSourceError {
    /// Returned when [`StringSource::Error`] is used.
    #[error("StringSource::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when [`reqwest::Error`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a requested HTTP response header is not found.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[error("The HTTP request response did not contain the requested header.")]
    HeaderNotFound,
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
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
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
    #[error(transparent)]
    RequestConfigError(#[from] RequestConfigError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
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
    EnvVarIsNotUtf8
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
        debug!("Source: {self:?}");
        Ok(match self {
            Self::String(x) => Some(Cow::Borrowed(x.as_str())),
            Self::Part(x) => x.get(job_state.url),
            Self::Var(x) => job_state.params.vars.get(x).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobVar(x) => job_state.vars.get(get_str!(x, job_state, StringSourceError)).map(|x| Cow::Borrowed(&**x)),
            Self::IfFlag {flag, then, r#else} => if job_state.params.flags.contains(flag) {then} else {r#else}.get(job_state)?,
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
            // I love that [`Result`] and [`Option`] implement [`FromIterator`].
            // It's so silly but it works SO well.
            Self::Join {sources, join} => sources.iter().map(|source| source.get(job_state)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            // Transpose wouldn't need to exist in a world where `.map(|x| x?)` worked.
            Self::ExtractPart{source, part} => source.get(job_state)?.map(|x| Url::parse(&x)).transpose()?.and_then(|x| part.get(&x).map(|x| Cow::Owned(x.into_owned()))),
            #[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))]
            Self::HttpRequest(config) => Some(Cow::Owned(config.response(job_state)?)),
            Self::NoneToEmptyString(source) => source.get(job_state)?.or(Some(Cow::Borrowed(""))),
            Self::NoneTo {source, if_none} => source.get(job_state).transpose().or_else(|| if_none.get(job_state).transpose()).transpose()?,
            Self::Map {source, map} => map.get(&get_option_string!(source, job_state)).ok_or(StringSourceError::StringNotInMap)?.get(job_state)?,
            Self::Debug(source) => {
                let ret=source.get(job_state);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nJob state: {job_state:?}\nret: {ret:?}");
                ret?
            },
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => Some(Cow::Owned(command.output(job_state)?)),
            Self::Error => Err(StringSourceError::ExplicitError)?,
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
            Self::EnvVar(name) => {
                match var(get_str!(name, job_state, StringSourceError)) {
                    Ok(value) => Some(Cow::Owned(value)),
                    Err(std::env::VarError::NotPresent) => None,
                    Err(std::env::VarError::NotUnicode(_)) => Err(StringSourceError::EnvVarIsNotUtf8)?
                }
            }
        })
    }
}

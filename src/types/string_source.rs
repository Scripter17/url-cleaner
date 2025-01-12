//! Provides [`StringSource`] which allows for getting strings from various parts of a [`JobStateView`].

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
    /// 
    /// Cannot be deserialized as `"Error"` becomes `Self::String("Error".into())`. I think this is less surprising behavior.
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// If the call to [`Self::get`] returns `None`, instead return `Some(Cow::Borrowed(""))`
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    NoneToEmptyString(Box<Self>),
    /// If [`Self::NoneTo::value`] returns `None`, instead return the value of [`Self::NoneTo::if_none`].
    /// 
    /// Please note that [`Self::NoneTo::if_none`] can still return [`None`] and does not return an error when it does so.
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    NoneTo {
        /// The [`Self`] to use by default.
        value: Box<Self>,
        /// The [`Self`] to use if [`Self::NoneTo::value`] returns [`None`].
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
    /// # use std::borrow::Cow;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(
    ///     StringSource::Join {
    ///         sources: vec![
    ///             StringSource::String(".".to_string()),
    ///             StringSource::Part(UrlPart::NotSubdomain)
    ///         ],
    ///         join: "".to_string()
    ///     }.get(&job_state.to_view()).unwrap(),
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
    /// # use std::borrow::Cow;
    /// # use std::collections::HashSet;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// // Putting this in the `job_state!` call doesn't work???`
    /// let params = url_cleaner::types::Params { flags: vec!["abc".to_string()].into_iter().collect(), ..Default::default() };
    /// job_state.params = &params;
    /// 
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: Box::new("abc".into()),
    ///         then: "abc".into(),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state.to_view()).unwrap(),
    ///     Some(Cow::Borrowed("abc"))
    /// );
    /// assert_eq!(
    ///     StringSource::IfFlag {
    ///         flag: Box::new("xyz".into()),
    ///         then: "xyz".into(),
    ///         r#else: Box::new(StringSource::Part(UrlPart::Domain))
    ///     }.get(&job_state.to_view()).unwrap(),
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
    /// If the value of `value` matches `matcher`, returns the value of `then`, otherwise returns the value of `else`.
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    IfSourceMatches {
        /// The [`Self`] to match on.
        value: Box<Self>,
        /// The matcher.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to return if the matcher passes.
        then: Box<Self>,
        /// The [`Self`] to return if the matcher fails.
        r#else: Box<Self>
    },
    /// If the value of `value` is [`None`], returns the value of `then`, otherwise returns the value of `else`.
    /// # Errors
    /// If any of the calls to [`StringSource::get`] return an error, that error is returned.
    IfSourceIsNone {
        /// The value to check the [`None`]ness of.
        value: Box<Self>,
        /// The value to return if `value` is [`None`].
        then: Box<Self>,
        /// The value to return if `value` is not [`None`]
        r#else: Box<Self>
    },
    /// Gets the `Option<String>` from [`Self::Map::value`] then, if it exists in [`Self::Map::map`], gets its corresponding [`Self`]'s value.
    /// 
    /// The main benefit of this over [`StringModification::Map`] is this can handle [`None`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If string returned by [`Self::Map::value`] is not in the specified map, returns the error [`StringModificationError::StringNotInMap`].
    Map {
        /// The string to index the map with.
        value: Box<Self>,
        /// The map to map the string with.
        /// 
        /// God these docs need a total rewrite.
        map: HashMap<String, Self>,
        /// JSON doesn't allow `null`/[`None`] to be a key in objects.
        /// 
        /// If `value` returns [`None`], there's no [`None`] in `map`, and `if_null` is not [`None`], the [`Self`] in `if_null` is used.
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
    /// # use std::borrow::Cow;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(StringSource::String("abc".to_string()).get(&job_state.to_view()).unwrap(), Some(Cow::Borrowed("abc")));
    /// ```
    String(String),
    /// Gets the specified URL part.
    /// # Errors
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(StringSource::Part(UrlPart::Domain).get(&job_state.to_view()).unwrap(), Some(Cow::Borrowed("example.com")));
    /// ```
    Part(UrlPart),
    /// Parses `value` as a URL and gets the specified part.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// 
    /// If the call to [`UrlPart::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(
    ///     StringSource::ExtractPart {
    ///         value: "https://example.com".into(),
    ///         part: UrlPart::Scheme
    ///     }.get(&job_state.to_view()).unwrap(),
    ///     Some(Cow::Borrowed("https"))
    /// );
    /// ```
    ExtractPart {
        /// The string to parse and extract `part` from.
        value: Box<Self>,
        /// The part to extract from `value`.
        part: UrlPart
    },
    /// Indexes [`JobState::common_args`].
    /// # Errors
    /// If [`JobState::common_args`] is [`None`], returns the error [`StringSourceError::NotInACommonContext`].
    CommonVar(Box<Self>),
    /// Gets the specified variable's value.
    /// 
    /// Returns [`None`] (NOT an error) if the variable is not set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use std::borrow::Cow;
    /// # use std::collections::HashMap;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// // Putting this in the `job_state!` call doesn't work???`
    /// let params = Params {vars: HashMap::from_iter([("abc".to_string(), "xyz".to_string())]), ..Params::default()};
    /// job_state.params = &params;
    /// 
    /// assert_eq!(StringSource::Var("abc".into()).get(&job_state.to_view()).unwrap(), Some(Cow::Borrowed("xyz")));
    /// ```
    Var(Box<Self>),
    /// Gets the value of the specified [`JobState::scratchpad`]'s [`JobScratchpad::vars`].
    /// 
    /// Returns [`None`] (NOT an error) if the string var is not set.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    ScratchpadVar(Box<Self>),
    /// Gets the value of the specified [`JobContext::vars`]
    /// 
    /// Returns [`None`] (NOT an error) if the string var is not set.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// 
    ContextVar(Box<Self>),
    /// Indexes into a [`Params::maps`] using `map` then indexes the returned [`HashMap`] with `key`.
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    ParamsMap {
        /// The map from [`Params::maps`] to index in.
        map: Box<Self>,
        /// The key to index the map with.
        key: Box<Self>
    },
    /// Gets a string with `value`, modifies it with `modification`, and returns the result.
    /// # Errors
    /// If the call to [`StringModification::apply`] errors, returns that error.
    Modified {
        /// The [`Self`] get the string from.
        value: Box<Self>,
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
    #[cfg(feature = "http")]
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
    /// If an entry is not found, calls [`StringSource::get`], writes its value to the cache (if it's not an error), then returns it.
    /// 
    /// Please note that [`Self::Cache::category`] and [`Self::Cache::key`] should be chosen to make all possible collisions intentional.
    /// # Errors
    /// If the call to [`Cache::read`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Cache::write`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    Cache {
        /// The category to cache in.
        category: Box<Self>,
        /// The key to cache with.
        key: Box<Self>,
        /// The [`Self`] to cache.
        value: Box<Self>
    },
    /// Extracts the substring of `value` found between the first `start` and the first subsequent `end`.
    /// 
    /// The same as [`StringModification::ExtractBetween`] but preserves borrowedness.
    /// 
    /// If `value` returns a [`Cow::Borrowed`], this will also return a [`Cow::Borrowed`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If any call to [`Self::get`] returns [`None`], returns the error [`StringSourceError::StringSourceIsNone`].
    /// 
    /// If `start` is not found in `value`, returns the error [`StringSourceError::ExtractBetweenStartNotFound`].
    /// 
    /// If `end` is not found in `value` after `start`, returns the error [`StringSourceError::ExtractBetweenEndNotFound`].
    ExtractBetween {
        /// The [`Self`] to get a substring from.
        value: Box<Self>,
        /// The [`Self`] to look for before the substring.
        start: Box<Self>,
        /// The [`Self`] to look for after the substring.
        end: Box<Self>
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::string_sources`].
    Common(CommonCall),
    /// Uses a function pointer.
    /// 
    /// Cannot be serialized or deserialized.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    Custom(FnWrapper<for<'a> fn(&'a JobStateView) -> Result<Option<Cow<'a, str>>, StringSourceError>>)
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
    /// Returns a [`Self::String`].
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Box<StringSource> {
    /// Returns a [`StringSource::String`].
    /// 
    /// Exists for convenience.
    fn from(value: &str) -> Self {
        Box::new(value.into())
    }
}

impl From<String> for Box<StringSource> {
    /// Returns a [`StringSource::String`].
    /// 
    /// Exists for convenience.
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
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    HeaderToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`RequestConfigError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    RequestConfigError(#[from] RequestConfigError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReponseHandlerError(#[from] ResponseHandlerError),
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(Box<CommandError>),
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
    /// Returned when the value of a requested environment variable is not UTF-8.
    #[error("The value of the requested environment variable was not UTF-8.")]
    EnvVarIsNotUtf8,
    /// Returned when the requested map is not found.
    #[error("The requested map was not found.")]
    MapNotFound,
    /// Returned when [`StringSource::Common`] is used outside of a common context.
    #[error("Not in a common context.")]
    NotInACommonContext,
    /// Returned when the `start` of a [`StringSource::ExtractBetween`] is not found in the `value`.
    #[error("The `start` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenStartNotFound,
    /// Returned when the `start` of a [`StringSource::ExtractBetween`] is not found in the `value`.
    #[error("The `end` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenEndNotFound,
    /// Returned when the common [`StringSource`] is not found.
    #[error("The common StringSource was not found.")]
    CommonStringSourceNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Custom error.
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
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
    pub fn get<'a>(&'a self, job_state: &'a JobStateView) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        debug!(StringSource::get, self, job_state);
        Ok(match self {
            Self::String(string) => Some(Cow::Borrowed(string.as_str())),
            Self::Error => Err(StringSourceError::ExplicitError)?,
            Self::Debug(source) => {
                let ret = source.get(job_state);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nJob state: {job_state:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneToEmptyString(value) => value.get(job_state)?.or(Some(Cow::Borrowed(""))),
            Self::NoneTo {value, if_none} => value.get(job_state).transpose().or_else(|| if_none.get(job_state).transpose()).transpose()?,



            // I love that [`Result`] and [`Option`] implement [`FromIterator`].
            // It's so silly but it works SO well.
            Self::Join {sources, join} => sources.iter().map(|value| value.get(job_state)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            Self::IfFlag {flag, then, r#else} => if job_state.params.flags.contains(&get_string!(flag, job_state, StringSourceError)) {then} else {r#else}.get(job_state)?,
            Self::IfSourceMatches {value, matcher, then, r#else} => {
                if matcher.satisfied_by(get_str!(value, job_state, StringSourceError), job_state)? {
                    then.get(job_state)?
                } else {
                    r#else.get(job_state)?
                }
            },
            Self::IfSourceIsNone {value, then, r#else} => {
                if value.get(job_state)?.is_none() {
                    then.get(job_state)?
                } else {
                    r#else.get(job_state)?
                }
            },
            Self::Map {value, map, if_null, r#else} => value.get(job_state)?.map(|x| map.get(&*x)).unwrap_or(if_null.as_deref()).or(r#else.as_deref()).ok_or(StringSourceError::StringNotInMap)?.get(job_state)?,



            Self::Part(part) => part.get(job_state.url),
            Self::ExtractPart{value, part} => value.get(job_state)?.map(|url_str| Url::parse(&url_str)).transpose()?.and_then(|url| part.get_url(&url).map(|part_value| Cow::Owned(part_value.into_owned()))),
            Self::CommonVar(name) => job_state.common_args.ok_or(StringSourceError::NotInACommonContext)?.vars.get(get_str!(name, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::Var(key) => job_state.params.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::ScratchpadVar(key) => job_state.scratchpad.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::ContextVar(key) => job_state.context.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::ParamsMap {map, key} => job_state.params.maps.get(get_str!(map, job_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(get_str!(key, job_state, StringSourceError)).map(|x| Cow::Borrowed(&**x)),
            Self::Modified {value, modification} => {
                match value.as_ref().get(job_state)? {
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
            #[cfg(feature = "http")]
            Self::HttpRequest(config) => Some(Cow::Owned(config.response(job_state)?)),
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => Some(Cow::Owned(command.output(job_state)?)),
            Self::ExtractBetween {value, start, end} => {
                Some(match value.get(job_state)?.ok_or(StringSourceError::StringSourceIsNone)? {
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
            #[cfg(feature = "cache")]
            Self::Cache {category, key, value} => {
                let category = get_string!(category, job_state, StringSourceError);
                let key = get_string!(key, job_state, StringSourceError);
                if job_state.params.read_cache {
                    if let Some(ret) = job_state.cache.read(&category, &key)? {
                        return Ok(ret.map(Cow::Owned));
                    }
                }
                let ret = value.get(job_state)?;
                if job_state.params.write_cache {
                    job_state.cache.write(&category, &key, ret.as_deref())?;
                }
                ret
            },
            Self::Common(common_call) => {
                job_state.commons.string_sources.get(get_str!(common_call.name, job_state, StringSourceError)).ok_or(StringSourceError::CommonStringSourceNotFound)?.get(&JobStateView {
                    url: job_state.url,
                    context: job_state.context,
                    params: job_state.params,
                    scratchpad: job_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: job_state.cache,
                    commons: job_state.commons,
                    common_args: Some(&common_call.args.make(job_state)?)
                })?.map(|x| Cow::Owned(x.into_owned()))
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        })
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(match self {
            Self::NoneToEmptyString(value) => value.is_suitable_for_release(config),
            Self::NoneTo {value, if_none} => value.is_suitable_for_release(config) && if_none.is_suitable_for_release(config),
            Self::Join {sources, ..} => sources.iter().all(|value| value.is_suitable_for_release(config)),
            Self::IfFlag {flag, then, r#else} => flag.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config) && check_docs!(config, flags, flag.as_ref()),
            Self::IfSourceMatches {value, matcher, then, r#else} => value.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::IfSourceIsNone {value, then, r#else} => value.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::Map {value, map, if_null, r#else} => value.is_suitable_for_release(config) && map.iter().all(|(_, value)| value.is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|if_null| if_null.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|r#else| r#else.is_suitable_for_release(config)),
            Self::Part(part) => part.is_suitable_for_release(config),
            Self::ExtractPart {value, part} => value.is_suitable_for_release(config) && part.is_suitable_for_release(config),
            Self::CommonVar(name) => name.is_suitable_for_release(config),
            Self::Var(name) => name.is_suitable_for_release(config) && check_docs!(config, vars, name.as_ref()),
            Self::ScratchpadVar(name) => name.is_suitable_for_release(config),
            Self::ContextVar(name) => name.is_suitable_for_release(config),
            Self::ParamsMap {map, key} => map.is_suitable_for_release(config) && key.is_suitable_for_release(config) && check_docs!(config, maps, map.as_ref()),
            Self::Modified {value, modification} => value.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::EnvVar(name) => name.is_suitable_for_release(config) && check_docs!(config, environment_vars, name.as_ref()),
            #[cfg(feature = "cache")] Self::Cache {category, key, value} => category.is_suitable_for_release(config) && key.is_suitable_for_release(config) && value.is_suitable_for_release(config),
            Self::Debug(_) => false,
            #[cfg(feature = "commands")]
            Self::CommandOutput(_) => false,
            Self::Error | Self::String(_) => true,
            #[cfg(feature = "http")]
            Self::HttpRequest(request_config) => request_config.is_suitable_for_release(config),
            Self::ExtractBetween {value, start, end} => value.is_suitable_for_release(config) && start.is_suitable_for_release(config) && end.is_suitable_for_release(config),
            Self::Common(common_call) => common_call.is_suitable_for_release(config),
            #[cfg(feature = "custom")]
            Self::Custom(_) => false
       }, "Unsuitable StringSource detected: {self:?}");
        true
    }
}

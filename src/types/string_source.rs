//! Dynamically get strings from various part of a [`JobState`].

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;
use std::env::var;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Always returns [`None`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    /// 
    /// assert_eq!(StringSource::None.get(&job_state_view).unwrap(), None);
    /// ```
    #[default]
    None,
    /// Always returns [`StringSourceError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    /// 
    /// StringSource::Error.get(&job_state_view).unwrap_err();
    /// ```
    Error,
    /// If the call to [`Self::get`] returns an error, instead returns [`None`].
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    ///
    /// assert_eq!(StringSource::ErrorToNone(Box::new(StringSource::Error)).get(&job_state_view).unwrap(), None);
    /// ```
    ErrorToNone(Box<Self>),
    /// If the call to [`Self::get`] returns an error, instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    ///
    /// assert_eq!(StringSource::ErrorToEmptyString(Box::new(StringSource::Error)).get(&job_state_view).unwrap(), Some("".into()));
    /// ```
    ErrorToEmptyString(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::get`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both [`Self::TryElse::try`] and [`Self::TryElse::else`]'s calls to [`Self::get`] return an error, [`Self::TryElse::else`]'s error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    ///
    /// assert_eq!(StringSource::TryElse {r#try: Box::new(StringSource::Error), r#else: Box::new(StringSource::None)}.get(&job_state_view).unwrap(), None);
    /// ```
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    /// Print debug info about the contained [`Self`] and its call to [`Self::get`].
    ///
    /// The exact info printed is unspecified and subject to change at any time for any reason.
    /// # Suitability
    /// Always unsuiable to be in the default config.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If the call to [`Self::get`] returns [`None`], instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// let job_state_view = job_state.to_view();
    ///
    /// assert_eq!(StringSource::NoneToEmptyString(Box::new(StringSource::None)).get(&job_state_view).unwrap(), Some("".into()));
    /// ```
    NoneToEmptyString(Box<Self>),
    NoneTo {
        value: Box<Self>,
        if_none: Box<Self>
    },

    // Logic.
    Join {
        sources: Vec<Self>,
        #[serde(default, skip_serializing_if = "is_default")]
        join: String
    },
    IfFlag {
        flag: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    IfScratchpadFlag {
        flag: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    IfCommonFlag {
        flag: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    IfSourceMatches {
        value: Box<Self>,
        matcher: Box<StringMatcher>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    IfSourceIsNone {
        value: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    Map {
        value: Box<Self>,
        #[serde(flatten)]
        map: Map<Self>,
    },

    // Basic stuff.
    String(String),
    Part(UrlPart),
    ExtractPart {
        value: Box<Self>,
        part: UrlPart
    },
    CommonVar(Box<Self>),
    Var(#[suitable(assert = "var_is_documented")] Box<Self>),
    ScratchpadVar(Box<Self>),
    ContextVar(#[suitable(assert = "context_var_is_documented")] Box<Self>),
    JobsContextVar(#[suitable(assert = "jobs_context_var_is_documented")] Box<Self>),
    ParamsMap {
        #[suitable(assert = "map_is_documented")]
        map: Box<Self>,
        key: Box<Self>
    },
    ParamsNamedPartitioning {
        #[suitable(assert = "named_partitioning_is_documented")]
        name: Box<Self>,
        element: Box<Self>
    },
    Modified {
        value: Box<Self>,
        modification: Box<StringModification>
    },

    // External state.
    EnvVar(#[suitable(assert = "env_var_is_documented")] Box<Self>),
    #[cfg(feature = "http")]
    HttpRequest(Box<RequestConfig>),
    #[cfg(feature = "commands")]
    CommandOutput(Box<CommandConfig>),
    #[cfg(feature = "cache")]
    Cache {
        category: Box<Self>,
        key: Box<Self>,
        value: Box<Self>
    },
    ExtractBetween {
        value: Box<Self>,
        start: Box<Self>,
        end: Box<Self>
    },
    #[cfg(feature = "regex")]
    RegexFind {
        value: Box<Self>,
        regex: RegexWrapper
    },
    Common(CommonCall),
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    Custom(FnWrapper<for<'a> fn(&'a JobStateView) -> Result<Option<Cow<'a, str>>, StringSourceError>>)
}

impl FromStr for StringSource {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}

impl From<&str> for StringSource {
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
impl Serialize for StringSource {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_none(),
            _ => StringSource::serialize(self, serializer)
        }
    }
}
impl<'de> Deserialize<'de> for StringSource {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = StringSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expected a string or a map.")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Self::Value::from_str(s).map_err(E::custom)
            }

            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(Self::Value::None)
            }

            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(Self::Value::None)
            }

            fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(V)
    }
}
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringSourceError {
    #[error("StringSource::Error was used.")]
    ExplicitError,
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "http")]
    #[error(transparent)]
    HeaderToStrError(#[from] reqwest::header::ToStrError),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[cfg(feature = "http")]
    #[error(transparent)]
    HttpResponseError(#[from] HttpResponseError),
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReponseHandlerError(#[from] ResponseHandlerError),
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(Box<CommandError>),
    #[error("The provided string was not in the specified map.")]
    StringNotInMap,
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    #[error("The value of the requested environment variable was not UTF-8.")]
    EnvVarIsNotUtf8,
    #[error("The requested map was not found.")]
    MapNotFound,
    #[error("The requested NamedPartitioning was not found.")]
    NamedPartitioningNotFound,
    #[error("Not in a common context.")]
    NotInACommonContext,
    #[error("The `start` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenStartNotFound,
    #[error("The `end` of an `ExtractBetween` was not found in the string.")]
    ExtractBetweenEndNotFound,
    #[error("The common StringSource was not found.")]
    CommonStringSourceNotFound,
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    #[error(transparent)]
    #[cfg(feature = "regex")]
    RegexError(#[from] ::regex::Error),
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
    /// Get the string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'a>(&'a self, job_state: &'a JobStateView) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        debug!(StringSource::get, self, job_state);
        Ok(match self {
            Self::String(string) => Some(Cow::Borrowed(string.as_str())),
            Self::None => None,
            Self::Error => Err(StringSourceError::ExplicitError)?,
            Self::ErrorToNone(value) => value.get(job_state).ok().flatten(),
            Self::ErrorToEmptyString(value) => value.get(job_state).unwrap_or(Some(Cow::Borrowed(""))),
            Self::TryElse{r#try, r#else} => r#try.get(job_state).or_else(|_| r#else.get(job_state))?,
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
            Self::IfScratchpadFlag {flag, then, r#else} => if job_state.scratchpad.flags.contains(&get_string!(flag, job_state, StringSourceError)) {then} else {r#else}.get(job_state)?,
            Self::IfCommonFlag     {flag, then, r#else} => if job_state.common_args.ok_or(StringSourceError::NotInACommonContext)?.flags.contains(&get_cow!(flag, job_state, StringSourceError)) {then} else {r#else}.get(job_state)?,
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
            Self::Map {value, map} => map.get(value.get(job_state)?).ok_or(StringSourceError::StringNotInMap)?.get(job_state)?,



            Self::Part(part) => part.get(job_state.url),
            Self::ExtractPart{value, part} => value.get(job_state)?.map(|url_str| BetterUrl::parse(&url_str)).transpose()?.and_then(|url| part.get(&url).map(|part_value| Cow::Owned(part_value.into_owned()))),
            Self::CommonVar(name) => job_state.common_args.ok_or(StringSourceError::NotInACommonContext)?.vars.get(get_str!(name, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::Var(key) => job_state.params.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(value.as_str())),
            Self::ScratchpadVar(key) => job_state.scratchpad.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::ContextVar(key) => job_state.context.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::JobsContextVar(key) => job_state.jobs_context.vars.get(get_str!(key, job_state, StringSourceError)).map(|value| Cow::Borrowed(&**value)),
            Self::ParamsMap {map, key} => job_state.params.maps.get(get_str!(map, job_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(key.get(job_state)?).map(|x| Cow::Borrowed(&**x)),
            Self::ParamsNamedPartitioning {name, element} => job_state.params.named_partitionings
                .get(get_str!(name, job_state, StringSourceError)).ok_or(StringSourceError::NamedPartitioningNotFound)?
                .get_partition(get_str!(element, job_state, StringSourceError)).map(Cow::Borrowed),
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
            #[cfg(feature = "regex")]
            Self::RegexFind {value, regex} => match value.get(job_state)?.ok_or(StringSourceError::StringSourceIsNone)? {
                Cow::Owned   (value) => regex.get()?.find(&value).map(|x| Cow::Owned   (x.as_str().to_string())),
                Cow::Borrowed(value) => regex.get()?.find( value).map(|x| Cow::Borrowed(x.as_str()))
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
                    common_args: Some(&common_call.args.build(job_state)?),
                    jobs_context: job_state.jobs_context
                })?.map(|x| Cow::Owned(x.into_owned()))
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        })
    }
}

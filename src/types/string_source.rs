//! Dynamically get strings from various part of a [`TaskState`].

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// # Implementation details
/// - Every contained [`Self`] is only ever called at most once per invocation.
/// # Terminology
/// "The value of {x}" and "{x}'s call to [`Self::get`]" are used interchangeably.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Always returns [`None`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    /// 
    /// assert_eq!(StringSource::None.get(&task_state_view).unwrap(), None);
    /// ```
    #[default]
    None,
    /// Always returns [`StringSourceError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    /// 
    /// StringSource::Error.get(&task_state_view).unwrap_err();
    /// ```
    Error,
    /// If the call to [`Self::get`] returns an error, instead returns [`None`].
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::ErrorToNone(Box::new(StringSource::Error)).get(&task_state_view).unwrap(), None);
    /// ```
    ErrorToNone(Box<Self>),
    /// If the call to [`Self::get`] returns an error, instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::ErrorToEmptyString(Box::new(StringSource::Error)).get(&task_state_view).unwrap(), Some("".into()));
    /// ```
    ErrorToEmptyString(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::get`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both [`Self::TryElse::try`] and [`Self::TryElse::else`]'s calls to [`Self::get`] return an error, [`Self::TryElse::else`]'s error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::TryElse {r#try: Box::new(StringSource::Error), r#else: Box::new(StringSource::None)}.get(&task_state_view).unwrap(), None);
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
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::NoneToEmptyString(Box::new(StringSource::None)).get(&task_state_view).unwrap(), Some("".into()));
    /// ```
    NoneToEmptyString(Box<Self>),
    /// If [`Self::NoneTo::value`]'s call to [`Self::get`] returns [`None`], returns the value of [`Self::NoneTo::if_none`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::NoneTo {
    ///     value: Box::new(StringSource::None),
    ///     if_none: Box::new(StringSource::String("none".to_string()))
    /// }.get(&task_state_view).unwrap(), Some("none".into()));
    ///
    /// assert_eq!(StringSource::NoneTo {
    ///     value: Box::new(StringSource::String("not none".to_string())),
    ///     if_none: Box::new(StringSource::String("none".to_string()))
    /// }.get(&task_state_view).unwrap(), Some("not none".into()));
    /// ```
    NoneTo {
        /// The value to get.
        value: Box<Self>,
        /// The value to return if [`Self::NoneTo::value`] is [`None`].
        if_none: Box<Self>
    },

    /// Joins a list of [`Self`]s delimited by [`Self::Join::join`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, the error is returned.
    ///
    /// If any call to [`Self::get`] returns [`None`], returns the error [`StringSourceError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state;);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::Join {
    ///     sources: vec![StringSource::String("abc".to_string()), StringSource::String("def".to_string())],
    ///     join: "/".to_string()
    /// }.get(&task_state_view).unwrap(), Some("abc/def".into()));
    /// ```
    Join {
        /// The values to join the values of with [`Self::Join::join`].
        sources: Vec<Self>,
        /// The string to join the values of [`Self::Join::sources`].
        ///
        /// Defaults to the empty string.
        #[serde(default, skip_serializing_if = "is_default")]
        join: String
    },
    /// If the [`Params::flags`] specified by [`Self::IfFlag::flag`] is set, return the value of [`Self::IfFlag::then`]. If it's not set, return the value of [`Self::IfFlag::else`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::IfFlag::flag`]'s call to [`Self::get`] returns an error, returns the error [`StringSourceError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state; params = Params {flags: ["abc".to_string()].into(), ..Default::default()};);
    /// let task_state_view = task_state.to_view();
    ///
    /// assert_eq!(StringSource::IfFlag {
    ///     flag: Box::new(FlagRef {r#type: FlagType::Params, name: StringSource::String("abc".to_string())}),
    ///     then: Box::new(StringSource::String("set!".to_string())),
    ///     r#else: Box::new(StringSource::String("unset".to_string()))
    /// }.get(&task_state_view).unwrap(), Some("set!".into()));
    ///
    /// assert_eq!(StringSource::IfFlag {
    ///     flag: Box::new(FlagRef {r#type: FlagType::Params, name: StringSource::String("def".to_string())}),
    ///     then: Box::new(StringSource::String("set!".to_string())),
    ///     r#else: Box::new(StringSource::String("unset".to_string()))
    /// }.get(&task_state_view).unwrap(), Some("unset".into()));
    /// ```
    IfFlag {
        /// The name of the flag to check.
        #[serde(flatten)]
        flag: Box<FlagRef>,
        /// The value to return if the flag is set.
        then: Box<Self>,
        /// The value to return if the flag is unset.
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
    Var(Box<VarRef>),
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
    Custom(FnWrapper<for<'a> fn(&'a TaskStateView) -> Result<Option<Cow<'a, str>>, StringSourceError>>)
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
    Custom(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    GetFlagError(#[from] GetFlagError),
    #[error(transparent)]
    GetVarError(#[from] GetVarError)
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
    pub fn get<'a>(&'a self, task_state: &'a TaskStateView) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        debug!(StringSource::get, self, task_state);
        Ok(match self {
            Self::String(string) => Some(Cow::Borrowed(string.as_str())),
            Self::None => None,
            Self::Error => Err(StringSourceError::ExplicitError)?,
            Self::ErrorToNone(value) => value.get(task_state).ok().flatten(),
            Self::ErrorToEmptyString(value) => value.get(task_state).unwrap_or(Some(Cow::Borrowed(""))),
            Self::TryElse{r#try, r#else} => r#try.get(task_state).or_else(|_| r#else.get(task_state))?,
            Self::Debug(source) => {
                let ret = source.get(task_state);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\ntask_state: {task_state:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneToEmptyString(value) => value.get(task_state)?.or(Some(Cow::Borrowed(""))),
            Self::NoneTo {value, if_none} => value.get(task_state).transpose().or_else(|| if_none.get(task_state).transpose()).transpose()?,



            // I love that [`Result`] and [`Option`] implement [`FromIterator`].
            // It's so silly but it works SO well.
            Self::Join {sources, join} => sources.iter().map(|value| value.get(task_state)).collect::<Result<Option<Vec<_>>, _>>()?.map(|x| Cow::Owned(x.join(join))),
            Self::IfFlag {flag, then, r#else} => if flag.get(task_state)? {then} else {r#else}.get(task_state)?,
            Self::IfSourceMatches {value, matcher, then, r#else} => {
                if matcher.satisfied_by(get_str!(value, task_state, StringSourceError), task_state)? {
                    then.get(task_state)?
                } else {
                    r#else.get(task_state)?
                }
            },
            Self::IfSourceIsNone {value, then, r#else} => {
                if value.get(task_state)?.is_none() {
                    then.get(task_state)?
                } else {
                    r#else.get(task_state)?
                }
            },
            Self::Map {value, map} => map.get(value.get(task_state)?).ok_or(StringSourceError::StringNotInMap)?.get(task_state)?,



            Self::Part(part) => part.get(task_state.url),
            Self::ExtractPart{value, part} => value.get(task_state)?.map(|url_str| BetterUrl::parse(&url_str)).transpose()?.and_then(|url| part.get(&url).map(|part_value| Cow::Owned(part_value.into_owned()))),
            Self::Var(var_ref) => var_ref.get(task_state)?,
            Self::ParamsMap {map, key} => task_state.params.maps.get(get_str!(map, task_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(key.get(task_state)?).map(|x| Cow::Borrowed(&**x)),
            Self::ParamsNamedPartitioning {name, element} => task_state.params.named_partitionings
                .get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::NamedPartitioningNotFound)?
                .get_partition(get_str!(element, task_state, StringSourceError)).map(Cow::Borrowed),
            Self::Modified {value, modification} => {
                match value.as_ref().get(task_state)? {
                    Some(x) => {
                        let mut x = x.into_owned();
                        modification.apply(&mut x, task_state)?;
                        Some(Cow::Owned(x))
                    },
                    None => None
                }
            },
            #[cfg(feature = "regex")]
            Self::RegexFind {value, regex} => match value.get(task_state)?.ok_or(StringSourceError::StringSourceIsNone)? {
                Cow::Owned   (value) => regex.get()?.find(&value).map(|x| Cow::Owned   (x.as_str().to_string())),
                Cow::Borrowed(value) => regex.get()?.find( value).map(|x| Cow::Borrowed(x.as_str()))
            },

            // External state.

            #[cfg(feature = "http")]
            Self::HttpRequest(config) => Some(Cow::Owned(config.response(task_state)?)),
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => Some(Cow::Owned(command.output(task_state)?)),
            Self::ExtractBetween {value, start, end} => {
                Some(match value.get(task_state)?.ok_or(StringSourceError::StringSourceIsNone)? {
                    Cow::Borrowed(x) => Cow::Borrowed(x
                        .split_once(get_str!(start, task_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenStartNotFound)?
                        .1
                        .split_once(get_str!(end, task_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenEndNotFound)?
                        .0),
                    Cow::Owned(x) => Cow::Owned(x
                        .split_once(get_str!(start, task_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenStartNotFound)?
                        .1
                        .split_once(get_str!(end, task_state, StringSourceError))
                        .ok_or(StringSourceError::ExtractBetweenEndNotFound)?
                        .0
                        .to_string())
                })
            },
            #[cfg(feature = "cache")]
            Self::Cache {category, key, value} => {
                let category = get_string!(category, task_state, StringSourceError);
                let key = get_string!(key, task_state, StringSourceError);
                if task_state.params.read_cache {
                    if let Some(ret) = task_state.cache.read(&category, &key)? {
                        return Ok(ret.map(Cow::Owned));
                    }
                }
                let ret = value.get(task_state)?;
                if task_state.params.write_cache {
                    task_state.cache.write(&category, &key, ret.as_deref())?;
                }
                ret
            },
            Self::Common(common_call) => {
                task_state.commons.string_sources.get(get_str!(common_call.name, task_state, StringSourceError)).ok_or(StringSourceError::CommonStringSourceNotFound)?.get(&TaskStateView {
                    url: task_state.url,
                    context: task_state.context,
                    params: task_state.params,
                    scratchpad: task_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: task_state.cache,
                    commons: task_state.commons,
                    common_args: Some(&common_call.args.build(task_state)?),
                    job_context: task_state.job_context
                })?.map(|x| Cow::Owned(x.into_owned()))
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

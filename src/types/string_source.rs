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
///
/// - All [`StringSource`]s are expected to be deterministic outside of errors. This property is relied on for [`Self::Cache`].
/// # Terminology
/// "The value of {x}" and "{x}'s call to [`Self::get`]" are used interchangeably.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Always returns [`None`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    /// 
    /// assert_eq!(StringSource::None.get(&task_state).unwrap(), None);
    /// ```
    #[default]
    None,
    /// Always returns [`StringSourceError::ExplicitError`] with the included error.
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    /// 
    /// StringSource::Error("Message".into()).get(&task_state).unwrap_err();
    /// ```
    Error(String),
    /// If the call to [`Self::get`] returns an error, instead returns [`None`].
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ErrorToNone(Box::new(StringSource::Error("Message".into()))).get(&task_state).unwrap(), None);
    /// ```
    ErrorToNone(Box<Self>),
    /// If the call to [`Self::get`] returns an error, instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ErrorToEmptyString(Box::new(StringSource::Error("Message".into()))).get(&task_state).unwrap(), Some("".into()));
    /// ```
    ErrorToEmptyString(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::get`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both [`Self::TryElse::try`] and [`Self::TryElse::else`]'s calls to [`Self::get`] return an error, [`Self::TryElse::else`]'s error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::TryElse {r#try: Box::new(StringSource::Error("Message".into())), r#else: Box::new(StringSource::None)}.get(&task_state).unwrap(), None);
    /// ```
    TryElse {
        /// The value to try to get.
        ///
        /// If it's an error, return the value of [`Self::TryElse::else`]
        r#try: Box<Self>,
        /// The value to return if [`Self::TryElse::try`] is an error.
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
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::NoneToEmptyString(Box::new(StringSource::None)).get(&task_state).unwrap(), Some("".into()));
    /// ```
    NoneToEmptyString(Box<Self>),
    /// If [`Self::NoneTo::value`]'s call to [`Self::get`] returns [`None`], returns the value of [`Self::NoneTo::if_none`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::NoneTo {
    ///     value: Box::new(StringSource::None),
    ///     if_none: Box::new("none".into())
    /// }.get(&task_state).unwrap(), Some("none".into()));
    ///
    /// assert_eq!(StringSource::NoneTo {
    ///     value: Box::new("not none".into()),
    ///     if_none: Box::new("none".into())
    /// }.get(&task_state).unwrap(), Some("not none".into()));
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
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::Join {
    ///     sources: vec!["abc".into(), "def".into()],
    ///     join: "/".into()
    /// }.get(&task_state).unwrap(), Some("abc/def".into()));
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
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, params = Params {flags: ["abc".into()].into(), ..Default::default()});
    ///
    /// assert_eq!(StringSource::IfFlag {
    ///     flag: Box::new(FlagRef {r#type: FlagType::Params, name: "abc".into()}),
    ///     then: Box::new("set!".into()),
    ///     r#else: Box::new("unset".into())
    /// }.get(&task_state).unwrap(), Some("set!".into()));
    ///
    /// assert_eq!(StringSource::IfFlag {
    ///     flag: Box::new(FlagRef {r#type: FlagType::Params, name: "def".into()}),
    ///     then: Box::new("set!".into()),
    ///     r#else: Box::new("unset".into())
    /// }.get(&task_state).unwrap(), Some("unset".into()));
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
    /// Gets the value of [`Self::IfStringMatches::value`] then, if it satisfies [`Self::IfStringMatches::matcher`], returns the value of [`Self::IfStringMatches::then`].
    /// If it doesn't match, returns the value of [`Self::IfStringMatches::else`]
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::IfStringMatches`]'s callt to [`Self::get`] returns [`None`], returns the errorr [`StringSourceError::StringSourceIsNone`].
    ///
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::IfStringMatches {
    ///     value  : Box::new("abc".into()),
    ///     matcher: Box::new(StringMatcher::Is("abc".into())),
    ///     then   : Box::new("matches".into()),
    ///     r#else : Box::new("doesn't match".into())
    /// }.get(&task_state).unwrap(), Some("matches".into()));
    ///
    /// assert_eq!(StringSource::IfStringMatches {
    ///     value  : Box::new("def".into()),
    ///     matcher: Box::new(StringMatcher::Is("abc".into())),
    ///     then   : Box::new("matches".into()),
    ///     r#else : Box::new("doesn't match".into())
    /// }.get(&task_state).unwrap(), Some("doesn't match".into()));
    /// ```
    IfStringMatches {
        /// The value to match.
        value: Box<Self>,
        /// The matcher to match [`Self::IfStringMatches::value`].
        matcher: Box<StringMatcher>,
        /// The value to return if [`Self::IfStringMatches::matcher`] passes.
        then: Box<Self>,
        /// The value to return if [`Self::IfStringMatches::matcher`] fails.
        r#else: Box<Self>
    },
    /// If the value of [`Self::IfStringIsNone::value`] is [`None`], returns the value of [`Self::IfValueIsNone::then`].
    /// If it's [`Some`], returns the value of [`Self::IfStringIsNone::else`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::IfStringIsNone {
    ///     value : Box::new(StringSource::None),
    ///     then  : Box::new("none".into()),
    ///     r#else: Box::new("some".into())
    /// }.get(&task_state).unwrap(), Some("none".into()));
    ///
    /// assert_eq!(StringSource::IfStringIsNone {
    ///     value : Box::new("some value. it's not returned".into()),
    ///     then  : Box::new("none".into()),
    ///     r#else: Box::new("some".into())
    /// }.get(&task_state).unwrap(), Some("some".into()));
    /// ```
    IfStringIsNone {
        /// The value whose [`None`]ness to check.
        value: Box<Self>,
        /// The value to return if [`Self::IfStringIsNone::value`] returns [`None`].
        then: Box<Self>,
        /// The value to return if [`Self::IfStringIsNone::value`] returns [`Some`].
        r#else: Box<Self>
    },
    /// Gets the value of [`Self::Map::value`] then uses it to index [`Self::Map::map`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// let map = Map {
    ///     map    : [("abc".into(), "def".into())].into(),
    ///     if_null: Some(Box::new("was none".into())),
    ///     r#else : Some(Box::new("wasn't abc or none".into()))
    /// };
    ///
    /// assert_eq!(StringSource::Map {
    ///     value: Box::new("abc".into()),
    ///     map: map.clone()
    /// }.get(&task_state).unwrap(), Some("def".into()));
    ///
    /// assert_eq!(StringSource::Map {
    ///     value: Box::new("else".into()),
    ///     map: map.clone()
    /// }.get(&task_state).unwrap(), Some("wasn't abc or none".into()));
    ///
    /// assert_eq!(StringSource::Map {
    ///     value: Box::new(StringSource::None),
    ///     map: map.clone()
    /// }.get(&task_state).unwrap(), Some("was none".into()));
    /// ```
    Map {
        /// The value to index [`Self::Map::map`] with.
        value: Box<Self>,
        /// The [`Map`] to index with [`Self::Map::value`].
        #[serde(flatten)]
        map: Map<Self>,
    },
    /// Return a referecnce to the contained [`String`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::String("abc".into()).get(&task_state).unwrap(), Some("abc".into()));
    /// ```
    String(String),
    /// Returns the value of the specified [`UrlPart`] of the [`TaskStateView::url`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert_eq!(StringSource::Part(UrlPart::Host).get(&task_state).unwrap(), Some("example.com".into()));
    /// ```
    Part(UrlPart),
    /// Parses [`Self::ExtractPart`] as a [`BetterUrl`] and returns the part specified by [`Self::ExtractPart::part`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ExtractPart {
    ///     value: Box::new("https://example.com".into()),
    ///     part: UrlPart::Host
    /// }.get(&task_state).unwrap(), Some("example.com".into()));
    /// ```
    ExtractPart {
        /// The [`BetterUrl`] to get [`Self::ExtractPart::part`] from.
        value: Box<Self>,
        /// The [`UrlPart`] to get from [`Self::ExtractPart::value`].
        part: UrlPart
    },
    /// Gets the var specified by the [`VarRef`].
    ///
    /// Can by any type of var supported by [`VarType`].
    /// # Errors
    /// If the call to [`VarRef::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, params = Params {
    ///     vars: [("abc".into(), "def".into())].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert_eq!(StringSource::Var(Box::new(VarRef {
    ///     r#type: VarType::Params,
    ///     name: "abc".into()
    /// })).get(&task_state).unwrap(), Some("def".into()));
    /// ```
    Var(Box<VarRef>),
    /// Gets the [`Map`] specified by [`Self::ParamsMap::map`] from [`Params::maps`] then indexes it with [`Self::ParamsMap::key`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    /// 
    /// If [`Self::ParamsMap::name`]'s call to [`Self::get`] returns [`None`], returns the error [`StringSourceError::MapNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, params = Params {
    ///     maps: [("map_name".into(), Map {
    ///         map    : [("abc".into(), "def".into())].into(),
    ///         if_null: Some(Box::new("was none".into())),
    ///         r#else : Some(Box::new("wasn't abc or none".into()))
    ///     })].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert_eq!(StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new("abc".into())
    /// }.get(&task_state).unwrap(), Some("def".into()));
    ///
    /// assert_eq!(StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new("else".into())
    /// }.get(&task_state).unwrap(), Some("wasn't abc or none".into()));
    ///
    /// assert_eq!(StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new(StringSource::None)
    /// }.get(&task_state).unwrap(), Some("was none".into()));
    /// ```
    ParamsMap {
        /// The name of the [`ParamsMap::name`] to index.
        #[suitable(assert = "map_is_documented")]
        name: Box<Self>,
        /// The value to index the [`Map`] with.
        key: Box<Self>
    },
    /// Gets the [`NamedPartitioning`] specified by [`Self::ParamsNamedPartitioning::name`] from [`Params::named_partitionings`] then gets the name of the partition containing [`Self::ParamsNamedPartitioning::element`].
    /// # Errors
    /// If either call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::NamedPartitionings::name`]'s call to [`Self::get`] returns [`None`], returns the error [`StringSourceError::NamedPartitioningNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, params = Params {
    ///     named_partitionings: [
    ///         (
    ///             "thing".into(),
    ///             NamedPartitioning::try_from_iter([
    ///                 ("abc".into(), vec!["a".into(), "b".into(), "c".into()]),
    ///                 ("def".into(), vec!["d".into(), "e".into(), "f".into()])
    ///             ]).unwrap()
    ///         )
    ///     ].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert_eq!(StringSource::ParamsNamedPartitioning {
    ///     name: Box::new("thing".into()),
    ///     element: Box::new("a".into())
    /// }.get(&task_state).unwrap(), Some("abc".into()));
    /// ```
    ParamsNamedPartitioning {
        /// The name of the [`Params::named_partitioning`] to index.
        #[suitable(assert = "named_partitioning_is_documented")]
        name: Box<Self>,
        /// The element whose partition to get the name of.
        element: Box<Self>
    },
    /// Gets the value of [`Self::Modified::value`] then applies [`Self::Modified::modification`].
    ///
    /// If the value of [`Self::Modified::value`] is [`None`], simply returns [`None`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::Modified {
    ///     value: Box::new("abc".into()),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }.get(&task_state).unwrap(), Some("ABC".into()));
    ///
    /// assert_eq!(StringSource::Modified {
    ///     value: Box::new(StringSource::None),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }.get(&task_state).unwrap(), None);
    /// ```
    Modified {
        /// The value to get and modify.
        value: Box<Self>,
        /// The modification to apply to [`Self::Modified::value`].
        modification: Box<StringModification>
    },

    /// Calls [`RequestConfig::response`] and returns the value.
    /// # Errors
    /// If the call to [`RequestConfig::response`] returns an error, that error is returned.
    #[cfg(feature = "http")]
    HttpRequest(Box<RequestConfig>),
    /// Calls [`CommandConfig::response`] and returns the value.
    /// # Errors
    /// If the call to [`CommandConfig::response`] returns an error, that error is returned.
    #[cfg(feature = "commands")]
    CommandOutput(Box<CommandConfig>),
    /// If [`TaskStateView::cache`] has an entry for [`Self::Cache::category`] and [`Self::Cache::key`], returns its value.
    ///
    /// If the cache doesn't have a matching entry, gets the value of [`Self::Cache::value`] and inserts it into the cache.
    ///
    /// If the value of [`Self::Cache::value`] is [`None`], the value is still inserted into the cache.
    ///
    /// If the value of [`Self::Cache::value`] is an error, nothing is cached.
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::read`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::write`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    Cache {
        /// The category of the thing to cache.
        category: Box<Self>,
        /// The key of the thing thing to cache.
        key: Box<Self>,
        /// The value to cache.
        value: Box<Self>
    },
    /// Gets the value of [`Self::ExtractBetween::value`] and gets the substring after the first instance of [`Self::ExtractBetween::start`] and the first subsequent instance of [`Self::ExtractBetween::end`].
    /// # Errors
    /// If any call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::ExtractBetween::start`] isn't found in [`Self::ExtractBetween::value`], returns the error [`StringSourceError::ExtractBetweenStartNotFound`].
    ///
    /// If [`Self::ExtractBetween::end`] isn't found in [`Self::ExtractBetween::value`], returns the error [`StringSourceError::ExtractBetweenEndNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ExtractBetween {
    ///     value: Box::new("abcaaba".into()),
    ///     start: Box::new("b".into()),
    ///     end  : Box::new("a".into())
    /// }.get(&task_state).unwrap(), Some("c".into()));
    /// ```
    ExtractBetween {
        /// The value to extract part of.
        value: Box<Self>,
        /// The value before the substring to get from [`Self::ExtractBetween::value`].
        start: Box<Self>,
        /// The value after the substring to get from [`Self::ExtractBetween::value`].
        end: Box<Self>
    },
    /// Gets the value of [`Self::RegexFind::value`] and calls [`Regex::find`] on it.
    ///
    /// If the value of [`Self::RegexFind::value`] is [`None`], simply returns [`None`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use url_cleaner::types::*;
    /// use url_cleaner::glue::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::RegexFind {
    ///     value: Box::new("<a><b><c>".into()),
    ///     regex: RegexWrapper::from_str("<[^aeiou]>").unwrap()
    /// }.get(&task_state).unwrap(), Some("<b>".into()));
    /// ```
    #[cfg(feature = "regex")]
    RegexFind {
        /// The value to get a substring of.
        value: Box<Self>,
        /// The [`Regex`] to use to find the substring.
        regex: RegexWrapper
    },
    /// Calls a [`Self`] from [`Params::commons`]'s [`Commons::string_sources`].
    /// # Errors
    /// If [`CommonCall::name`]'s call to [`Self::get`] returns an error, returns the error [`StringSourceError::StringSourceIsNone`].
    ///
    /// If [`Params::common`]'s [`Commons::string_sources`] doesn't contain a [`Self`] with the specified name, returns the error [`StringSourceError::CommonStringSourceNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, commons = Commons {
    ///     string_sources: [
    ///         ("abc".into(), "def".into()),
    ///         ("def".into(), StringSource::Var(Box::new(VarRef {
    ///             r#type: VarType::Common,
    ///             name: "common_var".into()
    ///         })))
    ///     ].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert_eq!(StringSource::Common(CommonCall {
    ///     name: Box::new("abc".into()),
    ///     args: Default::default()
    /// }).get(&task_state).unwrap(), Some("def".into()));
    ///
    /// assert_eq!(StringSource::Common(CommonCall {
    ///     name: Box::new("def".into()),
    ///     args: Default::default()
    /// }).get(&task_state).unwrap(), None);
    ///
    /// assert_eq!(StringSource::Common(CommonCall {
    ///     name: Box::new("def".into()),
    ///     args: CommonCallArgsSource {
    ///         vars: [("common_var".into(), "ghi".into())].into(),
    ///         ..Default::default()
    ///     }
    /// }).get(&task_state).unwrap(), Some("ghi".into()));
    /// ```
    Common(CommonCall),
    /// Calls the contained function and returns what it does.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner::types::*;
    /// use url_cleaner::glue::*;
    /// url_cleaner::task_state_view!(task_state);
    ///
    /// fn some_complex_operation<'a>(task_state: &'a TaskStateView) -> Result<Option<Cow<'a, str>>, StringSourceError> {
    ///     Ok(Some("a".into()))
    /// }
    /// 
    /// assert_eq!(StringSource::Custom(some_complex_operation).get(&task_state).unwrap(), Some("a".into()));
    /// ```
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(for<'a> fn(&'a TaskStateView) -> Result<Option<Cow<'a, str>>, StringSourceError>)
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
                f.write_str("Expected a string, a map, or null.")
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
    #[error("Explicit error: {0}")]
    ExplicitError(String),
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
            Self::Error(msg) => Err(StringSourceError::ExplicitError(msg.clone()))?,
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
            Self::IfStringMatches {value, matcher, then, r#else} => {
                if matcher.satisfied_by(get_str!(value, task_state, StringSourceError), task_state)? {
                    then.get(task_state)?
                } else {
                    r#else.get(task_state)?
                }
            },
            Self::IfStringIsNone {value, then, r#else} => {
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
            Self::ParamsMap {name, key} => task_state.params.maps.get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(key.get(task_state)?).map(|x| Cow::Borrowed(&**x)),
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
            Self::RegexFind {value, regex} => match value.get(task_state)? {
                Some(Cow::Owned   (value)) => regex.get()?.find(&value).map(|x| Cow::Owned   (x.as_str().to_string())),
                Some(Cow::Borrowed(value)) => regex.get()?.find( value).map(|x| Cow::Borrowed(x.as_str())),
                None => None
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

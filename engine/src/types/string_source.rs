//! Dynamically get strings from either literals or various parts of a [`TaskStateView`].

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
#[cfg(feature = "regex")]
#[expect(unused_imports, reason = "Used in docs.")]
use ::regex::Regex;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Dynamically get strings from either literals or various parts of a [`TaskStateView`].
///
/// The order things call [`Self::get`] is not considered stable. If your [`Cleaner`] cares about the order [`Self::get`] is called, you are in the wrong.
/// # Deserialization
/// Deserializing from a string produces a [`Self::String`] with that string.
///
/// Deserializing from a null/[`None`] produces a [`Self::None`].
/// # Terminology
/// "The value of {x}" and "{x}'s call to [`Self::get`]" are used interchangeably.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Return a reference to the contained [`String`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::String("abc".into()).get(&task_state).unwrap(), Some("abc".into()));
    /// ```
    String(String),
    /// Always returns [`None`].
    ///
    /// Deserializes from and serializes to `null`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::None.get(&task_state).unwrap(), None);
    ///
    /// assert_eq!(serde_json::from_str::<StringSource>("null").unwrap(), StringSource::None);
    /// assert_eq!(serde_json::to_string(&StringSource::None)  .unwrap(), "null");
    /// ```
    #[default]
    None,
    /// Always returns [`StringSourceError::ExplicitError`] with the included error.
    /// # Errors
    /// Always returns the error [`StringSourceError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// StringSource::Error("Message".into()).get(&task_state).unwrap_err();
    /// ```
    Error(String),
    /// If the call to [`Self::get`] returns an error, instead returns [`None`].
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ErrorToNone(Box::new(StringSource::Error("Message".into()))).get(&task_state).unwrap(), None);
    /// ```
    ErrorToNone(Box<Self>),
    /// If the call to [`Self::get`] returns an error, instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::ErrorToEmptyString(Box::new(StringSource::Error("Message".into()))).get(&task_state).unwrap(), Some("".into()));
    /// ```
    ErrorToEmptyString(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::get`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(geterrte(Self, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// Calls [`Self::get`] on each contained [`Self`] in order, returning the first to return [`Ok`].
    /// # Errors
    #[doc = edoc!(geterrfne(Self, StringSource))]
    FirstNotError(Vec<Self>),
    /// Print debug info about the contained [`Self`] and its call to [`Self::get`].
    ///
    /// The exact info printed is unspecified and subject to change at any time for any reason.
    /// # Suitability
    /// Always unsuiable to be in the default config.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If the call to [`Self::get`] returns [`None`], instead returns an empty string.
    ///
    /// Otherwise leaves the return value unchanged.
    /// # Errors
    #[doc = edoc!(geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::NoneToEmptyString(Box::new(StringSource::None)).get(&task_state).unwrap(), Some("".into()));
    /// ```
    NoneToEmptyString(Box<Self>),
    /// If [`Self::NoneTo::value`]'s call to [`Self::get`] returns [`None`], returns the value of [`Self::NoneTo::if_none`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// If [`Self::AssertMatches::value`] satisfies [`Self::AssertMatches::matcher`], return it. Otherwise return the error [`StringSourceError::AssertMatchesFailed`].
    /// # Errors
    /// If [`Self::AssertMatches::value`] doesn't satisfy [`Self::AssertMatches::matcher`], returns the error [`StringSourceError::AssertMatchesFailed`].
    AssertMatches {
        /// The [`Self`] to assert matches [`Self::AssertMatches::matcher`].
        value: Box<Self>,
        /// The [`StringMatcher`] to match [`Self::AssertMatches::value`].
        matcher: Box<StringMatcher>,
        /// The error message. Defaults to [`Self::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        message: Box<Self>
    },
    /// If the [`Params::flags`] specified by [`Self::IfFlag::flag`] is set, return the value of [`Self::IfFlag::then`]. If it's not set, return the value of [`Self::IfFlag::else`].
    /// # Errors
    #[doc = edoc!(geterr(FlagRef), geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, params = Params {flags: ["abc".into()].into(), ..Default::default()});
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
    /// If the value of [`Self::IfStringIsNone::value`] is [`None`], returns the value of [`Self::IfStringIsNone::then`].
    /// If it's [`Some`], returns the value of [`Self::IfStringIsNone::else`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// Gets the value of [`Self::IfStringMatches::value`] then, if it satisfies [`Self::IfStringMatches::matcher`], returns the value of [`Self::IfStringMatches::then`].
    /// If it doesn't match, returns the value of [`Self::IfStringMatches::else`]
    /// # Errors
    #[doc = edoc!(geterr(Self, 3), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// Indexes [`Self::Map::map`] with [`Self::Map::value`] and, if a [`Self`] is found, get it.
    /// # Errors
    #[doc = edoc!(geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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



    /// Returns the value of the specified [`UrlPart`] of the [`TaskStateView::url`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert_eq!(StringSource::Part(UrlPart::Host).get(&task_state).unwrap(), Some("example.com".into()));
    /// ```
    Part(UrlPart),
    /// Parses [`Self::ExtractPart`] as a [`BetterUrl`] and returns the part specified by [`Self::ExtractPart::part`].
    /// # Errors
    #[doc = edoc!(geterr(Self), getnone(StringSource, StringSource), callerr(BetterUrl::parse))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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



    /// Joins a list of [`Self`]s delimited by [`Self::Join::join`].
    ///
    /// Segments that evaluate to [`None`] are omitted.
    /// # Errors
    #[doc = edoc!(geterr(Self), getnone(Self, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::Join {
    ///     values: vec!["abc".into(), "def".into()],
    ///     join: "/".into()
    /// }.get(&task_state).unwrap(), Some("abc/def".into()));
    ///
    /// assert_eq!(StringSource::Join {
    ///     values: vec!["abc".into(), StringSource::None, "def".into()],
    ///     join: "/".into()
    /// }.get(&task_state).unwrap(), Some("abc/def".into()));
    /// ```
    Join {
        /// The values to join the values of with [`Self::Join::join`].
        values: Vec<Self>,
        /// The string to join the values of [`Self::Join::values`].
        ///
        /// Defaults to the empty string.
        #[serde(default, skip_serializing_if = "is_default")]
        join: String
    },



    /// Gets the var specified by the [`VarRef`].
    ///
    /// Can by any type of var supported by [`VarType`].
    /// # Errors
    #[doc = edoc!(callerr(VarRef::get))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, params = Params {
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
    /// Gets the [`Map`] specified by [`Self::ParamsMap::name`] from [`Params::maps`] then indexes it with [`Self::ParamsMap::key`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2), notfound(Map, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, params = Params {
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
        /// The name of the [`Params::maps`] to index.
        #[suitable(assert = "map_is_documented")]
        name: Box<Self>,
        /// The value to index the [`Map`] with.
        key: Box<Self>
    },
    /// Gets the [`NamedPartitioning`] specified by [`Self::NamedPartitioning::named_partitioning`] from [`Params::named_partitionings`] then gets the name of the partition containing [`Self::NamedPartitioning::element`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2), getnone(Self, StringSource, 2), notfound(NamedPartitioning, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, params = Params {
    ///     named_partitionings: [
    ///         (
    ///             "thing".into(),
    ///             NamedPartitioning::try_from_iter([
    ///                 ("abc".into(), vec![Some("a".into()), Some("b".into()), Some("c".into())]),
    ///                 ("def".into(), vec![Some("d".into()), Some("e".into()), Some("f".into())])
    ///             ]).unwrap()
    ///         )
    ///     ].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert_eq!(StringSource::NamedPartitioning {
    ///     named_partitioning: Box::new("thing".into()),
    ///     element: Box::new("a".into())
    /// }.get(&task_state).unwrap(), Some("abc".into()));
    /// ```
    NamedPartitioning {
        /// The name of the [`Params::named_partitionings`] to index.
        #[suitable(assert = "named_partitioning_is_documented")]
        named_partitioning: Box<Self>,
        /// The element whose partition to get the name of.
        element: Box<Self>
    },



    /// Gets the value of [`Self::Modified::value`] then applies [`Self::Modified::modification`].
    /// # Errors
    #[doc = edoc!(geterr(Self), applyerr(StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert_eq!(StringSource::Modified {
    ///     value: Box::new("abc".into()),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }.get(&task_state).unwrap(), Some("ABC".into()));
    ///
    /// StringSource::Modified {
    ///     value: Box::new(StringSource::None),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }.get(&task_state).unwrap_err();
    /// ```
    Modified {
        /// The value to get and modify.
        value: Box<Self>,
        /// The modification to apply to [`Self::Modified::value`].
        modification: Box<StringModification>
    },



    /// Gets the value of [`Self::RegexFind::value`] and calls [`Regex::find`] on it.
    ///
    /// If the value of [`Self::RegexFind::value`] is [`None`], simply returns [`None`].
    /// # Errors
    #[doc = edoc!(geterr(Self), geterr(RegexWrapper))]
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::glue::*;
    /// url_cleaner_engine::task_state_view!(task_state);
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



    /// Calls [`RequestConfig::response`] and returns the value.
    /// # Errors
    #[doc = edoc!(callerr(RequestConfig::response))]
    #[cfg(feature = "http")]
    HttpRequest(Box<RequestConfig>),



    /// Calls [`CommandConfig::output`] and returns the value.
    /// # Errors
    #[doc = edoc!(callerr(CommandConfig::output))]
    #[cfg(feature = "commands")]
    CommandOutput(Box<CommandConfig>),



    /// If an entry with a subject of [`Self::Cache::subject`] and a key of [`Self::Cache::key`] exists in the [`TaskStateView::cache`], returns the cached value.
    ///
    /// If no such entry exists, gets [`Self::Cache::value`] and inserts a new entry equivalent to getting it.
    /// # Errors
    #[doc = edoc!(callerr(Cache::read), geterr(Self), callerr(Cache::write))]
    #[cfg(feature = "cache")]
    Cache {
        /// The subject of the thing to cache.
        subject: Box<Self>,
        /// The key of the thing thing to cache.
        key: Box<Self>,
        /// The value to cache.
        value: Box<Self>
    },
    /// Calls a [`Self`] from [`TaskStateView::commons`]'s [`Commons::string_sources`].
    /// # Errors
    #[doc = edoc!(ageterr(Self, CommonCall::name), agetnone(Self, StringSource, CommonCall::name), commonnotfound(Self, StringSource), callerr(CommonCallArgsSource::build), geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, commons = Commons {
    ///     string_sources: [
    ///         ("abc".into(), "def".into()),
    ///         ("def".into(), StringSource::Var(Box::new(VarRef {
    ///             r#type: VarType::CommonArg,
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
    ///     args: Box::new(CommonCallArgsSource {
    ///         vars: [("common_var".into(), "ghi".into())].into(),
    ///         ..Default::default()
    ///     })
    /// }).get(&task_state).unwrap(), Some("ghi".into()));
    /// ```
    Common(CommonCall),
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::string_sources`] and applies it.
    /// # Errors
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`StringSourceError::NotInCommonContext`].
    ///
    #[doc = edoc!(commoncallargnotfound(Self, StringSource), geterr(Self))]
    CommonCallArg(Box<Self>),
    /// Calls the contained function and returns what it does.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::glue::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// fn some_complex_operation<'a>(task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, StringSourceError> {
    ///     Ok(Some("a".into()))
    /// }
    ///
    /// assert_eq!(StringSource::Custom(some_complex_operation).get(&task_state).unwrap(), Some("a".into()));
    /// ```
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(for<'a> fn(&TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, StringSourceError>)
}

impl FromStr for StringSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for StringSource {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for StringSource {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Option<&str>> for StringSource {
    fn from(value: Option<&str>) -> Self {
        value.map(ToString::to_string).into()
    }
}

impl From<Option<String>> for StringSource {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(x) => x.into(),
            None => Self::None
        }
    }
}

impl From<Option<StringSource>> for StringSource {
    fn from(value: Option<StringSource>) -> Self {
        match value {
            Some(x) => x,
            None => Self::None
        }
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

/// The enum of errors [`StringSource::get`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringSourceError {
    /// Returned when a [`StringSource::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when a [`StringSource::AssertMatches`]'s assertion fails.
    #[error("AssertMatches failed: {0}")]
    AssertMatchesFailed(String),
    /// Returned when both [`StringModification`]s in a [`StringModification::TryElse`] return errors.
    #[error("Both StringModifications in a StringModification::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringModification::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`StringModification::TryElse::else`]. 
        else_error: Box<Self>
    },
    /// Returned when all [`StringModification`]s in a [`StringModification::FirstNotError`] error.
    #[error("All StringModifications in a StringModification::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringModificationError`] is encounterd.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`Box<StringMatcherError>`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),

    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`StringSource::Map::map`] doesn't have the requested value.
    #[error("The StringSource::Map::map didn't have the requested value.")]
    StringNotInMap,
    /// Returned when the requested [`Map`] isn't found.
    #[error("The requested map was not found.")]
    MapNotFound,
    /// Returned when the requested [`Params::named_partitionings`] isn't found
    #[error("The requested Params NamedPartitioning was not found.")]
    NamedPartitioningNotFound,
    /// Returned when a [`GetFlagError`] is encountered.
    #[error(transparent)]
    GetFlagError(#[from] GetFlagError),
    /// Returned when a [`GetVarError`] is encountered.
    #[error(transparent)]
    GetVarError(#[from] GetVarError),

    /// Returned when a[`::regex::Error`]  is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    #[cfg(feature = "http")]
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`HttpResponseError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    HttpResponseError(#[from] HttpResponseError),
    /// Returned when a [`ResponseHandlerError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReponseHandlerError(#[from] ResponseHandlerError),
    #[cfg(feature = "http")]
    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[error(transparent)]
    HeaderToStrError(#[from] reqwest::header::ToStrError),
    /// Returned when a [`ReadFromCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    /// Returned when a [`WriteToCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] Box<CommandError>),

    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Returned when the requested [`Commons::string_sources`] isn't found.
    #[error("The requested common StringSource was not found.")]
    CommonStringSourceNotFound,
    /// Returned when trying to use [`StringSource::CommonCallArg`] outside of a common context.
    #[error("Tried to use StringSource::CommonCallArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the [`StringSource`] requested from a [`StringSource::CommonCallArg`] isn't found.
    #[error("The StringSource requested from a StringSource::CommonCallArg wasn't found.")]
    CommonCallArgStringSourceNotFound,

    /// An arbitrary [`std::error::Error`] for use with [`StringSource::Custom`].
    #[cfg(feature = "custom")]
    #[error(transparent)]
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
    /// "Deref patterns at home" for internal macros.
    pub(crate) fn get_self(&self) -> &Self {
        self
    }
    
    /// Get the string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'a>(&'a self, task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, StringSourceError> {
        debug!(StringSource::get, self);
        Ok(match self {
            Self::String(string) => Some(Cow::Borrowed(string)),
            Self::None => None,
            Self::Error(msg) => Err(StringSourceError::ExplicitError(msg.clone()))?,
            Self::ErrorToNone(value) => value.get(task_state).ok().flatten(),
            Self::ErrorToEmptyString(value) => value.get(task_state).unwrap_or(Some(Cow::Borrowed(""))),
            Self::TryElse{r#try, r#else} => r#try.get(task_state).or_else(|try_error| r#else.get(task_state).map_err(|else_error| StringSourceError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(sources) => {
                let mut errors = Vec::new();
                for source in sources {
                    match source.get(task_state) {
                        Ok(x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }
                Err(StringSourceError::FirstNotErrorErrors(errors))?
            },
            Self::Debug(source) => {
                let ret = source.get(task_state);
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\ntask_state: {task_state:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneToEmptyString(value) => value.get(task_state)?.or(Some(Cow::Borrowed(""))),
            Self::NoneTo {value, if_none} => match value.get(task_state)? {
                Some(x) => Some(x),
                None => if_none.get(task_state)?
            },
            Self::AssertMatches {value, matcher, message} => {
                let ret = value.get(task_state)?;
                if matcher.check(ret.as_deref(), task_state)? {
                    ret
                } else {
                    Err(StringSourceError::AssertMatchesFailed(message.get(task_state)?.unwrap_or_default().into_owned()))?
                }
            },
            Self::IfFlag          {flag ,          then, r#else} => if               flag .get(task_state)?                          {then} else {r#else}.get(task_state)?,
            Self::IfStringIsNone  {value,          then, r#else} => if               value.get(task_state)?.is_none()                {then} else {r#else}.get(task_state)?,
            Self::IfStringMatches {value, matcher, then, r#else} => if matcher.check(value.get(task_state)?.as_deref(), task_state)? {then} else {r#else}.get(task_state)?,
            Self::Map {value, map} => map.get(value.get(task_state)?).ok_or(StringSourceError::StringNotInMap)?.get(task_state)?,



            Self::Part(part) => part.get(task_state.url),
            Self::ExtractPart{value, part} => part.get(&BetterUrl::parse(&value.get(task_state)?.ok_or(StringSourceError::StringSourceIsNone)?)?).map(|x| Cow::Owned(x.into_owned())),



            Self::Join {values, join} => match join.as_str() {
                "" => Some(Cow::Owned(values.iter().filter_map(|value| value.get(task_state).transpose()).collect::<Result<String, _>>()?)),
                _  => Some(Cow::Owned(values.iter().filter_map(|value| value.get(task_state).transpose()).collect::<Result<Vec<_>, _>>()?.join(join)))
            },



            Self::Var(var_ref) => var_ref.get(task_state)?,
            Self::ParamsMap {name, key} => task_state.params.maps.get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(key.get(task_state)?).map(|x| Cow::Borrowed(&**x)),
            Self::NamedPartitioning {named_partitioning, element} => task_state.params.named_partitionings
                .get(get_str!(named_partitioning, task_state, StringSourceError)).ok_or(StringSourceError::NamedPartitioningNotFound)?
                .get_partition_of(element.get(task_state)?.as_deref()).map(Cow::Borrowed),



            Self::Modified {value, modification} => {
                let mut ret = value.get(task_state)?;
                modification.apply(&mut ret, task_state)?;
                ret
            },



            #[cfg(feature = "regex")]
            Self::RegexFind {value, regex} => match value.get(task_state)? {
                Some(Cow::Owned   (value)) => regex.get()?.find(&value).map(|x| Cow::Owned   (x.as_str().to_string())),
                Some(Cow::Borrowed(value)) => regex.get()?.find( value).map(|x| Cow::Borrowed(x.as_str())),
                None => None
            },



            #[cfg(feature = "http")]
            Self::HttpRequest(config) => {
                let _unthread_hanlde = task_state.unthreader.unthread();
                Some(Cow::Owned(config.response(task_state)?))
            },



            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => Some(Cow::Owned(command.output(task_state)?)),



            #[cfg(feature = "cache")]
            Self::Cache {subject, key, value} => {
                let _unthreader_lock = task_state.unthreader.unthread();
                let subject = get_cow!(subject, task_state, StringSourceError);
                let key = get_cow!(key, task_state, StringSourceError);
                if task_state.params.read_cache {
                    if let Some(entry) = task_state.cache.read(CacheEntryKeys {subject: &subject, key: &key})? {
                        return Ok(entry.value.map(Cow::Owned));
                    }
                }
                let start = std::time::Instant::now();
                let ret = value.get(task_state)?;
                let duration = start.elapsed();
                if task_state.params.write_cache {
                    task_state.cache.write(NewCacheEntry {
                        subject: &subject,
                        key: &key,
                        value: ret.as_deref(),
                        duration
                    })?;
                }
                ret
            },
            Self::Common(common_call) => {
                task_state.commons.string_sources.get(get_str!(common_call.name, task_state, StringSourceError)).ok_or(StringSourceError::CommonStringSourceNotFound)?.get(&TaskStateView {
                    common_args: Some(&common_call.args.build(task_state)?),
                    url        : task_state.url,
                    scratchpad : task_state.scratchpad,
                    context    : task_state.context,
                    job_context: task_state.job_context,
                    params     : task_state.params,
                    commons    : task_state.commons,
                    #[cfg(feature = "cache")]
                    cache      : task_state.cache,
                    unthreader : task_state.unthreader
                })?.map(|x| Cow::Owned(x.into_owned()))
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(StringSourceError::NotInCommonContext)?.string_sources.get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::CommonCallArgStringSourceNotFound)?.get(task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

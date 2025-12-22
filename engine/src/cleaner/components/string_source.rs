//! [`StringSource`].

#![allow(unused_assignments, reason = "False positive.")]

use std::str::FromStr;
use std::convert::Infallible;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in docs.")]
use regex::Regex;

use crate::prelude::*;

/// Get a string.
/// # Deserialization
/// Deserializing from a string produces a [`Self::String`] with that string.
///
/// Deserializing from a null/[`None`] produces a [`Self::None`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// Return a reference to the contained [`String`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("abc"), StringSource::String("abc".into()), &ts);
    /// ```
    String(String),
    /// Always returns [`None`].
    ///
    /// Deserializes from and serializes to `null`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, None, StringSource::None, &ts);
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// StringSource::Error("Message".into()).get(&ts).unwrap_err();
    /// ```
    Error(String),
    /// If [`Self::TryElse::try`]'s call to [`Self::get`] returns an error, instead return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(geterrte(Self, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, None, StringSource::TryElse {r#try: Box::new(StringSource::Error("Message".into())), r#else: Box::new(StringSource::None)}, &ts);
    /// ```
    TryElse {
        /// The value to try to get.
        ///
        /// If it's an error, return the value of [`Self::TryElse::else`].
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
    /// Always unsuitable to be in the bundled cleaner.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If [`Self::NoneTo::value`] is [`Some`], return it. Otherwise return [`Self::NoneTo::if_none`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("none"), StringSource::NoneTo {
    ///     value: Box::new(StringSource::None),
    ///     if_none: Box::new("none".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("not none"), StringSource::NoneTo {
    ///     value: Box::new("not none".into()),
    ///     if_none: Box::new("none".into())
    /// }, &ts);
    /// ```
    NoneTo {
        /// The value to return if it's [`Some`].
        value: Box<Self>,
        /// The value to return if [`Self::NoneTo::value`] is [`None`].
        if_none: Box<Self>
    },
    /// If the value of the contained [`Self`] is [`None`], return the empty string.
    /// # Errors
    #[doc = edoc!(geterr(Self))]
    NoneToEmpty(Box<Self>),
    /// If the value of the contained [`Self`] is the empty string, return [`None`].
    /// # Errors
    #[doc = edoc!(geterr(Self))]
    EmptyToNone(Box<Self>),
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
    #[doc = edoc!(geterr(FlagSource), geterr(Self))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, params = Params {flags: Cow::Owned(["abc".into()].into()), ..Default::default()});
    ///
    /// doc_test!(get, Some("set!"), StringSource::IfFlag {
    ///     flag: Box::new(FlagSource::Params("abc".into())),
    ///     then: Box::new("set!".into()),
    ///     r#else: Box::new("unset".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("unset"), StringSource::IfFlag {
    ///     flag: Box::new(FlagSource::Params("def".into())),
    ///     then: Box::new("set!".into()),
    ///     r#else: Box::new("unset".into())
    /// }, &ts);
    /// ```
    IfFlag {
        /// The name of the flag to check.
        flag: Box<FlagSource>,
        /// The value to return if the flag is set.
        then: Box<Self>,
        /// The value to return if the flag is unset.
        r#else: Box<Self>
    },
    /// If the value of [`Self::IfNone::value`] is [`None`], returns the value of [`Self::IfNone::then`].
    /// If it's [`Some`], returns the value of [`Self::IfNone::else`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("none"), StringSource::IfNone {
    ///     value : Box::new(StringSource::None),
    ///     then  : Box::new("none".into()),
    ///     r#else: Box::new("some".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("some"), StringSource::IfNone {
    ///     value : Box::new("some value. it's not returned".into()),
    ///     then  : Box::new("none".into()),
    ///     r#else: Box::new("some".into())
    /// }, &ts);
    /// ```
    IfNone {
        /// The value whose [`None`]ness to check.
        value: Box<Self>,
        /// The value to return if [`Self::IfNone::value`] returns [`None`].
        then: Box<Self>,
        /// The value to return if [`Self::IfNone::value`] returns [`Some`].
        r#else: Box<Self>
    },
    /// If [`Self::IfMatches::value`] satisfies [`Self::IfMatches::matcher`], returns the value of [`Self::IfMatches::then`], otherwise the value of [`Self::IfMatches::else`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 3), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("matches"), StringSource::IfMatches {
    ///     value  : Box::new("abc".into()),
    ///     matcher: Box::new(StringMatcher::Is("abc".into())),
    ///     then   : Box::new("matches".into()),
    ///     r#else : Box::new("doesn't match".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("doesn't match"), StringSource::IfMatches {
    ///     value  : Box::new("def".into()),
    ///     matcher: Box::new(StringMatcher::Is("abc".into())),
    ///     then   : Box::new("matches".into()),
    ///     r#else : Box::new("doesn't match".into())
    /// }, &ts);
    /// ```
    IfMatches {
        /// The value to match.
        value: Box<Self>,
        /// The matcher to match [`Self::IfMatches::value`].
        matcher: Box<StringMatcher>,
        /// The value to return if [`Self::IfMatches::matcher`] passes.
        then: Box<Self>,
        /// The value to return if [`Self::IfMatches::matcher`] fails.
        r#else: Box<Self>
    },
    /// Indexes [`Self::Map::map`] with [`Self::Map::value`] and, if a [`Self`] is found, get it.
    /// # Errors
    #[doc = edoc!(geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// let map = Box::new(Map {
    ///     map    : [("abc".into(), "def".into())].into(),
    ///     if_none: Some("was none".into()),
    ///     r#else : Some("wasn't abc or none".into())
    /// });
    ///
    /// doc_test!(get, Some("def"), StringSource::Map {
    ///     value: Box::new("abc".into()),
    ///     map: map.clone()
    /// }, &ts);
    ///
    /// doc_test!(get, Some("wasn't abc or none"), StringSource::Map {
    ///     value: Box::new("else".into()),
    ///     map: map.clone()
    /// }, &ts);
    ///
    /// doc_test!(get, Some("was none"), StringSource::Map {
    ///     value: Box::new(StringSource::None),
    ///     map: map.clone()
    /// }, &ts);
    /// ```
    Map {
        /// The value to index [`Self::Map::map`] with.
        value: Box<Self>,
        /// The [`Map`] to index with [`Self::Map::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>,
    },



    /// Returns the value of the specified [`UrlPart`] of the [`TaskState::url`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(get, Some("example.com"), StringSource::Part(UrlPart::Host), &ts);
    /// ```
    Part(UrlPart),
    /// Parses [`Self::ExtractPart`] as a [`BetterUrl`] and returns the part specified by [`Self::ExtractPart::part`].
    /// # Errors
    #[doc = edoc!(geterr(Self), getnone(StringSource, StringSource), callerr(BetterUrl::parse))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("example.com"), StringSource::ExtractPart {
    ///     value: Box::new("https://example.com".into()),
    ///     part: UrlPart::Host
    /// }, &ts);
    /// ```
    ExtractPart {
        /// The [`BetterUrl`] to get [`Self::ExtractPart::part`] from.
        value: Box<Self>,
        /// The [`UrlPart`] to get from [`Self::ExtractPart::value`].
        part: UrlPart
    },
    /// Gets the specified [`HostPart`] from the [`JobContext::source_host`].
    JobSourceHostPart(HostPart),



    /// Joins a list of [`Self`]s delimited by [`Self::Join::join`].
    ///
    /// Segments that evaluate to [`None`] are omitted.
    /// # Errors
    #[doc = edoc!(geterr(Self), getnone(Self, StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("abc/def"), StringSource::Join {
    ///     values: vec!["abc".into(), "def".into()],
    ///     join: "/".into()
    /// }, &ts);
    ///
    /// doc_test!(get, Some("abc/def"), StringSource::Join {
    ///     values: vec!["abc".into(), StringSource::None, "def".into()],
    ///     join: "/".into()
    /// }, &ts);
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



    /// Gets the var specified by the [`VarSource`].
    /// # Errors
    #[doc = edoc!(callerr(VarSource::get))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, params = Params {
    ///     vars: Cow::Owned([("abc".into(), "def".into())].into()),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(get, Some("def"), StringSource::Var(Box::new(VarSource::Params("abc".into()))), &ts);
    /// ```
    Var(Box<VarSource>),
    /// Gets the [`Map`] specified by [`Self::ParamsMap::name`] from [`Params::maps`] then indexes it with [`Self::ParamsMap::key`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2), notfound(Map, StringSource))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, params = Params {
    ///     maps: Cow::Owned([("map_name".into(), Map {
    ///         map    : [("abc".into(), "def".into())].into(),
    ///         if_none: Some("was none".into()),
    ///         r#else : Some("wasn't abc or none".into())
    ///     })].into()),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(get, Some("def"), StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new("abc".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("wasn't abc or none"), StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new("else".into())
    /// }, &ts);
    ///
    /// doc_test!(get, Some("was none"), StringSource::ParamsMap {
    ///     name: Box::new("map_name".into()),
    ///     key: Box::new(StringSource::None)
    /// }, &ts);
    /// ```
    ParamsMap {
        /// The name of the [`Params::maps`] to index.
        #[suitable(assert = "map_is_documented")]
        name: Box<Self>,
        /// The value to index the [`Map`] with.
        key: Box<Self>
    },
    /// Gets the [`Partitioning`] specified by [`Self::Partitioning::partitioning`] from [`Params::partitionings`] then gets the name of the partition containing [`Self::Partitioning::element`].
    /// # Errors
    #[doc = edoc!(geterr(Self, 2), getnone(Self, StringSource, 2), notfound(Partitioning, StringSource))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, params = Params {
    ///     partitionings: Cow::Owned([
    ///         (
    ///             "thing".into(),
    ///             Partitioning::try_from_iter([
    ///                 ("abc".into(), vec![Some("a".into()), Some("b".into()), Some("c".into())]),
    ///                 ("def".into(), vec![Some("d".into()), Some("e".into()), Some("f".into())])
    ///             ]).unwrap()
    ///         )
    ///     ].into()),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(get, Some("abc"), StringSource::Partitioning {
    ///     partitioning: Box::new("thing".into()),
    ///     element: Box::new("a".into())
    /// }, &ts);
    /// ```
    Partitioning {
        /// The name of the [`Params::partitionings`] to index.
        #[suitable(assert = "partitioning_is_documented")]
        partitioning: Box<Self>,
        /// The element whose partition to get the name of.
        element: Box<Self>
    },



    /// Gets the value of [`Self::Modified::value`] then applies [`Self::Modified::modification`].
    /// # Errors
    #[doc = edoc!(geterr(Self), applyerr(StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(get, Some("ABC"), StringSource::Modified {
    ///     value: Box::new("abc".into()),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }, &ts);
    ///
    /// doc_test!(get, Err, StringSource::Modified {
    ///     value: Box::new(StringSource::None),
    ///     modification: Box::new(StringModification::Uppercase)
    /// }, &ts);
    /// ```
    Modified {
        /// The value to get and modify.
        value: Box<Self>,
        /// The modification to apply to [`Self::Modified::value`].
        modification: Box<StringModification>
    },



    /// Sends an HTTP request and handles its response to return a value.
    /// # Errors
    #[doc = edoc!(callerr(HttpClient::get_response), callerr(HttpResponseHandler::handle))]
    #[cfg(feature = "http")]
    HttpRequest {
        /// The [`HttpRequestConfig`].
        ///
        /// Defaults to the default [`HttpRequestConfig`].
        #[serde(default, skip_serializing_if = "is_default")]
        request: Box<HttpRequestConfig>,
        /// The [`HttpResponseHandler`].
        ///
        /// Defaults to the default [`HttpResponseHandler`].
        #[serde(default, skip_serializing_if = "is_default")]
        response: Box<HttpResponseHandler>
    },



    /// If an entry with a subject of [`Self::Cache::subject`] and a key of [`Self::Cache::key`] exists in the [`Cache`], returns the cached value.
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
    /// Uses a [`Self`] from [`Cleaner::functions`].
    /// # Errors
    #[doc = edoc!(functionnotfound(Self, StringSource), geterr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, functions = Functions {
    ///     string_sources: [
    ///         ("abc".into(), "def".into()),
    ///         ("def".into(), StringSource::Var(Box::new(VarSource::CallArg("function_var".into()))))
    ///     ].into(),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(get, Some("def"), StringSource::Function(Box::new(FunctionCall {
    ///     name: "abc".into(),
    ///     args: Default::default()
    /// })), &ts);
    ///
    /// doc_test!(get, None, StringSource::Function(Box::new(FunctionCall {
    ///     name: "def".into(),
    ///     args: Default::default()
    /// })), &ts);
    ///
    /// doc_test!(get, Some("ghi"), StringSource::Function(Box::new(FunctionCall {
    ///     name: "def".into(),
    ///     args: CallArgs {
    ///         vars: [("function_var".into(), "ghi".into())].into(),
    ///         ..Default::default()
    ///     }
    /// })), &ts);
    /// ```
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    /// # Errors
    #[doc = edoc!(notinfunction(StringSource), callargfunctionnotfound(Self, StringSource), geterr(Self))]
    CallArg(Box<StringSource>),
    /// Calls the contained function and returns what it does.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// fn some_complex_operation<'j, 't>(ts: &'t TaskState<'j>) -> Result<Option<Cow<'t, str>>, StringSourceError> {
    ///     Ok(Some("a".into()))
    /// }
    ///
    /// doc_test!(get, Some("a"), StringSource::Custom(some_complex_operation), &ts);
    /// ```
    #[suitable(never)]
    #[serde(skip)]
    Custom(for<'j, 't> fn(&'t TaskState<'j>) -> Result<Option<Cow<'t, str>>, StringSourceError>)
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

impl From<url::Url> for StringSource {
    fn from(value: url::Url) -> Self {
        Self::String(value.into())
    }
}

impl From<BetterUrl> for StringSource {
    fn from(value: BetterUrl) -> Self {
        Self::String(value.into())
    }
}

impl From<UrlPart> for StringSource {
    fn from(value: UrlPart) -> Self {
        Self::Part(value)
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
    /// Returned when both [`StringSource`]s in a [`StringSource::TryElse`] return errors.
    #[error("Both StringSources in a StringSource::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringSource::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`StringSource::TryElse::else`].
        else_error: Box<Self>
    },
    /// Returned when all [`StringSource`]s in a [`StringSource::FirstNotError`] error.
    #[error("All StringSources in a StringSource::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringModificationError`] is encountered.
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
    /// Returned when the requested [`Params::partitionings`] isn't found.
    #[error("The requested Params Partitioning was not found.")]
    PartitioningNotFound,
    /// Returned when a [`FlagSourceError`] is encountered.
    #[error(transparent)]
    FlagSourceError(#[from] FlagSourceError),
    /// Returned when a [`VarSourceError`] is encountered.
    #[error(transparent)]
    VarSourceError(#[from] VarSourceError),

    /// Returned when a [`regex::Error`]  is encountered.
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Returned when a [`DoHttpRequestError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    DoHttpRequestError(#[from] Box<DoHttpRequestError>),
    /// Returned when a [`HttpResponseHandlerError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    HttpResponseHandlerError(#[from] Box<HttpResponseHandlerError>),
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

    /// Returned when the requested [`Functions::string_sources`] isn't found.
    #[error("The requested function StringSource was not found.")]
    FunctionNotFound,
    /// Returned when attempting to use [`CallArgs`] outside a function.
    #[error("Attempted to use CallArgs outside a function.")]
    NotInFunction,
    /// Returned when a [`CallArgs`] function ins't found.
    #[error("A CallArgs function wasn't found.")]
    CallArgFunctionNotFound,

    /// An arbitrary [`std::error::Error`] for use with [`StringSource::Custom`].
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl From<StringMatcherError> for StringSourceError {
    fn from(value: StringMatcherError) -> Self {
        Self::StringMatcherError(Box::new(value))
    }
}

#[cfg(feature = "http")]
impl From<DoHttpRequestError> for StringSourceError {
    fn from(value: DoHttpRequestError) -> Self {
        Self::DoHttpRequestError(Box::new(value))
    }
}

#[cfg(feature = "http")]
impl From<HttpResponseHandlerError> for StringSourceError {
    fn from(value: HttpResponseHandlerError) -> Self {
        Self::HttpResponseHandlerError(Box::new(value))
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
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>) -> Result<Option<Cow<'t, str>>, StringSourceError> {
        debug!(StringSource::get, self; Ok(match self {
            Self::String(string) => Some(Cow::Borrowed(&**string)),
            Self::None => None,
            Self::Error(msg) => Err(StringSourceError::ExplicitError(msg.clone()))?,
            Self::TryElse{r#try, r#else} => match r#try.get(task_state) {
                Ok(x) => x,
                Err(e1) => match r#else.get(task_state) {
                    Ok(x) => x,
                    Err(e2) => Err(StringSourceError::TryElseError {try_error: Box::new(e1), else_error: Box::new(e2)})?
                }
            },
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
                eprintln!("=== StringSource::Debug ===\nSource: {source:?}\nret: {ret:?}");
                ret?
            },
            Self::NoneTo {value, if_none} => match value.get(task_state)? {
                Some(x) => Some(x),
                None    => get_option_cow!(&**if_none, task_state)
            },
            Self::EmptyToNone(value) => {
                let x = value.get(task_state)?;
                if x == Some("".into()) {
                    None
                } else {
                    x
                }
            },
            Self::NoneToEmpty(value) => Some(value.get(task_state)?.unwrap_or(Cow::Borrowed(""))),
            Self::AssertMatches {value, matcher, message} => {
                let ret = value.get(task_state)?;
                if matcher.check(ret.as_deref(), task_state)? {
                    ret
                } else {
                    Err(StringSourceError::AssertMatchesFailed(message.get(task_state)?.unwrap_or_default().into_owned()))?
                }
            },
            Self::IfFlag    {flag ,          then, r#else} => if               flag .get(task_state)?                          {then} else {r#else}.get(task_state)?,
            Self::IfNone    {value,          then, r#else} => if               value.get(task_state)?.is_none()                {then} else {r#else}.get(task_state)?,
            Self::IfMatches {value, matcher, then, r#else} => if matcher.check(value.get(task_state)?.as_deref(), task_state)? {then} else {r#else}.get(task_state)?,
            Self::Map {value, map} => map.get(value.get(task_state)?).ok_or(StringSourceError::StringNotInMap)?.get(task_state)?,



            Self::Part(part) => part.get(&task_state.url),
            Self::ExtractPart{value, part} => part.get(&BetterUrl::parse(&value.get(task_state)?.ok_or(StringSourceError::StringSourceIsNone)?)?).map(|x| Cow::Owned(x.into_owned())),
            Self::JobSourceHostPart(part) => task_state.job.context.source_host.as_ref().and_then(|host| part.get(host)).map(Cow::Borrowed),



            Self::Join {values, join} => match join.as_str() {
                "" => Some(Cow::Owned(values.iter().filter_map(|value| value.get(task_state).transpose()).collect::<Result<String, _>>()?)),
                _  => Some(Cow::Owned(values.iter().filter_map(|value| value.get(task_state).transpose()).collect::<Result<Vec<_>, _>>()?.join(join)))
            },



            Self::Var(var_ref) => var_ref.get(task_state)?,
            Self::ParamsMap {name, key} => task_state.job.cleaner.params.maps.get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::MapNotFound)?.get(key.get(task_state)?).map(|x| Cow::Borrowed(&**x)),
            Self::Partitioning {partitioning, element} => task_state.job.cleaner.params.partitionings
                .get(get_str!(partitioning, task_state, StringSourceError)).ok_or(StringSourceError::PartitioningNotFound)?
                .get(element.get(task_state)?.as_deref()).map(Cow::Borrowed),



            Self::Modified {value, modification} => {
                let mut ret = value.get(task_state)?;
                modification.apply(&mut ret, task_state)?;
                ret
            },



            #[cfg(feature = "http")]
            Self::HttpRequest {request, response} => {
                let _unthread_handle = task_state.job.unthreader.unthread();
                response.handle(&mut task_state.job.http_client.get_response(request, task_state)?, task_state)?.map(Cow::Owned)
            },



            #[cfg(feature = "cache")]
            Self::Cache {subject, key, value} => {
                let _unthreader_lock = task_state.job.unthreader.unthread();
                let subject = get_cow!(subject, task_state, StringSourceError);
                let key = get_cow!(key, task_state, StringSourceError);
                if let Some(entry) = task_state.job.cache.read(CacheEntryKeys {subject: &subject, key: &key})? {
                    return Ok(entry.value.map(Cow::Owned));
                }
                let start = std::time::Instant::now();
                let ret = value.get(task_state)?;
                let duration = start.elapsed();
                task_state.job.cache.write(NewCacheEntry {
                    subject: &subject,
                    key: &key,
                    value: ret.as_deref(),
                    duration
                })?;
                ret
            },
            Self::Function(call) => {
                let func = task_state.job.cleaner.functions.string_sources.get(&call.name).ok_or(StringSourceError::FunctionNotFound)?;
                let old_args = task_state.call_args.replace(Some(&call.args));
                let ret = func.get(task_state);
                task_state.call_args.replace(old_args);
                ret?
            },
            Self::CallArg(name) => task_state.call_args.get().ok_or(StringSourceError::NotInFunction)?
                .string_sources.get(get_str!(name, task_state, StringSourceError)).ok_or(StringSourceError::CallArgFunctionNotFound)?
                .get(task_state)?,
            Self::Custom(function) => function(task_state)?
        }))
    }
}

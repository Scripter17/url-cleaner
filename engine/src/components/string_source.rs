//! [`StringSource`].

#![allow(unused_assignments, reason = "False positive.")]

use std::ops::Bound;
use std::convert::Infallible;

#[expect(unused_imports, reason = "Used in docs.")]
use regex::Regex;

use crate::prelude::*;

/// Get a string.
///
/// Serializes/deserializes strings to [`Self::String`] and null to [`Self::None`].
///
/// Defaults to [`Self::None`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub enum StringSource {
    /// [`None`].
    #[default]
    None,
    /// The contained [`String`].
    String(String),
    /// [`ExplicitError`].
    /// # Errors
    /// [`ExplicitError`].
    Error(String),
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it's [`Err`].
    /// # Errors
    /// If both return [`Err`], returns the error [`TryElseError`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// Returns the first [`Self`] to return [`Ok`].
    /// # Errors
    /// If all return [`Err`], returns the error [`FirstNotErrorErrors`].
    FirstNotError(Vec<Self>),



    /// [`Self::NoneTo::value`], or, if it's [`None`], [`Self::NoneTo::if_none`].
    NoneTo {
        /// The try.
        value: Box<Self>,
        /// The else.
        if_none: Box<Self>
    },
    /// [`Self::NoneToEmpty::0`] or, if [`None`], the empty string.
    NoneToEmpty(Box<Self>),
    /// [`Self::EmptyTo::value`] or, if the empty string, [`Self::EmptyTo::if_empty`].
    EmptyTo {
        /// The try.
        value: Box<Self>,
        /// The else.
        if_empty: Box<Self>
    },
    /// [`Self::EmptyToNone::0`] or, if the empty string, [`None`].
    EmptyToNone(Box<Self>),
    /// [`Self::AssertMatches::value`] if it satisfies [`Self::AssertMatches::matcher`].
    /// # Errors
    /// If [`Self::AssertMatches::matcher`] is unsatisfied, returns the error [`AssertError`].
    AssertMatches {
        /// The [`Self`] to assert matches [`Self::AssertMatches::matcher`].
        value: Box<Self>,
        /// The [`StringMatcher`] to match [`Self::AssertMatches::value`].
        matcher: Box<StringMatcher>,
        /// The error message.
        message: String,
    },
    /// [`Self::AssertSome::0`] or, if [`None`], the error [`StringNotFound`].
    /// # Errors
    /// If the call to [`Self::get`] returns [`None`], returns the error [`StringNotFound`].
    AssertSome(Box<Self>),



    /// Index the [`Map`] with the [`StringSource`] and use that or [`Self::StringMap::else`].
    StringMap {
        /// The [`StringSource`].
        value: Box<Self>,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// Index the [`Map`] with the [`UrlPart`] and use that or [`Self::PartMap::else`].
    PartMap {
        /// The [`UrlPart`].
        part: UrlPart,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },



    /// [`UrlPart::get`].
    Part(UrlPart),



    /// [`BetterUrl::domain_segment`].
    DomainSegment(isize),
    /// [`BetterUrl::domain_origin_segment`].
    DomainOriginSegment(isize),
    /// [`BetterUrl::domain_prefix_segment`].
    DomainPrefixSegment(isize),
    /// [`BetterUrl::domain_suffix_segment`]..
    DomainSuffixSegment(isize),
    /// [`BetterUrl::path_segment`] + [`PathSegment::decode`].
    PathSegment(isize),
    /// [`BetterUrl::path_segment`] + [`PathSegment::into_inner`].
    RawPathSegment(isize),
    /// [`BetterUrl::path_segment_range`] + [`PathSegments::into_inner`].
    RawPathSegmentRange {
        /// The start of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<isize>,
        /// The end of the range.
        ///
        /// Defaults to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<isize>
    },
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_value`].
    QueryParam(QueryParamSelector),
    /// [`BetterUrl::query_param`] + [`QuerySegment::into_raw_value`].
    RawQueryParam(QueryParamSelector),
    /// [`BetterUrl::fragment_query_param`] + [`QuerySegment::into_value`].
    FragmentParam(QueryParamSelector),
    /// [`BetterUrl::fragment_query_param`] + [`QuerySegment::into_raw_value`].
    RawFragmentParam(QueryParamSelector),



    /// Parses [`Self::ExtractPart`] as a [`BetterUrl`] and returns the part specified by [`Self::ExtractPart::part`].
    ExtractPart {
        /// The [`BetterUrl`] to get [`Self::ExtractPart::part`] from.
        value: Box<Self>,
        /// The [`UrlPart`] to get from [`Self::ExtractPart::value`].
        part: UrlPart
    },
    /// Gets the specified [`HostPart`] from the [`JobContext::source_host`].
    JobSourceHostPart(HostPart),



    /// Joins a list of [`Self`].
    ///
    /// Segments that evaluate to [`None`] are omitted.
    Join(Vec<Self>),



    /// [`VarSource::get`].
    Var(Box<VarSource>),
    /// [`MapSource::get`] + [`Map::get`].
    Map {
        /// The [`Map`] to index.
        map: Box<MapSource>,
        /// The value to index with.
        key: Box<Self>
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`].
    Partitioning {
        /// The [`Partitioning`].
        partitioning: Box<PartitioningSource>,
        /// The element.
        element: Box<Self>
    },



    /// Gets the value of [`Self::Modified::value`] then applies [`Self::Modified::modification`].
    Modified {
        /// The value to get and modify.
        value: Box<Self>,
        /// The modification to apply to [`Self::Modified::value`].
        modification: Box<StringModification>
    },



    /// [`regex::Regex::captures`] + [`RegexExpansion::expand`].
    RegexExpansion {
        /// The value to match with.
        value: Box<Self>,
        /// The [`LazyRegex`].
        regex: LazyRegex,
        /// The [`RegexExpansion`].
        expansion: Box<RegexExpansion>
    },



    /// Sends an HTTP request and handles its response to return a value.
    #[cfg(feature = "http")]
    HttpRequest {
        /// The [`HttpRequestSource`].
        ///
        /// Defaults to a default [`HttpRequestSource`].
        #[serde(default, skip_serializing_if = "is_default")]
        request: Box<HttpRequestSource>,
        /// The [`HttpResponseHandler`].
        ///
        /// Defaults to a default [`HttpResponseHandler`].
        #[serde(default, skip_serializing_if = "is_default")]
        response: Box<HttpResponseHandler>
    },



    /// Gets, or creates if missing, a cache entry with subject and key [`Self::Cache::subject`] and [`Self::Cache::key`].
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
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`FunctionArgs`].
    FunctionArg(Box<StringSource>),
    /// Calls the contained function and returns what it does.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    #[suitable(never)]
    #[serde(skip)]
    Extern(StringSourceExtern)
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

impl StringSource {
    /// Get the string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, StringSourceError> {
        debug!(StringSource::get, self, args; self._get(task_state, args))
    }

    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get`] returns [`None`], returns the error [`StringNotFound`].
    pub fn get_some<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<Cow<'t, str>, StringNotFound>, StringSourceError> {
        debug!(StringSource::get_some, self, args; self._get(task_state, args).map(|x| x.ok_or(StringNotFound)))
    }

    /// [`Self::get`] for use with [`BetterUrl`] setters.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    pub fn get_part<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'j, str>>, StringSourceError> {
        debug!(StringSource::get_part, self; match self {
            Self::None => Ok(None),
            Self::String(x) => Ok(Some(Cow::Borrowed(&**x))),
            _ => self._get(task_state, args).map(|x| x.map(|x| x.into_owned().into()))
        })
    }

    /// # Errors
    /// If the call to [`Self::get_part`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get_part`] returns [`None`], returns the error [`StringNotFound`].
    pub fn get_some_part<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<Cow<'j, str>, StringNotFound>, StringSourceError> {
        debug!(StringSource::get_some_part, self; match self {
            Self::String(x) => Ok(Ok(Cow::Borrowed(&**x))),
            _ => self._get(task_state, args).map(|x| x.ok_or(StringNotFound).map(|x| x.into_owned().into()))
        })
    }

    /// [`Self::get`].
    fn _get<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, StringSourceError> {
        Ok(match self {
            Self::None => None,
            Self::String(string) => Some(Cow::Borrowed(&**string)),
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            Self::TryElse {r#try, r#else} => match r#try.get(task_state, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.get(task_state, args) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(sources) => {
                let mut errors = FirstNotErrorErrors(Vec::new());

                for source in sources {
                    match source.get(task_state, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.0.push(e)
                    }
                }

                Err(errors)?
            },

            Self::NoneTo     {value, if_none } => match get!(?value) {None                    => get!(?if_none) , x => x},
            Self::NoneToEmpty(value          ) => match get!(?value) {None                    => Some("".into()), x => x},
            Self::EmptyTo    {value, if_empty} => match get!(?value) {Some(x) if x.is_empty() => get!(?if_empty), x => x},
            Self::EmptyToNone(value          ) => match get!(?value) {Some(x) if x.is_empty() => None           , x => x},

            Self::AssertMatches {value, matcher, message} => {
                let ret = get!(?value);
                match matcher.check(task_state, args, ret.as_deref())? {
                    true  => ret,
                    false => Err(AssertError(message.clone()))?
                }
            },
            Self::AssertSome(x) => Some(get!(x)),

            Self::StringMap {value, map, r#else} => get!(?map.get(get!(?&value)                       ).unwrap_or(r#else)),
            Self::PartMap   {part , map, r#else} => get!(?map.get(part.get(&task_state.url).as_deref()).unwrap_or(r#else)),

            Self::Part(part) => part.get(&task_state.url),



            Self::DomainSegment      (index) => task_state.url.domain_segment       (*index).map(DomainSegment::decode),
            Self::DomainOriginSegment(index) => task_state.url.domain_origin_segment(*index).map(DomainSegment::decode),
            Self::DomainPrefixSegment(index) => task_state.url.domain_prefix_segment(*index).map(DomainSegment::decode),
            Self::DomainSuffixSegment(index) => task_state.url.domain_suffix_segment(*index).map(DomainSegment::decode),

            Self::PathSegment         (index     ) => task_state.url.path_segment      (*index        ).map(PathSegment ::decode    ),
            Self::RawPathSegment      (index     ) => task_state.url.path_segment      (*index        ).map(PathSegment ::into_inner),
            Self::RawPathSegmentRange {start, end} => task_state.url.path_segment_range((*start, *end)).map(PathSegments::into_inner),

            Self::QueryParam   (param) => task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_value    ),
            Self::RawQueryParam(param) => task_state.url.query_param(&param.name, param.index).and_then(QuerySegment::into_raw_value),

            Self::FragmentParam   (param) => task_state.url.fragment_query_param(&param.name, param.index).and_then(FragmentQuerySegment::into_value    ),
            Self::RawFragmentParam(param) => task_state.url.fragment_query_param(&param.name, param.index).and_then(FragmentQuerySegment::into_raw_value),



            Self::ExtractPart{value, part} => part.get(&BetterUrl::parse(get!(&value))?).map(|x| x.into_owned().into()),
            Self::JobSourceHostPart(part) => task_state.job.context.source_host.as_ref().and_then(|host| part.get(host)),



            Self::Join(values) => Some(values.iter().filter_map(|value| value.get(task_state, args).transpose()).collect::<Result<String, _>>()?.into()),



            Self::Var          (var_ref              ) => get!(?var_ref),
            Self::Map          {map, key             } => get!(map).get(get!(?&key)).map(Into::into),
            Self::Partitioning {partitioning, element} => get!(partitioning).get(get!(?&element)).map(Into::into),



            Self::Modified {value, modification} => {
                let mut ret = get!(?value);
                modification.apply(task_state, args, &mut ret)?;
                ret
            },



            Self::RegexExpansion {value, regex, expansion} => {
                expansion.expand(task_state, args, &regex.get()?.captures(get!(&value)).ok_or(StringModificationError::RegexMatchNotFound)?)?.map(|x| x.into_owned().into())
            },



            #[cfg(feature = "http")]
            Self::HttpRequest {request, response} => {
                let _unthread_handle = task_state.job.unthreader.unthread();
                response.handle(task_state, args, &mut get!(?request).send()?)?
            },



            #[cfg(feature = "cache")]
            Self::Cache {subject, key, value} => {
                let _unthreader_lock = task_state.job.unthreader.unthread();
                let subject = get!(subject);
                let key = get!(key);
                if let Some(entry) = task_state.job.cache.read(CacheEntryKeys {subject: &subject, key: &key})? {
                    return Ok(entry.value.map(Cow::Owned));
                }
                let start = std::time::Instant::now();
                let ret = get!(?value);
                let duration = start.elapsed();
                task_state.job.cache.write(NewCacheEntry {
                    subject: &subject,
                    key: &key,
                    value: ret.as_deref(),
                    duration
                })?;
                ret
            },

            // Misc

            Self::Function(call) => task_state.job.cleaner.functions.string_sources
                .get(&call.name).ok_or(FunctionNotFound)?
                .get(task_state, Some(&call.args))?,

            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.string_sources
                .get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?
                .get(task_state, args)?,

            Self::Extern(function) => function(task_state, args)?
        })
    }
}

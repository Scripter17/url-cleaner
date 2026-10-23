//! [`StringSource`].

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
    ///
    /// The default.
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
        /** The try.  **/ r#try : Box<Self>,
        /** The else. **/ r#else: Box<Self>,
    },
    /// Returns the first [`Self`] to return [`Ok`].
    /// # Errors
    /// If all return [`Err`], returns the error [`FirstNotErrorErrors`].
    FirstNotError(Vec<Self>),



    /** Map [`None`] to the empty string. **/ NoneToEmpty(Box<Self>),
    /** Map the empty string to [`None`]. **/ EmptyToNone(Box<Self>),

    /// Map [`None`] to the error [`StringNotFound`].
    /// # Errors
    /// If [`Self::get`] returns [`None`], returns the error [`StringNotFound`].
    NoneToError(Box<Self>),

    /// Map [`None`] to [`Self::NoneTo::if_none`].
    NoneTo {
        /** The try.  **/ value  : Box<Self>,
        /** The else. **/ if_none: Box<Self>,
    },
    /// Map the empty string to [`Self::EmptyTo::if_empty`].
    EmptyTo {
        /** The try.  **/ value   : Box<Self>,
        /** The else. **/ if_empty: Box<Self>,
    },

    /// [`Self::AssertMatches::value`] if it satisfies [`Self::AssertMatches::matcher`].
    /// # Errors
    /// If [`Self::AssertMatches::matcher`] returns [`false`], returns the error [`AssertError`].
    AssertMatches {
        /** The value.         **/ value  : Box<Self>,
        /** The assert.        **/ matcher: Box<StringMatcher>,
        /** The error message. **/ message: String,
    },



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



    /// [`UrlPart::get`].
    Part(UrlPart),



    /** [`BetterUrl::domain_segment_str`].        **/ DomainSegment      (isize),
    /** [`BetterUrl::domain_origin_segment_str`]. **/ DomainOriginSegment(isize),
    /** [`BetterUrl::domain_prefix_segment_str`]. **/ DomainPrefixSegment(isize),
    /** [`BetterUrl::domain_suffix_segment_str`]. **/ DomainSuffixSegment(isize),

    /** [`BetterUrl::path_segment`] + [`PathSegment::lossy_decode`]. **/ PathSegment   (isize),
    /** [`BetterUrl::path_segment_str`].                             **/ RawPathSegment(isize),
    /// [`BetterUrl::path_segment_range_str`].
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
        end: Bound<isize>,
    },

    /** [`BetterUrl::query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::lossy_decode`].          **/ QueryParam      (QueryParamSelector),
    /** [`BetterUrl::query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::into_inner`].            **/ RawQueryParam   (QueryParamSelector),
    /** [`BetterUrl::fragment_query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::lossy_decode`]. **/ FragmentParam   (QueryParamSelector),
    /** [`BetterUrl::fragment_query_param`] + [`QuerySegment::into_value`] + [`MaybeQueryValue::into_inner`].   **/ RawFragmentParam(QueryParamSelector),



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
        /** The [`Map`] to index.    **/ map: Box<MapSource>,
        /** The value to index with. **/ key: Box<Self>,
    },
    /// [`PartitioningSource::get`] + [`Partitioning::get`].
    Partitioning {
        /** The [`Partitioning`]. **/ partitioning: Box<PartitioningSource>,
        /** The element.          **/ element     : Box<Self>,
    },



    /// [`Self::get`] + [`StringModification::apply`].
    Modified {
        /** The [`Self`].               **/ value       : Box<Self>,
        /** The [`StringModification`]. **/ modification: Box<StringModification>,
    },

    /** [`try_percent_decode`].    **/ TryPercentDecode  (Box<Self>),
    /** [`try_decode_query_part`]. **/ TryDecodeQueryPart(Box<Self>),



    /// [`Self::get`] + [`LazyRegex::get`] + [`regex::Regex::captures`] + [`RegexExpansion::expand`].
    RegexExpansion {
        /** The value to match with. **/ value    : Box<Self>,
        /** The [`LazyRegex`].       **/ regex    : Box<LazyRegex>,
        /** The [`RegexExpansion`].  **/ expansion: Box<RegexExpansion>,
    },



    /// [`HttpClient::do_sync`].
    ///
    /// PLEASE note that before using this, [`Action::AcquireThreadLock`] should be used.
    #[cfg(feature = "http")]
    HttpRequest {
        /// The [`HttpRequestSource`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        request: Box<HttpRequestSource>,
        /// The [`HttpResponseHandler`].
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        response: Box<HttpResponseHandler>,
    },



    /// Read and return or create a [`CacheClient`] entry.
    ///
    /// PLEASE note that before using this, [`Action::AcquireThreadLock`] should be used.
    #[cfg(feature = "cache")]
    Cache {
        /** The subject. **/ subject: Box<Self>,
        /** The key.     **/ key    : Box<Self>,
        /** The value.   **/ value  : Box<Self>,
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
    Extern(StringSourceExtern),
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
    /// [`Self::get`] with [`None`] replaced with [`StringNotFound`].
    /// # Errors
    /// If [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::get`] returns [`None`], returns the error [`StringNotFound`].
    pub fn get_some<'j: 't, 't>(&'j self, ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<Cow<'t, str>, StringNotFound>, StringSourceError> {
        self.get(ts, args).map(|x| x.ok_or(StringNotFound))
    }

    /// [`Self::get_part`] with [`None`] replaced with [`StringNotFound`].
    /// # Errors
    /// If [`Self::get_part`] returns an error, that error is returned.
    ///
    /// If [`Self::get_part`] returns [`None`], returns the error [`StringNotFound`].
    pub fn get_some_part<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<Cow<'j, str>, StringNotFound>, StringSourceError> {
        self.get_part(ts, args).map(|x| x.ok_or(StringNotFound))
    }

    /// [`Self::get`] for use with [`BetterUrl`] setters.
    /// # Errors
    /// If [`Self::get`] returns an error, that error is returned.
    pub fn get_part<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'j, str>>, StringSourceError> {
        Ok(match self {
            Self::None => None,
            Self::String(x) => Some(Cow::Borrowed(&**x)),
            _ => self.get(ts, args)?.map(|x| x.into_owned().into())
        })
    }

    /// Get the string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't>(&'j self, ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<Cow<'t, str>>, StringSourceError> {
        Ok(match self {
            Self::None => None,
            Self::String(string) => Some(Cow::Borrowed(&**string)),
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,



            Self::TryElse {r#try, r#else} => match r#try.get(ts, args) {
                Ok(x) => x,
                Err(try_error) => match r#else.get(ts, args) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(sources) => {
                let mut errors = FirstNotErrorErrors(Vec::new());

                for source in sources {
                    match source.get(ts, args) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.0.push(e)
                    }
                }

                Err(errors)?
            },



            Self::NoneTo  {value, if_none } => match value.get(ts, args)? {None                    => if_none .get(ts, args)?, x => x},
            Self::EmptyTo {value, if_empty} => match value.get(ts, args)? {Some(x) if x.is_empty() => if_empty.get(ts, args)?, x => x},

            Self::NoneToError(x) => Some(x.get_some(ts, args)??),

            Self::NoneToEmpty(value) => match value.get(ts, args)? {None                    => Some("".into()), x => x},
            Self::EmptyToNone(value) => match value.get(ts, args)? {Some(x) if x.is_empty() => None           , x => x},

            Self::AssertMatches {value, matcher, message} => {
                let ret = value.get(ts, args)?;
                match matcher.check(ts, args, ret.as_deref())? {
                    true  => ret,
                    false => Err(AssertError(message.clone()))?
                }
            },



            Self::StringMap {value, map, r#else} => map.get(value.get(ts, args)?.as_deref()).unwrap_or(r#else).get(ts, args)?,



            Self::Part(part) => part.get(&ts.url),



            Self::DomainSegment      (index) => ts.url.domain_segment_str       (*index).map(Into::into),
            Self::DomainOriginSegment(index) => ts.url.domain_origin_segment_str(*index).map(Into::into),
            Self::DomainPrefixSegment(index) => ts.url.domain_prefix_segment_str(*index).map(Into::into),
            Self::DomainSuffixSegment(index) => ts.url.domain_suffix_segment_str(*index).map(Into::into),

            Self::PathSegment         (index     ) => ts.url.path_segment          (*index        ).map(PathSegment::lossy_decode),
            Self::RawPathSegment      (index     ) => ts.url.path_segment_str      (*index        ).map(Cow::Borrowed),
            Self::RawPathSegmentRange {start, end} => ts.url.path_segment_range_str((*start, *end)).map(Cow::Borrowed),

            Self::QueryParam   (param) => ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()),
            Self::RawQueryParam(param) => ts.url.query_param(&param.name, param.index).and_then(|x| x.into_value().into_inner  ()),

            Self::FragmentParam   (param) => ts.url.fragment_query_param(&param.name, param.index).and_then(|x| x.into_value().lossy_decode()),
            Self::RawFragmentParam(param) => ts.url.fragment_query_param(&param.name, param.index).and_then(|x| x.into_value().into_inner  ()),



            Self::JobSourceHostPart(part) => ts.job.context.source_host.as_ref().and_then(|host| part.get(host)),



            Self::Join(values) => Some(values.iter().filter_map(|value| value.get(ts, args).transpose()).collect::<Result<String, _>>()?.into()),



            Self::Var          (var_ref              ) => var_ref.get(ts, args)?,
            Self::Map          {map, key             } => map.get_some(ts, args)??.get(key.get(ts, args)?.as_deref()).map(Into::into),
            Self::Partitioning {partitioning, element} => partitioning.get_some(ts, args)??.get(element.get(ts, args)?.as_deref()).map(Into::into),



            Self::Modified {value, modification} => {
                let mut ret = value.get(ts, args)?;
                modification.apply(ts, args, &mut ret)?;
                ret
            },



            Self::TryPercentDecode(value) => match value.get(ts, args)? {
                Some(value) => match try_percent_decode(value) {
                    Ok ((_, value)) => Some(value),
                    Err((e, _    )) => Err(e)?,
                },
                None => None,
            },

            Self::TryDecodeQueryPart(value) => match value.get(ts, args)? {
                Some(value) => match try_decode_query_part(value) {
                    Ok ((_, value)) => Some(value),
                    Err((e, _    )) => Err(e)?,
                },
                None => None,
            },



            Self::RegexExpansion {value, regex, expansion} => {
                expansion.expand(ts, args, &regex.get()?.captures(&value.get_some(ts, args)??).ok_or(StringModificationError::RegexMatchNotFound)?)?.map(|x| x.into_owned().into())
            },



            #[cfg(feature = "http")]
            Self::HttpRequest {request, response} => {
                #[cold]
                fn outline<'j: 't, 't>(ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, request: &'j HttpRequestSource, response: &'j HttpResponseHandler) -> Result<Option<Cow<'t, str>>, StringSourceError> {
                    Ok(ts.job.http_client.ok_or(NoHttpClient)?.do_sync(request, response, ts, args)?)
                }

                outline(ts, args, request, response)?
            },



            #[cfg(feature = "cache")]
            Self::Cache {subject, key, value} => {
                #[cold]
                fn outline<'j: 't, 't>(ts: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, subject: &'j StringSource, key: &'j StringSource, value: &'j StringSource) -> Result<Option<Cow<'t, str>>, StringSourceError> {
                    let subject = subject.get_some(ts, args)??;
                    let key     = key    .get_some(ts, args)??;

                    if let Some(value) = ts.job.cache_client.read_sync(&subject, &key, ts.job.cache_config)? {
                        return Ok(value.map(Cow::Owned));
                    }

                    let timer    = std::time::Instant::now();
                    let ret      = value.get(ts, args)?;
                    let duration = timer.elapsed();

                    ts.job.cache_client.write_sync(&subject, &key, ret.as_deref(), duration, ts.job.cache_config)?;

                    Ok(ret)
                }

                outline(ts, args, subject, key, value)?
            },

            // Misc

            Self::Function   (call    ) => ts.job.cleaner.functions.string_sources.get(&call.name).ok_or(FunctionNotFound)?.get(ts, Some(&call.args))?,
            Self::FunctionArg(name    ) => args.ok_or(NotInFunction)?.string_sources.get(&*name.get_some(ts, args)??).ok_or(FunctionArgFunctionNotFound)?.get(ts, args)?,
            Self::Extern     (function) => function(ts, args)?,
        })
    }
}

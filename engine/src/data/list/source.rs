//! [`ListSource`].

use crate::prelude::*;

/// Get a [`List`].
///
/// Defaults to [`Self::None`].
///
/// Null deserializes/serializes into/from [`Self::None`].
///
/// Strings deserialize/serialize into/from [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum ListSource {
    /// [`None`].
    ///
    /// The default.
    #[default]
    None,
    /// If [`Self::If::if`], [`Self::If::then`]. Otherwise [`Self::If::else`].
    If {
        /// The if.
        r#if: Box<Condition>,
        /// The then.
        r#then: Box<Self>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
    },
    /// Index [`Self::StringMap::map`] with [`Self::StringMap::value`] and use that [`Self`].
    ///
    /// If [`Map::get`] returns [`None`], returns [`None`].
    StringMap {
        /// The index.
        value: StringSource,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Index [`Self::PartMap::map`] with [`Self::PartMap::part`] and use that [`Self`].
    ///
    /// If [`Map::get`] returns [`None`], returns [`None`].
    PartMap {
        /// The index.
        part: UrlPart,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// The [`List`].
    Literal(List<String>),
    /// [`Params::lists`].
    Params(#[suitable(assert = list_source_params)] StringSource),
    /// [`FunctionArgs::lists`].
    FunctionArg(StringSource),
}

impl ListSource {
    /// [`Self::get`], replacing [`None`] with the sub-error [`ListNotFound`].
    /// # Errors
    /// If [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::get`] returns [`None`], returns the sub-error [`ListNotFound`].
    pub fn get_some<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<&'j List<String>, ListNotFound>, ListSourceError> {
        self.get(ts, args).map(|x| x.ok_or(ListNotFound))
    }

    /// Get the list.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j List<String>>, ListSourceError> {
        Ok(match self {
            Self::None                            => None,
            Self::Literal(x)                      => Some(x),
            Self::Params(StringSource::String(x)) => ts.job.cleaner.params.lists.get(x),
            _ => self._get(ts, args)?,
        })
    }

    /// [`Self::get`] without fast paths.
    #[cold]
    fn _get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j List<String>>, ListSourceError> {
        Ok(match self {
            Self::None => None,
            Self::If {r#if, r#then, r#else} => if r#if.check(ts, args)? {r#then} else {r#else}.get(ts, args)?,
            Self::StringMap {value, map} => match map.get(value.get(ts, args)?.as_deref()) {
                Some(source) => source.get(ts, args)?,
                None         => None
            },
            Self::PartMap {part, map} => match map.get(part.get(&ts.url).as_deref()) {
                Some(source) => source.get(ts, args)?,
                None         => None
            },
            Self::Literal    (x) => Some(x),
            Self::Params     (x) => ts.job.cleaner.params     .lists.get(&*x.get_some(ts, args)??),
            Self::FunctionArg(x) => args.ok_or(NotInFunction)?.lists.get(&*x.get_some(ts, args)??),
        })
    }
}



impl FromStr for ListSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(name.into())
    }
}

impl From<&str        > for ListSource {fn from(value: &str        ) -> Self {Self::Params(value.into())}}
impl From<String      > for ListSource {fn from(value: String      ) -> Self {Self::Params(value.into())}}
impl From<StringSource> for ListSource {fn from(value: StringSource) -> Self {Self::Params(value       )}}



impl Serialize for ListSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None                            => serializer.serialize_none()?,
            Self::Literal(list)                   => list.serialize(serializer)?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _                                     => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for ListSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = ListSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a none, str, string, seq, or map.")
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(Self::Value::None)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::Params(v.into()))
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Self::Value::Params(v.into()))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut ret = Vec::with_capacity(seq.size_hint().unwrap_or(8));

                while let Some(x) = seq.next_element()? {
                    ret.push(x);
                }

                Ok(Self::Value::Literal(List(ret)))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(V)
    }
}

//! [`MapSource`].

use crate::prelude::*;

/// Get a [`Map`].
///
/// Defaults to [`Self::None`].
///
/// Null deserializes/serializes into/from [`Self::None`].
///
/// Strings deserialize/serialize into/from [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum MapSource {
    /// [`None`].
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
    /// If the call to [`Map::get`] returns [`None`], returns [`None`].
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
    /// If the call to [`Map::get`] returns [`None`], returns [`None`].
    PartMap {
        /// The index.
        part: UrlPart,
        /// The [`Map`].
        ///
        /// Flattened
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// The [`Map`].
    Literal(Map<String>),
    /// [`Params::maps`].
    Params(#[suitable(assert = "map_source_params")] StringSource),
    /// [`FunctionArgs::maps`].
    FunctionArg(StringSource),
}

impl MapSource {
    /// [`Self::get`], replacing [`None`] with the sub-error [`MapNotFound`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get`] returns [`None`], returns the sub-error [`MapNotFound`].
    pub fn get_some<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<&'j Map<String>, MapNotFound>, MapSourceError> {
        self.get(task_state, args).map(|x| x.ok_or(MapNotFound))
    }

    /// Get the map.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Map<String>>, MapSourceError> {
        debug!(MapSource::get, self; self._get(task_state, args))
    }

    /// [`Self::get`].
    fn _get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Map<String>>, MapSourceError> {
        Ok(match self {
            Self::None => None,
            Self::If {r#if, r#then, r#else} => if r#if.check(task_state, args)? {get!(?r#then)} else {get!(?r#else)},
            Self::StringMap {value, map} => match map.get(get!(?&value)) {
                Some(source) => get!(?source),
                None         => None
            },
            Self::PartMap {part, map} => match map.get(part.get(&task_state.url).as_deref()) {
                Some(source) => get!(?source),
                None         => None
            },
            Self::Literal    (x) => Some(x),
            Self::Params     (x) => task_state.job.cleaner.params.maps.get(get!(&x)),
            Self::FunctionArg(x) => args.ok_or(NotInFunction)?   .maps.get(get!(&x))
        })
    }
}



impl FromStr for MapSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(name.into())
    }
}

impl From<&str        > for MapSource {fn from(value: &str        ) -> Self {Self::Params(value.into())}}
impl From<String      > for MapSource {fn from(value: String      ) -> Self {Self::Params(value.into())}}
impl From<StringSource> for MapSource {fn from(value: StringSource) -> Self {Self::Params(value       )}}



impl Serialize for MapSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None                            => serializer.serialize_none()?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _                                     => Self::serialize(self, serializer)?,
        })
    }
}

impl<'de> Deserialize<'de> for MapSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(MapSourceVisitor)
    }
}

/// [`Visitor`] for [`MapSource`].
#[derive(Debug)]
struct MapSourceVisitor;

impl<'de> Visitor<'de> for MapSourceVisitor {
    type Value = MapSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string, list, null, or another variant written normally.")
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

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
}

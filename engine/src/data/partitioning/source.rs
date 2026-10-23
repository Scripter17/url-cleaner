//! [`PartitioningSource`].

use crate::prelude::*;

/// Get a [`Partitioning`].
///
/// Defaults to [`Self::None`].
///
/// Null deserializes/serializes into/from [`Self::None`].
///
/// Strings deserialize/serialize into/from [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum PartitioningSource {
    /// [`None`]
    ///
    /// The default.
    #[default]
    None,
    /// [`Params::partitionings`]
    Params(#[suitable(assert = partitioning_source_params)] StringSource),
}

impl PartitioningSource {
    /// [`Self::get`], replacing [`None`] with the sub-error [`PartitioningNotFound`].
    /// # Errors
    /// If [`Self::get`] returns an error, that error is returned.
    ///
    /// If [`Self::get`] returns [`None`], returns the sub-error [`PartitioningNotFound`].
    pub fn get_some<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<&'j Partitioning, PartitioningNotFound>, PartitioningSourceError> {
        self.get(ts, args).map(|x| x.ok_or(PartitioningNotFound))
    }

    /// Get the [`Partitioning`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Partitioning>, PartitioningSourceError> {
        Ok(match self {
            Self::None => None,
            Self::Params(StringSource::String(x)) => ts.job.cleaner.params.partitionings.get(x),
            _ => self._get(ts, args)?,
        })
    }

    /// [`Self::get`] without fast paths.
    #[cold]
    fn _get<'j>(&'j self, ts: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Partitioning>, PartitioningSourceError> {
        Ok(match self {
            Self::None         => None,
            Self::Params(name) => ts.job.cleaner.params.partitionings.get(&*name.get_some(ts, args)??),
        })
    }
}



impl FromStr for PartitioningSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(name.into())
    }
}

impl From<&str        > for PartitioningSource {fn from(value: &str        ) -> Self {Self::Params(value.into())}}
impl From<String      > for PartitioningSource {fn from(value: String      ) -> Self {Self::Params(value.into())}}
impl From<StringSource> for PartitioningSource {fn from(value: StringSource) -> Self {Self::Params(value       )}}



impl Serialize for PartitioningSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(match self {
            Self::None                            => serializer.serialize_none()?,
            Self::Params(StringSource::String(x)) => serializer.serialize_str(x)?,
            _                                     => Self::serialize(self, serializer)?
        })
    }
}

impl<'de> Deserialize<'de> for PartitioningSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Debug)]
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = PartitioningSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a none, str, string, or map.")
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

        deserializer.deserialize_any(V)
    }
}

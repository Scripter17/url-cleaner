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
    #[default]
    None,
    /// [`Params::partitionings`]
    Params(StringSource),
}

impl PartitioningSource {
    /// [`Self::get`], replacing [`None`] with the sub-error [`PartitioningNotFound`].
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get`] returns [`None`], returns the sub-error [`PartitioningNotFound`].
    pub fn get_some<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Result<&'j Partitioning, PartitioningNotFound>, PartitioningSourceError> {
        self.get(task_state, args).map(|x| x.ok_or(PartitioningNotFound))
    }

    /// Get the [`Partitioning`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Partitioning>, PartitioningSourceError> {
        debug!(PartitioningSource::get, self; match self {
            Self::Params(StringSource::String(x)) => Ok(task_state.job.cleaner.params.partitionings.get(x)),
            _ => self._get(task_state, args),
        })
    }

    /// [`Self::get`].
    fn _get<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>) -> Result<Option<&'j Partitioning>, PartitioningSourceError> {
        Ok(match self {
            Self::None         => None,
            Self::Params(name) => task_state.job.cleaner.params.partitionings.get(get!(&name)),
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
        deserializer.deserialize_any(PartitioningSourceVisitor)
    }
}

/// [`Visitor`] for [`PartitioningSource`].
#[derive(Debug)]
struct PartitioningSourceVisitor;

impl<'de> Visitor<'de> for PartitioningSourceVisitor {
    type Value = PartitioningSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string, none, or another variant written normally.")
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

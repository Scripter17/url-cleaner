//! [`Task`] and co.

use crate::prelude::*;

/// A task for a [`Job`] to [`Job::do`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct Task {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context.
    ///
    /// Defaults to [`TaskContext::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

impl Serialize for Task {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <Task>::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Task;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expected a string or a map.")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Self::Value::from_str(s).map_err(E::custom)
            }

            fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(V)
    }
}

impl Task {
    /// Make a new [`Self`].
    /// # Errors
    /// If [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(task: T) -> Result<Self, T::Error> {
        task.try_into()
    }
}

impl From<BetterUrl> for Task {fn from(url: BetterUrl) -> Self {Self {url, context: Default::default()}}}

impl FromStr for Task {
    type Err = MakeTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes() {
            [b'{' | b'"'                  , ..] => serde_json::from_str(s)?,
            [b'a' ..= b'z' | b'A' ..= b'Z', ..] => BetterUrl::new(s)?.into(),
            [] => Err(MakeTaskError::IgnoreLineNotIgnored)?,
            _  => Err(MakeTaskError::OtherwiseInvalid)?
        })
    }
}

impl TryFrom<&str> for Task {
    type Error = MakeTaskError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&[u8]> for Task {
    type Error = MakeTaskError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(match value {
            [b'{' | b'"'                  , ..] => serde_json::from_slice(value)?,
            [b'a' ..= b'z' | b'A' ..= b'Z', ..] => BetterUrl::new(value)?.into(),
            [] => Err(MakeTaskError::IgnoreLineNotIgnored)?,
            _  => Err(MakeTaskError::OtherwiseInvalid)?
        })
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.context.is_empty() {
            true  => write!(formatter, "{}", self.url),
            // TODO: This is dumb.
            false => write!(formatter, "{}", serde_json::to_string(self).expect("To always work."))
        }
    }
}

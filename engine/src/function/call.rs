//! [`FunctionCall`]

use crate::prelude::*;

/// Instructions on how to call a [`Functions`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct FunctionCall {
    /// The name of the [`Functions`] thing to call.
    pub name: String,
    /// The [`FunctionArgs`].
    ///
    /// Defaults to [`FunctionArgs::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: FunctionArgs
}

impl Serialize for FunctionCall {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <FunctionCall>::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for FunctionCall {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = FunctionCall;

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

impl FromStr for FunctionCall {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for FunctionCall {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

impl From<String> for FunctionCall {
    fn from(name: String) -> Self {
        Self {
            name,
            args: Default::default()
        }
    }
}


//! [`QueryParamSelector`].

use crate::prelude::*;

/// Allows getting and setting specific instances of a query parameter.
///
/// For example, it allows getting and setting the second `a` in `https://example.com?a=1&a=2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct QueryParamSelector {
    /// The name of the query parameter to get.
    pub name: String,
    /// The index of matching query parameters to get.
    ///
    /// Defaults to `0`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub index: isize,
}

impl Serialize for QueryParamSelector {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <QueryParamSelector>::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for QueryParamSelector {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = QueryParamSelector;

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

impl FromStr for QueryParamSelector {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for QueryParamSelector {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for QueryParamSelector {
    fn from(value: String) -> Self {
        Self {
            name: value,
            index: Default::default()
        }
    }
}


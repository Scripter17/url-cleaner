#[cfg(feature = "regex"   )] mod regex;
#[cfg(feature = "regex"   )] pub use regex::*;
#[cfg(feature = "glob"    )] mod glob;
#[cfg(feature = "glob"    )] pub use glob::*;
#[cfg(feature = "commands")] mod command;
#[cfg(feature = "commands")] pub use command::*;
/// Serializing and deserializing [`reqwest::header::HeaderMap`].
#[cfg(all(feature = "http", not(target_family = "wasm")))]
pub mod headermap;

/// Macro that allows types to be have [`serde::Deserialize`] use [`std::str::FromStr`] as a fallback.
/// See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.
#[macro_export]
macro_rules! string_or_struct_magic {
    ($type:ty) => {
        impl Serialize for $type {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                <$type>::serialize(self, serializer)
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct V;

                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $type;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                        f.write_str("Expected a string or a struct.")
                    }

                    fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                        <$type>::from_str(s).map_err(E::custom)
                    }

                    fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                        <$type>::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    }
                }

                deserializer.deserialize_any(V)
            }
        }

        impl TryFrom<&str> for $type {
            type Error = <$type as FromStr>::Err;

            fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
                <$type>::from_str(value)
            }
        }
    }
}

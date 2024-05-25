//! Various macros to make repetitive tasks simpler and cleaner.

/// Macro that allows types to be have [`serde::Deserialize`] use [`std::str::FromStr`] as a fallback.
/// 
/// See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.
macro_rules! string_or_struct_magic {
    ($type:ty) => {
        /// Serialize the object. Although the macro this implementation came from allows [`Self::deserialize`]ing from a string, this currently always serializes to a map, though that may change eventually.
        impl Serialize for $type {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                <$type>::serialize(self, serializer)
            }
        }

        /// This particular implementation allows for deserializing from a string using [`Self::from_str`].
        /// 
        /// See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct V;

                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $type;

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
    }
}

/// A macro that makes handling the difference between [`StringSource`] and [`String`] easier.
macro_rules! get_string {
    ($value:expr, $job_state:expr, $error:ty) => {
        $value.get($job_state)?.ok_or(<$error>::StringSourceIsNone)?.into_owned()
    }
}

/// A macro that makes handling the difference between [`StringSource`] and [`String`] easier.
macro_rules! get_str {
    ($value:expr, $job_state:expr, $error:ty) => {
        &*$value.get($job_state)?.ok_or(<$error>::StringSourceIsNone)?
    }
}

/// A macro that makes handling the difference between [`Option`]s of [`StringSource`] and [`String`] easier.
macro_rules! get_option_string {
    ($value:expr, $job_state:expr) => {
        $value.as_ref().map(|source| source.get($job_state)).transpose()?.flatten().map(|x| x.into_owned())
    }
}

/// A macro that makes handling the difference between [`Option`]s of [`StringSource`] and [`String`] easier.
macro_rules! get_option_str {
    ($value:expr, $job_state:expr) => {
        $value.as_ref().map(|source| source.get($job_state)).transpose()?.flatten().as_deref()
    }
}

pub(crate) use string_or_struct_magic;
pub(crate) use get_str;
pub(crate) use get_string;
pub(crate) use get_option_str;
pub(crate) use get_option_string;

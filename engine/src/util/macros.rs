//! Macros.

/// Helper macro to make serde use [`FromStr`] to deserialize strings.
///
/// See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.
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

/// Helper macro to get a [`StringSource`]'s value as a [`String`] or return an error if it's [`None`].
macro_rules! get_string {
    ($value:expr, $job_state:expr, $error:ty) => {
        $value.get(&$job_state.to_view())?.ok_or(<$error>::StringSourceIsNone)?.into_owned()
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`str`] or return an error if it's [`None`].
macro_rules! get_str {
    ($value:expr, $job_state:expr, $error:ty) => {
        &*$value.get(&$job_state.to_view())?.ok_or(<$error>::StringSourceIsNone)?
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`Cow`] or return an error if it's [`None`].
macro_rules! get_cow {
    ($value:expr, $job_state:expr, $error:ty) => {
        $value.get(&$job_state.to_view())?.ok_or(<$error>::StringSourceIsNone)?
    }
}

/// Helper macro to impl [`From`] for unit types like [`SetPortError`].
macro_rules! from_units {
    ($sum:ty, $($unit:tt),*) => {
        $(impl From<$unit> for UrlPartSetError {
            #[doc = concat!("[`Self::", stringify!($unit), "`]")]
            fn from(value: $unit) -> Self {
                // Ensures the type is actually a unit.
                match value {$unit => Self::$unit}
            }
        })*
    }
}

pub(crate) use string_or_struct_magic;
pub(crate) use get_str;
pub(crate) use get_string;
pub(crate) use get_cow;
pub(crate) use from_units;
pub(crate) use url_cleaner_macros::edoc;

//! Macros.

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::str::FromStr;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::borrow::Cow;

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

/// Helper macro to get a [`StringSource`]'s value as an [`Option`] of a [`String`].
macro_rules! get_option_string {
    ($value:expr, $task_state:expr) => {
        get_option_cow!($value, $task_state).map(std::borrow::Cow::into_owned)
    }
}

/// Helper macro to get a [`StringSource`]'s value as an [`Option`] of a [`str`].
macro_rules! get_option_str {
    ($value:expr, $task_state:expr) => {
        get_option_cow!($value, $task_state).as_deref()
    }
}

/// Helper macro to get a [`StringSource`]'s value as an [`Option`] of a [`str`].
macro_rules! get_new_option_str {
    ($value:expr, $task_state:expr) => {
        match $value {
            StringSource::String(value) => Some(std::borrow::Cow::Borrowed(value.as_str())),
            StringSource::None => None,
            value => value.get($task_state)?.map(std::borrow::Cow::into_owned).map(Cow::Owned)
        }.as_deref()
    }
}

/// Helper macro to get a [`StringSource`]'s value as an [`Option`] of a [`Cow`].
macro_rules! get_option_cow {
    ($value:expr, $task_state:expr) => {
        match $value {
            StringSource::String(value) => Some(std::borrow::Cow::Borrowed(value.as_str())),
            StringSource::None => None,
            value => value.get($task_state)?
        }
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`String`] or return an error if it's [`None`].
#[allow(unused_macros, reason = "Used when some features are enabled.")]
macro_rules! get_string {
    ($value:expr, $task_state:expr, $error:ty) => {
        get_cow!($value, $task_state, $error).into_owned()
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`str`] or return an error if it's [`None`].
macro_rules! get_str {
    ($value:expr, $task_state:expr, $error:ty) => {
        &*get_cow!($value, $task_state, $error)
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`str`] or return an error if it's [`None`].
macro_rules! get_new_str {
    ($value:expr, $task_state:expr, $error:ty) => {
        &*match $value.get_self() {
            StringSource::String(value) => std::borrow::Cow::Borrowed(value.as_str()),
            value => Cow::Owned(value.get($task_state)?.ok_or(<$error>::StringSourceIsNone)?.into_owned())
        }
    }
}

/// Helper macro to get a [`StringSource`]'s value as a [`Cow`] or return an error if it's [`None`].
macro_rules! get_cow {
    ($value:expr, $task_state:expr, $error:ty) => {
        match $value.get_self() {
            StringSource::String(value) => std::borrow::Cow::Borrowed(value.as_str()),
            value => value.get($task_state)?.ok_or(<$error>::StringSourceIsNone)?
        }
    }
}

pub(crate) use string_or_struct_magic;
pub(crate) use get_str;
pub(crate) use get_new_str;
pub(crate) use get_string;
pub(crate) use get_cow;
pub(crate) use get_option_str;
pub(crate) use get_new_option_str;
pub(crate) use get_option_string;
pub(crate) use get_option_cow;
pub(crate) use url_cleaner_macros::edoc;

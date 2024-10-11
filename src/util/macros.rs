//! Various macros to make repetitive tasks simpler and cleaner.

#[cfg(feature = "debug")]
use std::sync::Mutex;

/// Used by [`debug`] to control the indentation level.
#[cfg(feature = "debug")]
pub(crate) static DEBUG_INDENT: Mutex<usize> = Mutex::new(0);

/// The thing that decrements [`DEBUG_INDENT`] when dropped.
#[cfg(feature = "debug")]
pub(crate) struct Deindenter;

/// Decrements [`DEBUG_INDENT`].
#[cfg(feature = "debug")]
impl std::ops::Drop for Deindenter {
    /// Decrements [`DEBUG_INDENT`]
    #[allow(clippy::arithmetic_side_effects, reason = "God help you if your config gets [`usize::MAX`] layers deep.")]
    fn drop(&mut self) {
        *crate::util::DEBUG_INDENT.lock().expect("The DEBUG_INDENT mutex to never be poisoned.")-=1;
    }
}

/// Print debugging for the win!
macro_rules! debug {
    ($func:pat, $($comment:literal,)? $($name:ident),*) => {
        #[cfg(feature = "debug")]
        #[allow(clippy::arithmetic_side_effects, reason = "God help you if your config gets [`usize::MAX`] layers deep.")]
        let _deindenter = {
            let mut indent = crate::util::DEBUG_INDENT.lock().expect("The DEBUG_INDENT mutex to never be poisoned.");
            eprint!("{}{}", "|   ".repeat(*indent), stringify!($func));
            $(eprint!($comment);)?
            $(eprint!(concat!("; ", stringify!($name), ": {:?}"), $name);)*
            eprintln!();
            *indent+=1;
            crate::util::Deindenter
        };
    }
}

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
        $value.get(&$job_state.to_view())?.ok_or(<$error>::StringSourceIsNone)?.into_owned()
    }
}

/// A macro that makes handling the difference between [`StringSource`] and [`String`] easier.
macro_rules! get_str {
    ($value:expr, $job_state:expr, $error:ty) => {
        &*$value.get(&$job_state.to_view())?.ok_or(<$error>::StringSourceIsNone)?
    }
}

/// A macro that makes handling the difference between [`Option`]s of [`StringSource`] and [`String`] easier.
macro_rules! get_option_string {
    ($value:expr, $job_state:expr) => {
        // $value.as_ref().map(|source| source.get(&$job_state.to_view())).transpose()?.flatten().map(|x| x.into_owned())
        {
            let view = &$job_state.to_view();
            match $value.as_ref() {
                Some(source) => source.get(view),
                None => Ok(None)
            }?.map(|x| x.into_owned())
        }
    }
}

/// A macro that makes handling the difference between [`Option`]s of [`StringSource`] and [`String`] easier.
macro_rules! get_option_str {
    ($value:expr, $job_state:expr) => {
        // $value.as_ref().map(|source| source.get(&$job_state.to_view())).transpose()?.flatten().as_deref()
        {
            let view = &$job_state.to_view();
            match $value.as_ref() {
                Some(source) => source.get(view),
                None => Ok(None)
            }?.as_deref()
        }
    }
}

/// Shorthand for checking a [`Config::docs`] has an entry for [`StringSource::String`] flags/sets/etc..
macro_rules! check_docs {
    ($config:expr, $type:ident, $name:expr) => {
        match $name {
            StringSource::String(name) => if $config.docs.$type.contains_key(name) {
                true
            } else {
                panic!(concat!("Missing docs for ", stringify!($type), " {}"), name)
            },
            _ => true
        }
    }
}

pub(crate) use debug;
pub(crate) use string_or_struct_magic;
pub(crate) use get_str;
pub(crate) use get_string;
pub(crate) use get_option_str;
pub(crate) use get_option_string;
pub(crate) use check_docs;

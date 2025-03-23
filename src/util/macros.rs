//! Macros.

#[cfg(feature = "debug")]
use std::sync::{Mutex, OnceLock};
#[cfg(feature = "debug")]
pub(crate) static DEBUG_INDENT: Mutex<usize> = Mutex::new(0);

#[cfg(feature = "debug")]
pub(crate) static DEBUG_TIME: Mutex<Option<std::time::Instant>> = Mutex::new(None);

#[cfg(feature = "debug")]
pub(crate) static DEBUG_JUST_PRINT_TIMES: OnceLock<bool> = OnceLock::new();
#[cfg(feature = "debug")]
pub(crate) struct Deindenter;
#[cfg(feature = "debug")]
impl std::ops::Drop for Deindenter {
    #[allow(clippy::arithmetic_side_effects, reason = "DEBUG_INDENT gets decremented exactly once per increment and always after.")]
    fn drop(&mut self) {
        *crate::util::DEBUG_INDENT.lock().expect("The DEBUG_INDENT mutex to never be poisoned.")-=1;
    }
}

/// When the debug feature is enabled, print debug info.
macro_rules! debug {
    ($func:pat, $($comment:literal,)? $($name:ident),*) => {
        #[cfg(feature = "debug")]
        #[allow(clippy::arithmetic_side_effects, reason = "God help you if your config gets [`usize::MAX`] layers deep.")]
        let _deindenter = {
            let mut time = crate::util::DEBUG_TIME.lock().expect("The DEBUG_TIME mutex to never be poisoned.");
            match *time {
                Some(x) => eprint!("{:>8?}", x.elapsed()),
                None => eprint!("        ")
            }
            let mut indent = crate::util::DEBUG_INDENT.lock().expect("The DEBUG_INDENT mutex to never be poisoned.");
            if !*crate::util::DEBUG_JUST_PRINT_TIMES.get().expect("The DEBUG_JUST_PRINT_TIMES OnceLock to be set.") {
                eprint!("\t{}{}", "|   ".repeat(*indent), stringify!($func));
                $(eprint!($comment);)?
                $(eprint!(concat!("; ", stringify!($name), ": {:?}"), $name);)*
            }
            eprintln!();
            *indent+=1;
            *time = Some(std::time::Instant::now());
            crate::util::Deindenter
        };
    }
}

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

pub(crate) use debug;
pub(crate) use string_or_struct_magic;
pub(crate) use get_str;
pub(crate) use get_string;
pub(crate) use get_cow;

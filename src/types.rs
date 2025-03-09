//! The core tools of URL Cleaner.

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use thiserror::Error;
use serde::{Serialize, Deserialize};

pub mod better_url;
pub use better_url::*;
pub mod url_part;
pub use url_part::*;
pub mod config;
pub use config::*;
pub mod tests;
pub use tests::*;
pub mod rules;
pub use rules::*;
pub mod string_location;
pub use string_location::*;
pub mod string_modification;
pub use string_modification::*;
pub mod string_source;
pub use string_source::*;
pub mod string_matcher;
pub use string_matcher::*;
pub mod char_matcher;
pub use char_matcher::*;
pub mod jobs;
pub use jobs::*;

use crate::util::*;

/// Wrapper around a function pointer that fakes [`Serialize`] and [`Deserialize`] implementations.
/// 
/// Please note that, once it's stabilized, this will require [`T: FnPtr`](FnPtr) and that will not be considered a breaking change.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[cfg(feature = "custom")]
pub struct FnWrapper<T>(pub T);

#[cfg(feature = "custom")]
impl<T> std::ops::Deref for FnWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "custom")]
impl<T> std::ops::DerefMut for FnWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "custom")]
impl<T> serde::Serialize for FnWrapper<T> {
    /// Always returns [`Err`].
    fn serialize<S: serde::ser::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        Err(S::Error::custom("FnWrapper fakes its Serialize impl."))
    }
}

#[cfg(feature = "custom")]
impl<'de, T> serde::Deserialize<'de> for FnWrapper<T> {
    /// Always returns [`Err`].
    fn deserialize<D: serde::de::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        Err(D::Error::custom("FnWrapper fakes its Deserialize impl."))
    }
}

/// Helper type to handle [`HashMap`]s with fallbacks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map<T> {
    /// The map to branch with.
    pub map: HashMap<String, T>,
    /// The value to return if the provided key is [`None`].
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_null: Option<Box<T>>,
    /// The value to return if neither [`Self::map`] not [`Self::if_null`] return anything.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Box<T>>
}

impl<T> Map<T> {
    /// Index tha map
    ///
    /// 1. If `value` is `Some(k)` and `k` is in [`Self::map`], return [`Self::map`]'s value for `k`.
    /// 2. If `value` is `None` and [`Self::if_null`] is `Some(v)`, return `Some(v)`.
    /// 3. If 1 and 2 both don't return anything and [`Self::r#else`] is `Some(v)`, return `Some(v)`.
    /// 4. If 1, 2, and 3 all don't return anything, return [`None`].
    pub fn get<U: AsRef<str>>(&self, value: Option<U>) -> Option<&T> {
        value.map(|x| self.map.get(x.as_ref())).unwrap_or(self.if_null.as_deref()).or(self.r#else.as_deref())
    }
}

/// Allows changing [`Map`]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapDiff<T> {
    /// The values to insert into [`Map::map`].
    #[serde(default)]
    pub insert_into_map: HashMap<String, T>,
    /// The values to remove from [`Map::map`].
    #[serde(default)]
    pub remove_from_map: HashSet<String>
}

impl<T> MapDiff<T> {
    /// Applies the differences.
    ///
    /// 1. Extends `to.map` with `self.insert_into_map`.
    /// 2. Removes all keys found in `self.remove_from_map` from `to.map`.
    pub fn apply(self, to: &mut Map<T>) {
        to.map.extend(self.insert_into_map);
        to.map.retain(|k, _| !self.remove_from_map.contains(k));
    }
}

/// Internal trait to handle whether or not a value is "suitable" for being in the default config.
///
/// Mainly about ensuring documentation and no `Debug` variants.
pub(crate) trait Suitable {
    /// Returns [`true`] if [`self`] is "suitable" for being in the default config.
    ///
    /// May panic with an error message.
    fn is_suitable_for_release(&self, config: &Config) -> bool;
}

impl<T: Suitable> Suitable for Map<T> {
    fn is_suitable_for_release(&self, config: &Config) -> bool {
        self.map.values().all(|x| x.is_suitable_for_release(config)) &&
            self.if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) &&
            self.r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config))
    }
}

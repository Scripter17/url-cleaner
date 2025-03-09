//! Extends [`HashMap`]s to have fallback values.

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

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
    /// Index the map
    ///
    /// 1. If `value` is `Some(k)` and `k` is in [`Self::map`], return [`Self::map`]'s value for `k`.
    /// 2. If `value` is `None` and [`Self::if_null`] is `Some(v)`, return `Some(v)`.
    /// 3. If 1 and 2 both don't return anything and [`Self::else`] is `Some(v)`, return `Some(v)`.
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

impl<T: Suitable> Suitable for Map<T> {
    fn is_suitable_for_release(&self, config: &Config) -> bool {
        self.map.values().all(|x| x.is_suitable_for_release(config)) &&
            self.if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) &&
            self.r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config))
    }
}

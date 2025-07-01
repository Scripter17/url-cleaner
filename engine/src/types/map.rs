//! Faster `HashMap<Option<String>, _>` with fallbacks.

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, MapPreventDuplicates, SetPreventDuplicates};

use crate::util::*;

/// Allows semantics similar to `HashMap<Option<String>, _>` without having to convert `Option<&str>`s to `Option<String>`s.
///
/// Also has [`Self::else`] to specify a return value when a key isn't otherwise found.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct Map<T: Debug> {
    /// The map from [`Some`] to `T`.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    pub map: HashMap<String, T>,
    /// The map from [`None`] to `T`.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_null: Option<Box<T>>,
    /// The value to return when a value is otherwise not found.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Box<T>>
}

impl<T: Debug> Map<T> {
    /// [`HashMap::with_capacity`].
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            map    : HashMap::with_capacity(capacity),
            if_null: None,
            r#else : None
            
        }
    }

    /// If [`Some`], returns the corresponding value from [`Self::map`].
    ///
    /// If [`None`], returns the value of [`Self::if_null`].
    ///
    /// If either of the above return [`None`], returns the value of [`Self::else`].
    pub fn get<U: Debug + AsRef<str>>(&self, key: Option<U>) -> Option<&T> {
        debug!(Map::get, self, key);
        match key {
            Some(key) => self.map.get(key.as_ref()),
            None => self.if_null.as_deref()
        }.or(self.r#else.as_deref())
    }
}

impl<T: Debug> Default for Map<T> {
    fn default() -> Self {
        Self {
            map    : Default::default(),
            if_null: Default::default(),
            r#else : Default::default()
        }
    }
}

/// Rules for updating a [`Map`].
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct MapDiff<T> {
    /// Values to insert/replace into [`Map::map`].
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
    pub insert: HashMap<String, T>,
    /// Values to remove from [`Map::map`].
    #[serde_as(as = "SetPreventDuplicates<_>")]
    #[serde(default)]
    pub remove: HashSet<String>
}

impl<T: Debug> MapDiff<T> {
    /// Applies the diff.
    pub fn apply(self, to: &mut Map<T>) {
        to.map.extend(self.insert);
        to.map.retain(|k, _| !self.remove.contains(k));
    }
}

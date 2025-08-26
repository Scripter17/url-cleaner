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
#[serde(deny_unknown_fields)]
pub struct Map<T: Debug> {
    /// The map from [`Some`] to `T`.
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    pub map: HashMap<String, T>,
    /// The map from [`None`] to `T`.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_none: Option<Box<T>>,
    /// The value to return when a value is otherwise not found.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Box<T>>
}

impl<T: Debug> Map<T> {
    /// [`HashMap::with_capacity`].
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            map    : HashMap::with_capacity(capacity),
            if_none: None,
            r#else : None
        }
    }

    /// If [`Some`], returns the corresponding value from [`Self::map`].
    ///
    /// If [`None`], returns the value of [`Self::if_none`].
    ///
    /// If either of the above return [`None`], returns the value of [`Self::else`].
    pub fn get<U: Debug + AsRef<str>>(&self, key: Option<U>) -> Option<&T> {
        debug!(Map::get, self, key);
        match key {
            Some(key) => self.map.get(key.as_ref()),
            None => self.if_none.as_deref()
        }.or(self.r#else.as_deref())
    }
}

impl<T: Debug> Default for Map<T> {
    fn default() -> Self {
        Self {
            map    : Default::default(),
            if_none: Default::default(),
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
    ///
    /// If you want to apply `self` multiple times, use [`Self::apply_multiple`] as it's slightly faster than [`Clone::clone`]ing this then using [`Self::apply_once`] on each clone.
    pub fn apply_once(self, to: &mut Map<T>) {
        debug!(MapDiff::apply_once, &self, to);
        to.map.extend(self.insert);
        to.map.retain(|k, _| !self.remove.contains(k));
    }
}

impl<T: Debug + Clone> MapDiff<T> {
    /// Applies the diff.
    ///
    /// If you only want to apply `self` once, use [`Self::apply_once`].
    pub fn apply_multiple(&self, to: &mut Map<T>) {
        debug!(MapDiff::apply_multiple, self, to);
        to.map.extend(self.insert.iter().map(|(k, v)| (k.clone(), v.clone())));
        to.map.retain(|k, _| !self.remove.contains(k));
    }
}

impl<T: Debug, const N: usize> From<[(String, T); N]> for Map<T> {
    fn from(value: [(String, T); N]) -> Self {
        Self {
            map: value.into(),
            if_none: None,
            r#else: None
        }
    }
}

impl<T: Debug, const N: usize> From<[(Option<String>, T); N]> for Map<T> {
    fn from(value: [(Option<String>, T); N]) -> Self {
        let mut ret = Self {
            map: HashMap::with_capacity(N),
            if_none: None,
            r#else: None
        };

        for (k, v) in value {
            match k {
                Some(k) => {ret.map.insert(k, v);},
                None => ret.if_none = Some(Box::new(v))
            }
        }

        ret
    }
}

impl<T: Debug> From<HashMap<String, T>> for Map<T> {
    fn from(value: HashMap<String, T>) -> Self {
        Self {
            map: value,
            if_none: None,
            r#else: None
        }
    }
}

impl<T: Debug> From<HashMap<Option<String>, T>> for Map<T> {
    fn from(value: HashMap<Option<String>, T>) -> Self {
        let mut ret = Self {
            map: HashMap::with_capacity(value.len()),
            if_none: None,
            r#else: None
        };

        for (k, v) in value {
            match k {
                Some(k) => {ret.map.insert(k, v);},
                None => ret.if_none = Some(Box::new(v))
            }
        }

        ret
    }
}

impl<T: Debug> FromIterator<(String, T)> for Map<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().collect(),
            if_none: None,
            r#else: None
        }
    }
}

impl<T: Debug> FromIterator<(Option<String>, T)> for Map<T> {
    fn from_iter<I: IntoIterator<Item = (Option<String>, T)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint();
        let mut ret = Self {
            map: HashMap::with_capacity(size_hint.1.unwrap_or(size_hint.0)),
            if_none: None,
            r#else: None
        };
        for (k, v) in iter {
            match k {
                Some(k) => {ret.map.insert(k, v);},
                None => ret.if_none = Some(Box::new(v))
            }
        }
        ret
    }
}

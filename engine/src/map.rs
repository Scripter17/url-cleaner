//! [`Map`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, MapPreventDuplicates};

use crate::prelude::*;

/// A `HashMap<Option<String>, T>` that allows indexing with `Option<&str>`.
///
/// Also has [`Self::else`] to specify a return value when a key isn't otherwise found.
///
/// Please note that the components that use [`Map`] generally use [`#[serde(flatten)]`](https://serde.rs/field-attrs.html#flatten).
///
/// A component that looks like it'd be written as
///
/// ```Json
/// {
///   "map": {
///     "map": {
///       "a": "b",
///       "c": "d"
///     },
///     "if_none": "e",
///     "else": "f"
///   }
/// }
/// ```
///
/// would instead be written as just
///
/// ```Json
/// {
///   "map": {
///     "a": "b",
///     "c": "d"
///   },
///   "if_none": "e",
///   "else": "f"
/// }
/// ```
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Map<T> {
    /// The map from [`Some`] to `T`.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    pub map: HashMap<String, T>,
    /// The map from [`None`] to `T`.
    ///
    /// Defaults to [`None`].
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_none: Option<T>,
    /// The value to return when a value is otherwise not found.
    ///
    /// Defaults to [`None`].
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<T>
}

impl<T> Map<T> {
    /// [`HashMap::with_capacity`].
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            map    : HashMap::with_capacity(capacity),
            if_none: None,
            r#else : None
        }
    }

    /// [`HashMap::get`].
    ///
    /// If [`Some`], returns the corresponding value from [`Self::map`].
    ///
    /// If [`None`], returns the value of [`Self::if_none`].
    ///
    /// If either of the above return [`None`], returns the value of [`Self::else`].
    pub fn get<U: AsRef<str>>(&self, key: Option<U>) -> Option<&T> {
        match key {
            Some(key) => self.map.get(key.as_ref()),
            None => self.if_none.as_ref()
        }.or(self.r#else.as_ref())
    }

    /// [`HashMap::remove`].
    pub fn remove<U: AsRef<str>>(&mut self, key: Option<U>) -> Option<T> {
        match key {
            Some(key) => self.map.remove(key.as_ref()),
            None => self.if_none.take()
        }
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self {
            map    : Default::default(),
            if_none: Default::default(),
            r#else : Default::default()
        }
    }
}

impl<T, const N: usize> From<[(String, T); N]> for Map<T> {
    fn from(value: [(String, T); N]) -> Self {
        Self {
            map: value.into(),
            if_none: None,
            r#else: None
        }
    }
}

impl<T, const N: usize> From<[(Option<String>, T); N]> for Map<T> {
    fn from(value: [(Option<String>, T); N]) -> Self {
        let mut ret = Self {
            map: HashMap::with_capacity(N),
            if_none: None,
            r#else: None
        };

        for (k, v) in value {
            match k {
                Some(k) => {ret.map.insert(k, v);},
                None => ret.if_none = Some(v)
            }
        }

        ret
    }
}

impl<T> From<HashMap<String, T>> for Map<T> {
    fn from(value: HashMap<String, T>) -> Self {
        Self {
            map: value,
            if_none: None,
            r#else: None
        }
    }
}

impl<T> From<HashMap<Option<String>, T>> for Map<T> {
    fn from(value: HashMap<Option<String>, T>) -> Self {
        let mut ret = Self {
            map: HashMap::with_capacity(value.len()),
            if_none: None,
            r#else: None
        };

        for (k, v) in value {
            match k {
                Some(k) => {ret.map.insert(k, v);},
                None => ret.if_none = Some(v)
            }
        }

        ret
    }
}

impl<T> FromIterator<(String, T)> for Map<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().collect(),
            if_none: None,
            r#else: None
        }
    }
}

impl<T> FromIterator<(Option<String>, T)> for Map<T> {
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
                None => ret.if_none = Some(v)
            }
        }
        ret
    }
}

impl<T> Extend<(String, T)> for Map<T> {
    fn extend<I: IntoIterator<Item = (String, T)>>(&mut self, iter: I) {
        self.map.extend(iter)
    }
}

impl<T> Extend<(Option<String>, T)> for Map<T> {
    fn extend<I: IntoIterator<Item = (Option<String>, T)>>(&mut self, iter: I) {
        for (k, v) in iter {
            match k {
                Some(k) => {self.map.insert(k, v);},
                None => self.if_none = Some(v)
            }
        }
    }
}

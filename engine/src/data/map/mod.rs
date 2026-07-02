//! [`Map`].

use crate::prelude::*;

mod source;
mod iter;
mod into_iter;

pub use source::*;
pub use iter::*;
pub use into_iter::*;

/// A `HashMap<Option<String>, T>` that allows indexing with `Option<&str>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Map<T> {
    /// The map from [`Some`] to `T`.
    ///
    /// Defaulted.
    pub map: HashMap<String, T>,
    /// The map from [`None`] to `T`.
    ///
    /// Defaulted.
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_none: Option<T>,
}

impl<T> Map<T> {
    /// [`HashMap::get`].
    ///
    /// If [`Some`], returns the corresponding value from [`Self::map`].
    ///
    /// If [`None`], returns the value of [`Self::if_none`].
    pub fn get<'a>(&'a self, key: Option<&str>) -> Option<&'a T> {
        match key {
            Some(key) => self.map.get(key),
            None      => self.if_none.as_ref(),
        }
    }

    /// [`HashMap::remove`].
    pub fn remove(&mut self, key: Option<&str>) -> Option<T> {
        match key {
            Some(key) => self.map.remove(key),
            None      => self.if_none.take(),
        }
    }

    /// Keep only elements satisfying `f`.
    pub fn retain<F: FnMut(Option<&String>, &T) -> bool>(&mut self, mut f: F) {
        self.map.retain(|k, v| f(Some(k), v));
        if let Some(v) = &self.if_none && !f(None, v) {
            self.if_none = None;
        }
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self {
            map    : Default::default(),
            if_none: Default::default(),
        }
    }
}



impl<T> FromIterator<(String, T)> for Map<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl<T> Extend<(String, T)> for Map<T> {
    fn extend<I: IntoIterator<Item = (String, T)>>(&mut self, iter: I) {
        self.map.extend(iter)
    }
}



impl<T> FromIterator<(Option<String>, T)> for Map<T> {
    fn from_iter<I: IntoIterator<Item = (Option<String>, T)>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
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

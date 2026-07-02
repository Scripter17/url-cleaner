//! [`MapIntoIter`].

use crate::prelude::*;

/// An owning [`Iterator`] of a [`Map`].
#[derive(Debug)]
pub struct MapIntoIter<T> {
    /// The [`Some`]s.
    map: std::collections::hash_map::IntoIter<String, T>,
    /// The [`None`].
    none: Option<T>
}

impl<T> IntoIterator for Map<T> {
    type IntoIter = MapIntoIter<T>;
    type Item = (Option<String>, T);

    fn into_iter(self) -> Self::IntoIter {
        MapIntoIter {
            map: self.map.into_iter(),
            none: self.if_none
        }
    }
}

impl<T> Iterator for MapIntoIter<T> {
    type Item = (Option<String>, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.map.next() {
            Some((k, v)) => Some((Some(k), v)),
            None => Some((None, self.none.take()?))
        }
    }
}

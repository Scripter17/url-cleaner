//! [`MapIter`].

use crate::prelude::*;

/// A borrowing [`Iterator`] of a [`Map`].
#[derive(Debug)]
pub struct MapIter<'a, T> {
    /// The [`Some`]s.
    map: std::collections::hash_map::Iter<'a, String, T>,
    /// The [`None`].
    none: Option<&'a T>,
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type IntoIter = MapIter<'a, T>;
    type Item = (Option<&'a String>, &'a T);

    fn into_iter(self) -> Self::IntoIter {
        MapIter {
            map: self.map.iter(),
            none: self.if_none.as_ref()
        }
    }
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = (Option<&'a String>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.map.next() {
            Some((k, v)) => Some((Some(k), v)),
            None => Some((None, self.none.take()?))
        }
    }
}


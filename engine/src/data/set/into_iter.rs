//! [`SetIntoIter`].

use crate::prelude::*;

/// An owning [`Iterator`] over a [`Set`].
#[derive(Debug)]
pub struct SetIntoIter<T> {
    /// [`Set::set`].
    somes: <HashSet::<T> as IntoIterator>::IntoIter,
    /// [`Set::if_none`].
    has_none: bool
}

impl<T: Hash + Eq> IntoIterator for Set<T> {
    type IntoIter = SetIntoIter<T>;
    type Item = Option<T>;

    fn into_iter(self) -> Self::IntoIter {
        SetIntoIter {
            somes   : self.set.into_iter(),
            has_none: self.if_none,
        }
    }
}

impl<T: Hash + Eq> Iterator for SetIntoIter<T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.somes.next() {
            Some(x) => Some(Some(x)),
            None => match self.has_none {
                true  => {self.has_none = false; Some(None)},
                false => None
            }
        }
    }
}

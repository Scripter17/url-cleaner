//! [`SetIter`].

use crate::prelude::*;

/// A borrowing [`Iterator`] over a [`Set`].
#[derive(Debug)]
pub struct SetIter<'a, T> {
    /// [`Set::set`].
    somes: <&'a HashSet<T> as IntoIterator>::IntoIter,
    /// [`Set::if_none`].
    has_none: bool,
}

impl<'a, T: Hash + Eq> IntoIterator for &'a Set<T> {
    type IntoIter = SetIter<'a, T>;
    type Item = Option<&'a T>;

    fn into_iter(self) -> Self::IntoIter {
        SetIter {
            somes   : self.set.iter(),
            has_none: self.if_none,
        }
    }
}

impl<'a, T: Hash + Eq> Iterator for SetIter<'a, T> {
    type Item = Option<&'a T>;

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

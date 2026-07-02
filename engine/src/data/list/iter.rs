//! [`ListIter`].

use crate::prelude::*;

/// A borrowing [`Iterator`] for a [`List`].
#[derive(Debug, Clone)]
pub struct ListIter<'a, T>(pub std::slice::Iter<'a, T>);

impl<'a, T> IntoIterator for &'a List<T> {
    type IntoIter = ListIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self.0.iter())
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

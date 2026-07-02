//! [`ListIntoIter`].

use crate::prelude::*;

/// An owning [`Iterator`] for a [`List`].
#[derive(Debug, Clone)]
pub struct ListIntoIter<T>(pub std::vec::IntoIter<T>);

impl<T> IntoIterator for List<T> {
    type IntoIter = ListIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self.0.into_iter())
    }
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}


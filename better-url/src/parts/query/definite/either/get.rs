//! Getters.

use crate::prelude::*;

impl<'a> Query<'a> {
    /// The [`QueryIter`].
    pub fn iter(&self) -> QueryIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`QuerySegment`]s whose [`QueryName::decode`] is `name`.
    pub fn find_iter<T: AsRef<[u8]>>(&self, name: T) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        self.iter().filter(move |x| x.name().decode() == name.as_ref())
    }

    /// The `index`th [`QuerySegment`].
    pub fn get(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QuerySegment`] whose [`QueryName::decode`] is `name`.
    pub fn find<T: AsRef<[u8]>>(&self, name: T, index: isize) -> Option<QuerySegment<'_>> {
        self.find_iter(name.as_ref()).neg_nth(index)
    }
}

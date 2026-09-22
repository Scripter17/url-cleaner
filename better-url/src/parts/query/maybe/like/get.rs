//! Getters.

use crate::prelude::*;

impl<'a> MaybeQueryLike<'a> {
    /// The [`QueryLikeIter`].
    pub fn iter(&self) -> QueryLikeIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`QueryLikeSegment`]s named `name`.
    pub fn find_iter<T: AsRef<[u8]>>(&self, name: T) -> impl DoubleEndedIterator<Item = QueryLikeSegment<'_>> {
        self.iter().filter(move |x| x.name().decode() == name.as_ref())
    }

    /// The `index`th [`QueryLikeSegment`].
    pub fn get(&self, index: isize) -> Option<QueryLikeSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QueryLikeSegment`] named `name`.
    pub fn find<T: AsRef<[u8]>>(&self, name: T, index: isize) -> Option<QueryLikeSegment<'_>> {
        self.find_iter(name.as_ref()).neg_nth(index)
    }
}

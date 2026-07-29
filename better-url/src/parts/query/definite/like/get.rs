//! Getters.

use crate::prelude::*;

impl<'a> QueryLike<'a> {
    /// The [`QueryLikeIter`].
    pub fn iter(&self) -> QueryLikeIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`QueryLikeSegment`]s named `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QueryLikeSegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// The `index`th [`QueryLikeSegment`].
    pub fn get(&self, index: isize) -> Option<QueryLikeSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QueryLikeSegment`] named `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QueryLikeSegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

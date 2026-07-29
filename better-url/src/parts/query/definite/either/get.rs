//! Getters.

use crate::prelude::*;

impl Query<'_> {
    /// The [`QueryIter`].
    pub fn iter(&self) -> QueryIter<'_> {
        self.into_iter()
    }

    /// The [`QuerySegment`]s named `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QuerySegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// The `index`th [`QuerySegment`].
    pub fn get(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QuerySegment`] named `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

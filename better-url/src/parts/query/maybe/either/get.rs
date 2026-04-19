//! Getters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// [`Query::iter`].
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        self.0.as_ref().into_iter().flat_map(Query::iter)
    }

    /// [`Query::find_iter`].
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QuerySegment<'b>> {
        self.0.as_ref().into_iter().flat_map(|x| x.find_iter(name))
    }

    /// [`Query::get`].
    pub fn get(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.0.as_ref()?.get(index)
    }

    /// [`Query::find`].
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QuerySegment<'b>> {
        self.0.as_ref()?.find(name, index)
    }
}

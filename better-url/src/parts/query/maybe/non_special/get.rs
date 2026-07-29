//! Getters.

use crate::prelude::*;

impl MaybeNonSpecialQuery<'_> {
    /// The [`NonSpecialQueryIter`].
    pub fn iter(&self) -> NonSpecialQueryIter<'_> {
        self.into_iter()
    }

    /// [`NonSpecialQuery::find_iter`].
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = NonSpecialQuerySegment<'b>> {
        self.0.iter().flat_map(|x| x.find_iter(name))
    }

    /// [`NonSpecialQuery::get`].
    pub fn get(&self, index: isize) -> Option<NonSpecialQuerySegment<'_>> {
        self.0.as_ref()?.get(index)
    }

    /// [`NonSpecialQuery::find`].
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<NonSpecialQuerySegment<'b>> {
        self.0.as_ref()?.find(name, index)
    }
}


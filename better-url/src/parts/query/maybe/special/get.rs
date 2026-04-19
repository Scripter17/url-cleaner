//! Getters.

use crate::prelude::*;

impl MaybeSpecialQuery<'_> {
    /// [`SpecialQuery::iter`].
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = SpecialQuerySegment<'_>> {
        self.0.iter().flat_map(SpecialQuery::iter)
    }

    /// [`SpecialQuery::find_iter`].
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = SpecialQuerySegment<'b>> {
        self.0.iter().flat_map(|x| x.find_iter(name))
    }

    /// [`SpecialQuery::get`].
    pub fn get(&self, index: isize) -> Option<SpecialQuerySegment<'_>> {
        self.0.as_ref()?.get(index)
    }

    /// [`SpecialQuery::find`].
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<SpecialQuerySegment<'b>> {
        self.0.as_ref()?.find(name, index)
    }
}

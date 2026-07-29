//! Getters.

use crate::prelude::*;

impl MaybeFragmentQuery<'_> {
    /// The [`FragmentQueryIter`].
    pub fn iter(&self) -> FragmentQueryIter<'_> {
        self.into_iter()
    }

    /// [`FragmentQuery::find_iter`].
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'b>> {
        self.0.iter().flat_map(|x| x.find_iter(name))
    }

    /// [`FragmentQuery::get`].
    pub fn get(&self, index: isize) -> Option<FragmentQuerySegment<'_>> {
        self.0.as_ref()?.get(index)
    }

    /// [`FragmentQuery::find`].
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<FragmentQuerySegment<'b>> {
        self.0.as_ref()?.find(name, index)
    }
}

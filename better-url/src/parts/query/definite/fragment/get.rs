//! Getters.

use crate::prelude::*;

impl FragmentQuery<'_> {
    /// The [`FragmentQueryIter`].
    pub fn iter(&self) -> FragmentQueryIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of [`FragmentQuerySegment`]s whose [`FragmentQuerySegment::name`]s are `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'b>> {
        self.iter().filter(move |segment| segment.name() == name)
    }

    /// Gets the `index`th [`FragmentQuerySegment`].
    pub fn get(&self, index: isize) -> Option<FragmentQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Gets the `index`th [`FragmentQuerySegment`] whose [`FragmentQuerySegment::name`] is `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<FragmentQuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

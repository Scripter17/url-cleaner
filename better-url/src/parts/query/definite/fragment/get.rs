//! Getters.

use crate::prelude::*;

impl FragmentQuery<'_> {
    /// [`SplitAmpersands`].
    pub fn iter_strs(&self) -> SplitAmpersands<'_> {
        SplitAmpersands(Some(&self.0))
    }

    /// A [`DoubleEndedIterator`] of [`FragmentQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'_>> {
        self.iter_strs().map(FragmentQuerySegment::new_unchecked)
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

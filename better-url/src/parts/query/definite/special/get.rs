//! Getters.

use crate::prelude::*;

impl SpecialQuery<'_> {
    /// [`SplitAmpersands`].
    pub fn iter_strs(&self) -> SplitAmpersands<'_> {
        SplitAmpersands(Some(&self.0))
    }

    /// A [`DoubleEndedIterator`] of [`SpecialQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = SpecialQuerySegment<'_>> {
        self.iter_strs().map(SpecialQuerySegment::new_unchecked)
    }

    /// A [`DoubleEndedIterator`] of [`SpecialQuerySegment`]s whose [`SpecialQuerySegment::name`]s are `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = SpecialQuerySegment<'b>> {
        self.iter().filter(move |segment| segment.name() == name)
    }

    /// Gets the `index`th [`SpecialQuerySegment`].
    pub fn get(&self, index: isize) -> Option<SpecialQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Gets the `index`th [`SpecialQuerySegment`] whose [`SpecialQuerySegment::name`] is `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<SpecialQuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

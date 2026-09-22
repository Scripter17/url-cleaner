//! Getters.

use crate::prelude::*;

impl<'a> MaybeSpecialQuery<'a> {
    /// The [`SpecialQueryIter`].
    pub fn iter(&self) -> SpecialQueryIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`SpecialQuerySegment`]s named `name`.
    pub fn find_iter<T: AsRef<[u8]>>(&self, name: T) -> impl DoubleEndedIterator<Item = SpecialQuerySegment<'_>> {
        self.iter().filter(move |x| x.name().decode() == name.as_ref())
    }

    /// The `index`th [`SpecialQuerySegment`].
    pub fn get(&self, index: isize) -> Option<SpecialQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`SpecialQuerySegment`] named `name`.
    pub fn find<T: AsRef<[u8]>>(&self, name: T, index: isize) -> Option<SpecialQuerySegment<'_>> {
        self.find_iter(name.as_ref()).neg_nth(index)
    }
}

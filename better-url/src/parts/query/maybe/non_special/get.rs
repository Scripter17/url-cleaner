//! Getters.

use crate::prelude::*;

impl<'a> MaybeNonSpecialQuery<'a> {
    /// The [`NonSpecialQueryIter`].
    pub fn iter(&self) -> NonSpecialQueryIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`NonSpecialQuerySegment`]s named `name`.
    pub fn find_iter<T: AsRef<[u8]>>(&self, name: T) -> impl DoubleEndedIterator<Item = NonSpecialQuerySegment<'_>> {
        self.iter().filter(move |x| x.name().decode() == name.as_ref())
    }

    /// The `index`th [`NonSpecialQuerySegment`].
    pub fn get(&self, index: isize) -> Option<NonSpecialQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`NonSpecialQuerySegment`] named `name`.
    pub fn find<T: AsRef<[u8]>>(&self, name: T, index: isize) -> Option<NonSpecialQuerySegment<'_>> {
        self.find_iter(name.as_ref()).neg_nth(index)
    }
}


//! Getters.

use crate::prelude::*;

impl<'a> FragmentQuery<'a> {
    /// The [`FragmentQueryIter`].
    pub fn iter(&self) -> FragmentQueryIter<'_> {
        self.into_iter()
    }

    /// A [`DoubleEndedIterator`] of the [`FragmentQuerySegment`]s whose [`FragmentQueryName::decode`] is `name`.
    pub fn find_iter<T: AsRef<[u8]>>(&self, name: T) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'_>> {
        self.iter().filter(move |x| x.name().decode() == name.as_ref())
    }

    /// The `index`th [`FragmentQuerySegment`].
    pub fn get(&self, index: isize) -> Option<FragmentQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`FragmentQuerySegment`] whose [`FragmentQueryName::decode`] is `name`.
    pub fn find<T: AsRef<[u8]>>(&self, name: T, index: isize) -> Option<FragmentQuerySegment<'_>> {
        self.find_iter(name.as_ref()).neg_nth(index)
    }
}

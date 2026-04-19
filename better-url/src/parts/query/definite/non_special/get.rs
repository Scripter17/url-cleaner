//! Getters.

use crate::prelude::*;

impl NonSpecialQuery<'_> {
    /// A [`DoubleEndedIterator`] of [`NonSpecialQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = NonSpecialQuerySegment<'_>> {
        self.as_str().split('&').map(NonSpecialQuerySegment::new_unchecked)
    }

    /// A [`DoubleEndedIterator`] of [`NonSpecialQuerySegment`]s whose [`NonSpecialQuerySegment::name`]s are `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = NonSpecialQuerySegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// Gets the `index`th [`NonSpecialQuerySegment`].
    pub fn get(&self, index: isize) -> Option<NonSpecialQuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Gets the `index`th [`NonSpecialQuerySegment`] whose [`NonSpecialQuerySegment::name`] is `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<NonSpecialQuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

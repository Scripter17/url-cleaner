//! Getters.

use crate::prelude::*;

impl Query<'_> {
    /// A [`DoubleEndedIterator`] of [`QuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        let is_special = self.is_special();
        self.as_str().split('&').map(move |x| match is_special {
            true  => SpecialQuerySegment   ::new_unchecked(x).into(),
            false => NonSpecialQuerySegment::new_unchecked(x).into(),
        })
    }

    /// A [`DoubleEndedIterator`] of [`QuerySegment`]s whose [`QuerySegment::name`]s are `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QuerySegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// Gets the `index`th [`QuerySegment`].
    pub fn get(&'_ self, index: isize) -> Option<QuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Gets the `index`th [`QuerySegment`] whose [`QuerySegment::name`] is `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

//! Getters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// A [`DoubleEndedIterator`] of the [`QuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        let is_special = self.is_special();

        SplitAmpersands(self.as_str()).map(move |x| {
            match is_special {
                true  => SpecialQuerySegment   ::new_unchecked(x).into(),
                false => NonSpecialQuerySegment::new_unchecked(x).into(),
            }
        })
    }

    /// [`Query::find_iter`].
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QuerySegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// [`Query::get`].
    pub fn get(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// [`Query::find`].
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

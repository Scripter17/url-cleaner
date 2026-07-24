//! Getters.

use crate::prelude::*;

impl Query<'_> {
    /// [`SplitAmpersands`].
    pub fn iter_strs(&self) -> SplitAmpersands<'_> {
        SplitAmpersands(Some(self.as_str()))
    }

    /// The [`QuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        let r#type = self.r#type();

        self.iter_strs().map(move |x| match r#type {
            QueryType::Special    => unsafe {SpecialQuerySegment   ::new_unchecked(x)}.into(),
            QueryType::NonSpecial => unsafe {NonSpecialQuerySegment::new_unchecked(x)}.into(),
        })
    }

    /// The [`QuerySegment`]s named `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QuerySegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// The `index`th [`QuerySegment`].
    pub fn get(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QuerySegment`] named `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QuerySegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

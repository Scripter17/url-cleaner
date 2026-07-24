//! Getters.

use crate::prelude::*;

impl<'a> QueryLike<'a> {
    /// A [`DoubleEndedIterator`] of the [`QueryLikeSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QueryLikeSegment<'_>> {
        let r#type = self.r#type();

        SplitAmpersands(Some(self.as_str())).map(move |x| {
            match r#type {
                QueryLikeType::Query(QueryType::Special   ) => unsafe {SpecialQuerySegment   ::new_unchecked(x)}.into(),
                QueryLikeType::Query(QueryType::NonSpecial) => unsafe {NonSpecialQuerySegment::new_unchecked(x)}.into(),
                QueryLikeType::Fragment                     => unsafe {FragmentQuerySegment  ::new_unchecked(x)}.into(),
            }
        })
    }

    /// A [`DoubleEndedIterator`] of the [`QueryLikeSegment`]s named `name`.
    pub fn find_iter<'b>(&'b self, name: &str) -> impl DoubleEndedIterator<Item = QueryLikeSegment<'b>> {
        self.iter().filter(move |x| x.name() == name)
    }

    /// The `index`th [`QueryLikeSegment`].
    pub fn get(&self, index: isize) -> Option<QueryLikeSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The `index`th [`QueryLikeSegment`] named `name`.
    pub fn find<'b>(&'b self, name: &str, index: isize) -> Option<QueryLikeSegment<'b>> {
        self.find_iter(name).neg_nth(index)
    }
}

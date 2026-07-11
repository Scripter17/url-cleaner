//! Getters.

use crate::prelude::*;

impl<'a> MaybeQueryLike<'a> {
    /// The [`QueryLikeMode`].
    fn mode(&self) -> QueryLikeMode {
        match self {
            Self::Query   (MaybeQuery::Special(_)   ) => QueryLikeMode::Special   ,
            Self::Query   (MaybeQuery::NonSpecial(_)) => QueryLikeMode::NonSpecial,
            Self::Fragment(_                        ) => QueryLikeMode::Fragment  ,
        }
    }

    /// A [`DoubleEndedIterator`] of the [`QueryLikeSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QueryLikeSegment<'_>> {
        let mode = self.mode();

        SplitAmpersands(self.as_str()).map(move |x| {
            match mode {
                QueryLikeMode::Special    => unsafe {SpecialQuerySegment   ::new_unchecked(x)}.into(),
                QueryLikeMode::NonSpecial => unsafe {NonSpecialQuerySegment::new_unchecked(x)}.into(),
                QueryLikeMode::Fragment   => unsafe {FragmentQuerySegment  ::new_unchecked(x)}.into(),
            }
        })
    }

    /// A [`DoubleEndedIterator`] of the [`QuerySegment`]s named `name`.
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

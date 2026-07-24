//! Getters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// The [`QueryType`].
    pub fn r#type(&self) -> QueryType {
        match self {
            Self::Special   (_) => QueryType::Special   ,
            Self::NonSpecial(_) => QueryType::NonSpecial,
        }
    }

    /// A [`DoubleEndedIterator`] of the [`QuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        let r#type = self.r#type();

        SplitAmpersands(self.as_str()).map(move |x| {
            match r#type {
                QueryType::Special    => unsafe {SpecialQuerySegment   ::new_unchecked(x)}.into(),
                QueryType::NonSpecial => unsafe {NonSpecialQuerySegment::new_unchecked(x)}.into(),
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

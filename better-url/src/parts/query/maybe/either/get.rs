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

    /// The [`QueryIter`].
    pub fn iter(&self) -> QueryIter<'_> {
        self.into_iter()
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

//! Setters.

use crate::prelude::*;

impl<'a> QueryLike<'a> {
    /// Either [`Query::set`] or [`FragmentQuery::set`].
    /// # Errors
    /// If [`Query::set`] returns an error, tha error is returned.
    ///
    /// If [`FragmentQuery::set`] returns an error, that error is returned.
    pub fn set<'b, T: Into<MaybeSpecialQueryValue<'b>> + Into<MaybeNonSpecialQueryValue<'b>> + Into<MaybeFragmentQueryValue<'b>>>(&mut self, name: &str, index: isize, value: Option<T>) -> Result<bool, SetQueryError> {
        match self {
            Self::Query   (x) => x.set(name, index, value),
            Self::Fragment(x) => x.set(name, index, value),
        }
    }
}

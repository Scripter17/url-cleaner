//! Setters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// Either [`MaybeSpecialQuery::set`] or [`MaybeNonSpecialQuery::set`].
    /// # Errors
    /// If [`MaybeSpecialQuery::set`] returns an error, that error is returned.
    ///
    /// If [`MaybeNonSpecialQuery::set`] returns an error, that error is returned.
    pub fn set<'b, T: Into<MaybeSpecialQueryValue<'b>> + Into<MaybeNonSpecialQueryValue<'b>>>(&mut self, name: &str, index: isize, value: Option<T>) -> Result<bool, SetQueryError> {
        match self {
            Self::Special   (x) => x.set(name, index, value),
            Self::NonSpecial(x) => x.set(name, index, value),
        }
    }
}

//! Setters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// Either [`MaybeSpecialQuery::set`] or [`MaybeNonSpecialQuery::set`].
    /// # Errors
    /// If the call to [`MaybeSpecialQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeNonSpecialQuery::set`] returns an error, that error is returned.
    pub fn set(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        match self {
            Self::Special   (x) => x.set(name, index, value),
            Self::NonSpecial(x) => x.set(name, index, value),
        }
    }
}

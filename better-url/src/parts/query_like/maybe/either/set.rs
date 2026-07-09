//! Setters.

use crate::prelude::*;

impl<'a> MaybeQueryLike<'a> {
    /// Either [`MaybeQuery::set`] or [`MaybeFragmentQuery::set`].
    /// # Errors
    /// If the call to [`MaybeQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeFragmentQuery::set`] returns an error, that error is returned.
    pub fn set(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        match self {
            Self::Query   (x) => x.set(name, index, value),
            Self::Fragment(x) => x.set(name, index, value)
        }
    }
}

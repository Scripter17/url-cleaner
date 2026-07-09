//! Setters.

use crate::prelude::*;

impl<'a> QueryLike<'a> {
    /// Either [`Query::set`] or [`FragmentQuery::set`].
    /// # Errors
    /// If the call to [`Query::set`] returns an error, tha error is returned.
    ///
    /// If the call to [`FragmentQuery::set`] returns an error, that error is returned.
    pub fn set(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        match self {
            Self::Query   (x) => x.set(name, index, value),
            Self::Fragment(x) => x.set(name, index, value),
        }
    }
}

//! Setters.

use crate::prelude::*;

impl Query<'_> {
    /// Either [`SpecialQuery::set`] or, [`NonSpecialQuery::set`].
    /// # Errors
    /// If the call to [`SpecialQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialQuery::set`] returns an error, that error is returned.
    pub fn set(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        match self {
            Self::Special   (x) => x.set(name, index, value),
            Self::NonSpecial(x) => x.set(name, index, value),
        }
    }
}

//! Removers.

use crate::prelude::*;

impl SpecialNotFileSegmentedPath<'_> {
    /// Remove the last segment.
    /// # Errors
    /// If there's only one segment, returns the error [`CantBeEmpty`].
    pub fn pop(&mut self) -> Result<(), SetPathError> {
        self.0.retain_substr(self.0[1..].rsplit_once("/").ok_or(CantBeEmpty)?.0);

        Ok(())
    }

    /// Remove the last segment if it's empty.
    /// # Errors
    /// If there is only one segment and it's empty, returns the error [`CantBeNone`].
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        if self.0 == "/" {
            Err(CantBeEmpty)?;
        }

        if let Some(x) = self.0.strip_prefix("/") {
            self.0.retain_substr(x);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

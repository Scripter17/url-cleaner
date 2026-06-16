//! Removers.

use crate::prelude::*;

impl SpecialNotFileSegmentedPath<'_> {
    /// Remove the last segment.
    /// # Errors
    /// If there's only one segment, returns the error [`CantBeEmpty`].
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn pop(&mut self) -> Result<(), SetPathError> {
        match self.0.rfind('/').expect("???") {
            0 => Err(CantBeEmpty)?,
            x => self.0.retain_range(..x)
        }

        Ok(())
    }

    /// Remove the last segment if it's empty.
    /// # Errors
    /// If there is only one segment and it's empty, returns the error [`CantBeNone`].
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        match self.0.strip_suffix("/") {
            Some("") => Err(CantBeEmpty)?,
            Some(x ) => {self.0.retain_substr(x); Ok(true)},
            None     => Ok(false)
        }
    }

    /// Remove the `index`th segment.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`].
    ///
    /// If there is only one segment, returns the error [`CantBeEmpty`].
    pub fn remove(&mut self, index: isize) -> Result<(), SetPathError> {
        let mut range = self.as_str().my_substr_range(self.get(index).ok_or(SegmentNotFound)?.as_str());
        range.start -= 1;

        if range.len() == self.len() {
            Err(CantBeEmpty)?;
        }

        self.0.replace_range(range, "");

        Ok(())
    }
}

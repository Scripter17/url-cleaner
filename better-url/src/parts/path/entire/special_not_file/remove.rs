//! Removers.

use crate::prelude::*;

impl SpecialNotFilePath<'_> {
    /// Remove the last segment if it exists.
    /// # Errors
    /// If there's only one segment, returns the error [`CantBeEmpty`].
    #[expect(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn pop(&mut self) -> Result<bool, SetPathError> {
        match self.0.memrchr(b'/').expect("???") {
            0 => Err(CantBeEmpty)?,
            x => unsafe {self.0.truncate_unchecked(x)}
        }

        Ok(true)
    }

    /// Remove the last segment if it exists and is empty.
    /// # Errors
    /// If there is only one segment and it's empty, returns the error [`CantBeNone`].
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        Ok(match self.as_str().as_bytes() {
            [        b'/'] => Err(CantBeEmpty)?,
            [x @ .., b'/'] => {
                unsafe {
                    self.0.truncate_unchecked(x.len())
                }

                true
            },
            _ => false
        })
    }

    /// Remove the `index`th segment if it exists.
    /// # Errors
    /// If there is only one segment, returns the error [`CantBeEmpty`].
    pub fn remove(&mut self, index: isize) -> Result<bool, SetPathError> {
        Ok(match self.get_str(index) {
            Some(temp) => {
                let mut range = self.as_str().my_substr_range(temp);
                range.start -= 1;

                if range.len() == self.len() {
                    Err(CantBeEmpty)?;
                }

                self.0.replace_range(range, "");

                true
            },
            None => false
        })
    }
}

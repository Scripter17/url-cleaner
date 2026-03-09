//! Removers.

use crate::prelude::*;

impl<'a> BetterRefPathSegments<'a> {
    /// Remove and return the last [`RawPathSegment`].
    /// # Errors
    /// If there is only one segment, return the error [`CantBeNone`].
    pub fn pop(&mut self) -> Result<RawPathSegment<'a>, CantBeNone> {
        let (rem, seg) = self.0.rsplit_once('/').ok_or(CantBeNone)?;

        self.0 = rem;

        Ok(seg.into())
    }

    /// Remove the last segment if it's empty.
    /// # Errors
    /// If there's only one segment and it's empty, returns the error [`CantBeNone`].
    pub fn pop_if_empty(&mut self) -> Result<(), CantBeNone> {
        if self.0.is_empty() {
            Err(CantBeNone)?;
        }

        if let Some(x) = self.0.strip_suffix("/") {
            self.0 = x;
        }

        Ok(())
    }

    /// Remove and return the first [`RawPathSegment`].
    /// # Errors
    /// If there is only one segment, return the error [`CantBeNone`].
    pub fn shift(&mut self) -> Result<RawPathSegment<'a>, CantBeNone> {
        let (seg, rem) = self.0.split_once('/').ok_or(CantBeNone)?;

        self.0 = rem;

        Ok(seg.into())
    }
}

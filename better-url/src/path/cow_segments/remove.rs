//! Removers.

use std::borrow::Cow;
use std::ops::RangeBounds;

use crate::prelude::*;

impl BetterPathSegments<'_> {
    /// Remove the last segment.
    /// # Errors
    /// If there's only one segment, returns the error [`CantBeNone`].
    pub fn pop(&mut self) -> Result<(), CantBeNone> {
        self.0.retain_substr(self.0.rsplit_once('/').ok_or(CantBeNone)?.0);

        Ok(())
    }

    /// Remove the last segment if it's empty.
    /// # Errors
    /// If there's only one segment and it's empty, returns the error [`CantBeNone`].
    pub fn pop_if_empty(&mut self) -> Result<(), CantBeNone> {
        if self.0.is_empty() {
            Err(CantBeNone)?;
        }

        if let Some(x) = self.0.strip_suffix("/") {
            self.0.retain_substr(x);
        }

        Ok(())
    }

    /// Remove the first segment.
    /// # Errors
    /// If there's only one segment, returns the error [`CantBeNone`].
    pub fn shift(&mut self) -> Result<(), CantBeNone> {
        self.0.retain_substr(self.0.split_once('/').ok_or(CantBeNone)?.1);

        Ok(())
    }

    /// Remove the `index`th segment.
    /// # Errors
    /// If the segment ins't found, returns the error [`RemoveError::SegmentNotFound`].
    ///
    /// If there's only one segment, returns the error [`RemoveError::CantBeNone`].
    pub fn remove(&mut self, index: isize) -> Result<(), RemoveError> {
        let (before, after) = self.0.split_around_substr(self.get(index).ok_or(SegmentNotFound)?.0);

        match (before.strip_suffix("/"), after.strip_prefix("/")) {
            (None        , None       ) => Err(CantBeNone)?,
            (None        , Some(after)) => self.0.retain_substr(after),
            (Some(before), None       ) => self.0.retain_substr(before),
            (Some(before), Some(after)) => self.0 = Cow::Owned(format!("{before}/{after}"))
        }

        Ok(())
    }

    /// Remove the range of segments.
    /// # Errors
    /// If the call to [`Self::get_range`] returns [`None`], returns the erorr [`RangeNotFound`].
    ///
    /// If attempting to remove all segments, returns the error [`CantBeNone`].
    pub fn remove_range<B: RangeBounds<isize>>(&mut self, range: B) -> Result<(), RemoveRangeError> {
        let (before, after) = self.0.split_around_substr(self.get_range(range).ok_or(RangeNotFound)?.0);

        match (before.strip_suffix("/"), after.strip_prefix("/")) {
            (None        , None       ) => Err(CantBeNone)?,
            (None        , Some(after)) => self.0.retain_substr(after),
            (Some(before), None       ) => self.0.retain_substr(before),
            (Some(before), Some(after)) => self.0 = Cow::Owned(format!("{before}/{after}"))
        }

        Ok(())
    }
}

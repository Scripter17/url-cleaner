//! Setters.

use std::ops::RangeBounds;

use crate::prelude::*;

impl BetterPathSegments<'_> {
    /// Set or the range of raw segments.
    /// # Errors
    /// If the call to [`Self::get_range`] reurns [`None`], returns the error [`RangeNotFound`].
    pub fn set_raw_range<B: RangeBounds<isize>>(&mut self, range: B, value: &str) -> Result<(), RangeNotFound> {
        let range = self.0.my_substr_range(self.get_range(range).ok_or(RangeNotFound)?.as_str());

        self.0.to_mut().replace_range(range, value);

        Ok(())
    }

    /// Set or remove the range of raw segemnts.
    /// # Errors
    /// If the call to [`Self::set_raw_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::remove_range`] returns an error, that error is returned.
    pub fn set_or_remove_raw_range<B: RangeBounds<isize>>(&mut self, range: B, value: Option<&str>) -> Result<(), SetOrRemoveRangeError> {
        match value {
            Some(value) => self.set_raw_range(range, value)?,
            None => self.remove_range(range)?
        }

        Ok(())
    }
}

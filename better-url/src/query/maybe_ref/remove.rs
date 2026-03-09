//! Removers

use crate::prelude::*;

impl<'a> BetterMaybeRefQuery<'a> {
    /// Removes and returns the last segment.
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    pub fn pop(&mut self) -> Result<RawQuerySegment<'a>, SegmentNotFound> {
        let (rem, seg) = self.0.ok_or(SegmentNotFound)?.popped();

        *self = rem;

        Ok(seg)
    }

    /// Removes and returns the first segment.
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    pub fn shift(&mut self) -> Result<RawQuerySegment<'a>, SegmentNotFound> {
        let (seg, rem) = self.0.ok_or(SegmentNotFound)?.shifted();

        *self = rem;

        Ok(seg)
    }
}

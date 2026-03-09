//! Removers.

use crate::prelude::*;

impl BetterMaybeQuery<'_> {
    /// [`BetterQuery::pop`], replacing [`RemoveError::CantBeNone`] with setting [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    pub fn pop(&mut self) -> Result<(), SegmentNotFound> {
        match self.0.as_mut().ok_or(SegmentNotFound)?.pop() {
            Ok(()) => {},
            Err(CantBeNone) => self.0 = None
        }

        Ok(())
    }

    /// [`BetterQuery::shift`], replacing [`RemoveError::CantBeNone`] with setting [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    pub fn shift(&mut self) -> Result<(), SegmentNotFound> {
        match self.0.as_mut().ok_or(SegmentNotFound)?.shift() {
            Ok(()) => {},
            Err(CantBeNone) => self.0 = None
        }

        Ok(())
    }

    /// [`BetterQuery::remove`], replacing [`RemoveError::CantBeNone`] with setting [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    ///
    /// If the call to [`BetterQuery::remove`] returns [`RemoveError::SegmentNotFound`], returns the error [`SegmentNotFound`].
    pub fn remove(&mut self, index: isize) -> Result<(), SegmentNotFound> {
        match self.0.as_mut().ok_or(SegmentNotFound)?.remove(index) {
            Ok(()) => {},
            Err(RemoveError::CantBeNone(_)) => self.0 = None,
            Err(RemoveError::SegmentNotFound(e)) => Err(e)?
        }

        Ok(())
    }

    /// [`BetterQuery::find_remove`], replacing [`RemoveError::CantBeNone`] with setting [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`None`], returns the error [`SegmentNotFound`].
    ///
    /// If the call to [`BetterQuery::remove`] returns [`RemoveError::SegmentNotFound`], returns the error [`SegmentNotFound`].
    pub fn find_remove(&mut self, name: &str, index: isize) -> Result<(), SegmentNotFound> {
        match self.0.as_mut().ok_or(SegmentNotFound)?.find_remove(name, index) {
            Ok(()) => {},
            Err(RemoveError::CantBeNone(_)) => self.0 = None,
            Err(RemoveError::SegmentNotFound(e)) => Err(e)?
        }

        Ok(())
    }
}

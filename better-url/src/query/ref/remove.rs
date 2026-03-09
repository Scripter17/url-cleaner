//! Removers.

use crate::prelude::*;

impl<'a> BetterRefQuery<'a> {
    /// Remove the last segment.
    /// # Errors
    /// If there is only one segment, returns the error [`CantBeNone`].
    pub fn pop(&mut self) -> Result<RawQuerySegment<'a>, CantBeNone> {
        let (rem, ret) = self.0.rsplit_once('&').ok_or(CantBeNone)?;

        self.0 = rem;

        Ok(ret.into())
    }

    /// Make a [`BetterMaybeRefQuery`] with the result of removing the last segment and a [`RawQuerySegment`] of that segment.
    pub fn popped(&self) -> (BetterMaybeRefQuery<'a>, RawQuerySegment<'a>) {
        match self.0.rsplit_once('&') {
            Some((rem, seg)) => (rem.into()         , seg.into()   ),
            None             => (None::<&str>.into(), self.0.into())
        }
    }

    /// Remove the first segment.
    /// # Errors
    /// If there is only one segment, returns the error [`CantBeNone`].
    pub fn shift(&mut self) -> Result<RawQuerySegment<'a>, CantBeNone> {
        let (ret, rem) = self.0.split_once('&').ok_or(CantBeNone)?;

        self.0 = rem;

        Ok(ret.into())
    }

    /// Make a [`BetterMaybeRefQuery`] with the result of removing the first segment and a [`RawQuerySegment`] of that segment.
    pub fn shifted(&self) -> (RawQuerySegment<'a>, BetterMaybeRefQuery<'a>) {
        match self.0.split_once('&') {
            Some((seg, rem)) => (seg.into()   , rem.into()         ),
            None             => (self.0.into(), None::<&str>.into())
        }
    }
}

//! Setters.

use crate::prelude::*;

impl MaybeNonSpecialQuery<'_> {
    /// [`NonSpecialQuery::set`].
    ///
    /// If [`Self::0`] is [`None`], `index` is 0 or 1, and `value` is [`Some`], creates a new [`NonSpecialQuery`] with [`NonSpecialQuerySegment::from_pair`].
    ///
    /// If the call to [`NonSpecialQuery::set`] returns the error [`CantBeNone`], sets [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`None`], `index` is neither `0` nor `-1`, and `value` is [`Some`], returns the error [`InsertNotFound`].
    ///
    /// If [`Self::0`] is [`None`] and `value` is [`None`], returns the error [`SegmentNotFound`].
    ///
    /// If the call to [`NonSpecialQuery::set`] returns an error, that error is returned.
    pub fn set(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        match &mut self.0 {
            Some(query) => match query.set(index, name, value) {
                Err(SetQueryError::CantBeNone(CantBeNone)) => {self.0 = None; Ok(true)},
                x => x
            },
            None => match (index, value) {
                (0 | -1, Some(value)) => {self.0 = Some(NonSpecialQuerySegment::from_pair(name, value).into_owned().into()); Ok(true)},
                (_     , Some(_)    ) => Err(InsertNotFound)?,
                (_     , None       ) => Err(SegmentNotFound)?,
            }
        }
    }
}

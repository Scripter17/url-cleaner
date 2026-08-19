//! Setters.

use crate::prelude::*;

impl MaybeSpecialQuery<'_> {
    /// [`SpecialQuery::set`].
    ///
    /// If [`Self::0`] is [`None`], `index` is 0 or 1, and `value` is [`Some`], creates a new [`FragmentQuery`] with [`FragmentQuerySegment::from_pair`].
    ///
    /// If [`FragmentQuery::set`] returns the error [`CantBeNone`], sets [`Self::0`] to [`None`].
    /// # Errors
    /// If [`Self::0`] is [`Some`] and [`FragmentQuery::set`] returns an error, that error is returned.
    ///
    /// If [`Self::0`] is [`None`], `index` is neither `0` nor `-1`, and `value` is [`Some`], returns the error [`InsertNotFound`].
    pub fn set<'b, T: Into<MaybeSpecialQueryValue<'b>>>(&mut self, name: &str, index: isize, value: Option<T>) -> Result<bool, SetQueryError> {
        Ok(match &mut self.0 {
            Some(query) => match query.set(name, index, value) {
                Err(SetQueryError::CantBeNone(CantBeNone)) => {self.0 = None; true},
                x => x?
            },
            None => match (index, value) {
                (0 | -1, Some(value)) => {self.0 = Some(SpecialQuerySegment::from_pair(name, value).into_owned().into()); true},
                (_     , Some(_)    ) => Err(InsertNotFound)?,
                (_     , None       ) => false,
            }
        })
    }
}

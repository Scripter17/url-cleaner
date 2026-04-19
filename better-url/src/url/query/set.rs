//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the query.
    /// # Errors
    /// If setting a query that's too long, returns the error [`TooLong`].
    pub fn set_query<'a, T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(&mut self, query: T) -> Result<bool, SetQueryError> {
        let query = match self.is_special() {
            true  => MaybeQuery::new_special    (query),
            false => MaybeQuery::new_non_special(query),
        };

        let new_len = match (self.query_str(), query.as_str()) {
            (None     , None     ) => return Ok(false),
            (Some(old), Some(new)) if old == new => return Ok(false),

            (None     , Some(new)) => self.len() + new.len() + 1,
            (Some(old), None     ) => self.len() - old.len() - 1,
            (Some(old), Some(new)) => self.len() - old.len() + new.len(),
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.url.set_query(query.as_str());

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.query(), query);

        Ok(true)
    }

    /// Remove the query.
    pub fn remove_query(&mut self) -> bool {
        if self.query_str().is_some() {
            self.url.set_query(None);
            true
        } else {
            false
        }
    }

    /// Remove the query if it's empty.
    pub fn remove_empty_query(&mut self) -> bool {
        if self.query_str() == Some("") {
            self.url.set_query(None);
            true
        } else {
            false
        }
    }

    /// [`MaybeQuery::set`].
    /// # Errors
    /// If the call to [`MaybeQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_query`] returns an error, that error is returned.
    pub fn set_query_param(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        let mut query = self.query();

        if query.set(name, index, value)? {
            self.set_query(query.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`MaybeQuery::filtered`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn filter_query<F: FnMut(&QuerySegment<'_>) -> bool>(&mut self, f: F) -> bool {
        if let (true, query) = self.query().filtered(f) {
            self.set_query(query.into_owned()).expect("???");
            true
        } else {
            false
        }
    }

    /// [`MaybeQuery::try_filtered`].
    /// # Errors
    /// If the call to [`MaybeQuery::try_filtered`] returns an error, that error is returned in an `Ok(Err(_))`.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn try_filter_query<F: FnMut(&QuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<bool, E> {
        if let (true, query) = self.query().try_filtered(f)? {
            self.set_query(query.into_owned()).expect("???");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

//! Implementing query stuff for [`BetterUrl`].

use std::borrow::Cow;

#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::prelude::*;

impl BetterUrl {
    /// [`Url::set_query`].
    pub fn set_query<T: AsQueryStr>(&mut self, query: T) {
        let query = query.as_query_str();

        if self.query_str() != query {
            self.url.set_query(query)
        }
    }

    /// [`Url::query`].
    pub fn query_str(&self) -> Option<&str> {
        self.url.query()
    }

    /// Get a [`BetterQuery`] if available.
    pub fn query(&self) -> Option<BetterQuery<'_>> {
        Some(BetterQuery(Cow::Borrowed(self.query_str()?)))
    }

    /// Get a [`BetterMaybeQuery`].
    pub fn maybe_query(&self) -> BetterMaybeQuery<'_> {
        BetterMaybeQuery(self.query())
    }

    /// Get a [`BetterRefQuery`].
    pub fn ref_query(&self) -> Option<BetterRefQuery<'_>> {
        Some(BetterRefQuery(self.query_str()?))
    }

    /// Get a [`BetterMaybeRefQuery`].
    pub fn maybe_ref_query(&self) -> BetterMaybeRefQuery<'_> {
        BetterMaybeRefQuery(self.ref_query())
    }

    /// Get, modify, then apply a [`Self::maybe_query`].
    pub fn modify_maybe_query<F: FnOnce(&mut BetterMaybeQuery<'_>)>(&mut self, f: F) {
        let mut query = self.maybe_query();
        f(&mut query);
        self.set_query(query.into_owned());
    }

    /// Get, modify, then apply a [`Self::maybe_query`].
    /// # Errors
    /// If the call to `f` returns an error, that error is returned.
    pub fn try_modify_maybe_query<F: FnOnce(&mut BetterMaybeQuery<'_>) -> Result<(), E>, E>(&mut self, f: F) -> Result<(), E> {
        let mut query = self.maybe_query();
        f(&mut query)?;
        self.set_query(query.into_owned());
        Ok(())
    }

    /// [`BetterMaybeQuery::filter`].
    pub fn filter_query<F: FnMut(RawQuerySegment<'_>) -> bool>(&mut self, f: F) {
        let mut query = self.maybe_query();
        query.filter(f);
        self.set_query(query.into_owned());
    }

    /// [`BetterMaybeQuery::try_filter`].
    /// # Errors
    /// If the call to [`BetterMaybeQuery::try_filter`] returns an error, that error is returned.
    pub fn try_filter_query<F: FnMut(RawQuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<(), E> {
        let mut query = self.maybe_query();
        query.try_filter(f)?;
        self.set_query(query.into_owned());
        Ok(())
    }
}

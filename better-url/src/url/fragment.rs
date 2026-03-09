//! Implementing fragment stuff for [`BetterUrl`].

use std::borrow::Cow;

#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::prelude::*;

impl BetterUrl {
    /// [`Url::set_fragment`].
    pub fn set_fragment<T: AsQueryStr>(&mut self, fragment: T) {
        let fragment = fragment.as_query_str();

        if self.fragment() != fragment {
            self.url.set_fragment(fragment)
        }
    }

    /// [`Url::fragment`].
    pub fn fragment_str(&self) -> Option<&str> {
        self.url.fragment()
    }

    /// Get a [`BetterQuery`] using the fragment, if available.
    pub fn fragment_query(&self) -> Option<BetterQuery<'_>> {
        Some(BetterQuery(Cow::Borrowed(self.fragment_str()?)))
    }

    /// Get a [`BetterMaybeQuery`] using the fragment.
    pub fn maybe_fragment_query(&self) -> BetterMaybeQuery<'_> {
        BetterMaybeQuery(self.fragment_query())
    }

    /// Get a [`BetterRefQuery`] using the fragment, if available.
    pub fn ref_fragment_query(&self) -> Option<BetterRefQuery<'_>> {
        Some(BetterRefQuery(self.fragment_str()?))
    }

    /// Get a [`BetterMaybeRefQuery`] using the fragment.
    pub fn maybe_ref_fragment_query(&self) -> BetterMaybeRefQuery<'_> {
        BetterMaybeRefQuery(self.ref_fragment_query())
    }

    /// [`BetterMaybeQuery::filter`] but for [`Self::fragment_query`].
    pub fn filter_fragment_query<F: FnMut(RawQuerySegment<'_>) -> bool>(&mut self, f: F) {
        let mut fragment = self.maybe_fragment_query();
        fragment.filter(f);
        self.set_fragment(fragment.into_owned());
    }

    /// [`BetterMaybeQuery::try_filter`] but for [`Self::fragment_query`].
    /// # Errors
    /// If the call to [`BetterMaybeQuery::try_filter`] returns an error, that error is returned.
    pub fn try_filter_fragment_query<F: FnMut(RawQuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<(), E> {
        let mut fragment = self.maybe_fragment_query();
        fragment.try_filter(f)?;
        self.set_fragment(fragment.into_owned());
        Ok(())
    }
}

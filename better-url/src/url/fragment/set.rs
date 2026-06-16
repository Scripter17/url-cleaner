//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the fragment.
    /// # Errors
    /// If setting a fragment that's too long, returns the error [`TooLong`].
    pub fn set_fragment<'a, T: Into<MaybeFragment<'a>>>(&mut self, fragment: T) -> Result<bool, SetFragmentError> {
        let fragment = fragment.into();

        let new_len = match (self.fragment_str(), fragment.as_str()) {
            (None     , None     ) => return Ok(false),
            (Some(old), Some(new)) if old == new => return Ok(false),

            (None     , Some(new)) => self.len() + new.len() + 1,
            (Some(old), None     ) => self.len() - old.len() - 1,
            (Some(old), Some(new)) => self.len() - old.len() + new.len()
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.url.set_fragment(fragment.as_str());

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.fragment(), fragment);

        Ok(true)
    }

    /// Remove the fragment.
    pub fn remove_fragment(&mut self) -> bool {
        if self.fragment_str().is_some() {
            self.url.set_fragment(None);
            true
        } else {
            false
        }
    }

    /// Remove the fragment if it's empty.
    pub fn remove_empty_fragment(&mut self) -> bool {
        if self.fragment_str() == Some("") {
            self.url.set_fragment(None);
            true
        } else {
            false
        }
    }

    /// [`MaybeFragmentQuery::set`].
    /// # Errors
    /// If the call to [`MaybeFragmentQuery::set`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_fragment`] returns an error, that error is returned.
    pub fn set_fragment_query_param(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<bool, SetFragmentError> {
        let mut fragment = self.fragment_query();

        if fragment.set(name, index, value)? {
            self.set_fragment(fragment.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`MaybeFragmentQuery::filtered`].
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn filter_fragment_query<F: FnMut(&QuerySegment<'_>) -> bool>(&mut self, f: F) -> bool {
        if let (true, fragment) = MaybeQuery::from(self.fragment_query()).filtered(f) {
            self.set_fragment(fragment.into_owned()).expect("To be at most u32::MAX.");
            true
        } else {
            false
        }
    }

    /// [`MaybeFragmentQuery::try_filtered`].
    /// # Errors
    /// If the call to [`MaybeFragmentQuery::try_filtered`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn try_filter_fragment_query<F: FnMut(&QuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<bool, E> {
        if let (true, fragment) = MaybeQuery::from(self.fragment_query()).try_filtered(f)? {
            self.set_fragment(fragment.into_owned()).expect("To be at most u32::MAX.");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

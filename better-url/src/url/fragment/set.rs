//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the fragment.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_fragment<'a, T: Into<MaybeFragment<'a>>>(&mut self, value: T) -> Result<(), SetFragmentError> {
        let new = value.into();

        match (self.details.fragment_mark, new.as_str()) {
            (None, None) => {},

            (None, Some(new)) => {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.details.fragment_mark = NonZero::new(self.len() as u32);
                self.serialization.extend(["#", new]);
            },

            (Some(mark), None) => {
                self.serialization.truncate(mark.get() as usize);
                self.details.fragment_mark = None;
            },

            (Some(mark), Some(new)) => {
                if mark.get() as usize + new.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.replace_range(mark.get() as usize + 1 .., new);
            },
        }

        Ok(())
    }

    /// Remove the fragment.
    pub fn remove_fragment(&mut self) -> bool {
        if let Some(x) = self.details.fragment_mark {
            self.serialization.truncate(x.get() as usize);
            self.details.fragment_mark = None;
            true
        } else {
            false
        }
    }

    /// Remove the fragment if it's empty.
    pub fn remove_empty_fragment(&mut self) -> bool {
        if let Some(x) = self.details.fragment_mark && x.get() as usize == self.len() - 1 {
            self.serialization.truncate(x.get() as usize);
            self.details.fragment_mark = None;
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
    pub fn filter_fragment_query<F: FnMut(FragmentQuerySegment<'_>) -> bool>(&mut self, f: F) -> bool {
        if let (true, fragment) = self.fragment_query().filtered(f) {
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
    pub fn try_filter_fragment_query<F: FnMut(FragmentQuerySegment<'_>) -> Result<bool, E>, E>(&mut self, f: F) -> Result<bool, E> {
        if let (true, fragment) = self.fragment_query().try_filtered(f)? {
            self.set_fragment(fragment.into_owned()).expect("To be at most u32::MAX.");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

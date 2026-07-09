//! [`Fragment`] and co..

use crate::prelude::*;

impl MyUrl {
    /// The [`Range::start`] of the fragment.
    pub(crate) fn fragment_mark(&self) -> Option<usize> {
        Some(self.fragment_mark?.get() as usize + 1)
    }

    /// The [`Range::end`] of the fragment.
    pub(crate) fn fragment_after(&self) -> Option<usize> {
        match self.fragment_mark.is_some() {
            true  => Some(self.len()),
            false => None,
        }
    }

    /// The [`Range`] of the fragment.
    pub(crate) fn fragment_range(&self) -> Option<Range<usize>> {
        Some(self.fragment_mark()? .. self.fragment_after()?)
    }

    /// The fragment as a [`str`].
    pub fn fragment(&self) -> Option<&str> {
        Some(&self.serialization[self.fragment_range()?])
    }

    /// Set the fragment.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_fragment<'a, T: Into<MaybeFragment<'a>>>(&mut self, value: T) -> Result<(), SetFragmentError> {
        let new = value.into();

        match (self.fragment_mark, new.as_str()) {
            (None, None) => {},

            (None, Some(new)) => {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.fragment_mark = NonZero::new(self.len() as u32);
                self.serialization.extend(["#", new]);
            },

            (Some(old), None) => {
                self.serialization.truncate(old.get() as usize);
                self.fragment_mark = None;
            },

            (Some(old), Some(new)) => {
                if old.get() as usize + new.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.replace_range(old.get() as usize + 1 .., new);
            },
        }

        Ok(())
    }
}

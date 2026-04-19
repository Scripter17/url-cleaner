//! Username stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the username.
    pub(crate) fn username_start(&self) -> usize {
        // Can't just use the address of [`Url::username`].
        match self.has_authority() {
            true  => self.scheme().len() + 3,
            false => self.scheme().len() + 1,
        }
    }

    /// The [`Range::end`] of the username.
    pub(crate) fn username_after(&self) -> usize {
        self.username_start() + self.url.username().len()
    }

    /// The [`Range`] of the username.
    pub(crate) fn username_range(&self) -> Range<usize> {
        self.username_start() .. self.username_after()
    }

    /// The username.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterUrl::parse("https://example.com"      ).unwrap().username_str(), ""  );
    /// assert_eq!(BetterUrl::parse("https://ab@example.com"   ).unwrap().username_str(), "ab");
    /// assert_eq!(BetterUrl::parse("https://:cd@example.com"  ).unwrap().username_str(), ""  );
    /// assert_eq!(BetterUrl::parse("https://ab:cd@example.com").unwrap().username_str(), "ab");
    /// ```
    pub fn username_str(&self) -> &str {
        &self.as_str()[self.username_range()]
    }

    /// The [`Username`],
    pub fn username(&self) -> Username<'_> {
        Username::new_unchecked(self.username_str())
    }

    /// Set the username.
    /// # Errors
    /// If the URL doesn't have a host, returns the error [`NoHost`].
    ///
    /// If setting the username to be too long, returns the error [`TooLong`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_username<'a, T: Into<Username<'a>>>(&mut self, username: T) -> Result<(), SetUsernameError> {
        if !self.has_host() {
            Err(NoHost)?;
        }

        let username = username.into();

        let new_len = match (self.username().len(), self.password().len(), username.len()) {
            (0, 0, 0) => self.len(),
            (0, 0, y) => self.len() + y + 1,
            (0, _, 0) => self.len(),
            (0, _, y) => self.len() + y,

            (x, 0, 0) => self.len() - x - 1,
            (x, 0, y) => self.len() - x + y,
            (x, _, 0) => self.len() - x,
            (x, _, y) => self.len() - x + y
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        if self.username() != username {
            self.url.set_username(username.as_str()).expect("To be valid.");
        }

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.username(), username);

        Ok(())
    }
}

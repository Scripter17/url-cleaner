//! Password stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the password.
    pub(crate) fn password_start(&self) -> usize {
        match self.url.password() {
            Some(x) => x.addr() - self.as_str().addr(),
            None => self.username_after()
        }
    }

    /// The [`Range::end`] of the password.
    pub(crate) fn password_after(&self) -> usize {
        match self.url.password() {
            Some(x) => x.end_addr() - self.as_str().addr(),
            None => self.username_after()
        }
    }

    /// The [`Range`] of the password.
    pub(crate) fn password_range(&self) -> Range<usize> {
        self.password_start() .. self.password_after()
    }

    /// The password.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterUrl::parse("https://example.com"      ).unwrap().password_str(), ""  );
    /// assert_eq!(BetterUrl::parse("https://ab@example.com"   ).unwrap().password_str(), ""  );
    /// assert_eq!(BetterUrl::parse("https://:cd@example.com"  ).unwrap().password_str(), "cd");
    /// assert_eq!(BetterUrl::parse("https://ab:cd@example.com").unwrap().password_str(), "cd");
    /// ```
    pub fn password_str(&self) -> &str {
        &self.as_str()[self.password_range()]
    }

    /// The [`Password`].
    pub fn password(&self) -> Password<'_> {
        Password::new_unchecked(self.password_str())
    }

    /// Set the password.
    /// # Errors
    /// If the URL doesn't have a host, returns the error [`NoHost`].
    ///
    /// If setting the password to be too long, returns the error [`TooLong`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_password<'a, T: Into<Password<'a>>>(&mut self, password: T) -> Result<(), SetPasswordError> {
        if !self.has_host() {
            Err(NoHost)?;
        }

        let password = password.into();

        let new_len = match (self.username().len(), self.password().len(), password.len()) {
            (_, 0, 0) => self.len(),

            (0, 0, y) => self.len()     + y + 2,
            (0, x, 0) => self.len() - x     - 2,
            (0, x, y) => self.len() - x + y + 1,

            (_, 0, y) => self.len()     + y + 1,
            (_, x, 0) => self.len() - x     - 1,
            (_, x, y) => self.len() - x + y    ,
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        if self.password() != password {
            self.url.set_password(Some(password.as_str())).expect("To be valid.");
        }

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.password(), password);

        Ok(())
    }
}

//! Userinfo stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the userinfo.
    fn userinfo_start(&self) -> usize {
        self.username_start()
    }

    /// The [`Range::end`] of the userinfo.
    fn userinfo_after(&self) -> usize {
        self.password_after()
    }

    /// The [`Range`] of the userinfo.
    fn userinfo_range(&self) -> Range<usize> {
        self.userinfo_start() .. self.userinfo_after()
    }

    /// The userinfo.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterUrl::parse("https://example.com"      ).unwrap().userinfo_str(), ""     );
    /// assert_eq!(BetterUrl::parse("https://ab@example.com"   ).unwrap().userinfo_str(), "ab"   );
    /// assert_eq!(BetterUrl::parse("https://:cd@example.com"  ).unwrap().userinfo_str(), ":cd"  );
    /// assert_eq!(BetterUrl::parse("https://ab:cd@example.com").unwrap().userinfo_str(), "ab:cd");
    /// ```
    pub fn userinfo_str(&self) -> &str {
        &self.as_str()[self.userinfo_range()]
    }

    /// The [`Userinfo`].
    pub fn userinfo(&self) -> Userinfo<'_> {
        Userinfo::new_unchecked(self.userinfo_str())
    }

    /// Set the userinfo.
    /// # Errors
    /// If the URL doesn't have a host, returns the error [`NoHost`].
    ///
    /// If setting the userinfo to be too long, returns the error [`TooLong`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_userinfo<'b, T: Into<Userinfo<'b>>>(&mut self, userinfo: T) -> Result<(), SetUserinfoError> {
        if !self.has_host() {
            Err(NoHost)?;
        }

        let userinfo = userinfo.into();

        let new_len = match (self.userinfo().len(), userinfo.len()) {
            (0, 0) => self.len(),
            (0, y) => self.len()     + y + 1,
            (x, 0) => self.len() - x     - 1,
            (x, y) => self.len() - x + y
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        if self.username() != userinfo.username() {
            self.url.set_username(userinfo.username_str()).expect("To be valid");
        }

        if self.password() != userinfo.password() {
            self.url.set_password(Some(userinfo.password_str())).expect("To be valid");
        }

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.userinfo(), userinfo);

        Ok(())
    }
}

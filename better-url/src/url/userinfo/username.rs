//! [`Username`].

use crate::prelude::*;

impl BetterUrl {
    /// If it has a visible username.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(!BetterUrl::new("https://example.com"                  ).unwrap().has_visible_username());
    /// assert!( BetterUrl::new("https://username@example.com"         ).unwrap().has_visible_username());
    /// assert!( BetterUrl::new("https://:password@example.com"        ).unwrap().has_visible_username());
    /// assert!( BetterUrl::new("https://username:password@example.com").unwrap().has_visible_username());
    /// ```
    pub fn has_visible_username(&self) -> bool {
        self.details.username_after.is_some()
    }

    /// The [`Range::start`] of the username.
    pub(crate) fn username_start(&self) -> Option<usize> {
        if self.has_visible_username() {
            Some(self.details.scheme_mark as usize + 3)
        } else {
            None
        }
    }

    /// The [`Range::end`] of ther username.
    pub(crate) fn username_after(&self) -> Option<usize> {
        Some(self.details.username_after?.get() as usize)
    }

    /// The [`Range`] of the username.
    pub(crate) fn username_range(&self) -> Option<Range<usize>> {
        Some(self.username_start()? .. self.username_after()?)
    }



    /// The visible username as a [`str`].
    ///
    /// If the userinfo (not just the username) is the empty string, and thus doesn't show in the URL, returns [`None`].
    ///
    /// Thus, any [`str`] returned is guaranteed to be a substring of the URL.
    ///
    /// Note that a userinfo of `:password` has a visible but empty username.
    pub fn visible_username_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.username_range()?)})
    }

    /// The username as a [`str`].
    ///
    /// Please note that in the case of an empty userinfo the returned `str` will not be a substring of the URL.
    ///
    /// If you need that property, see [`Self::visible_username_str`].
    pub fn username_str(&self) -> &str {
        self.visible_username_str().unwrap_or_default()
    }



    /// The visible [`Username`].
    ///
    /// See [`Self::visible_username_str`] foe details.
    pub fn visible_username(&self) -> Option<Username<'_>> {
        Some(unsafe {
            Username::new_unchecked(self.visible_username_str()?)
        })
    }

    /// The [`Username`].
    ///
    /// See [`Self::username_str`] foe details.
    pub fn username(&self) -> Username<'_> {
        unsafe {
            Username::new_unchecked(self.username_str())
        }
    }



    /// Set the username.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_username<'a, T: Into<Username<'a>>>(&mut self, value: T) -> Result<(), SetUsernameError> {
        let new = value.into();

        if self.cannot_have_userinfo_or_port() {
            return Ok(())
        }

        match self.username_range() {
            Some(range) => match new.is_empty() && !self.has_visible_password() {
                true => {
                    let diff = range.len() as u32 + 1;

                    let i = self.details.host_start.expect("???").get();

                    self.serialization.replace_range((i - diff) as usize .. i as usize, "");

                    self.details.username_after = None;

                    self.details.host_start = NonZero::new(i - diff);

                    if let Some(x) = self.details.port_mark {self.details.port_mark = NonZero::new(x.get() - diff);}

                    self.details.path_start -= diff;

                    if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get() - diff);}
                },
                false => {
                    if self.len() - range.len() + new.len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    let diff = (new.len() as u32).wrapping_sub(range.len() as u32);

                    self.serialization.replace_range(range, new.as_str());

                    self.details.username_after = NonZero::new(self.details.scheme_mark + 3 + new.len() as u32);

                    if let Some(x) = self.details.host_start     {self.details.host_start     = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.details.port_mark      {self.details.port_mark      = NonZero::new(x.get().wrapping_add(diff));}

                    self.details.path_start = self.details.path_start.wrapping_add(diff);

                    if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
                }
            },
            None => if !new.is_empty() {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = new.len() as u32 + 1;

                let i = self.details.host_start.expect("???").get() as usize;

                self.serialization.insert(i, '@');
                self.serialization.insert_str(i, new.as_str());

                self.details.username_after = NonZero::new(self.details.scheme_mark + 3 + new.len() as u32);

                if let Some(x) = self.details.host_start     {self.details.host_start     = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.details.port_mark      {self.details.port_mark      = NonZero::new(x.get().wrapping_add(diff));}

                self.details.path_start = self.details.path_start.wrapping_add(diff);

                if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            }
        }

        Ok(())
    }
}

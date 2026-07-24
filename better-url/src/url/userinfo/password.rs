//! [`Password`].

use crate::prelude::*;

impl BetterUrl {
    /// If it has a visible password.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(!BetterUrl::new("https://example.com"                  ).unwrap().has_visible_password());
    /// assert!(!BetterUrl::new("https://username@example.com"         ).unwrap().has_visible_password());
    /// assert!( BetterUrl::new("https://:password@example.com"        ).unwrap().has_visible_password());
    /// assert!( BetterUrl::new("https://username:password@example.com").unwrap().has_visible_password());
    /// ```
    pub fn has_visible_password(&self) -> bool {
        self.details.username_after.zip(self.details.host_start).is_some_and(|(ua, hs)| ua.get() != hs.get() - 1)
    }

    /// The [`Range::start`] of the password.
    pub(crate) fn password_start(&self) -> Option<usize> {
        if self.has_visible_password() {
            Some(self.details.username_after?.get() as usize + 1)
        } else {
            None
        }
    }

    /// The [`Range::end`] of the password.
    pub(crate) fn password_after(&self) -> Option<usize> {
        if self.has_visible_password() {
            Some(self.details.host_start?.get() as usize - 1)
        } else {
            None
        }
    }

    /// The [`Range`] of the password.
    fn password_range(&self) -> Option<Range<usize>> {
        Some(self.password_start()? .. self.password_after()?)
    }

    /// The visible password as a [`str`].
    ///
    /// If the password is the empty string, and thus doesn't show in the URL, returns [`None`].
    ///
    /// Thus, any [`str`] returned is guaranteed to be a substring of the URL.
    pub fn visible_password_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.password_range()?)})
    }

    /// The password as a [`str`].
    ///
    /// Please note that in the case of an empty password the returned `str` will not be a substring of the URL.
    ///
    /// If you need that property, see [`Self::visible_password_str`].
    pub fn password_str(&self) -> &str {
        self.visible_password_str().unwrap_or_default()
    }



    /// The visible [`Password`].
    ///
    /// See [`Self::visible_password_str`] foe details.
    pub fn visible_password(&self) -> Option<Password<'_>> {
        Some(unsafe {
            Password::new_unchecked(self.visible_password_str()?)
        })
    }

    /// The [`Password`].
    ///
    /// See [`Self::password_str`] foe details.
    pub fn password(&self) -> Password<'_> {
        unsafe {
            Password::new_unchecked(self.password_str())
        }
    }



    /// Set the password.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_password<'a, T: Into<Username<'a>>>(&mut self, value: T) -> Result<(), SetUsernameError> {
        let new = value.into();

        if self.cannot_have_userinfo_or_port() {
            return Ok(());
        }

        match self.password_range() {
            Some(pr) => match new.is_empty() {
                true  => {
                    let (r, diff) = match self.username_str() {
                        "" => {
                            self.details.username_after = None;

                            let r = pr.start - 1 .. pr.end + 1;

                            let diff = r.len() as u32;

                            (r, diff)
                        },
                        _  => {
                            let r = pr.start - 1 .. pr.end;

                            let diff = r.len() as u32;

                            (r, diff)
                        }
                    };

                    self.serialization.replace_range(r, "");

                    if let Some(x) = self.details.host_start {self.details.host_start = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.details.port_mark  {self.details.port_mark  = NonZero::new(x.get() - diff);}

                    self.details.path_start -= diff;

                    if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get() - diff);}
                },
                false => {
                    if self.len() - pr.len() + new.len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    let diff = (new.len() as u32).wrapping_sub(pr.len() as u32);

                    self.serialization.replace_range(pr, new.as_str());

                    if let Some(x) = self.details.host_start {self.details.host_start = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.details.port_mark  {self.details.port_mark  = NonZero::new(x.get().wrapping_add(diff));}

                    self.details.path_start = self.details.path_start.wrapping_add(diff);

                    if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
                },
            },
            None => if !new.is_empty() {
                match self.username_range() {
                    Some(ur) => {
                        if self.len() + new.len() + 1 > u32::MAX as usize {
                            Err(TooLong)?;
                        }

                        let diff = new.len() as u32 + 1;

                        self.serialization.insert_str(ur.end, new.as_str());
                        self.serialization.insert    (ur.end, ':');

                        self.details.host_start = NonZero::new(ur.end as u32 + 1 + diff);

                        if let Some(x) = self.details.port_mark {self.details.port_mark = NonZero::new(x.get() + diff);}

                        self.details.path_start += diff;

                        if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get() + diff);}
                        if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get() + diff);}
                    },
                    None => {
                        if self.len() + new.len() + 2 > u32::MAX as usize {
                            Err(TooLong)?;
                        }

                        let diff = new.len() as u32 + 2;

                        let i = self.details.host_start.expect("???").get();

                        self.serialization.insert    (i as usize, '@');
                        self.serialization.insert_str(i as usize, new.as_str());
                        self.serialization.insert    (i as usize, ':');

                        self.details.username_after = NonZero::new(i);

                        self.details.host_start = NonZero::new(i + diff);

                        if let Some(x) = self.details.port_mark {self.details.port_mark = NonZero::new(x.get() + diff);}

                        self.details.path_start += diff;

                        if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get() + diff);}
                        if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get() + diff);}
                    },
                }
            }
        }

        Ok(())
    }
}

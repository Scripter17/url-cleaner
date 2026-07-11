//! [`Username`].

use crate::prelude::*;

impl BetterUrl {
    /// If it has a visible username.
    pub fn has_username(&self) -> bool {
        self.username_after.is_some()
    }

    /// If it has a visible password.
    pub fn has_password(&self) -> bool {
        self.username_after.zip(self.host_start).is_none_or(|(ua, hs)| ua.get() != hs.get() - 1)
    }

    /// The [`Range::start`] of the username.
    pub(crate) fn username_start(&self) -> Option<usize> {
        if self.has_username() {
            Some(self.scheme_mark as usize + 3)
        } else {
            None
        }
    }

    /// The [`Range::end`] of ther username.
    pub(crate) fn username_after(&self) -> Option<usize> {
        Some(self.username_after?.get() as usize)
    }

    /// The [`Range`] of the username.
    pub(crate) fn username_range(&self) -> Option<Range<usize>> {
        Some(self.username_start()? .. self.username_after()?)
    }



    /// The username as a [`str`], or [`None`] if absent.
    pub fn maybe_username_str(&self) -> Option<&str> {
        Some(&self.serialization[self.username_range()?])
    }

    /// The username as a [`str`].
    pub fn username_str(&self) -> &str {
        self.maybe_username_str().unwrap_or_default()
    }

    /// The [`Username`].
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
            Some(range) => match new.is_empty() && !self.has_password() {
                true => {
                    let diff = range.len() as u32 + 1;

                    let i = self.host_start.expect("???").get();

                    self.serialization.replace_range((i - diff) as usize .. i as usize, "");

                    self.username_after = None;

                    self.host_start = NonZero::new(i - diff);

                    if let Some(x) = self.port_mark {self.port_mark = NonZero::new(x.get() - diff);}

                    self.path_start -= diff;

                    if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() - diff);}
                },
                false => {
                    if self.len() - range.len() + new.len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    let diff = (new.len() as u32).wrapping_sub(range.len() as u32);

                    self.serialization.replace_range(range, new.as_str());

                    self.username_after = NonZero::new(self.scheme_mark + 3 + new.len() as u32);

                    if let Some(x) = self.host_start     {self.host_start     = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.port_mark      {self.port_mark      = NonZero::new(x.get().wrapping_add(diff));}

                    self.path_start = self.path_start.wrapping_add(diff);

                    if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
                }
            },
            None => if !new.is_empty() {
                if self.len() + new.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = new.len() as u32 + 1;

                let i = self.host_start.expect("???").get() as usize;

                self.serialization.insert(i, '@');
                self.serialization.insert_str(i, new.as_str());

                self.username_after = NonZero::new(self.scheme_mark + 3 + new.len() as u32);

                if let Some(x) = self.host_start     {self.host_start     = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.port_mark      {self.port_mark      = NonZero::new(x.get().wrapping_add(diff));}

                self.path_start = self.path_start.wrapping_add(diff);

                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            }
        }

        Ok(())
    }
}

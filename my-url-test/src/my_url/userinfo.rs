//! [`Userinfo`] and co..

use crate::prelude::*;

impl MyUrl {
    /// If it has a visible username.
    pub fn has_username(&self) -> bool {
        self.username_after.is_some()
    }

    /// If it has a visible password.
    pub fn has_password(&self) -> bool {
        self.username_after.zip(self.host_start).is_none_or(|(ua, hs)| ua.get() != hs.get() - 1)
    }

    /// The [`Range::start`] of the username.
    fn username_start(&self) -> Option<usize> {
        if self.has_username() {
            Some(self.scheme_mark as usize + 3)
        } else {
            None
        }
    }

    /// The [`Range::end`] of ther username.
    fn username_after(&self) -> Option<usize> {
        Some(self.username_after?.get() as usize)
    }

    /// The [`Range`] of the username.
    fn username_range(&self) -> Option<Range<usize>> {
        Some(self.username_start()? .. self.username_after()?)
    }

    /// The username as a [`str`], or [`None`] if absent.
    pub fn maybe_username(&self) -> Option<&str> {
        Some(&self.serialization[self.username_range()?])
    }

    /// The username as a [`str`].
    pub fn username(&self) -> &str {
        self.maybe_username().unwrap_or_default()
    }

    /// The [`Range::start`] of the password.
    fn password_start(&self) -> Option<usize> {
        if self.has_password() {
            Some(self.username_after?.get() as usize + 1)
        } else {
            None
        }
    }

    /// The [`Range::end`] of the password.
    fn password_after(&self) -> Option<usize> {
        if self.has_password() {
            Some(self.host_start?.get() as usize - 1)
        } else {
            None
        }
    }

    /// The [`Range`] of the password.
    fn password_range(&self) -> Option<Range<usize>> {
        Some(self.password_start()? .. self.password_after()?)
    }

    /// The password as a [`str`], or [`None`] if absent.
    pub fn maybe_password(&self) -> Option<&str> {
        Some(&self.serialization[self.password_range()?])
    }

    /// The password as a [`str`].
    pub fn password(&self) -> &str {
        self.maybe_password().unwrap_or_default()
    }

    /// Set the username.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
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

    /// Set the password.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_password<'a, T: Into<Username<'a>>>(&mut self, value: T) -> Result<(), SetUsernameError> {
        let new = value.into();

        if self.cannot_have_userinfo_or_port() {
            return Ok(());
        }

        match self.password_range() {
            Some(pr) => match new.is_empty() {
                true  => {
                    let (r, diff) = match self.username() {
                        "" => {
                            self.username_after = None;

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

                    if let Some(x) = self.host_start {self.host_start = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.port_mark  {self.port_mark  = NonZero::new(x.get() - diff);}

                    self.path_start -= diff;

                    if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() - diff);}
                    if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() - diff);}
                },
                false => {
                    if self.len() - pr.len() + new.len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    let diff = (new.len() as u32).wrapping_sub(pr.len() as u32);

                    self.serialization.replace_range(pr, new.as_str());

                    if let Some(x) = self.host_start {self.host_start = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.port_mark  {self.port_mark  = NonZero::new(x.get().wrapping_add(diff));}

                    self.path_start = self.path_start.wrapping_add(diff);

                    if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                    if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
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

                        self.host_start = NonZero::new(ur.end as u32 + 1 + diff);

                        if let Some(x) = self.port_mark {self.port_mark = NonZero::new(x.get() + diff);}

                        self.path_start += diff;

                        if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() + diff);}
                        if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() + diff);}
                    },
                    None => {
                        if self.len() + new.len() + 2 > u32::MAX as usize {
                            Err(TooLong)?;
                        }

                        let diff = new.len() as u32 + 2;

                        let i = self.host_start.expect("???").get();

                        self.serialization.insert    (i as usize, '@');
                        self.serialization.insert_str(i as usize, new.as_str());
                        self.serialization.insert    (i as usize, ':');

                        self.username_after = NonZero::new(i);

                        self.host_start = NonZero::new(i + diff);

                        if let Some(x) = self.port_mark {self.port_mark = NonZero::new(x.get() + diff);}

                        self.path_start += diff;

                        if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() + diff);}
                        if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() + diff);}
                    },
                }
            }
        }

        Ok(())
    }
}

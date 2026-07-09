//! [`Host`] and co..

use crate::prelude::*;

impl MyUrl {
    /// If it has a host.
    pub fn has_host(&self) -> bool {
        self.host_start.is_some()
    }

    /// The [`Range::start`] of the host.
    fn host_start(&self) -> Option<usize> {
        Some(self.host_start?.get() as usize)
    }

    /// The [`Range::end`] of the host.
    pub(crate) fn host_after(&self) -> Option<usize> {
        if self.host_start.is_some() {
            Some(self.port_mark.map_or(self.path_start, NonZero::get) as usize)
        } else {
            None
        }
    }

    /// The [`Range`] of the host.
    fn host_range(&self) -> Option<Range<usize>> {
        Some(self.host_start()? .. self.host_after()?)
    }

    /// The host as a [`str`].
    pub fn host(&self) -> Option<&str> {
        Some(&self.serialization[self.host_range()?])
    }

    /// If the host is [`DomainHost`].
    pub fn host_is_domain(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Domain(_)))
    }

    /// If the host is [`Ipv4Host`].
    pub fn host_is_ipv4(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Ipv4(_)))
    }

    /// If the host is [`Ipv6Host`].
    pub fn host_is_ipv6(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Ipv6(_)))
    }

    /// If the host is [`Ipv4Host`] or [`Ipv6Host`].
    pub fn host_is_ip(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_)))
    }

    /// If the host is [`OpaqueHost`].
    pub fn host_is_opaque(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Opaque(_)))
    }

    /// If the host is [`EmptyHost`].
    pub fn host_is_empty(&self) -> bool {
        matches!(self.details.host, Some(HostDetails::Empty(_)))
    }

    /// Set the host.
    /// # Errors
    /// If the call to [`Host::new`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::cannot_be_a_base`] returns true, returns the error [`CantHaveHost`].
    ///
    /// If the URL would become too long, returns the error [`TooLong`].
    ///
    /// If the URL has a visible userinfo and/or port and the host is empty, returns the error [`CantBeNone`].
    pub fn set_host<'a, T: TryInto<FileHost<'a>> + TryInto<SpecialNotFileHost<'a>> + TryInto<NonSpecialHost<'a>>>(&mut self, value: T) -> Result<(), SetHostError>
        where InvalidHost: From<<T as TryInto<FileHost<'a>>>::Error> + From<<T as TryInto<SpecialNotFileHost<'a>>>::Error> + From<<T as TryInto<NonSpecialHost<'a>>>::Error>
    {
        let new = Host::new(value, self.scheme_type())?;

        if self.cannot_be_a_base() {
            Err(CantHaveHost)?;
        }

        match self.host_range() {
            None => {
                if self.path_start - self.scheme_mark == 3 {
                    let diff = new.len();

                    if self.len() + diff > u32::MAX as usize {
                        Err(TooLong)?;
                    }
                    unsafe {
                        self.serialization.as_mut_vec()[self.scheme_mark as usize + 2] = b'/';
                    };
                    self.serialization.insert_str(self.scheme_mark as usize + 3, new.as_str());
                    self.details.host = Some(new.details());

                    self.host_start = NonZero::new(self.scheme_mark + 3);
                    self.path_start += diff as u32;

                    if let Some(x) = self.port_mark     {self.port_mark     = NonZero::new(x.get() + diff as u32);}
                    if let Some(q) = self.query_mark    {self.query_mark    = NonZero::new(q.get() + diff as u32);}
                    if let Some(f) = self.fragment_mark {self.fragment_mark = NonZero::new(f.get() + diff as u32);}
                } else {
                    let diff = new.len() + 2;

                    if self.len() + diff > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    self.serialization.insert_str(self.scheme_mark as usize + 1, "//");
                    self.serialization.insert_str(self.scheme_mark as usize + 3, new.as_str());
                    self.details.host = Some(new.details());

                    self.host_start = NonZero::new(self.scheme_mark + 3);
                    self.path_start += diff as u32;

                    if let Some(x) = self.port_mark     {self.port_mark     = NonZero::new(x.get() + diff as u32);}
                    if let Some(q) = self.query_mark    {self.query_mark    = NonZero::new(q.get() + diff as u32);}
                    if let Some(f) = self.fragment_mark {self.fragment_mark = NonZero::new(f.get() + diff as u32);}
                }
            },

            Some(range) => {
                if new.is_empty() && (self.username_after.is_some() || range.end != self.path_start as usize) {
                    Err(CantBeNone)?;
                }

                let start_len = self.len();
                let after_len = self.len() - range.len() + new.len();

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(start_len as u32);

                self.serialization.replace_range(range.clone(), new.as_str());
                self.details.host = Some(new.details());

                self.path_start = self.path_start.wrapping_add(diff);

                if let Some(x) = self.port_mark     {self.port_mark     = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(q) = self.query_mark    {self.query_mark    = NonZero::new(q.get().wrapping_add(diff));}
                if let Some(f) = self.fragment_mark {self.fragment_mark = NonZero::new(f.get().wrapping_add(diff));}
            },
        }

        Ok(())
    }
}

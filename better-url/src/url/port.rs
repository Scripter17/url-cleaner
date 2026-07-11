//! [`Port`] and co..

use crate::prelude::*;

impl BetterUrl {
    /// If it cannot have a userinfo or port.
    ///
    /// Specifically, if it has no host, the host is empty, or the scheme is `file`.
    pub fn cannot_have_userinfo_or_port(&self) -> bool {
        !self.has_host() || self.host_is_empty() || self.is_file()
    }

    /// If it has a port.
    pub fn has_port(&self) -> bool {
        self.port_mark.is_some()
    }

    /// The [`Range::start`] of the port.
    fn port_start(&self) -> Option<usize> {
        Some(self.port_mark?.get() as usize + 1)
    }

    /// The [`Range::end`] of the port.
    fn port_after(&self) -> Option<usize> {
        if self.port_mark.is_some() {
            Some(self.path_start as usize)
        } else {
            None
        }
    }

    /// The [`Range`] of the port.
    fn port_range(&self) -> Option<Range<usize>> {
        Some(self.port_start()? .. self.port_after()?)
    }

    /// The port as a [`u16`].
    pub fn port_num(&self) -> Option<u16> {
        match self.has_port() {
            true  => Some(self.port),
            false => None,
        }
    }

    /// The port as a [`str`].
    pub fn port_str(&self) -> Option<&str> {
        Some(&self.serialization[self.port_range()?])
    }

    /// The [`MaybePort`].
    pub fn port(&self) -> MaybePort<'_> {
        unsafe {
            MaybePort::new_unchecked(self.port_str().zip(self.port_num()))
        }
    }

    /// Set the port.
    /// # Errors
    /// If the call to [`MaybePort::new`] returns an error, returns the error [`InvalidPort`].
    ///
    /// If the call to [`Self::cannot_have_userinfo_or_port`] returns [`true`], returns the error [`SetPortError::CantHavePort`].
    ///
    /// If the URL would become too long, returns the error [`TooLong`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_port<'a, T: TryInto<MaybePort<'a>>>(&mut self, value: T) -> Result<(), SetPortError> where InvalidPort: From<T::Error> {
        let new = value.try_into().map_err(InvalidPort::from)?;

        if self.cannot_have_userinfo_or_port() {
            Err(SetPortError::CantHavePort)?;
        }

        let host_after = self.host_after().expect("???");

        match (self.port_range(), new.0) {
            (None, None     ) => {},
            (None, Some(new)) => if Some(new.as_num()) != self.details.scheme.default_port_num() {
                let diff = new.as_str().len() + 1;

                if self.len() + diff > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.insert_str(self.path_start as usize, new.as_str());
                self.serialization.insert    (self.path_start as usize, ':');

                self.port = new.as_num();

                self.path_start += diff as u32;

                self.port_mark = NonZero::new(host_after as u32);
                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() + diff as u32);}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() + diff as u32);}
            },
            (Some(pr), Some(new)) if Some(new.as_num()) != self.details.scheme.default_port_num() => {
                let diff = new.as_str().len() as isize - pr.len() as isize;

                if self.len().wrapping_add_signed(diff) > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.serialization.replace_range(pr, new.as_str());
                self.port = new.as_num();

                self.path_start = self.path_start.wrapping_add_signed(diff as i32);

                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add_signed(diff as i32));}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add_signed(diff as i32));}
            },
            (Some(pr), _) => {
                let r = pr.start - 1 .. pr.end;
                let diff = r.len();

                self.serialization.replace_range(r, "");

                self.path_start -= diff as u32;
                self.port_mark = None;

                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get() - diff as u32);}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get() - diff as u32);}
            },
        }

        Ok(())
    }
}

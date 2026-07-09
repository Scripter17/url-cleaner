//! Port stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the port.
    fn port_start(&self) -> Option<usize> {
        self.url.port()?;
        Some(self.host_str()?.end_addr() - self.as_str().addr() + 1)
    }

    /// The [`Range::end`] of the port.
    fn port_after(&self) -> Option<usize> {
        self.url.port()?;
        Some(self.path_str().addr() - self.as_str().addr())
    }

    /// The [`Range`] of the port.
    fn port_range(&self) -> Option<Range<usize>> {
        Some(self.port_start()? .. self.port_after()?)
    }

    /// The port as a [`str`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterUrl::parse("https://example.com"    ).unwrap().port_str(), None);
    /// assert_eq!(BetterUrl::parse("https://example.com:123").unwrap().port_str(), Some("123"));
    /// ```
    pub fn port_str(&self) -> Option<&str> {
        Some(&self.as_str()[self.port_range()?])
    }

    /// [`Self::port_str`] or [`SchemeDetails::default_port_str`].
    pub fn port_str_or_known_default(&self) -> Option<&str> {
        self.port_str().or_else(|| self.scheme_details().default_port_str())
    }

    /// The port as a [`u16`].
    pub fn port_num(&self) -> Option<u16> {
        self.url.port()
    }

    /// The port or known default as a [`u16`]
    pub fn port_num_or_known_default(&self) -> Option<u16> {
        self.url.port_or_known_default()
    }

    /// The port.
    pub fn port(&self) -> MaybePort<'_> {
        match self.port_str().zip(self.port_num()) {
            Some((s, n)) => MaybePort(Some(Port {port: s.into(), port_num: n})),
            None => MaybePort(None)
        }
    }

    /// [`Self::port`] or [`SchemeDetails::default_port`].
    pub fn port_or_known_default(&self) -> MaybePort<'_> {
        match self.port() {
            x @ MaybePort(Some(_)) => x,
            MaybePort(None) => MaybePort(self.scheme_details().default_port())
        }
    }

    /// Set the port.
    /// # Errors
    /// If there is no host, returns the error [`NoHost`].
    ///
    /// If the host is [`Host::Empty`], returns the error [`SetPortError::EmptyHost`].
    ///
    /// If the scheme is `file`, returns the error [`SetPortError::SchemeIsFile`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_port<'a, T: TryInto<MaybePort<'a>>>(&mut self, port: T) -> Result<bool, SetPortError> where SetPortError: From<T::Error> {
        let port = port.try_into()?;

        if self.host().ok_or(NoHost)?.is_empty() {
            Err(SetPortError::EmptyHost)?;
        }

        if self.scheme().is_file() {
            Err(SetPortError::SchemeIsFile)?;
        }

        if port != self.port_or_known_default() {
            self.url.set_port(port.as_num()).expect("To always work.");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

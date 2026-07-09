//! Host stuff.

use crate::prelude::*;

mod domain;

impl BetterUrl {
    /// If it has a host.
    pub fn has_host(&self) -> bool {
        self.host_details().is_some()
    }

    /// The host.
    pub fn host_str(&self) -> Option<&str> {
        self.url.host_str()
    }

    /// The [`Host`].
    pub fn host(&self) -> Option<Host<'_>> {
        let host = self.host_str()?.into();

        Some(match self.host_details()? {
            HostDetails::Domain(details) => DomainHost {host, details}.into(),
            HostDetails::Ipv4  (details) => Ipv4Host   {host, details}.into(),
            HostDetails::Ipv6  (details) => Ipv6Host   {host, details}.into(),
            HostDetails::Opaque(details) => OpaqueHost {host, details}.into(),
            HostDetails::Empty (_      ) => EmptyHost::default()      .into(),
        })
    }

    /// Set the host.
    /// # Errors
    /// If the call to [`Host::new_file`] returns an error, that error is returned.
    ///
    /// If the call to [`Host::new_special_not_file`] returns an error, that error is returned.
    ///
    /// If the call to [`Host::new_non_special`] returns an error, that error is returned.
    ///
    /// If attempting to set a non-empty host to the empty host, reutrns the error [`CantBeEmpty`].
    ///
    /// If attempting to set a [`SchemeType::SpecialNotFile`] URL's host to [`None`], returns the error [`CantBeNone`].
    ///
    /// If setting the host would make the URL too long, returns the error [`TooLong`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_host(Some("example.co.uk")).unwrap    ();
    /// url.set_host(Some("")             ).unwrap_err();
    /// url.set_host(None::<&str>         ).unwrap_err();
    ///
    /// let mut url = BetterUrl::parse("file://example.com").unwrap();
    ///
    /// url.set_host(Some("example.co.uk")).unwrap    ();
    /// url.set_host(Some("")             ).unwrap_err();
    /// url.set_host(None::<&str>         ).unwrap    ();
    ///
    /// let mut url = BetterUrl::parse("file://example.com/a/b/c").unwrap();
    ///
    /// url.set_host(Some("example.co.uk")).unwrap    ();
    /// url.set_host(Some("")             ).unwrap_err();
    /// url.set_host(None::<&str>         ).unwrap    ();
    ///
    /// let mut url = BetterUrl::parse("other://example.com/a/b/c").unwrap();
    ///
    /// url.set_host(Some("example.co.uk")).unwrap    ();
    /// url.set_host(Some("")             ).unwrap_err();
    /// url.set_host(None::<&str>         ).unwrap    ();
    /// ```
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_host<'a, T: TryInto<FileHost<'a>> + TryInto<SpecialNotFileHost<'a>> + TryInto<NonSpecialHost<'a>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> 
        where InvalidHost: From<<T as TryInto<FileHost<'a>>>::Error> + From<<T as TryInto<SpecialNotFileHost<'a>>>::Error> + From<<T as TryInto<NonSpecialHost<'a>>>::Error>
    {
        let host = value.map(|x| Host::new(x, self.scheme_type())).transpose()?;

        let new_len = match (self.host_str(), host.as_ref()) {
            (None     , None     )                   => return Ok(false),
            (Some(old), Some(new)) if old == new     => return Ok(false),

            (Some(_  ), Some(new)) if new.is_empty() => Err(CantBeEmpty)?,

            (Some(old), None) => match self.scheme_type() {
                SchemeType::SpecialNotFile => Err(CantBeNone)?,
                SchemeType::File           => self.len() - old.len(),
                SchemeType::NonSpecial     => self.len() - old.len() - 2,
            },

            (None     , Some(new)) => self.len()             + new.len(),
            (Some(old), Some(new)) => self.len() - old.len() + new.len(),
        };

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.url.set_host(host.as_ref().map(Host::as_str)).expect("To always work.");
        self.details.host = host.as_ref().map(Host::details);

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.host(), host);

        Ok(true)
    }



    /// The [`HostDetails`].
    pub fn host_details(&self) -> Option<HostDetails> {
        self.details.host
    }

    /// The [`DomainDetails`].
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.host_details()?.domain()
    }

    /// The [`Ipv4Details`].
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.host_details()?.ipv4()
    }

    /// The [`Ipv6Details`].
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.host_details()?.ipv6()
    }

    /// The [`IpDetails`].
    pub fn ip_details(&self) -> Option<IpDetails> {
        self.host_details()?.ip()
    }

    /// The [`OpaqueHostDetails`].
    pub fn opaque_host_details(&self) -> Option<OpaqueHostDetails> {
        self.host_details()?.opaque()
    }

    /// The [`EmptyHostDetails`].
    pub fn empty_host_details(&self) -> Option<EmptyHostDetails> {
        self.host_details()?.empty()
    }



    /// If [`Self::host`] is [`Host::Domain`].
    pub fn host_is_domain(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Domain(_)))
    }

    /// If [`Self::host`] is [`Host::Ipv4`].
    pub fn host_is_ipv4(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Ipv4(_)))
    }

    /// If [`Self::host`] is [`Host::Ipv6`].
    pub fn host_is_ipv6(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Ipv6(_)))
    }

    /// If [`Self::host`] is [`Host::Ipv4`] or [`Host::ipv6`].
    pub fn host_is_ip(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_)))
    }

    /// If [`Self::host`] is [`Host::Opaque`].
    pub fn host_is_opaque(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Opaque(_)))
    }

    /// If [`Self::host`] is [`Host::Empty`].
    pub fn host_is_empty(&self) -> bool {
        matches!(self.host_details(), Some(HostDetails::Empty(_)))
    }
}

//! Host stuff.

use std::net::IpAddr;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use url::Url;

use crate::prelude::*;

mod domain;

impl BetterUrl {
    /// Get a borrowing [`BetterHost`].
    pub fn host(&self) -> Option<BetterHost<'_>> {
        self.ref_host().map(Into::into)
    }

    /// Get a borrowing [`BetterRefHost`].
    pub fn ref_host(&self) -> Option<BetterRefHost<'_>> {
        let host = self.host_str()?;

        Some(match self.host_details? {
            HostDetails::Domain(details) => BetterRefDomainHost {host, details}.into(),
            HostDetails::Ipv4  (details) => BetterRefIpv4Host   {host, details}.into(),
            HostDetails::Ipv6  (details) => BetterRefIpv6Host   {host, details}.into(),
        })
    }

    /// Get the [`HostDetails`].
    pub fn host_details(&self) -> Option<HostDetails> {
        self.host_details
    }

    /// Get the [`DomainDetails`].
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.host_details()?.domain()
    }

    /// Get the [`Ipv4Details`].
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.host_details()?.ipv4()
    }

    /// Get the [`Ipv6Details`].
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.host_details()?.ipv6()
    }

    /// Get the [`IpDetails`].
    pub fn ip_details(&self) -> Option<IpDetails> {
        self.host_details()?.ip()
    }

    /// [`BetterRefHost::normal`].
    pub fn normal_host(&self) -> Option<&str> {
        Some(self.ref_host()?.normal())
    }

    /// [`Url::set_host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, the error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), SetHostError> {
        if self.host_str() != host {
            self.url.set_host(host)?;
            self.host_details = HostDetails::from_url(self);
        }

        Ok(())
    }

    /// [`Url::set_ip_host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, returns the error [`SetIpHostError`].
    pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), SetIpHostError> {
        self.url.set_ip_host(address).map_err(|()| SetIpHostError)?;
        self.host_details = Some(address.into());

        Ok(())
    }

    /// [`Url::set_host`] without recalculating the [`HostDetails`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, that error is returned.
    pub fn set_better_host<'a, T: Into<BetterHost<'a>>>(&mut self, host: Option<T>) -> Result<(), SetHostError> {
        let host = host.map(Into::into);

        if host != self.host() {
            self.url.set_host(host.as_ref().map(BetterHost::as_str))?;
            self.host_details = host.as_ref().map(BetterHost::details);
        }

        Ok(())
    }
}

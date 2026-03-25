//! [`HostDetails`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use url::{Url, Host};

use crate::prelude::*;

/// Details for a host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostDetails {
    /// A [`DomainDetails`].
    Domain(DomainDetails),
    /// An [`Ipv4Details`].
    Ipv4(Ipv4Details),
    /// An [`Ipv6Details`].
    Ipv6(Ipv6Details),
}

impl HostDetails {
    /// Parse a host.
    ///
    /// - If `s` starts with `[`, attempts to parse as a [`Self::Ipv6`].
    ///
    /// - If `s` ends in a number, attempts to parse as a [`Self::Ipv4`].
    ///
    /// - Otherwise, attempts to parse as a [`Self::Domain`].
    /// # Errors
    /// If the call to [`Ipv6Details::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`Ipv4Details::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`DomainDetails::parse`] returns an error, that error is returned.
    pub fn parse(s: &str) -> Result<Self, InvalidHost> {
        Ok(if s.starts_with("[") {
            Self::Ipv6(s.parse()?)
        } else if ends_in_a_number(s) {
            Self::Ipv4(s.parse()?)
        } else {
            Self::Domain(DomainDetails::parse_not_eian(s)?)
        })
    }

    /// Make from [`Url`], allowing for skipping some checks.
    pub fn from_url(url: &Url) -> Option<Self> {
        Some(match url.host()? {
            Host::Domain(x) => Self::Domain(DomainDetails::parse_unchecked(x)),
            Host::Ipv4  (x) => x.into(),
            Host::Ipv6  (x) => x.into(),
        })
    }

    /// Get the [`DomainDetails`].
    pub fn domain(self) -> Option<DomainDetails> {
        self.try_into().ok()
    }

    /// Get the [`Ipv4Details`].
    pub fn ipv4(self) -> Option<Ipv4Details> {
        self.try_into().ok()
    }

    /// Get the [`Ipv6Details`].
    pub fn ipv6(self) -> Option<Ipv6Details> {
        self.try_into().ok()
    }

    /// Get the [`IpDetails`].
    pub fn ip(self) -> Option<IpDetails> {
        self.try_into().ok()
    }
}

impl From<DomainDetails> for HostDetails {fn from(value: DomainDetails) -> Self {Self::Domain(value)}}
impl From<Ipv4Details  > for HostDetails {fn from(value: Ipv4Details  ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6Details  > for HostDetails {fn from(value: Ipv6Details  ) -> Self {Self::Ipv6  (value)}}

impl From<Ipv4Addr> for HostDetails {fn from(value: Ipv4Addr) -> Self {Self::Ipv4(value.into())}}
impl From<Ipv6Addr> for HostDetails {fn from(value: Ipv6Addr) -> Self {Self::Ipv6(value.into())}}

impl From<IpAddr> for HostDetails {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(addr) => addr.into(),
            IpAddr::V6(addr) => addr.into(),
        }
    }
}

impl From<IpDetails> for HostDetails {
    fn from(value: IpDetails) -> Self {
        match value {
            IpDetails::V4(x) => x.into(),
            IpDetails::V6(x) => x.into(),
        }
    }
}

impl TryFrom<&str> for HostDetails {
    type Error = InvalidHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for HostDetails {
    type Err = InvalidHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

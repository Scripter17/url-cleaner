//! Details for hosts not stored/exposed by [`url`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use thiserror::Error;

use url::Url;

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

mod domain;
pub use domain::*;
mod ip;
pub use ip::*;

/// The details of a [`BetterUrl`]'s host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum HostDetails {
    /// Details of a [`BetterUrl`]'s domain host.
    Domain(DomainDetails),
    /// Details of a [`BetterUrl`]'s IPv4 host.
    Ipv4(Ipv4Details),
    /// Details of a [`BetterUrl`]'s IPv6 host.
    Ipv6(Ipv6Details)
}

impl From<DomainDetails> for HostDetails {
    fn from(value: DomainDetails) -> Self {
        Self::Domain(value)
    }
}

impl From<Ipv4Details> for HostDetails {
    fn from(value: Ipv4Details) -> Self {
        Self::Ipv4(value)
    }
}

impl From<Ipv6Details> for HostDetails {
    fn from(value: Ipv6Details) -> Self {
        Self::Ipv6(value)
    }
}

/// The error returned when trying to convert a non-[`HostDetails::Domain`] into a [`DomainDetails`].
#[derive(Debug, Error)]
#[error("The host is not a domain.")]
pub struct HostIsNotDomain;

impl TryFrom<HostDetails> for DomainDetails {
    type Error = HostIsNotDomain;

    fn try_from(value: HostDetails) -> Result<DomainDetails, Self::Error> {
        match value {
            HostDetails::Domain(x) => Ok(x),
            _ => Err(HostIsNotDomain)
        }
    }
}

/// The error returned when trying to convert a non-[`HostDetails::Ipv4`] into a [`Ipv4Details`].
#[derive(Debug, Error)]
#[error("The host is not an IPv4 address.")]
pub struct HostIsNotIpv4;

impl TryFrom<HostDetails> for Ipv4Details {
    type Error = HostIsNotIpv4;

    fn try_from(value: HostDetails) -> Result<Ipv4Details, Self::Error> {
        match value {
            HostDetails::Ipv4(x) => Ok(x),
            _ => Err(HostIsNotIpv4)
        }
    }
}

/// The error returned when trying to convert a non-[`HostDetails::Ipv6`] into a [`Ipv6Details`].
#[derive(Debug, Error)]
#[error("The host is not an IPv6 address.")]
pub struct HostIsNotIpv6;

impl TryFrom<HostDetails> for Ipv6Details {
    type Error = HostIsNotIpv6;

    fn try_from(value: HostDetails) -> Result<Ipv6Details, Self::Error> {
        match value {
            HostDetails::Ipv6(x) => Ok(x),
            _ => Err(HostIsNotIpv6)
        }
    }
}

impl FromStr for HostDetails {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl HostDetails {
    /// Gets the details of a host [`str`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(matches!(HostDetails::parse("example.com").unwrap(), HostDetails::Domain(_)));
    /// assert!(matches!(HostDetails::parse("127.0.0.1"  ).unwrap(), HostDetails::Ipv4  (_)));
    /// assert!(matches!(HostDetails::parse("[::1]"      ).unwrap(), HostDetails::Ipv6  (_)));
    /// ```
    pub fn parse(host: &str) -> Result<Self, url::ParseError> {
        url::Host::parse(host).map(|host| Self::from_host(&host))
    }

    /// Gets the details of a [`url::Host`].
    ///
    /// Assumes [`url::Host::Domain`] always contains valid domains, which isn't true, but is true enough to work.
    /// # Examples
    /// ```
    /// use url::Host;
    /// use better_url::prelude::*;
    ///
    /// assert!(matches!(HostDetails::from_host(&Host::parse("example.com").unwrap()), HostDetails::Domain(_)));
    /// assert!(matches!(HostDetails::from_host(&Host::parse("127.0.0.1"  ).unwrap()), HostDetails::Ipv4  (_)));
    /// assert!(matches!(HostDetails::from_host(&Host::parse("[::1]"      ).unwrap()), HostDetails::Ipv6  (_)));
    /// ```
    pub fn from_host<T: AsRef<str>>(host: &url::Host<T>) -> Self {
        match host {
            url::Host::Domain(domain) => Self::Domain(DomainDetails::parse_unchecked(domain.as_ref())),
            url::Host::Ipv4  (addr  ) => (*addr).into(),
            url::Host::Ipv6  (addr  ) => (*addr).into()
        }
    }

    /// Gets the details of a [`url::Url`].
    pub fn from_url(url: &Url) -> Option<Self> {
        url.host().map(|host| Self::from_host(&host))
    }

    /// If `self` is [`Self::Domain`], return it.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(matches!(HostDetails::parse("example.com").unwrap().domain_details(), Some(_)));
    /// assert!(matches!(HostDetails::parse("127.0.0.1"  ).unwrap().domain_details(), None   ));
    /// assert!(matches!(HostDetails::parse("[::1]"      ).unwrap().domain_details(), None   ));
    /// ```
    pub fn domain_details(&self) -> Option<DomainDetails> {
        match self {
            Self::Domain(details) => Some(*details),
            _ => None
        }
    }

    /// If `self` is an IP, return the corresponding [`IpDetails`].
    pub fn ip_details(&self) -> Option<IpDetails> {
        match self {
            Self::Ipv4(details) => Some(IpDetails::V4(*details)),
            Self::Ipv6(details) => Some(IpDetails::V6(*details)),
            _ => None
        }
    }

    /// If `self` is [`Self::Ipv4`], return it.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(matches!(HostDetails::parse("example.com").unwrap().ipv4_details(), None   ));
    /// assert!(matches!(HostDetails::parse("127.0.0.1"  ).unwrap().ipv4_details(), Some(_)));
    /// assert!(matches!(HostDetails::parse("[::1]"      ).unwrap().ipv4_details(), None   ));
    /// ```
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        match self {
            Self::Ipv4(details) => Some(*details),
            _ => None
        }
    }

    /// If `self` is [`Self::Ipv6`], return it.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(matches!(HostDetails::parse("example.com").unwrap().ipv6_details(), None   ));
    /// assert!(matches!(HostDetails::parse("127.0.0.1"  ).unwrap().ipv6_details(), None   ));
    /// assert!(matches!(HostDetails::parse("[::1]"      ).unwrap().ipv6_details(), Some(_)));
    /// ```
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        match self {
            Self::Ipv6(details) => Some(*details),
            _ => None
        }
    }
}


impl From<Ipv4Addr> for HostDetails {
    fn from(addr: Ipv4Addr) -> Self {
        Self::Ipv4(Ipv4Details::from(addr))
    }
}

impl From<Ipv6Addr> for HostDetails {
    fn from(addr: Ipv6Addr) -> Self {
        Self::Ipv6(Ipv6Details::from(addr))
    }
}

impl From<IpAddr> for HostDetails {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(addr) => addr.into(),
            IpAddr::V6(addr) => addr.into()
        }
    }
}

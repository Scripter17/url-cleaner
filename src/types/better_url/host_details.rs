//! Details of a host.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[allow(unused_imports, reason = "Doc links.")]
use url::Url;
#[allow(unused_imports, reason = "Doc links.")]
use crate::types::*;

mod domain;
pub use domain::*;
mod ipv4;
pub use ipv4::*;
mod ipv6;
pub use ipv6::*;

/// Details for a [`BetterUrl`]'s [`Url`]'s host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostDetails {
    /// Details for a host that is a [`url::Host::Domain`].
    Domain(DomainDetails),
    /// Details for a host that is a [`url::Host::Ipv4`].
    Ipv4(Ipv4Details),
    /// Details for a host that is a [`url::Host::Ipv6`].
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

/// Returned when trying to convert a non-domain [`HostDetails`] into a [`DomainDetails`].
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

/// Returned when trying to convert a non-IPv4 address [`HostDetails`] into a [`Ipv4Details`].
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

/// Returned when trying to convert a non-IPv6 address [`HostDetails`] into a [`Ipv6Details`].
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

impl HostDetails {
    /// Gets the details of a host [`str`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    pub fn from_host_str(host: &str) -> Result<Self, url::ParseError> {
        url::Host::parse(host).map(Self::from_host)
    }

    /// Gets the details of a domain [`str`].
    ///
    /// PLEASE note that passing, for example, `"127.0.0.1"` gives very nonsensical results.
    ///
    /// If you are even remotely possibly not always handling domains, please use [`HostDetails::from_host`] or [`HostDetails::from_host_str`].
    pub fn from_domain_str(domain: &str) -> Self {
        Self::Domain(DomainDetails::from_domain_str(domain.as_ref()))
    }

    /// Gets the details of an [`Ipv4Addr`].
    pub fn from_ipv4_addr(addr: Ipv4Addr) -> Self {
        Self::Ipv4(Ipv4Details::from_addr(addr))
    }

    /// Gets the details of an [`Ipv6Addr`].
    pub fn from_ipv6_addr(addr: Ipv6Addr) -> Self {
        Self::Ipv6(Ipv6Details::from_addr(addr))
    }

    /// Gets the details of an [`IpAddr`].
    pub fn from_ip_addr(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(addr) => Self::from_ipv4_addr(addr),
            IpAddr::V6(addr) => Self::from_ipv6_addr(addr)
        }
    }

    /// Gets the details of a [`url::Host`] as long as its domain variant is [`AsRef<str>`].
    pub fn from_host<T: AsRef<str>>(host: url::Host<T>) -> Self {
        match host {
            url::Host::Domain(domain) => Self::from_domain_str(domain.as_ref()),
            url::Host::Ipv4  (addr  ) => Self::from_ipv4_addr(addr),
            url::Host::Ipv6  (addr  ) => Self::from_ipv6_addr(addr)
        }
    }
}

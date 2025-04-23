//! Details for hosts not stored/exposed by [`url`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[expect(unused_imports, reason = "Doc links.")]
use url::Url;
#[expect(unused_imports, reason = "Doc links.")]
use crate::types::*;

pub mod domain;
pub use domain::*;
pub mod ip;
pub use ip::*;

#[expect(unused_imports, reason = "Doc links.")]
use crate::types::*;

/// The details of a [`BetterUrl`]'s host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl HostDetails {
    /// Gets the details of a host [`str`].
    /// # Errors
    /// If the call to [`url::Host::parse`] returns an error, that error is returned.
    pub fn from_host_str(host: &str) -> Result<Self, url::ParseError> {
        url::Host::parse(host).map(|host| Self::from_host(&host))
    }

    /// Gets the details of a domain host.
    /// # Errors
    /// If the call to [`DomainDetails::from_domain_str] returns an error, that error is returned.
    pub fn from_domain_str(domain: &str) -> Result<Self, GetDomainDetailsError> {
        Ok(DomainDetails::from_domain_str(domain)?.into())
    }

    /// Gets the details of a domain host without checking if it's a domain.
    pub fn from_domain_str_unchecked(domain: &str) -> Self {
        DomainDetails::from_domain_str_unchecked(domain).into()
    }

    /// Gets the details of an [`Ipv4Addr`] host.
    pub fn from_ipv4_addr(addr: Ipv4Addr) -> Self {
        Self::Ipv4(Ipv4Details::from_addr(addr))
    }

    /// Gets the details of an [`Ipv6Addr`] host.
    pub fn from_ipv6_addr(addr: Ipv6Addr) -> Self {
        Self::Ipv6(Ipv6Details::from_addr(addr))
    }

    /// Gets the details of an [`IpAddr`] host.
    pub fn from_ip_addr(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(addr) => Self::from_ipv4_addr(addr),
            IpAddr::V6(addr) => Self::from_ipv6_addr(addr)
        }
    }

    /// Gets the details of a [`url::Host`].
    pub fn from_host<T: AsRef<str>>(host: &url::Host<T>) -> Self {
        match host {
            url::Host::Domain(domain) => Self::from_domain_str_unchecked(domain.as_ref()),
            url::Host::Ipv4  (addr  ) => Self::from_ipv4_addr(*addr),
            url::Host::Ipv6  (addr  ) => Self::from_ipv6_addr(*addr)
        }
    }

    /// If `self` is [`Self::Domain`], return it.
    pub fn domain_details(self) -> Option<DomainDetails> {
        match self {
            Self::Domain(ret) => Some(ret),
            _ => None
        }
    }

    /// If `self` is [`Self::Ipv4`], return it.
    pub fn ipv4_details(self) -> Option<Ipv4Details> {
        match self {
            Self::Ipv4(ret) => Some(ret),
            _ => None
        }
    }

    /// If `self` is [`Self::Ipv6`], return it.
    pub fn ipv6_details(self) -> Option<Ipv6Details> {
        match self {
            Self::Ipv6(ret) => Some(ret),
            _ => None
        }
    }
}

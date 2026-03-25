//! [`IpDetails`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use crate::prelude::*;

/// Either an [`Ipv4Details`] or an [`Ipv6Details`].
///
/// See [`HostDetails`] for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpDetails {
    /// [`Ipv4Details`].
    V4(Ipv4Details),
    /// [`Ipv6Details`].
    V6(Ipv6Details),
}

impl IpDetails {
    /// Parse an IP host.
    ///
    /// - If `s` starts with `[`, attempts to parse as a [`Self::V6`].
    ///
    /// - Otherwise, attempts to parse as a [`Self::V4`].
    /// # Errors
    /// If the call to [`Ipv6Details::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`Ipv4Details::parse`] returns an error, that error is returned.
    pub fn parse(s: &str) -> Result<Self, InvalidIpHost> {
        Ok(if s.starts_with("[") {
            Self::V6(s.parse()?)
        } else {
            Self::V4(s.parse()?)
        })
    }

    /// Make the [`IpAddr`].
    pub fn to_ip(self) -> IpAddr {
        match self {
            Self::V4(Ipv4Details {parsed}) => parsed.into(),
            Self::V6(Ipv6Details {parsed}) => parsed.into(),
        }
    }

    /// Returns [`true`] if `self` is [`Self::V4`].
    pub fn is_ipv4(self) -> bool {
        matches!(self, Self::V4(_))
    }

    /// Returns [`true`] if `self` is [`Self::V6`].
    pub fn is_ipv6(self) -> bool {
        matches!(self, Self::V6(_))
    }

    /// [`IpAddr::is_loopback`].
    pub fn is_loopback(self) -> bool {
        self.to_ip().is_loopback()
    }

    /// [`IpAddr::is_multicast`].
    pub fn is_multicast(self) -> bool {
        self.to_ip().is_multicast()
    }

    /// [`IpAddr::is_unspecified`].
    pub fn is_unspecified(self) -> bool {
        self.to_ip().is_unspecified()
    }
}

impl From<Ipv4Details> for IpDetails {fn from(value: Ipv4Details) -> Self {Self::V4(value)}}
impl From<Ipv6Details> for IpDetails {fn from(value: Ipv6Details) -> Self {Self::V6(value)}}

impl From<Ipv4Addr> for IpDetails {fn from(value: Ipv4Addr) -> Self {Self::V4(value.into())}}
impl From<Ipv6Addr> for IpDetails {fn from(value: Ipv6Addr) -> Self {Self::V6(value.into())}}

impl From<IpAddr> for IpDetails {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(x) => x.into(),
            IpAddr::V6(x) => x.into(),
        }
    }
}

impl TryFrom<HostDetails> for IpDetails {
    type Error = DomainDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Domain(x) => Err(x),
            HostDetails::Ipv4  (x) => Ok(x.into()),
            HostDetails::Ipv6  (x) => Ok(x.into()),
        }
    }
}

impl TryFrom<&str> for IpDetails {
    type Error = InvalidIpHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for IpDetails {
    type Err = InvalidIpHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

//! [`Ipv6Details`].

use std::net::Ipv6Addr;

use crate::prelude::*;

/// Details for an [`Ipv6Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Details {
    /// The parsed [`Ipv6Addr`].
    pub parsed: Ipv6Addr
}

impl Ipv6Details {
    /// Parse an IPv6 host.
    /// # Errors
    /// If the call to [`Ipv6Addr::from_str`] returns an error, returns the error [`InvalidIpv6Host`].
    pub fn parse(s: &str) -> Result<Self, InvalidIpv6Host> {
        Ok(Self {
            parsed: s
                .strip_prefix("[").ok_or(InvalidIpv6Host)?
                .strip_suffix("]").ok_or(InvalidIpv6Host)?
                .parse().map_err(|_| InvalidIpv6Host)?
        })
    }

    /// [`Ipv6Addr::is_loopback`].
    pub fn is_loopback(self) -> bool {
        self.parsed.is_loopback()
    }

    /// [`Ipv6Addr::is_multicast`].
    pub fn is_multicast(self) -> bool {
        self.parsed.is_multicast()
    }

    /// [`Ipv6Addr::is_unicast_link_local`].
    pub fn is_unicast_link_local(self) -> bool {
        self.parsed.is_unicast_link_local()
    }

    /// [`Ipv6Addr::is_unique_local`].
    pub fn is_unique_local(self) -> bool {
        self.parsed.is_unique_local()
    }

    /// [`Ipv6Addr::is_unspecified`].
    pub fn is_unspecified(self) -> bool {
        self.parsed.is_unspecified()
    }
}

impl FromStr for Ipv6Details {
    type Err = InvalidIpv6Host;

    /// [`Self::parse`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Ipv6Details {
    type Error = InvalidIpv6Host;

    /// [`Self::parse`].
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Ipv6Addr> for Ipv6Details {
    fn from(value: Ipv6Addr) -> Self {
        Self {parsed: value}
    }
}

impl TryFrom<IpDetails> for Ipv6Details {
    type Error = Ipv4Details;

    fn try_from(value: IpDetails) -> Result<Self, Self::Error> {
        match value {
            IpDetails::V4(x) => Err(x),
            IpDetails::V6(x) => Ok(x),
        }
    }
}

impl TryFrom<HostDetails> for Ipv6Details {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Ipv6(details) => Ok(details),
            details => Err(details),
        }
    }
}

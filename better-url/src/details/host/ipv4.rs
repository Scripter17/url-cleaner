//! [`Ipv4Details`].

use std::net::Ipv4Addr;

use crate::prelude::*;

/// Details for an [`Ipv4Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Details {
    /// The parsed [`Ipv4Addr`].
    pub parsed: Ipv4Addr
}

impl Ipv4Details {
    /// Parse an IPv4 host.
    /// # Errors
    /// If the call to [`parse_ipv4_host`] returns an error, that error is returned.
    pub fn parse(s: &str) -> Result<Self, InvalidIpv4Host> {
        Ok(Self {
            parsed: parse_ipv4_host(s).ok_or(InvalidIpv4Host)?
        })
    }
    
    /// [`Ipv4Addr::is_broadcast`].
    pub fn is_broadcast(self) -> bool {
        self.parsed.is_broadcast()
    }

    /// [`Ipv4Addr::is_documentation`].
    pub fn is_documentation(self) -> bool {
        self.parsed.is_documentation()
    }

    /// [`Ipv4Addr::is_link_local`].
    pub fn is_link_local(self) -> bool {
        self.parsed.is_link_local()
    }

    /// [`Ipv4Addr::is_loopback`].
    pub fn is_loopback(self) -> bool {
        self.parsed.is_loopback()
    }

    /// [`Ipv4Addr::is_multicast`].
    pub fn is_multicast(self) -> bool {
        self.parsed.is_multicast()
    }

    /// [`Ipv4Addr::is_private`].
    pub fn is_private(self) -> bool {
        self.parsed.is_private()
    }

    /// [`Ipv4Addr::is_unspecified`].
    pub fn is_unspecified(self) -> bool {
        self.parsed.is_unspecified()
    }
}

impl FromStr for Ipv4Details {
    type Err = InvalidIpv4Host;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Ipv4Details {
    type Error = InvalidIpv4Host;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Ipv4Addr> for Ipv4Details {
    fn from(value: Ipv4Addr) -> Self {
        Self {parsed: value}
    }
}

impl TryFrom<IpDetails> for Ipv4Details {
    type Error = Ipv6Details;

    fn try_from(value: IpDetails) -> Result<Self, Self::Error> {
        match value {
            IpDetails::V4(x) => Ok(x),
            IpDetails::V6(x) => Err(x),
        }
    }
}

impl TryFrom<HostDetails> for Ipv4Details {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Ipv4(details) => Ok(details),
            details => Err(details),
        }
    }
}

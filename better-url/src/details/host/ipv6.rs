//! [`Ipv6HostDetails`].

use std::net::Ipv6Addr;

use crate::prelude::*;

/// Details for an [`Ipv6Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ipv6HostDetails {
    /// The parsed [`Ipv6Addr`].
    pub parsed: Ipv6Addr
}

impl Ipv6HostDetails {
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



impl FromStr for Ipv6HostDetails {
    type Err = InvalidIpv6Host;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Ipv6HostDetails {
    type Error = InvalidIpv6Host;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Ipv6Addr> for Ipv6HostDetails {
    fn from(value: Ipv6Addr) -> Self {
        Self {parsed: value}
    }
}



impl TryFrom<HostDetails> for Ipv6HostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Ipv6(details) => Ok (details),
            details                    => Err(details),
        }
    }
}

impl TryFrom<FileHostDetails> for Ipv6HostDetails {
    type Error = FileHostDetails;

    fn try_from(value: FileHostDetails) -> Result<Self, Self::Error> {
        match value {
            FileHostDetails::Ipv6(details) => Ok (details),
            details                        => Err(details),
        }
    }
}

impl TryFrom<SpecialNotFileHostDetails> for Ipv6HostDetails {
    type Error = SpecialNotFileHostDetails;

    fn try_from(value: SpecialNotFileHostDetails) -> Result<Self, Self::Error> {
        match value {
            SpecialNotFileHostDetails::Ipv6(details) => Ok (details),
            details                                  => Err(details),
        }
    }
}

impl TryFrom<NonSpecialHostDetails> for Ipv6HostDetails {
    type Error = NonSpecialHostDetails;

    fn try_from(value: NonSpecialHostDetails) -> Result<Self, Self::Error> {
        match value {
            NonSpecialHostDetails::Ipv6(details) => Ok (details),
            details                              => Err(details),
        }
    }
}

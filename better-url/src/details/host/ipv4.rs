//! [`Ipv4HostDetails`].

use std::net::Ipv4Addr;

use crate::prelude::*;

/// Details for an [`Ipv4Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ipv4HostDetails {
    /// The parsed [`Ipv4Addr`].
    pub parsed: Ipv4Addr,
}

impl Ipv4HostDetails {
    /// Parse an [`Ipv4Host`] literal.
    /// # Errors
    /// If [`Ipv4Addr::from_str`] returns an error, returns the error [`InvalidIpv4Host`].
    pub fn parse(s: &str) -> Result<Self, InvalidIpv4Host> {
        Ok(Self {
            parsed: s.parse().map_err(|_| InvalidIpv4Host)?
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



impl FromStr for Ipv4HostDetails {
    type Err = InvalidIpv4Host;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Ipv4HostDetails {
    type Error = InvalidIpv4Host;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Ipv4Addr> for Ipv4HostDetails {
    fn from(value: Ipv4Addr) -> Self {
        Self {parsed: value}
    }
}



impl TryFrom<HostDetails> for Ipv4HostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Ipv4(details) => Ok (details),
            details                    => Err(details),
        }
    }
}

impl TryFrom<FileHostDetails> for Ipv4HostDetails {
    type Error = FileHostDetails;

    fn try_from(value: FileHostDetails) -> Result<Self, Self::Error> {
        match value {
            FileHostDetails::Ipv4(details) => Ok (details),
            details                        => Err(details),
        }
    }
}

impl TryFrom<SpecialNotFileHostDetails> for Ipv4HostDetails {
    type Error = SpecialNotFileHostDetails;

    fn try_from(value: SpecialNotFileHostDetails) -> Result<Self, Self::Error> {
        match value {
            SpecialNotFileHostDetails::Ipv4(details) => Ok (details),
            details                                  => Err(details),
        }
    }
}

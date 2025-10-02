//! [`Ipv4Details`].

use std::net::Ipv4Addr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Details of an IPv4 host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Ipv4Details {
    /// The [`Ipv4Addr`].
    pub parsed: Ipv4Addr
}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv4Details {
    // /// [`Ipv4Addr::is_benchmarking`].
    // pub fn is_benchmarking(&self) -> bool {
    //     self.parsed.is_benchmarking()
    // }

    /// [`Ipv4Addr::is_broadcast`].
    pub fn is_broadcast(&self) -> bool {
        self.parsed.is_broadcast()
    }

    /// [`Ipv4Addr::is_documentation`].
    pub fn is_documentation(&self) -> bool {
        self.parsed.is_documentation()
    }

    // /// [`Ipv4Addr::is_global`].
    // pub fn is_global(&self) -> bool {
    //     self.parsed.is_global()
    // }

    /// [`Ipv4Addr::is_link_local`].
    pub fn is_link_local(&self) -> bool {
        self.parsed.is_link_local()
    }

    /// [`Ipv4Addr::is_loopback`].
    pub fn is_loopback(&self) -> bool {
        self.parsed.is_loopback()
    }

    /// [`Ipv4Addr::is_multicast`].
    pub fn is_multicast(&self) -> bool {
        self.parsed.is_multicast()
    }

    /// [`Ipv4Addr::is_private`].
    pub fn is_private(&self) -> bool {
        self.parsed.is_private()
    }

    // /// [`Ipv4Addr::is_reserved`].
    // pub fn is_reserved(&self) -> bool {
    //     self.parsed.is_reserved()
    // }

    // /// [`Ipv4Addr::is_shared`].
    // pub fn is_shared(&self) -> bool {
    //     self.parsed.is_shared()
    // }

    /// [`Ipv4Addr::is_unspecified`].
    pub fn is_unspecified(&self) -> bool {
        self.parsed.is_unspecified()
    }
}

impl From<Ipv4Addr> for Ipv4Details {
    fn from(value: Ipv4Addr) -> Self {
        Self {
            parsed: value
        }
    }
}



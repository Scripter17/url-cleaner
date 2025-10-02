//! [`Ipv6Details`].

use std::net::Ipv6Addr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Details of an IPv6 host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Ipv6Details {
    /// The [`Ipv6Addr`].
    pub parsed: Ipv6Addr
}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv6Details {
    // /// [`Ipv6Addr::is_benchmarking`].
    // pub fn is_benchmarking(&self) -> bool {
    //     self.parsed.is_benchmarking()
    // }

    // /// [`Ipv6Addr::is_documentation`].
    // pub fn is_documentation(&self) -> bool {
    //     self.parsed.is_documentation()
    // }

    // /// [`Ipv6Addr::is_global`].
    // pub fn is_global(&self) -> bool {
    //     self.parsed.is_global()
    // }

    // /// [`Ipv6Addr::is_ipv4_mapped`].
    // pub fn is_ipv4_mapped(&self) -> bool {
    //     self.parsed.is_ipv4_mapped()
    // }

    /// [`Ipv6Addr::is_loopback`].
    pub fn is_loopback(&self) -> bool {
        self.parsed.is_loopback()
    }

    /// [`Ipv6Addr::is_multicast`].
    pub fn is_multicast(&self) -> bool {
        self.parsed.is_multicast()
    }

    // /// [`Ipv6Addr::is_unicast`].
    // pub fn is_unicast(&self) -> bool {
    //     self.parsed.is_unicast()
    // }

    // /// [`Ipv6Addr::is_unicast_global`].
    // pub fn is_unicast_global(&self) -> bool {
    //     self.parsed.is_unicast_global()
    // }

    /// [`Ipv6Addr::is_unicast_link_local`].
    pub fn is_unicast_link_local(&self) -> bool {
        self.parsed.is_unicast_link_local()
    }

    /// [`Ipv6Addr::is_unique_local`].
    pub fn is_unique_local(&self) -> bool {
        self.parsed.is_unique_local()
    }

    /// [`Ipv6Addr::is_unspecified`].
    pub fn is_unspecified(&self) -> bool {
        self.parsed.is_unspecified()
    }
}

impl From<Ipv6Addr> for Ipv6Details {
    fn from(value: Ipv6Addr) -> Self {
        Self {
            parsed: value
        }
    }
}


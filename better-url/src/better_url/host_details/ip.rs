//! Details of an IP host.

use std::net::Ipv6Addr;
use std::net::Ipv4Addr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Details of an IPv4 host.
///
/// Currently empty and only exists for completeness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Ipv4Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv4Details {
    /// Construct a [`Self`] from an [`Ipv4Addr`].
    pub fn from_addr(addr: Ipv4Addr) -> Self {
        Self {}
    }
}

/// Details of an IPv6 host.
///
/// Currently empty and only exists for completeness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Ipv6Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv6Details {
    /// Construct a [`Self`] from an [`Ipv6Addr`].
    pub fn from_addr(addr: Ipv6Addr) -> Self {
        Self {}
    }
}


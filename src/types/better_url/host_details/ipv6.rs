//! Details of an IPv6 host.

use std::net::Ipv6Addr;

use serde::{Serialize, Deserialize};

/// Details of an IPv4 address.
///
/// Currently only exists for completeness, but may be extended in the future.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv6Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv6Details {
    /// Creates a [`Self`] from an [`Ipv6Addr`].
    pub fn from_addr(addr: Ipv6Addr) -> Self {
        Self {}
    }
}

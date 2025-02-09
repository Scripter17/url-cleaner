//! Details of an IPv4 host.

use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};

/// Details of an IPv4 address.
///
/// Currently only exists for completeness, but may be extended in the future.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv4Details {
    /// Creates a [`Self`] from an [`Ipv4Addr`].
    pub fn from_addr(addr: Ipv4Addr) -> Self {
        Self {}
    }
}



use std::net::Ipv6Addr;

use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv6Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv6Details {
    pub fn from_addr(addr: Ipv6Addr) -> Self {
        Self {}
    }
}



use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Details {}

#[allow(unused_variables, reason = "API completeness.")]
#[allow(dead_code, reason = "API completeness.")]
impl Ipv4Details {
    pub fn from_addr(addr: Ipv4Addr) -> Self {
        Self {}
    }
}

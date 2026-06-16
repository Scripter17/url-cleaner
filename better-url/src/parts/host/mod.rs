//! Hosts.

mod either;
mod domain;
mod ip;
mod ipv4;
mod ipv6;
mod opaque;
mod empty;

pub use either::*;
pub use domain::*;
pub use ip::*;
pub use ipv4::*;
pub use ipv6::*;
pub use opaque::*;
pub use empty::*;

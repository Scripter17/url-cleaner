//! Hosts.
//!
//! Please note that these types deal exclusively with non-opaque hosts that have already undergone percent-decoding and, if applicable, "domain to ASCII"ing.
//!
//! Please see [URL specification](https://url.spec.whatwg.org/#host-parsing) for details.

mod r#ref;
mod cow;
mod details;

pub use r#ref::*;
pub use cow::*;
pub use details::*;

pub mod domain;
pub mod ip;
pub mod ipv4;
pub mod ipv6;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::r#ref::*;
    pub use super::cow::*;
    pub use super::details::*;

    pub use super::domain::*;
    pub use super::ip::*;
    pub use super::ipv4::*;
    pub use super::ipv6::*;
}

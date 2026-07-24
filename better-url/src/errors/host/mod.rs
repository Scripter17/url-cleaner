//! Host stuff.

use thiserror::Error;

use crate::prelude::*;

mod domain; pub use domain::*;

mod either          ; pub use either          ::*;
mod file            ; pub use file            ::*;
mod special_not_file; pub use special_not_file::*;
mod non_special     ; pub use non_special     ::*;

/// Returned when failing to parse an IPv4 host.
#[derive(Debug, Error)]
#[error("Failed to parse an IPv4 host.")]
pub struct InvalidIpv4Host;

/// Returned when failing to parse an IPv6 host.
#[derive(Debug, Error)]
#[error("Failed to parse an IPv6 host.")]
pub struct InvalidIpv6Host;

/// Returned when failing to parse an IP host.
#[derive(Debug, Error)]
pub enum InvalidIpHost {
    /** [`InvalidIpv4Host`]. **/ #[error(transparent)] V4(#[from] InvalidIpv4Host),
    /** [`InvalidIpv6Host`]. **/ #[error(transparent)] V6(#[from] InvalidIpv6Host),
}

/// Returned when failing to parse an opaque host.
#[derive(Debug, Error)]
#[error("Failed to parse an opaque host.")]
pub struct InvalidOpaqueHost;

/// Returned when failing to parse an empty host.
#[derive(Debug, Error)]
#[error("Failed to parse an empty host.")]
pub struct InvalidEmptyHost;

/// Returned when failing to get a host.
#[derive(Debug, Error)]
#[error("Failed to get a host.")]
pub struct NoHost;

/// Returned when failing to get a domain.
#[derive(Debug, Error)]
#[error("Failed to get a domain.")]
pub struct NoDomain;

/// Returned when attempting to set a URL that can't have a host to have a host.
#[derive(Debug, Error)]
#[error("Attempted to set a URL that can't have a host to have a host.")]
pub struct CantHaveHost;

/// Returned when failing to set a host.
#[derive(Debug, Error)]
pub enum SetHostError {
    /** [`InvalidHost`].    **/ #[error(transparent)] InvalidHost   (#[from] InvalidHost   ),
    /** [`SetDomainError`]. **/ #[error(transparent)] SetDomainError(#[from] SetDomainError),
    /** [`NoDomain`]        **/ #[error(transparent)] NoDomain      (#[from] NoDomain      ),
    /** [`CantBeNone`].     **/ #[error(transparent)] CantBeNone    (#[from] CantBeNone    ),
    /** [`CantBeEmpty`].    **/ #[error(transparent)] CantBeEmpty   (#[from] CantBeEmpty   ),
    /** [`CantHaveHost`].   **/ #[error(transparent)] CantHaveHost  (#[from] CantHaveHost  ),
    /** [`TooLong`].        **/ #[error(transparent)] TooLong       (#[from] TooLong       ),
}

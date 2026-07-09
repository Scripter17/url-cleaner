//! Host stuff.

use thiserror::Error;

use crate::prelude::*;

mod domain;
pub use domain::*;

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

/// Returned when failing to parse a host.
#[derive(Debug, Error)]
#[error("Failed to parse a host.")]
pub struct InvalidHost;

impl From<InvalidDomainHost> for InvalidHost {fn from(_: InvalidDomainHost) -> Self {Self}}
impl From<InvalidIpv4Host  > for InvalidHost {fn from(_: InvalidIpv4Host  ) -> Self {Self}}
impl From<InvalidIpv6Host  > for InvalidHost {fn from(_: InvalidIpv6Host  ) -> Self {Self}}
impl From<InvalidIpHost    > for InvalidHost {fn from(_: InvalidIpHost    ) -> Self {Self}}
impl From<InvalidOpaqueHost> for InvalidHost {fn from(_: InvalidOpaqueHost) -> Self {Self}}
impl From<InvalidEmptyHost > for InvalidHost {fn from(_: InvalidEmptyHost ) -> Self {Self}}

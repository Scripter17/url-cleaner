//! Host stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when failing to parse an empty host.
#[derive(Debug, Error)]
#[error("Failed to parse an empty host.")]
pub struct InvalidEmptyHost;

/// Returned when failing to parse an opaque host.
#[derive(Debug, Error)]
#[error("Failed to parse an opaque host.")]
pub struct InvalidOpaqueHost;

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
    /// [`InvalidIpv4Host`].
    #[error(transparent)] V4(#[from] InvalidIpv4Host),
    /// [`InvalidIpv6Host`].
    #[error(transparent)] V6(#[from] InvalidIpv6Host),
}

/// Returned when failing to get a host.
#[derive(Debug, Error)]
#[error("Failed to get a host.")]
pub struct NoHost;

/// Returned when failing to get a domain.
#[derive(Debug, Error)]
#[error("Failed to get a domain.")]
pub struct NoDomain;

/// Returned when failing to parse a domain host.
#[derive(Debug, Error)]
#[error("Invalid domain host.")]
pub struct InvalidDomainHost;

/// Returned when encountering an invalid domain byte.
#[derive(Debug, Error)]
#[error("Encountered an invalid domain byte.")]
pub struct InvalidDomainByte;

/// Returned when attempting end a domain in a number.
#[derive(Debug, Error)]
#[error("Attemped to end a domain in a number.")]
pub struct CantEndInANumber;

/// Returned when failing to set a host.
#[derive(Debug, Error)]
pub enum SetHostError {
    /// [`InvalidHost`].
    #[error(transparent)]
    InvalidHost(#[from] InvalidHost),
    /// [`CantBeNone`].
    #[error(transparent)]
    CantBeNone(#[from] CantBeNone),
    /// [`CantBeEmpty`].
    #[error(transparent)]
    CantBeEmpty(#[from] CantBeEmpty),
    /// [`TooLong`].
    #[error(transparent)]
    TooLong(#[from] TooLong),
}

impl From<std::convert::Infallible> for SetHostError {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

/// Returned when failing to parse a host.
#[derive(Debug, Error)]
pub enum InvalidHost {
    /// [`InvalidDomainHost`].
    #[error(transparent)]
    InvalidDomainHost(#[from] InvalidDomainHost),
    /// [`InvalidIpv4Host`].
    #[error(transparent)]
    InvalidIpv4Host(#[from] InvalidIpv4Host),
    /// [`InvalidIpv6Host`].
    #[error(transparent)]
    InvalidIpv6Host(#[from] InvalidIpv6Host),
}

/// Returned when failing to set some/all of a domain.
#[derive(Debug, Error)]
pub enum SetDomainError {
    /// [`SetHostError`].
    #[error(transparent)] SetHostError     (#[from] SetHostError    ),
    /// [`NoDomain`].
    #[error(transparent)] NoDomain         (#[from] NoDomain        ),
    /// [`TooLong`].
    #[error(transparent)] TooLong          (#[from] TooLong         ),
    /// [`SegmentNotFound`].
    #[error(transparent)] SegmentNotFound  (#[from] SegmentNotFound ),
    /// [`InsertNotFound`].
    #[error(transparent)] InsertNotFound   (#[from] InsertNotFound  ),
    /// [`CantEndInANumber`].
    #[error(transparent)] CantEndInANumber (#[from] CantEndInANumber),
    /// [`InvalidDomainByte`].
    #[error(transparent)] InvalidDomainByte(#[from] InvalidDomainByte),
    /// [`CantBeNone`].
    #[error(transparent)] CantBeEmpty      (#[from] CantBeEmpty     ),
    /// [`InvalidDomainHost`].
    #[error(transparent)] InvalidDomainHost(#[from] InvalidDomainHost),
}

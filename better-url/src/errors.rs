//! Error types.

use url::ParseError;
use thiserror::Error;

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

/// Returned when trying to parse an invalid [`BetterPosition`].
#[derive(Debug, Error)]
#[error("Invalid BetterPosition.")]
pub struct InvalidBetterPosition;

/// Error for when an insert location isn't found.
#[derive(Debug, Error)]
#[error("Insert not found.")]
pub struct InsertNotFound;

/// Error for when a segment isn't found.
#[derive(Debug, Error)]
#[error("Segment not found.")]
pub struct SegmentNotFound;

/// Error for when a range isn't found.
#[derive(Debug, Error)]
#[error("Range not found.")]
pub struct RangeNotFound;

/// Can't be [`None`].
#[derive(Debug, Error)]
#[error("Can't be None.")]
pub struct CantBeNone;

/// Can't be empty.
#[derive(Debug, Error)]
#[error("Can't be Empty.")]
pub struct CantBeEmpty;

/// Opaque path.
#[derive(Debug, Error)]
#[error("OpaquePath.")]
pub struct OpaquePath;


/// The error [`BetterUrl::set_scheme`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the scheme.")]
pub struct SetSchemeError;

/// The error [`BetterUrl::set_username`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the username.")]
pub struct SetUsernameError;

/// The error [`BetterUrl::set_password`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the password.")]
pub struct SetPasswordError;

/// The error [`BetterUrl::set_port`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the port.")]
pub struct SetPortError;


/// Error for removing segments.
#[derive(Debug, Error)]
pub enum RemoveError {
    /// [`SegmentNotFound`].
    #[error(transparent)]
    SegmentNotFound(#[from] SegmentNotFound),
    /// [`CantBeNone`].
    #[error(transparent)]
    CantBeNone(#[from] CantBeNone),
}

/// Erorrs for setting or removing a segment.
#[derive(Debug, Error)]
pub enum SetOrRemoveError {
    /// [`SegmentNotFound`].
    #[error(transparent)]
    SegmentNotFound(#[from] SegmentNotFound),
    /// [`RemoveError`].
    #[error(transparent)]
    RemoveError(#[from] RemoveError),
}

/// Errors for setting or inserting or removing as segment.
#[derive(Debug, Error)]
pub enum SetOrInsertOrRemoveError {
    /// [`InsertNotFound`].
    #[error(transparent)]
    InsertNotFound(#[from] InsertNotFound),
    /// [`RemoveError`].
    #[error(transparent)]
    RemoveError(#[from] RemoveError),
}



/// Error for removing segments.
#[derive(Debug, Error)]
pub enum RemoveRangeError {
    /// [`RangeNotFound`].
    #[error(transparent)]
    RangeNotFound(#[from] RangeNotFound),
    /// [`CantBeNone`].
    #[error(transparent)]
    CantBeNone(#[from] CantBeNone),
}

/// Erorrs for setting or removing a range.
#[derive(Debug, Error)]
pub enum SetOrRemoveRangeError {
    /// [`RangeNotFound`].
    #[error(transparent)]
    RangeNotFound(#[from] RangeNotFound),
    /// [`RemoveRangeError`].
    #[error(transparent)]
    RemoveRangeError(#[from] RemoveRangeError),
}



/// Errors for setting or inserting or removing as segment.
#[derive(Debug, Error)]
pub enum SetOrInsertOrRemoveMaybeError {
    /// [`InsertNotFound`].
    #[error(transparent)]
    InsertNotFound(#[from] InsertNotFound),
    /// [`SegmentNotFound`].
    #[error(transparent)]
    SegmentNotFound(#[from] SegmentNotFound),
}

/// The error [`BetterUrl::set_ip_host`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the host to an IP.")]
pub struct SetIpHostError;

/// The error [`BetterUrl::set_host`] returns when it fails.
#[derive(Debug, Error)]
pub enum SetHostError {
    /// Returned when a [`ParseError`] is encountered.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// Returned when a [`NoHost`] is encountered.
    #[error(transparent)]
    NoHost(#[from] NoHost)
}

/// The enum of errors that can happen when trying to parse a host.
#[derive(Debug, Error)]
pub enum InvalidHost {
    /// Returned when an [`InvalidDomainHost`] is encountered.
    #[error(transparent)]
    InvalidDomainHost(#[from] InvalidDomainHost),
    /// Returned when an [`InvalidIpv4Host`] is encountered.
    #[error(transparent)]
    InvalidIpv4Host(#[from] InvalidIpv4Host),
    /// Returned when an [`InvalidIpv6Host`] is encountered.
    #[error(transparent)]
    InvalidIpv6Host(#[from] InvalidIpv6Host),
}

/// Returned when parsing a domain host fails.
#[derive(Debug, Error)]
#[error("Invalid domain host.")]
pub struct InvalidDomainHost;

/// Returned when parsing an IP host fails.
#[derive(Debug, Error)]
pub enum InvalidIpHost {
    /// Returned when an [`InvalidIpv4Host`] is encountered.
    #[error(transparent)]
    V4(#[from] InvalidIpv4Host),
    /// Returned when an [`InvalidIpv6Host`] is encountered.
    #[error(transparent)]
    V6(#[from] InvalidIpv6Host),
}

/// Returned when parsing an IPv4 host fails.
#[derive(Debug, Error)]
#[error("Invalid IPv4 host.")]
pub struct InvalidIpv4Host;

/// Returned when parsing an IPv6 host fails.
#[derive(Debug, Error)]
#[error("Invalid IPv6 host.")]
pub struct InvalidIpv6Host;

/// Returned when trying to modify the domain of a URL with no domain.
#[derive(Debug, Error)]
#[error("Attemped to modify the domain of a URL with no domain.")]
pub struct NoDomain;

/// Returned when trying to modify the host of a URL with no host.
#[derive(Debug, Error)]
#[error("Attempted to modify the host of a URL with no host.")]
pub struct NoHost;

/// Returned when trying to insert an invalid domain byte.
#[derive(Debug, Error)]
#[error("There was an invalid domain byte.")]
pub struct InvalidDomainByte;

/// Returned when trying to insert a value that's too long.
#[derive(Debug, Error)]
#[error("The insertion was too long.")]
pub struct TooLong;

/// Returned when trying to set a domain to end in a number.
#[derive(Debug, Error)]
#[error("Attemped to set a domain to end in a number.")]
pub struct CantEndInANumber;

/// The errors domain setting functions can return.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert_eq!(std::mem::size_of::<SetDomainError>(), 1);
/// ```
#[derive(Debug, Error)]
pub enum SetDomainError {
    /// Returned when an [`SetHostError`] is encountered.
    #[error(transparent)] SetHostError     (#[from] SetHostError    ),
    /// Returned when an [`NoDomain`] is encountered.
    #[error(transparent)] NoDomain         (#[from] NoDomain        ),
    /// Returned when an [`TooLong`] is encountered.
    #[error(transparent)] TooLong          (#[from] TooLong         ),
    /// Returned when an [`SegmentNotFound`] is encountered.
    #[error(transparent)] SegmentNotFound  (#[from] SegmentNotFound ),
    /// Returned when an [`InsertNotFound`] is encountered.
    #[error(transparent)] InsertNotFound   (#[from] InsertNotFound  ),
    /// Returned when an [`CantEndInANumber`] is encountered.
    #[error(transparent)] CantEndInANumber (#[from] CantEndInANumber),
    /// Returned when an [`InvalidDomainByte`] is encountered.
    #[error(transparent)] InvalidDomainByte(#[from] InvalidDomainByte),
    /// Returned when an [`CantBeNone`] is encountered.
    #[error(transparent)] CantBeEmpty      (#[from] CantBeEmpty     ),
    /// Returned when an [`InvalidDomainHost`] is encountered.
    #[error(transparent)] InvalidDomainHost(#[from] InvalidDomainHost),
}

/// Returned when trying to parse an invalid [`DomainPart`].
#[derive(Debug, Error)]
#[error("Invalid DomainPart.")]
pub struct InvalidDomainPart;

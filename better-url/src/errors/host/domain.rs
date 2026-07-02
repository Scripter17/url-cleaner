//! Domain stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse invalid Punycode.
#[derive(Debug, Error)]
#[error("Attempted to parse invalid Punycode.")]
pub struct InvalidPunycode;

/// Returned when failing to parse a domain segment.
#[derive(Debug, Error)]
#[error("Invalid domain segment.")]
pub struct InvalidDomainSegment;

/// Returned when failing to parse a list of domain segments.
#[derive(Debug, Error)]
#[error("Invalid domain segments.")]
pub struct InvalidDomainSegments;

/// Returned when failing to parse a domain host.
#[derive(Debug, Error)]
#[error("Invalid domain host.")]
pub struct InvalidDomainHost;

impl From<InvalidDomainSegment > for InvalidDomainSegments {fn from(_: InvalidDomainSegment ) -> Self {Self}}
impl From<InvalidDomainSegment > for InvalidDomainHost     {fn from(_: InvalidDomainSegment ) -> Self {Self}}
impl From<InvalidDomainSegments> for InvalidDomainHost     {fn from(_: InvalidDomainSegments) -> Self {Self}}

/// Returned when attempting end a domain in a number.
#[derive(Debug, Error)]
#[error("Attemped to end a domain in a number.")]
pub struct CantEndInANumber;

/// Returned when attempting to end a non-FQDN [`DomainHost`] in an empty segment.
#[derive(Debug, Error)]
#[error("Attempted to end a non-FQDN DomainHost in an empty segment.")]
pub struct NonFqdnCantEndInEmpty;

/// Returned when failing to set some/all of a domain.
#[derive(Debug, Error)]
pub enum SetDomainError {
    /** [`InvalidDomainSegments`]. **/ #[error(transparent)] InvalidDomainSegments(#[from] InvalidDomainSegments),
    /** [`TooLong`].               **/ #[error(transparent)] TooLong              (#[from] TooLong              ),
    /** [`SegmentNotFound`].       **/ #[error(transparent)] SegmentNotFound      (#[from] SegmentNotFound      ),
    /** [`InsertNotFound`].        **/ #[error(transparent)] InsertNotFound       (#[from] InsertNotFound       ),
    /** [`RangeNotFound`].         **/ #[error(transparent)] RangeNotFound        (#[from] RangeNotFound        ),
    /** [`CantEndInANumber`].      **/ #[error(transparent)] CantEndInANumber     (#[from] CantEndInANumber     ),
    /** [`NonFqdnCantEndInEmpty`]. **/ #[error(transparent)] NonFqdnCantEndInEmpty(#[from] NonFqdnCantEndInEmpty),
    /** [`InvalidDomainSegment`].  **/ #[error(transparent)] InvalidDomainSegment (#[from] InvalidDomainSegment ),
    /** [`CantBeNone`].            **/ #[error(transparent)] CantBeEmpty          (#[from] CantBeEmpty          ),
    /** [`InvalidDomainHost`].     **/ #[error(transparent)] InvalidDomainHost    (#[from] InvalidDomainHost    ),
}

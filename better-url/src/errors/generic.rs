//! Generic errors.

use thiserror::Error;

/// Returend when failing to find an insert.
#[derive(Debug, Error)]
#[error("Failed to find an insert.")]
pub struct InsertNotFound;

/// Returned when failing to find a segment.
#[derive(Debug, Error)]
#[error("Failed to find a segment.")]
pub struct SegmentNotFound;

/// Returned when failing to find a range.
#[derive(Debug, Error)]
#[error("Failed to find a range.")]
pub struct RangeNotFound;

/// Returned when failing to set a value to [`None`].
#[derive(Debug, Error)]
#[error("Failed to set a value to empty.")]
pub struct CantBeNone;

/// Returned when failing to set a value to empty.
#[derive(Debug, Error)]
#[error("Failed to set a value to empty.")]
pub struct CantBeEmpty;

/// Returned when a value is too long.
#[derive(Debug, Error)]
#[error("A value was too long.")]
pub struct TooLong;

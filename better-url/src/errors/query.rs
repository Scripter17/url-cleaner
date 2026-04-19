//! Query stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when failing to set some/all of a path.
#[derive(Debug, Error)]
pub enum SetQueryError {
    /// [`SegmentNotFound`].
    #[error(transparent)]
    SegmentNotFound(#[from] SegmentNotFound),
    /// [`InsertNotFound`].
    #[error(transparent)]
    InsertNotFound(#[from] InsertNotFound),
    /// [`RangeNotFound`].
    #[error(transparent)]
    RangeNotFound(#[from] RangeNotFound),
    /// [`CantBeNone`].
    #[error(transparent)]
    CantBeNone(#[from] CantBeNone),
    /// [`TooLong`].
    #[error(transparent)]
    TooLong(#[from] TooLong),
}

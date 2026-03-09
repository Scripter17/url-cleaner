//! Error types.

use thiserror::Error;

/// Error for when an insert location isn't found.
#[derive(Debug, Error)]
#[error("Insert not found")]
pub struct InsertNotFound;

/// Error for when a segment isn't found.
#[derive(Debug, Error)]
#[error("Segment not found")]
pub struct SegmentNotFound;

/// Error for when a range isn't found.
#[derive(Debug, Error)]
#[error("Range not found")]
pub struct RangeNotFound;

/// Can't be [`None`].
#[derive(Debug, Error)]
#[error("Can't be None")]
pub struct CantBeNone;

/// Opaque path.
#[derive(Debug, Error)]
#[error("OpaquePath.")]
pub struct OpaquePath;



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

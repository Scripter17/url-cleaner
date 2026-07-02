//! [`StringLocationError`].

use crate::prelude::*;

/// The enum of errors [`StringLocation::check`] can return.
#[derive(Debug, Error)]
pub enum StringLocationError {
    /** [`ExplicitError`]. **/ #[error(transparent)] ExplicitError(#[from] ExplicitError          ),
    /** [`TryElseError`].  **/ #[error(transparent)] TryElseError (#[from] Box<TryElseError<Self>>),

    /// Returned when a slice is either not on UTF-8 boundaries or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundaries or out of bounds.")]
    InvalidSlice,
    /// Returned when an index is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound
}

impl From<TryElseError<Self>> for StringLocationError {fn from(value: TryElseError<Self>) -> Self {Box::new(value).into()}}

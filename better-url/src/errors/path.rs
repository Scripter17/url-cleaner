//! Path stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to get a segmented path from an opaque path.
#[derive(Debug, Error)]
#[error("Attempted to get a segmented path from an opaque path.")]
pub struct PathIsOpaque;

/// Returned when failing to set some/all of a path.
#[derive(Debug, Error)]
pub enum SetPathError {
    /** [`InsertNotFound`]. **/ #[error(transparent)] InsertNotFound(#[from] InsertNotFound),
    /** [`RangeNotFound`].  **/ #[error(transparent)] RangeNotFound (#[from] RangeNotFound ),
    /** [`CantBeEmpty`].    **/ #[error(transparent)] CantBeEmpty   (#[from] CantBeEmpty   ),
    /** [`PathIsOpaque`].   **/ #[error(transparent)] PathIsOpaque  (#[from] PathIsOpaque  ),
    /** [`TooLong`].        **/ #[error(transparent)] TooLong       (#[from] TooLong       ),
}

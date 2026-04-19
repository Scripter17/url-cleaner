//! Fragment stuff.

use thiserror::Error;

use crate::prelude::*;

/// The errors that can happen when setting a fragment.
#[derive(Debug, Error)]
pub enum SetFragmentError {
    /// [`TooLong`].
    #[error(transparent)]
    TooLong(#[from] TooLong),
    /// [`SetQueryError`].
    #[error(transparent)]
    SetQueryError(#[from] SetQueryError),
}

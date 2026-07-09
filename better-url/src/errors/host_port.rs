//! [`SetHostPortError`].

use thiserror::Error;

use crate::prelude::*;

/// Returned when failing to set the host + port.
#[derive(Debug, Error)]
pub enum SetHostPortError {
    /** [`SetHostError`]. **/ #[error(transparent)] SetHostError(#[from] SetHostError),
    /** [`SetPortError`]. **/ #[error(transparent)] SetPortError(#[from] SetPortError),
    /** [`CantBeEmpty`].  **/ #[error(transparent)] CantBeEmpty (#[from] CantBeEmpty ),
}

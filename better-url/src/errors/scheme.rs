//! Scheme stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when failing to parse a scheme.
#[derive(Debug, Error)]
#[error("Failed to parse a scheme")]
pub struct InvalidScheme;

/// The errors that can happen when setting a scheme.
#[derive(Debug, Error)]
pub enum SetSchemeError {
    /** [`InvalidScheme`]. **/ #[error(transparent)] InvalidScheme(#[from] InvalidScheme),
    /** [`TooLong`].       **/ #[error(transparent)] TooLong      (#[from] TooLong      ),
}

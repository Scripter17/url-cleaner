//! Port stuff.

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse an invalid port.
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid port.")]
pub struct InvalidPort;

/// The error [`BetterUrl::set_port`] returns when it fails.
#[derive(Debug, Error)]
pub enum SetPortError {
    /// [`NoHost`].
    #[error(transparent)]
    NoHost(#[from] NoHost),
    /// Returned when attempting to set the port of an empty host.
    #[error("Attempted to set the port of an empty host.")]
    EmptyHost,
    /// Returned when attempting to set the port of a file URL.
    #[error("Attempted to set the port of a file URL.")]
    SchemeIsFile,
}

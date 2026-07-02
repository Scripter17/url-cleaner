//! [`InvalidHostPart`].

use crate::prelude::*;

/// Returned when attempting to parse an invalid [`HostPart`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid HostPart.")]
pub struct InvalidHostPart;

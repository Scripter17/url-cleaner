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

    /// Returned when attempting to set a cannot-be-a-base URL's scheme to a non-file spcial scheme.
    #[error("Attemppted to set a cannot-be-a-base URL's scheme to a non-file special scheme.")]
    CannotBeABaseToSpecialNotFile,
    /// Returned when attempting to set a special URL's scheme to a non-special scheme.
    #[error("Attemppted to set a special URL's scheme to a non-special scheme.")]
    SpecialToNonSpecial,
    /// Returned when attempting to set a non-special URL's scheme to a special scheme.
    #[error("Attemppted to set a non-special URL's scheme to a special scheme.")]
    NonSpecialToSpecial,
    /// Returned when attempting to set a URL without an authority's scheme to file.
    #[error("Attemppted to set a URL without an authority's scheme to file.")]
    FileCantHaveAuthority,
    /// Returned when attemtping to set a URL without a host to a special scheme.
    #[error("Attemtpted to set a URL without a host to a special scheme.")]
    NoHostToSpecial,
}

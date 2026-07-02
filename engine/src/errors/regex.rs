//! [`RegexExpansionError`].

use crate::prelude::*;

/// [`RegexExpansion::expand`].
#[derive(Debug, Error)]
pub enum RegexExpansionError {
    /** [`StringSourceError`].       **/ #[error(transparent)] StringSourceError      (#[from] StringSourceError      ),
    /** [`StringModificationError`]. **/ #[error(transparent)] StringModificationError(#[from] StringModificationError),
}

/// Returned when attempting to parse an invalid [`RegexFlags`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid RegexFlags.")]
pub struct InvalidRegexFlags;

//! [`InvalidUrl`] and [`InvalidJoin`].

use thiserror::Error;

use crate::prelude::*;

/// The enum of errors that can occur when trying to parse an invalid URL.
#[derive(Debug, Error)]
pub enum InvalidUrl {
    /** [`InvalidScheme`].             **/ #[error(transparent)] InvalidScheme            (#[from] InvalidScheme            ),
    /** [`InvalidFileHost`].           **/ #[error(transparent)] InvalidFileHost          (#[from] InvalidFileHost          ),
    /** [`InvalidSpecialNotFileHost`]. **/ #[error(transparent)] InvalidSpecialNotFileHost(#[from] InvalidSpecialNotFileHost),
    /** [`InvalidNonSpecialHost`].     **/ #[error(transparent)] InvalidNonSpecialHost    (#[from] InvalidNonSpecialHost    ),
    /** [`InvalidPort`].               **/ #[error(transparent)] InvalidPort              (#[from] InvalidPort              ),
    /** [`TooLong`].                   **/ #[error(transparent)] TooLong                  (#[from] TooLong                  ),

    /// Returned when attempting to parse a URL with no schene.
    #[error("Attempted to parse a URL with no scheme.")]
    MissingScheme,
    /// Returned when attempting to parse a URL with an empty host and a userinfo and/or port.
    #[error("Attempted to parse a URL with an empty host and a userinfo and/or port.")]
    EmptyHostCantHaveUserinfoOrPort,
}

/// [`BetterUrl::join`].
#[derive(Debug, Error)]
pub enum InvalidJoin {
    /** [`TooLong`].          **/ #[error(transparent)] TooLong         (#[from] TooLong          ),
    /** [`InvalidUrl`].       **/ #[error(transparent)] InvalidUrl      (#[from] InvalidUrl       ),
    /** [`InvalidScheme`].    **/ #[error(transparent)] InvalidScheme   (#[from] InvalidScheme    ),
    /** [`CannotBeABase`].    **/ #[error(transparent)] CannotBeABase   (#[from] CannotBeABase    ),
    /// Returned when attempting to join a non-relative URL with something other than just a fragment or a whole URL.
    #[error("Attempted to join a non-relative URL with something other than a fragment or a whole URL.")]
    MissingSchemeNonRelativeUrl,
}

/// Returned when attempting to join more than the fragment on a cannot-be-a-base URL.
#[derive(Debug, Error)]
#[error("Attempted to join more than the fragment on a cannot-be-a-base URL.")]
pub struct CannotBeABase;

//! [`Base64DecodePaddingMode`].

use serde::{Serialize, Deserialize};
use base64::engine::DecodePaddingMode;

use crate::prelude::*;

/// [`serde`] compatible [`DecodePaddingMode`].
///
/// Defaults to [`Self::Indifferent`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Base64DecodePaddingMode {
    /// Don't care whether or not the canonical padding is present.
    ///
    /// The default value.
    #[default]
    Indifferent,
    /// Require that the canonical padding is present.
    RequireCanonical,
    /// Require that the canonical padding isn't present.
    RequireNone
}

impl Base64DecodePaddingMode {
    /// Builds a [`DecodePaddingMode`].
    pub fn build(&self) -> DecodePaddingMode {
        match self {
            Self::Indifferent      => DecodePaddingMode::Indifferent,
            Self::RequireCanonical => DecodePaddingMode::RequireCanonical,
            Self::RequireNone      => DecodePaddingMode::RequireNone
        }
    }
}

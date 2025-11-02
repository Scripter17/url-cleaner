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
    /// Make a [`DecodePaddingMode`].
    pub fn make(self) -> DecodePaddingMode {
        self.into()
    }
}

impl From<Base64DecodePaddingMode> for DecodePaddingMode {
    fn from(value: Base64DecodePaddingMode) -> Self {
        match value {
            Base64DecodePaddingMode::Indifferent      => Self::Indifferent,
            Base64DecodePaddingMode::RequireCanonical => Self::RequireCanonical,
            Base64DecodePaddingMode::RequireNone      => Self::RequireNone
        }
    }
}

impl From<DecodePaddingMode> for Base64DecodePaddingMode {
    fn from(value: DecodePaddingMode) -> Self {
        match value {
            DecodePaddingMode::Indifferent      => Self::Indifferent,
            DecodePaddingMode::RequireCanonical => Self::RequireCanonical,
            DecodePaddingMode::RequireNone      => Self::RequireNone
        }
    }
}

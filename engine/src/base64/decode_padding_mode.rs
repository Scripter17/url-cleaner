//! [`Base64DecodePaddingMode`].

use base64::engine::DecodePaddingMode;

use crate::prelude::*;

/// [`serde`] compatible [`DecodePaddingMode`].
///
/// Defaults to [`Self::Indifferent`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Base64DecodePaddingMode {
    /// [`DecodePaddingMode::Indifferent`].
    ///
    /// The default.
    #[default]
    Indifferent,
    /// [`DecodePaddingMode::RequireCanonical`].
    RequireCanonical,
    /// [`DecodePaddingMode::RequireNone`].
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

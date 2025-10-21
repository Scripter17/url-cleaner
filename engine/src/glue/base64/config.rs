//! [`Base64Config`].

use serde::{Serialize, Deserialize};
use base64::engine::{general_purpose::GeneralPurpose, GeneralPurposeConfig};

use crate::prelude::*;

/// The config for how to encode and decode base64 text.
/// # Examples
/// ```
/// use base64::engine::Engine;
/// use url_cleaner_engine::prelude::*;
///
/// let base64 = Base64Config::default().build();
///
/// let mut encoded = String::new();
/// base64.encode_string("ab~d", &mut encoded);
/// assert_eq!(encoded, "YWJ-ZA=="); // Note that - is used instead of +, because [`Base64Alphabet`] defaults to [`Base64Alphabet::UrlSafe`].
///
/// let mut decoded = Vec::new();
/// base64.decode_vec(&encoded, &mut decoded).unwrap();
/// assert_eq!(decoded, b"ab~d");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Base64Config {
    /// The alphabet to use.
    ///
    /// Defaults to [`Base64Alphabet::UrlSafe`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub alphabet: Base64Alphabet,
    /// If [`true`], encodes the `=` padding at the end.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub encode_padding: bool,
    /// Whether or not to require, refuse, or not care about padding when decoding.
    ///
    /// Defaults to [`Base64DecodePaddingMode::Indifferent`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub decode_padding: Base64DecodePaddingMode,
    /// [`GeneralPurposeConfig::with_decode_allow_trailing_bits`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub decode_allow_trailing_bits: bool
}

impl Base64Config {
    /// Builds the [`GeneralPurpose`] base64 engine.
    pub fn build(&self) -> GeneralPurpose {
        GeneralPurpose::new(
            self.alphabet.get(),
            GeneralPurposeConfig::new()
                .with_decode_padding_mode(self.decode_padding.build())
                .with_encode_padding(self.encode_padding)
                .with_decode_allow_trailing_bits(self.decode_allow_trailing_bits)
        )
    }
}

impl Default for Base64Config {
    fn default() -> Self {
        Self {
            alphabet: Default::default(),
            encode_padding: true,
            decode_padding: Default::default(),
            decode_allow_trailing_bits: false
        }
    }
}

//! Glue for [`base64`].

use serde::{Serialize, Deserialize};
use base64::{engine::{general_purpose::GeneralPurpose, GeneralPurposeConfig, DecodePaddingMode}, alphabet::Alphabet};

use crate::prelude::*;

/// The config for how to encode and decode base64 text.
/// # Examples
/// ```
/// use url_cleaner_engine::glue::prelude::*;
///
/// use base64::engine::Engine;
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
    /// Defaults to [`DecodePaddingMode::Indifferent`].
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

/// The alphabet to use.
///
/// Defaults to [`Self::UrlSafe`], given this is a URL cleaning library.
///
/// See [Wikipedia's Base64 alphabet summary table](https://en.wikipedia.org/wiki/Base64#Variants_summary_table) for details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Base64Alphabet {
    /// The standard alphabet, where characters 62 and 63 are `+` and `/`.
    Standard,
    /// The URL safe alphabet, where characters 62 and 63 are `-` and `_`.
    #[default]
    UrlSafe,
    /// [`base64::alphabet::CRYPT`].
    Crypt,
    /// [`base64::alphabet::BCRYPT`].
    Bcrypt,
    /// [`base64::alphabet::IMAP_MUTF7`].
    IMAPMUTF7,
    /// [`base64::alphabet::BIN_HEX`].
    BinHex
}

impl Base64Alphabet {
    /// Makes a [`base64::alphabet::Alphabet`].
    pub fn get(&self) -> &Alphabet {
        match self {
            Self::Standard  => &base64::alphabet::STANDARD,
            Self::UrlSafe   => &base64::alphabet::URL_SAFE,
            Self::Crypt     => &base64::alphabet::CRYPT,
            Self::Bcrypt    => &base64::alphabet::BCRYPT,
            Self::IMAPMUTF7 => &base64::alphabet::IMAP_MUTF7,
            Self::BinHex    => &base64::alphabet::BIN_HEX
        }
    }
}

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

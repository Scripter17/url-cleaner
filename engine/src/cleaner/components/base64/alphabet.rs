//! [`Base64Alphabet`].

use serde::{Serialize, Deserialize};
use base64::alphabet::Alphabet;

use crate::prelude::*;

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
    ///
    /// The default.
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

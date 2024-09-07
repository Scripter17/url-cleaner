//! Glue to make using [`base64`] easier.
//! 
//! Enabled by the `base64` feature flag.

use std::str::FromStr;

use thiserror::Error;
use serde::{Serialize, Deserialize, Deserializer, de::Visitor};

use crate::util::*;

/// A wrapper around [`base64::engine::DecodePaddingMode`] that has a more complete set of trait implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DecodePaddingMode(pub base64::engine::DecodePaddingMode);

/// Returned when trying to create an invalid [`DecodePaddingMode`].
#[derive(Debug, Error)]
#[error("Invalid decode padding mode.")]
pub struct InvalidDecodePaddingMode;

impl FromStr for DecodePaddingMode {
    type Err = InvalidDecodePaddingMode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Indifferent"      => Self(base64::engine::DecodePaddingMode::Indifferent),
            "RequireCanonical" => Self(base64::engine::DecodePaddingMode::RequireCanonical),
            "RequireNone"      => Self(base64::engine::DecodePaddingMode::RequireNone),
            _ => Err(InvalidDecodePaddingMode)?
        })
    }
}

impl TryFrom<&str> for DecodePaddingMode {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl DecodePaddingMode {
    /// Get the string representation of the decode padding mode.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self(base64::engine::DecodePaddingMode::Indifferent)      => "Indifferent",
            Self(base64::engine::DecodePaddingMode::RequireCanonical) => "RequireCanonical",
            Self(base64::engine::DecodePaddingMode::RequireNone)      => "RequireNone"
        }
    }
}

impl From<DecodePaddingMode> for &'static str {
    fn from(value: DecodePaddingMode) -> Self {
        value.as_str()
    }
}

impl std::fmt::Display for DecodePaddingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl From<DecodePaddingMode> for base64::engine::DecodePaddingMode {
    fn from(value: DecodePaddingMode) -> Self {
        value.0
    }
}

impl From<base64::engine::DecodePaddingMode> for DecodePaddingMode {
    fn from(value: base64::engine::DecodePaddingMode) -> Self {
        Self(value)
    }
}

impl Serialize for DecodePaddingMode {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DecodePaddingMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Visitor for deserialization.
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = DecodePaddingMode;

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                DecodePaddingMode::from_str(s).map_err(E::custom)
            }

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expected a string.")
            }
        }

        deserializer.deserialize_any(V)
    }
}

impl Default for DecodePaddingMode {
    fn default() -> Self {
        Self(base64::engine::DecodePaddingMode::Indifferent)
    }
}

/// The alphabet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Base64Alphabet {
    /// [`base64::alphabet::URL_SAFE`].
    #[default]
    UrlSafe,
    /// [`base64::alphabet::STANDARD`].
    Standard,
    /// A custom alphabet.
    Custom(String)
}

impl std::fmt::Display for Base64Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Base64Alphabet {
    type Err = <Self as TryFrom<String>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<&str> for Base64Alphabet {
    type Error = <Self as TryFrom<String>>::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl TryFrom<String> for Base64Alphabet {
    type Error = base64::alphabet::ParseAlphabetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        base64::alphabet::Alphabet::new(&value)?;
        Ok(Self::Custom(value))
    }
}

/// The enum of errors [`Base64Alphabet::make_real_alphabet`] can return.
#[derive(Debug, Error)]
pub enum MakeRealBase64AlphabetError {
    /// Returned when a [`base64::alphabet::ParseAlphabetError`] is encountered.
    #[error(transparent)] ParseAlphabetError(#[from] base64::alphabet::ParseAlphabetError)
}

impl Base64Alphabet {
    /// Gets the alphabet as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::UrlSafe => base64::alphabet::URL_SAFE.as_str(),
            Self::Standard => base64::alphabet::STANDARD.as_str(),
            Self::Custom(alphabet) => alphabet
        }
    }

    /// Creates a [`base64::alphabet::Alphabet`].
    /// # Errors
    /// If the call to [`base64::alphabet::Alphabet::new`] returns an error, that error is returned.
    pub fn make_real_alphabet(&self) -> Result<base64::alphabet::Alphabet, MakeRealBase64AlphabetError> {
        Ok(match self {
            Self::UrlSafe => base64::alphabet::URL_SAFE.clone(),
            Self::Standard => base64::alphabet::STANDARD.clone(),
            Self::Custom(alphabet) => base64::alphabet::Alphabet::new(alphabet)?
        })
    }
}

impl TryFrom<Base64Alphabet> for base64::alphabet::Alphabet {
    type Error = MakeRealBase64AlphabetError;

    fn try_from(value: Base64Alphabet) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&Base64Alphabet> for base64::alphabet::Alphabet {
    type Error = MakeRealBase64AlphabetError;

    fn try_from(value: &Base64Alphabet) -> Result<Self, Self::Error> {
        value.make_real_alphabet()
    }
}

/// Instructions on how to make a [`base64::engine::general_purpose::GeneralPurpose`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Base64Config {
    /// Defaults to [`Base64Alphabet::UrlSafe`].
    #[serde(default)]
    pub alphabet: Base64Alphabet,
    /// Defaults to [`true`].
    #[serde(default = "get_true")]
    pub encode_padding: bool,
    /// Defaults to [`base64::engine::DecodePaddingMode::Indifferent`].
    #[serde(default)]
    pub decode_padding: DecodePaddingMode
}

impl Default for Base64Config {
    fn default() -> Self {
        Self {
            alphabet: Default::default(),
            encode_padding: true,
            decode_padding: Default::default()
        }
    }
}

/// The enum of errors [`Base64Config::make_engine`] can return.
#[derive(Debug, Error)]
pub enum MakeBase64EngineError {
    /// Returned when a [`MakeRealBase64AlphabetError`] is encountered.
    #[error(transparent)] MakeRealBase64AlphabetError(#[from] MakeRealBase64AlphabetError)
}

impl TryFrom<Base64Config> for base64::engine::general_purpose::GeneralPurpose {
    type Error = MakeBase64EngineError;

    fn try_from(value: Base64Config) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&Base64Config> for base64::engine::general_purpose::GeneralPurpose {
    type Error = MakeBase64EngineError;

    fn try_from(value: &Base64Config) -> Result<Self, Self::Error> {
        value.make_engine()
    }
}

impl Base64Config {
    /// Creates a [`base64::engine::general_purpose::GeneralPurpose`].
    /// # Errors
    /// If the call to [`Base64Alphabet::make_real_alphabet`] returns an error, that error is returned.
    pub fn make_engine(&self) -> Result<base64::engine::general_purpose::GeneralPurpose, MakeBase64EngineError> {
        Ok(base64::engine::general_purpose::GeneralPurpose::new(
            &self.alphabet.make_real_alphabet()?,
            base64::engine::general_purpose::GeneralPurposeConfig::new()
                .with_decode_padding_mode(self.decode_padding.into())
                .with_encode_padding(self.encode_padding)
        ))
    }
}

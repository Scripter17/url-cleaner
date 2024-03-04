use std::io::Error as IoError;

use url::ParseError;
use thiserror::Error;

mod url_part;
pub use url_part::*;
mod config;
pub use config::*;
#[cfg(feature = "string-location"    )] mod string_location;
#[cfg(feature = "string-location"    )] pub use string_location::*;
#[cfg(feature = "string-modification")] mod string_modification;
#[cfg(feature = "string-modification")] pub use string_modification::*;
#[cfg(feature = "string-source"      )] mod string_source;
#[cfg(feature = "string-source"      )] pub use string_source::*;
#[cfg(feature = "string-matcher"     )] mod string_matcher;
#[cfg(feature = "string-matcher"     )] pub use string_matcher::*;
#[cfg(feature = "bool-source"        )] mod bool_source;
#[cfg(feature = "bool-source"        )] pub use bool_source::*;
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] mod advanced_requests;
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] pub use advanced_requests::*;

/// An enum that, if I've done my job properly, contains any possible error that can happen when cleaning a URL.
/// Except for if a [`crate::rules::Mapper::ExpandShortLink`] response can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the config.
    #[error(transparent)]
    GetConfigError(#[from] config::GetConfigError),
    /// There was an error executing a rule.
    #[error(transparent)]
    RuleError(#[from] crate::rules::RuleError),
    /// There was an error parsing the URL.
    #[error(transparent)]
    UrlParseError(#[from] ParseError),
    /// IO error.
    #[error(transparent)]
    IoError(#[from] IoError)
}

/// Miscellaneous errors that can happen when handling strings.
#[derive(Debug, Error)]
pub enum StringError {
    /// The requested slice was either not on a UTF-8 boundary or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundary or out of bounds.")]
    InvalidSlice,
    /// The requested index was either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// The requested segment was not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// The provided string did not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// The provided string did not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
}

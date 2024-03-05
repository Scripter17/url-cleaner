use std::io;

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
    /// Returned when a [`GetConfigError`] os encountered.
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    /// Returned when a [`crate::rules::RuleError`] is encountered.
    #[error(transparent)]
    RuleError(#[from] crate::rules::RuleError),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when an [`io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
}

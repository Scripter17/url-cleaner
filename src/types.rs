//! Various types to make URL Cleaner far more powerful.

use thiserror::Error;

mod url_part;
pub use url_part::*;
mod config;
pub use config::*;
mod rules;
pub use rules::*;
#[cfg(feature = "string-location"    )] mod string_location;
#[cfg(feature = "string-location"    )] pub use string_location::*;
#[cfg(feature = "string-modification")] mod string_modification;
#[cfg(feature = "string-modification")] pub use string_modification::*;
#[cfg(feature = "string-source"      )] mod string_source;
#[cfg(feature = "string-source"      )] pub use string_source::*;
#[cfg(feature = "string-matcher"     )] mod string_matcher;
#[cfg(feature = "string-matcher"     )] pub use string_matcher::*;
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] mod advanced_requests;
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] pub use advanced_requests::*;

/// An enum that transitively contains any possible error that can happen when cleaning a URL.
#[derive(Debug, Error)]
pub enum CleaningError {
    /// Returned when a [`GetConfigError`] os encountered.
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)]
    RuleError(#[from] RuleError),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
}

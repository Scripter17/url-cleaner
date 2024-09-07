//! Various types to make URL Cleaner far more powerful.

use std::collections::HashMap;

use thiserror::Error;

mod url_part;
pub use url_part::*;
mod config;
pub use config::*;
mod tests;
pub use tests::*;
mod rules;
pub use rules::*;
mod string_location;
pub use string_location::*;
mod string_modification;
pub use string_modification::*;
mod string_source;
pub use string_source::*;
mod string_matcher;
pub use string_matcher::*;
mod char_matcher;
pub use char_matcher::*;
mod jobs;
pub use jobs::*;
mod stop_loop_condition;
pub use stop_loop_condition::*;

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
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a [`crate::glue::MakeCacheHandlerError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    MakeCacheHandlerError(#[from] crate::glue::MakeCacheHandlerError)
}

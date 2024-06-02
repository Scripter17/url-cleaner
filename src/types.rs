//! Various types to make URL Cleaner far more powerful.

use std::collections::HashMap;
use std::sync::OnceLock;

use thiserror::Error;
use url::Url;

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
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] mod advanced_requests;
#[cfg(all(feature = "advanced-requests", not(target_family = "wasm")))] pub use advanced_requests::*;

/// The current state of the job.
#[derive(Debug, PartialEq, Eq)]
pub struct JobState<'a> {
    /// The URL being modified.
    pub url: &'a mut Url,
    /// The flags, variables, etc. defined by the job initiator.
    pub params: &'a Params,
    /// The string vars created and managed by the config.
    pub string_vars: HashMap<String, String>
}

/// Annoyingly I can't make a `Params::const_default` because `reqwest::header::HeaderMap`'s implementation details.
static DEFAULT_PARAMS: OnceLock<Params> = OnceLock::new();

impl<'a> JobState<'a> {
    pub fn new(url: &'a mut Url) -> JobState<'a> {
        JobState {
            url,
            params: DEFAULT_PARAMS.get_or_init(Params::default),
            string_vars: Default::default()
        }
    }

    pub fn new_with_params(url: &'a mut Url, params: &'a Params) -> JobState<'a> {
        JobState {
            url,
            params,
            string_vars: Default::default()
        }
    }
}

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

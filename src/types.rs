use url::ParseError;
use thiserror::Error;
use std::io::Error as IoError;

mod url_part;
pub use url_part::*;
mod dcr;
pub use dcr::*;
mod string_location;
pub use string_location::*;
mod string_modification;
pub use string_modification::*;

/// An enum that, if I've done my job properly, contains any possible error that can happen when cleaning a URL.
/// Except for if a [`crate::rules::mappers::Mapper::ExpandShortLink`] response can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the config.
    #[error(transparent)]
    GetConfigError(#[from] crate::config::GetConfigError),
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

/// The enum of all possible errors that can happen when using `StringModification`.
#[derive(Debug, Clone, Error)]
pub enum StringError {
    /// The requested slice either was not on a UTF-8 boundary or was out of bounds.
    #[error("The requested slice either was not on a UTF-8 boundary or was out of bounds.")]
    InvalidSlice,
    /// The provided string did not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// The provided string did not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound
}

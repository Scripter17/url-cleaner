use url::ParseError;
use thiserror::Error;
use std::io::Error as IoError;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

mod url_part;
pub use url_part::*;
mod dcr;
pub use dcr::*;
mod string_location;
pub use string_location::*;
mod string_modification;
pub use string_modification::*;

/// Configuration options to choose the behaviour of a few select [`crate::rules::conditions::Condition`]s and [`crate::rules::mappers::Mapper`]s.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleConfig {
    /// Chooses how [`crate::rules::conditions::Condition::DomainCondition`] works.
    #[serde(default)]
    pub dcr: DomainConditionRule,
    /// Works with [`crate::rules::conditions::Condition::RuleVariableIs'`].
    #[serde(default)]
    pub variables: HashMap<String, String>
}

/// Parses CLI variable strings.
/// # Examples
/// ```
/// # use std::collections::HashMap;
/// # use url_cleaner::types::parse_variables;
/// assert_eq!(parse_variables("a=2;b=3"), HashMap::from([("a".to_string(), "2".to_string()), ("b".to_string(), "3".to_string())]));
/// ````
pub fn parse_variables(s: &str) -> HashMap<String, String> {
    s.split(';')
        .flat_map(|kv| kv.split_once('='))
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}

/// An enum that, if I've done my job properly, contains any possible error that can happen when cleaning a URL.
/// Except for if a [`crate::rules::mappers::Mapper::ExpandShortLink`] response can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the rules.
    #[error(transparent)]
    GetRulesError(#[from] crate::rules::GetRulesError),
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

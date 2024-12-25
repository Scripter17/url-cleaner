//! Things that [`Jobs`] uses to make [`Job`]s.

use std::error::Error;
use std::str::FromStr;
use std::io;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// Defines how each [`Job`] from a [`Jobs`] should be constructed.
/// 
/// When [`Deserialize`]ing from a string or using [`FromStr::from_str`]/[`TryFrom<&str>`], if the string starts with `{`, it's deserialized as JSON.
/// 
/// For example, `{"url": "https://example.com"}` and `"{\"url\": \"https://example.com\"}"` deserialize to the same value.
/// 
/// This allows for more flexible APIs where having to input JSON objects is infeasable, like in command line interfaces.
/// ```
/// # use std::str::FromStr;
/// # use url_cleaner::types::*;
/// assert_eq!(
///     serde_json::from_str::<JobConfig>("{\"url\": \"https://example.com\"}").unwrap(),
///     serde_json::from_str::<JobConfig>("\"{\\\"url\\\": \\\"https://example.com\\\"}\"").unwrap()
/// );
/// assert_eq!(
///     JobConfig::from_str("https://example.com").unwrap(),
///     JobConfig::from_str("{\"url\": \"https://example.com\"}").unwrap()
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct JobConfig {
    /// The URL to modify.
    pub url: Url,
    /// The context surrounding the URL.
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext
}

impl From<Url> for JobConfig {
    fn from(value: Url) -> Self {
        Self {
            url: value,
            context: Default::default()
        }
    }
}

/// The enum of errors [`JobConfig::from_str`] and [`<JobConfig as TryFrom<&str>>::try_from`] can return.
/// 
/// Additionally has [`Self::IoError`] and [`Self::Other`] to accomodate [`Jobs::job_configs_source`] iterators.
#[derive(Debug, Error)]
pub enum MakeJobConfigError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when an [`io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// Generic error wrapper.
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send>)
}

impl FromStr for JobConfig {
    type Err = MakeJobConfigError;

    /// If `s` starts with `{`, deserializes it as a JSON object to allow both a more complete CLI and a more versatile API.
    /// 
    /// Otherwise uses [`Url::parse`] and [`Into::into`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('{') {
            serde_json::from_str(s)?
        } else {
            Url::parse(s)?.into()
        })
    }
}

impl TryFrom<&str> for JobConfig {
    type Error = <Self as FromStr>::Err;

    /// [`Self::from_str`].
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

string_or_struct_magic!(JobConfig);

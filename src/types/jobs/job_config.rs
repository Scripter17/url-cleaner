//! Things that [`Jobs`] uses to make [`Job`]s.

use std::error::Error;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;

use crate::types::*;
use crate::util::*;

/// Defines how each [`Job`] from a [`Jobs`] should be constructed.
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

impl FromStr for JobConfig {
    type Err = <Url as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for JobConfig {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

string_or_struct_magic!(JobConfig);

/// The enum of errors that can happen when [`Jobs::iter`] tries to get a URL.
#[derive(Debug, Error)]
pub enum JobConfigSourceError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Catch-all for user-defined URL sources with errors not listed here.
    #[allow(dead_code, reason = "Public API for use in other people's code.")]
    #[error(transparent)]
    Other(#[from] Box<dyn Error>)
}

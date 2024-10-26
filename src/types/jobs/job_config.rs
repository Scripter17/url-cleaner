//! Things that [`Jobs`] uses to make [`Job`]s.

use std::error::Error;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

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

/// The enum of errors [`JobConfig::from_str`] and [`<JobConfig as TryFrom<&str>>::try_from`] can return.
#[derive(Debug, Error)]
pub enum MakeJobConfigError {
    /// Returned when a [`url::ParseError`] is encoutered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
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

/// The enum of errors that can happen when [`Jobs::iter`] tries to get a URL.
#[derive(Debug, Error)]
pub enum JobConfigSourceError {
    /// Returned when a [`MakeJobConfigError`] is encountered.
    #[error(transparent)]
    MakeJobConfigError(#[from] MakeJobConfigError),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Catch-all for user-defined URL sources with errors not listed here.
    #[allow(dead_code, reason = "Public API for use in other people's code.")]
    #[error(transparent)]
    Other(#[from] Box<dyn Error>)
}

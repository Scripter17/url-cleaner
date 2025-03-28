//! The configuration on how to make a [`Task`].

use std::error::Error;
use std::str::FromStr;
use std::io;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// Configuration for a specific [`Task`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct TaskConfig {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context for this [`Task`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

impl From<Url> for TaskConfig {
    fn from(value: Url) -> Self {
        Self {
            url: value.into(),
            context: Default::default()
        }
    }
}

impl From<BetterUrl> for TaskConfig {
    fn from(value: BetterUrl) -> Self {
        Self {
            url: value,
            context: Default::default()
        }
    }
}

/// The enum of errros that can happen when making a [`TaskConfig`].
#[derive(Debug, Error)]
pub enum MakeTaskConfigError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when an [`io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// Any other errror that your [`TaskConfig`] source can return.
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send>)
}

impl FromStr for TaskConfig {
    type Err = MakeTaskConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with(['{', '"']) {
            serde_json::from_str(s)?
        } else {
            Url::parse(s)?.into()
        })
    }
}

impl TryFrom<&str> for TaskConfig {
    type Error = <Self as FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

string_or_struct_magic!(TaskConfig);

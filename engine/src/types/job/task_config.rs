//! The configuration on how to make a [`Task`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// Configuration for a specific [`Task`].
///
/// In general, you should instead make [`LazyTaskConfig`]s, provide them to a [`Job`], get back [`LazyTask`]s, then [`LazyTask::make`] and [`Task::do`] them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct TaskConfig {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context for this [`Task`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

string_or_struct_magic!(TaskConfig);

impl TryFrom<LazyTaskConfig> for TaskConfig {
    type Error = MakeTaskConfigError;

    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `value` is [`LazyTaskConfig::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `value` is [`LazyTaskConfig::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    fn try_from(value: LazyTaskConfig) -> Result<TaskConfig, Self::Error> {
        Ok(match value {
            LazyTaskConfig::Made(task_config) => task_config,
            LazyTaskConfig::Url(url)          => url.into(),
            LazyTaskConfig::BetterUrl(url)    => url.into(),
            LazyTaskConfig::String(string)    => (&*string).try_into()?,
            LazyTaskConfig::Bytes(bytes)      => (&*bytes).try_into()?,
            LazyTaskConfig::JsonValue(value)  => value.try_into()?
        })
    }
}

/// The enum of errors that can happen when making a [`TaskConfig`].
#[derive(Debug, Error)]
pub enum MakeTaskConfigError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),    
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
}

impl From<Url> for TaskConfig {
    /// Defaults [`Self::context`].
    fn from(value: Url) -> Self {
        Self {
            url: value.into(),
            context: Default::default()
        }
    }
}

impl From<BetterUrl> for TaskConfig {
    /// Defaults [`Self::context`].
    fn from(value: BetterUrl) -> Self {
        Self {
            url: value,
            context: Default::default()
        }
    }
}

impl FromStr for TaskConfig {
    type Err = MakeTaskConfigError;

    /// If `s` starts with `{` or `"`, deserializes it as JSON using [`serde_json::from_str`].
    ///
    /// Otherwise uses [`Url::parse`].
    /// # Errors
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    ///
    /// If the call to [`Url::parse`] returns an error, that error is returned.
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

    /// If `s` starts with `{` or `"`, deserializes it as JSON using [`serde_json::from_str`].
    ///
    /// Otherwise uses [`Url::parse`].
    /// # Errors
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    ///
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&[u8]> for TaskConfig {
    type Error = MakeTaskConfigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(value)?.try_into()
    }
}

impl TryFrom<serde_json::Value> for TaskConfig {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

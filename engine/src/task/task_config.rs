//! [`TaskConfig`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;
use thiserror::Error;

use crate::prelude::*;

/// Configuration for a [`Task`].
///
/// Mainly used in 2 ways:
///
/// 1. Given to [`JobConfig::make_task`] to make a [`Task`].
///
/// 2. Turned into a [`LazyTaskConfig`] and used as such.
///
/// In general, APIs that could take [`TaskConfig`]s should instead take [`LazyTaskConfig`]s when possible.
///
/// See the documentation for [`LazyTaskConfig`] for the benefits.
/// # Tests
/// ```
/// use std::str::FromStr;
/// use serde_json::from_str;
/// use url_cleaner_engine::prelude::*;
///
/// TaskConfig::from_str(r#"https://example.com"#).unwrap();
/// TaskConfig::from_str(r#""https://example.com""#).unwrap();
/// TaskConfig::from_str(r#"{"url":"https://example.com"}"#).unwrap();
///
/// serde_json::from_str::<TaskConfig>(r#""https://example.com""#).unwrap();
/// serde_json::from_str::<TaskConfig>(r#"{"url":"https://example.com"}"#).unwrap();
///
/// serde_json::from_slice::<TaskConfig>(br#""https://example.com""#).unwrap();
/// serde_json::from_slice::<TaskConfig>(br#"{"url":"https://example.com"}"#).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct TaskConfig {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The context for this [`Task`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: TaskContext
}

string_or_struct_magic!(TaskConfig);

impl TryFrom<LazyTaskConfig<'_>> for TaskConfig {
    type Error = MakeTaskConfigError;

    /// [`LazyTaskConfig::make`].
    /// # Errors
    #[doc = edoc!(callerr(LazyTaskConfig::make))]
    fn try_from(value: LazyTaskConfig) -> Result<TaskConfig, Self::Error> {
        value.make()
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
    #[doc = edoc!(callerr(serde_json::from_str), callerr(Url::parse))]
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

    /// [`Self::from_str`].
    /// # Errors
    #[doc = edoc!(callerr(Self::from_str))]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&[u8]> for TaskConfig {
    type Error = MakeTaskConfigError;

    /// If `value` starts with `{` or `"`, deserializes it as JSON using [`serde_json::from_slice`].
    ///
    /// Otherwise uses [`str::from_utf8`] then [`Url::parse`].
    /// # Errors
    #[doc = edoc!(callerr(serde_json::from_slice), callerr(str::from_utf8), callerr(Url::parse))]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(if value.starts_with(b"{") || value.starts_with(b"\"") {
            serde_json::from_slice(value)?
        } else {
            Url::parse(str::from_utf8(value)?)?.into()
        })
    }
}

impl TryFrom<serde_json::Value> for TaskConfig {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

//! The configuration on how to make a [`Task`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;

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

string_or_struct_magic!(TaskConfig);

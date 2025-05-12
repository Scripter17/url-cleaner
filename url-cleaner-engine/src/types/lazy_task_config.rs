//! Unified API for things you'd want to lazily turn into a [`TaskConfig`].

use std::str::FromStr;

use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};
use thiserror::Error;

use crate::types::*;

/// Various common things you'd want to lazily turn into a [`TaskConfig`].
///
/// Allows actually using [`Job`] without it being a bottleneck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskConfigSource {
    /// An already made [`TaskConfig`].
    Made(TaskConfig),
    /// A [`String`] for use in [`TaskConfig::from_str`].
    String(String),
    /// A [`serde_json::Value`] for use in [`serde_json::from_value`].
    JsonValue(serde_json::Value)
}

impl TaskConfigSource {
    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `self` is [`Self::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `self` is [`Self::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    pub fn make(self) -> Result<TaskConfig, MakeTaskConfigError> {
        self.try_into()
    }
}

impl Serialize for TaskConfigSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Made(task_config) => task_config.serialize(serializer),
            Self::String(string)    => string.serialize(serializer),
            Self::JsonValue(value)  => value.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for TaskConfigSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::JsonValue(serde_json::Value::deserialize(deserializer)?))
    }
}

impl FromStr for TaskConfigSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string().into())
    }
}

impl From<&str> for TaskConfigSource {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<TaskConfig> for TaskConfigSource {
    fn from(value: TaskConfig) -> Self {
        Self::Made(value)
    }
}

impl From<String> for TaskConfigSource {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<serde_json::Value> for TaskConfigSource {
    fn from(value: serde_json::Value) -> Self {
        Self::JsonValue(value)
    }
}

impl TryFrom<TaskConfigSource> for TaskConfig {
    type Error = MakeTaskConfigError;

    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `self` is [`Self::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `self` is [`Self::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    fn try_from(value: TaskConfigSource) -> Result<TaskConfig, Self::Error> {
        Ok(match value {
            TaskConfigSource::Made(task_config) => task_config,
            TaskConfigSource::String(string)    => TaskConfig::from_str(&string)?,
            TaskConfigSource::JsonValue(value)  => serde_json::from_value(value)?
        })
    }
}

/// The enum of errors that can happen when making a [`TaskConfig`].
#[derive(Debug, Error)]
pub enum MakeTaskConfigError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error)
}

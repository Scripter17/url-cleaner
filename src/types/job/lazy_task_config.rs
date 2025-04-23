//! Allows lazily making a [`TaskConfig`].

use std::str::FromStr;

use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};
use thiserror::Error;

use crate::types::*;

/// Allows lazily making a [`TaskConfig`].
///
/// Given to [`Job`]s to allow doing the expensive conversion into [`TaskConfig`]s in parallel worker threads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyTaskConfig {
    /// An already made [`TaskConfig`].
    Made(TaskConfig),
    /// A [`String`] for use in [`TaskConfig::from_str`].
    ///
    /// Please note that if the string starts with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_str`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use url_cleaner::types::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(LazyTaskConfig::from_str(          "https://example.com"   ).unwrap().make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from_str(        "\"https://example.com\"" ).unwrap().make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from_str(r#"{"url":"https://example.com"}"#).unwrap().make().unwrap(), target);
    /// ```
    String(String),
    /// A [`serde_json::Value`] for use in [`serde_json::from_value`].
    ///
    /// Please note that string literals (`"https://example.com"`) are valid.
    ///
    /// You don't need to manually special case those into [`Self::String`].
    /// # Examples
    /// ```
    /// use serde_json::json;
    /// use url_cleaner::types::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(LazyTaskConfig::from(json!(        "https://example.com" )).make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from(json!({"url": "https://example.com"})).make().unwrap(), target);
    /// ```
    JsonValue(serde_json::Value)
}

impl LazyTaskConfig {
    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `self` is [`Self::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `self` is [`Self::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    pub fn make(self) -> Result<TaskConfig, MakeTaskConfigError> {
        self.try_into()
    }
}

impl Serialize for LazyTaskConfig {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Made(task_config) => task_config.serialize(serializer),
            Self::String(string)    => string.serialize(serializer),
            Self::JsonValue(value)  => value.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for LazyTaskConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::JsonValue(serde_json::Value::deserialize(deserializer)?))
    }
}

impl FromStr for LazyTaskConfig {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string().into())
    }
}

impl From<&str> for LazyTaskConfig {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<TaskConfig> for LazyTaskConfig {
    fn from(value: TaskConfig) -> Self {
        Self::Made(value)
    }
}

impl From<String> for LazyTaskConfig {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<serde_json::Value> for LazyTaskConfig {
    fn from(value: serde_json::Value) -> Self {
        Self::JsonValue(value)
    }
}

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
            LazyTaskConfig::String(string)    => TaskConfig::from_str(&string)?,
            LazyTaskConfig::JsonValue(value)  => serde_json::from_value(value)?
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


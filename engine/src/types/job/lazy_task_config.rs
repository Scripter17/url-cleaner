//! Allows lazily making a [`TaskConfig`].

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::str::FromStr;
use std::borrow::Cow;

use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};
use url::Url;

use crate::types::*;

/// Allows lazily making a [`TaskConfig`].
///
/// Given to [`Job`]s to allow doing the expensive conversion into [`TaskConfig`]s in parallel worker threads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyTaskConfig<'a> {
    /// An already made [`TaskConfig`].
    Made(TaskConfig),
    /// A [`Url`].
    Url(Url),
    /// A [`BetterUrl`].
    BetterUrl(BetterUrl),
    /// A [`str`] for use in [`TaskConfig::from_str`].
    ///
    /// Please note that if the string starts with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_str`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    Str(&'a str),
    /// A [`String`] for use in [`TaskConfig::from_str`].
    ///
    /// Please note that if the string starts with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_str`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use url_cleaner_engine::types::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(LazyTaskConfig::from(          "https://example.com"   ).make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from(       r#""https://example.com""# ).make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from(r#"{"url":"https://example.com"}"#).make().unwrap(), target);
    /// ```
    String(String),
    /// A UTF-8 byte sequence that is turned into a [`str`] and passed to [`TaskConfig::from_str`].
    ByteSlice(&'a [u8]),
    /// A UTF-8 byte sequence that is turned into a [`String`] and passed to [`TaskConfig::from_str`].
    Bytes(Vec<u8>),
    /// A [`serde_json::Value`] for use in [`serde_json::from_value`].
    ///
    /// Please note that [`serde_json::Value::String`]s are valid.
    ///
    /// You don't need to manually special case those into [`Self::String`].
    /// # Examples
    /// ```
    /// use serde_json::json;
    /// use url_cleaner_engine::types::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(LazyTaskConfig::from(json!(        "https://example.com" )).make().unwrap(), target);
    /// assert_eq!(LazyTaskConfig::from(json!({"url": "https://example.com"})).make().unwrap(), target);
    /// ```
    JsonValue(serde_json::Value)
}

impl LazyTaskConfig<'_> {
    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `self` is [`Self::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `self` is [`Self::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    pub fn make(self) -> Result<TaskConfig, MakeTaskConfigError> {
        self.try_into()
    }
}

impl Serialize for LazyTaskConfig<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Made(task_config) => task_config.serialize(serializer),
            Self::Url(url)          => url.serialize(serializer),
            Self::BetterUrl(url)    => url.serialize(serializer),
            Self::Str(string)       => string.serialize(serializer),
            Self::String(string)    => string.serialize(serializer),
            Self::Bytes(bytes)      => bytes.serialize(serializer),
            Self::ByteSlice(bytes)  => bytes.serialize(serializer),
            Self::JsonValue(value)  => value.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for LazyTaskConfig<'_> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::JsonValue(serde_json::Value::deserialize(deserializer)?))
    }
}

impl From<TaskConfig> for LazyTaskConfig<'_> {
    fn from(value: TaskConfig) -> Self {
        Self::Made(value)
    }
}

impl From<Url> for LazyTaskConfig<'_> {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<BetterUrl> for LazyTaskConfig<'_> {
    fn from(value: BetterUrl) -> Self {
        Self::BetterUrl(value)
    }
}

impl<'a> From<&'a str> for LazyTaskConfig<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<String> for LazyTaskConfig<'_> {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<'a> From<Cow<'a, str>> for LazyTaskConfig<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(string) => Self::Str(string),
            Cow::Owned(string) => Self::String(string)
        }
    }
}

impl<'a> From<&'a [u8]> for LazyTaskConfig<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::ByteSlice(value)
    }
}

impl From<Vec<u8>> for LazyTaskConfig<'_> {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl<'a> From<Cow<'a, [u8]>> for LazyTaskConfig<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        match value {
            Cow::Borrowed(bytes) => Self::ByteSlice(bytes),
            Cow::Owned(bytes) => Self::Bytes(bytes)
        }
    }
}

impl From<serde_json::Value> for LazyTaskConfig<'_> {
    fn from(value: serde_json::Value) -> Self {
        Self::JsonValue(value)
    }
}

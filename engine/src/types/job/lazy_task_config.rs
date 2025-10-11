//! Allows lazily making a [`TaskConfig`].

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::str::FromStr;
use std::borrow::Cow;

use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor, MapAccess}};
use url::Url;

use crate::types::*;

/// Allows lazily making a [`TaskConfig`].
///
/// Given to [`Job`]s to allow doing the expensive conversion into [`TaskConfig`]s in parallel worker threads.
///
/// When deserializing a string value (JSON `"abc"`/Rust `"\"abc\""`), borrows the input when possible.
///
/// For example, the JSON `"abc"` will become a [`Self::Str`], but a JSON `"\u0041BC"` will become a [`Self::String`] containing `ABC`.
///
/// The same is true for [`Self::ByteSlice`] and [`Self::Bytes`].
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
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(matches!(serde_json::from_str(r#""https://example.com""#), Ok(LazyTaskConfig::Str(_))))
    /// ```
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
    ///
    /// // Guard patterns when?
    /// // https://github.com/rust-lang/rust/issues/129967
    /// match serde_json::from_str(r#""https://example.com/\u0041""#) {
    ///   Ok(LazyTaskConfig::String(x)) if x == "https://example.com/A" => {},
    ///   x => panic!("{x:?}")
    /// }
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

impl<'a> LazyTaskConfig<'a> {
    /// Makes the [`TaskConfig`].
    /// # Errors
    /// If `self` is [`Self::String`] and the call to [`TaskConfig::from_str`] returns an error, that error is returned.
    ///
    /// If `self` is [`Self::JsonValue`] and the call to [`serde_json::from_value`] returns an error, that error is returned.
    pub fn make(self) -> Result<TaskConfig, MakeTaskConfigError> {
        self.try_into()
    }

    /// Convert [`Self::Str`] to [`Self::String`] and [`Self::ByteSlice`] to [`Self::Bytes`].
    pub fn into_owned(self) -> LazyTaskConfig<'static> {
        match self {
            Self::Made     (x) => LazyTaskConfig::Made     (x),
            Self::Url      (x) => LazyTaskConfig::Url      (x),
            Self::BetterUrl(x) => LazyTaskConfig::BetterUrl(x),
            Self::Str      (x) => LazyTaskConfig::String   (x.into()),
            Self::String   (x) => LazyTaskConfig::String   (x),
            Self::ByteSlice(x) => LazyTaskConfig::Bytes    (x.into()),
            Self::Bytes    (x) => LazyTaskConfig::Bytes    (x),
            Self::JsonValue(x) => LazyTaskConfig::JsonValue(x)
        }
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

/// [`Visitor`] for [`LazyTaskConfig`].
struct LazyTaskConfigVisitor;

impl<'de> Visitor<'de> for LazyTaskConfigVisitor {
    type Value = LazyTaskConfig<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a string or struct")
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(Self::Value::Str(v))
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Self::Value::String(v.into()))
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Self::Value::String(v))
    }

    fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(Self::Value::ByteSlice(v))
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Self::Value::Bytes(v.into()))
    }

    fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        Ok(Self::Value::Bytes(v))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut ret = serde_json::Map::new();

        while let Some((k, v)) = map.next_entry()? {
            ret.insert(k, v);
        }

        Ok(Self::Value::JsonValue(ret.into()))
    }
}

impl<'de> Deserialize<'de> for LazyTaskConfig<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(LazyTaskConfigVisitor)
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

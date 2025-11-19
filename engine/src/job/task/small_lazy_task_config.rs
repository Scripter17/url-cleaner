//! [`SmallLazyTaskConfig`].

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::str::FromStr;
use std::borrow::Cow;

use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor, MapAccess}};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use url::Url;

use crate::prelude::*;

/// [`LazyTaskConfig`] but made smaller by sacrificing some variants.
///
/// Mainly either [`Into::into`]'d a [`LazyTaskConfig`] but can be [`Self::make`]d into a [`TaskConfig`].
///
/// By sacrificing, [`LazyTaskConfig::Made`], [`LazyTaskConfig::Url`], and [`LazyTaskConfig::BetterUrl`], URL Cleaner Site was able to reduce memory usage by around 30% in extreme cases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallLazyTaskConfig<'t> {
    /// A [`str`] for use in [`TaskConfig::from_str`].
    ///
    /// Please note that if the string starts with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_str`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    ///
    /// This is very useful for, for example, getting tasks from lines of a file.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// assert!(matches!(serde_json::from_str(r#""https://example.com""#), Ok(SmallLazyTaskConfig::Str(_))))
    /// ```
    Str(&'t str),
    /// A [`String`] for use in [`TaskConfig::from_str`].
    ///
    /// Please note that if the string starts with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_str`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    ///
    /// This is very useful for, for example, getting tasks from lines of a file.
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(SmallLazyTaskConfig::from(          "https://example.com"   ).make().unwrap(), target);
    /// assert_eq!(SmallLazyTaskConfig::from(       r#""https://example.com""# ).make().unwrap(), target);
    /// assert_eq!(SmallLazyTaskConfig::from(r#"{"url":"https://example.com"}"#).make().unwrap(), target);
    ///
    /// // Guard patterns when?
    /// // https://github.com/rust-lang/rust/issues/129967
    /// match serde_json::from_str(r#""https://example.com/\u0041""#) {
    ///     Ok(SmallLazyTaskConfig::String(x)) if x == "https://example.com/A" => {},
    ///     x => panic!("{x:?}")
    /// }
    /// ```
    String(String),
    /// A UTF-8 byte sequence that is treated the same as a [`str`].
    ///
    /// Please note that if the bytes start with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_slice`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    ///
    /// This is very useful for, for example, getting tasks from lines of a file.
    ByteSlice(&'t [u8]),
    /// A UTF-8 byte sequence that is treated the same as a [`String`].
    ///
    /// Please note that if the bytes start with a `{` or a `"` it is deserialized as JSON using [`serde_json::from_slice`].
    ///
    /// You don't need to manually special case those into [`Self::JsonValue`].
    ///
    /// This is very useful for, for example, getting tasks from lines of a file.
    Bytes(Vec<u8>),
    /// A [`serde_json::Value`] for use in [`serde_json::from_value`].
    ///
    /// Please note that [`serde_json::Value::String`]s are valid.
    ///
    /// You don't need to manually special case those into [`Self::String`].
    /// # Examples
    /// ```
    /// use serde_json::json;
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let target = TaskConfig {url: "https://example.com".parse().unwrap(), context: Default::default()};
    ///
    /// assert_eq!(SmallLazyTaskConfig::from(json!(        "https://example.com" )).make().unwrap(), target);
    /// assert_eq!(SmallLazyTaskConfig::from(json!({"url": "https://example.com"})).make().unwrap(), target);
    /// ```
    JsonValue(serde_json::Value)
}

impl<'a> From<SmallLazyTaskConfig<'a>> for LazyTaskConfig<'a> {
    fn from(value: SmallLazyTaskConfig<'a>) -> Self {
        match value {
            SmallLazyTaskConfig::Str      (string) => LazyTaskConfig::Str      (string),
            SmallLazyTaskConfig::String   (string) => LazyTaskConfig::String   (string),
            SmallLazyTaskConfig::ByteSlice(bytes)  => LazyTaskConfig::ByteSlice(bytes) ,
            SmallLazyTaskConfig::Bytes    (bytes)  => LazyTaskConfig::Bytes    (bytes) ,
            SmallLazyTaskConfig::JsonValue(value)  => LazyTaskConfig::JsonValue(value) 
        }
    }
}

impl<'t> SmallLazyTaskConfig<'t> {
    /// Makes the [`TaskConfig`].
    ///
    /// If possible, this should be done in worker threads instead of the main thread.
    ///
    /// Simply calling [`Url::parse`] takes up a significant chunk of the time it takes to do a [`Job`].
    /// # Errors
    /// For each variant, uses thier respective [`Into::into`] or [`TryInto::try_into`] implementation.
    pub fn make(self) -> Result<TaskConfig, MakeTaskConfigError> {
        Ok(match self {
            SmallLazyTaskConfig::Str      (string) =>    string .try_into()?,
            SmallLazyTaskConfig::String   (string) => (&*string).try_into()?,
            SmallLazyTaskConfig::ByteSlice(bytes)  =>    bytes  .try_into()?,
            SmallLazyTaskConfig::Bytes    (bytes)  => (&*bytes ).try_into()?,
            SmallLazyTaskConfig::JsonValue(value)  =>    value  .try_into()?
        })
    }

    /// Convert [`Self::Str`] to [`Self::String`] and [`Self::ByteSlice`] to [`Self::Bytes`].
    pub fn into_owned(self) -> SmallLazyTaskConfig<'static> {
        match self {
            Self::Str      (x) => SmallLazyTaskConfig::String   (x.into()),
            Self::String   (x) => SmallLazyTaskConfig::String   (x),
            Self::ByteSlice(x) => SmallLazyTaskConfig::Bytes    (x.into()),
            Self::Bytes    (x) => SmallLazyTaskConfig::Bytes    (x),
            Self::JsonValue(x) => SmallLazyTaskConfig::JsonValue(x)
        }
    }
}

impl Serialize for SmallLazyTaskConfig<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Str      (string) => string.serialize(serializer),
            Self::String   (string) => string.serialize(serializer),
            Self::Bytes    (bytes)  => bytes .serialize(serializer),
            Self::ByteSlice(bytes)  => bytes .serialize(serializer),
            Self::JsonValue(value)  => value .serialize(serializer)
        }
    }
}

/// [`Visitor`] for [`SmallLazyTaskConfig`].
struct SmallLazyTaskConfigVisitor;

impl<'de> Visitor<'de> for SmallLazyTaskConfigVisitor {
    type Value = SmallLazyTaskConfig<'de>;

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

impl<'de> Deserialize<'de> for SmallLazyTaskConfig<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(SmallLazyTaskConfigVisitor)
    }
}

impl<'t> From<&'t str> for SmallLazyTaskConfig<'t> {
    fn from(value: &'t str) -> Self {
        Self::Str(value)
    }
}

impl From<String> for SmallLazyTaskConfig<'_> {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<'t> From<Cow<'t, str>> for SmallLazyTaskConfig<'t> {
    fn from(value: Cow<'t, str>) -> Self {
        match value {
            Cow::Borrowed(string) => Self::Str(string),
            Cow::Owned(string) => Self::String(string)
        }
    }
}

impl<'t> From<&'t [u8]> for SmallLazyTaskConfig<'t> {
    fn from(value: &'t [u8]) -> Self {
        Self::ByteSlice(value)
    }
}

impl From<Vec<u8>> for SmallLazyTaskConfig<'_> {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl<'t> From<Cow<'t, [u8]>> for SmallLazyTaskConfig<'t> {
    fn from(value: Cow<'t, [u8]>) -> Self {
        match value {
            Cow::Borrowed(bytes) => Self::ByteSlice(bytes),
            Cow::Owned(bytes) => Self::Bytes(bytes)
        }
    }
}

impl From<serde_json::Value> for SmallLazyTaskConfig<'_> {
    fn from(value: serde_json::Value) -> Self {
        Self::JsonValue(value)
    }
}


//! Glue for [`serde_json`].

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::prelude::*;

/// Allow making [`serde_json::Value`]s using [`StringSource`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub enum StringSourceJsonValue {
    /// [`Value::Null`].
    Null,
    /// [`Value::Bool`].
    Bool(bool),
    /// [`Value::Number`].
    Number(serde_json::value::Number),
    /// [`Value::String`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringSourceError::StringSourceIsNone`].
    String(StringSource),
    /// [`Value::Array`].
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    Array(Vec<Self>),
    /// [`Value::Object`].
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    Object(HashMap<String, Self>)
}

impl FromStr for StringSourceJsonValue {
    type Err = std::convert::Infallible;

    /// Makes a [`Self::String`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for StringSourceJsonValue {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for StringSourceJsonValue {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

#[allow(clippy::missing_errors_doc, reason = "Who cares?")]
impl Serialize for StringSourceJsonValue {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <StringSourceJsonValue>::serialize(self, serializer)
    }
}

#[allow(clippy::missing_errors_doc, reason = "Who cares?")]
impl<'de> Deserialize<'de> for StringSourceJsonValue {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = StringSourceJsonValue;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expected all JSON values to be valid. If you're getting this error from JSON it's a bug and you should tell me.")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::from(s))
            }

            fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                Self::Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Value::Number(value.into()).into())
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Value::Number(value.into()).into())
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number).into())
            }
        }

        deserializer.deserialize_any(V)
    }
}

impl From<Value> for StringSourceJsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null      => Self::Null,
            Value::Bool  (x) => Self::Bool  (x),
            Value::Number(x) => Self::Number(x),
            Value::String(x) => Self::String(x.into()),
            Value::Array (x) => Self::Array (x.into_iter().map(Into::into).collect()),
            Value::Object(x) => Self::Object(x.into_iter().map(|(k, v)| (k, v.into())).collect())
        }
    }
}

impl StringSourceJsonValue {
    /// Makes a [`serde_json::Value`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    ///
    /// But TL;DR: If any call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make(&self, task_state: &TaskStateView) -> Result<Value, StringSourceError> {
        Ok(match self {
            Self::Null      => Value::Null,
            Self::Bool  (x) => Value::Bool(*x),
            Self::Number(x) => Value::Number(x.clone()),
            Self::String(x) => Value::String(get_string!(x, task_state, StringSourceError)),
            Self::Array (x) => Value::Array(x.iter().map(|x| x.make(task_state)).collect::<Result<_, _>>()?),
            Self::Object(x) => Value::Object(x.iter().map(|(k, v)| Ok::<_, StringSourceError>((k.clone(), v.make(task_state)?))).collect::<Result<_, _>>()?)
        })
    }
}

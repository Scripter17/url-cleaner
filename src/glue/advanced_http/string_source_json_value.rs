//! "What if [`serde_json::Value`] used [`StringSource`]" and other horrible sentences my eternal torment has doomed me to.

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::types::*;
use crate::util::*;

/// "What if [`serde_json::Value`] used [`StringSource`]" and other horrible sentences my eternal torment has doomed me to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum StringSourceJsonValue {
    /// [`serde_json::Value::Null`].
    Null,
    /// [`serde_json::Value::Bool`].
    Bool(bool),
    /// [`serde_json::Value::Number`].
    Number(serde_json::value::Number),
    /// [`serde_json::Value::String`].
    String(StringSource),
    /// [`serde_json::Value::Array`].
    Array(Vec<Self>),
    /// [`serde_json::Value::Object`].
    Object(HashMap<String, Self>)
}

impl FromStr for StringSourceJsonValue {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.into()))
    }
}

/// Serialize the object. Although the macro this implementation came from allows [`Self::deserialize`]ing from a string, this currently always serializes to a map, though that may change eventually.
impl Serialize for StringSourceJsonValue {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <StringSourceJsonValue>::serialize(self, serializer)
    }
}

/// This particular implementation allows for deserializing from a string using [`Self::from_str`], an [`i64`], [`u64`], and [`f64`].
/// 
/// See [serde_with#702](https://github.com/jonasbb/serde_with/issues/702#issuecomment-1951348210) for details.
impl<'de> Deserialize<'de> for StringSourceJsonValue {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Gonna be honest, this feels like an odd way to design the API.
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = StringSourceJsonValue;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("Expected a string or a map.")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Self::Value::from_str(s).map_err(E::custom)
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

impl<T: Into<Value>> From<T> for StringSourceJsonValue {
    fn from(value: T) -> Self {
        match value.into() {
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
    /// Turns a [`Self`] into a [`serde_json::Value`] using [`StringSource::get`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make(&self, job_state: &JobStateView) -> Result<Value, StringSourceError> {
        debug!(StringSourceJsonValue::make, self, job_state);

        Ok(match self {
            Self::Null      => Value::Null,
            Self::Bool  (x) => Value::Bool(*x),
            Self::Number(x) => Value::Number(x.clone()),
            Self::String(x) => Value::String(get_string!(x, job_state, StringSourceError)),
            Self::Array (x) => Value::Array(x.iter().map(|x| x.make(job_state)).collect::<Result<_, _>>()?),
            Self::Object(x) => Value::Object(x.iter().map(|(k, v)| Ok::<_, StringSourceError>((k.clone(), v.make(job_state)?))).collect::<Result<_, _>>()?)
        })
    }
}

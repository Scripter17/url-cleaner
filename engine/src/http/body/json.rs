//! [`HttpJsonBodySource`].

use serde_json::{Number, Value};

use crate::prelude::*;

/// Allow making [`serde_json::Value`]s using [`StringSource`]s.
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
pub enum HttpJsonBodySource {
    /// [`Value::Null`].
    Null,
    /// [`Value::Bool`].
    Bool(bool),
    /// [`Value::Number`].
    Number(Number),
    /// [`Value::String`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringNotFound`].
    String(StringSource),
    /// [`Value::Array`].
    Array(Vec<Self>),
    /// [`Value::Object`].
    Object(HashMap<String, Self>)
}

impl HttpJsonBodySource {
    /// Makes a [`serde_json::Value`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    ///
    /// But TL;DR: If any call to [`StringSource::get`] returns an error, that error is returned.
    pub fn get(&self, task_state: &TaskState<'_>, args: Option<&FunctionArgs>) -> Result<Value, HttpJsonBodySourceError> {
        Ok(match self {
            Self::Null      => Value::Null,
            Self::Bool  (x) => Value::Bool  (*x),
            Self::Number(x) => Value::Number(x.clone()),
            Self::String(x) => Value::String(get!(*x)),
            Self::Array (x) => Value::Array (x.iter().map(|x|      x.get(task_state, args)                        ).collect::<Result<_, _>>()?),
            Self::Object(x) => Value::Object(x.iter().map(|(k, v)| v.get(task_state, args).map(|v| (k.clone(), v))).collect::<Result<_, _>>()?)
        })
    }
}

impl FromStr for HttpJsonBodySource {
    type Err = std::convert::Infallible;

    /// Makes a [`Self::String`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str  > for HttpJsonBodySource {fn from(value: &str  ) -> Self {Self::String(value.into())}}
impl From<String> for HttpJsonBodySource {fn from(value: String) -> Self {Self::String(value.into())}}

impl From<StringSource         > for HttpJsonBodySource {fn from(value: StringSource         ) -> Self {Self::String(value)}}
impl From<Vec<Self>            > for HttpJsonBodySource {fn from(value: Vec<Self>            ) -> Self {Self::Array (value)}}
impl From<HashMap<String, Self>> for HttpJsonBodySource {fn from(value: HashMap<String, Self>) -> Self {Self::Object(value)}}

impl From<Value> for HttpJsonBodySource {
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

impl From<Number> for HttpJsonBodySource {
    fn from(value: Number) -> Self{
        Self::Number(value)
    }
}

impl Serialize for HttpJsonBodySource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null      => serializer.serialize_unit(),
            Self::Bool  (x) => x.serialize(serializer),
            Self::Number(x) => x.serialize(serializer),
            Self::String(x) => x.serialize(serializer),
            Self::Array (x) => x.serialize(serializer),
            Self::Object(x) => x.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for HttpJsonBodySource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = HttpJsonBodySource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str("A JSON value.")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(Self::Value::from(s))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut ret = Vec::new();

                while let Some(x) = seq.next_element()? {
                    ret.push(x);
                }

                Ok(Self::Value::Array(ret))
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut ret = HashMap::new();

                while let Some((k, v)) = map.next_entry()? {
                    ret.insert(k, v);
                }

                Ok(Self::Value::Object(ret))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Self::Value::Number(value.into()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Self::Value::Number(value.into()))
            }

            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(Number::from_f64(value).map_or(Self::Value::Null, Self::Value::Number))
            }
        }

        deserializer.deserialize_any(V)
    }
}

use std::str::FromStr;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::{Deserializer, Error as _}, ser::Serializer};
use reqwest::{Method, header::HeaderMap};

use crate::types::*;
use crate::glue::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestConfig {
    #[serde(deserialize_with = "deserialize_method", serialize_with = "serialize_method")]
    method: Method,
    #[serde(with = "headermap")]
    headers: HeaderMap,
    #[serde(deserialize_with = "optional_string_or_struct")]
    body: Option<RequestBody>
}

fn deserialize_method<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
    Method::from_str(Deserialize::deserialize(d)?).map_err(|e| D::Error::custom(e))
}

fn serialize_method<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(method.as_str())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestBody {
    Text(#[serde(deserialize_with = "string_or_struct")] StringSource),
    Form(#[serde(deserialize_with = "hashmap_value_string_or_struct")] HashMap<String, StringSource>)
}

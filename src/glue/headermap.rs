use std::collections::HashMap;

use serde::{Deserialize, ser::{Serializer}, de::{Deserializer, Error as _}};
use reqwest::header::HeaderMap;

/// Deserializes a [`HeaderMap`]
/// # Errors
/// If one of the keys or values aren't a valid header key or value, this function errors.
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderMap, D::Error> {
    let temp: HashMap<String, String> = Deserialize::deserialize(d)?;
    (&temp).try_into().map_err(|e| D::Error::custom(e))
}

/// Serializes [`HeaderMap`].
/// # Errors
/// Actually just panics when a non-ASCII header value is found.
/// TODO: Fix that.
/// # Panics
/// Panics when a non-ASCII header value is found.
/// TODO: Fix that.
pub fn serialize<S: Serializer>(x: &HeaderMap, s: S) -> Result<S::Ok, S::Error> {
    s.collect_map(x.into_iter().map(|(k, v)| (k.as_str().to_string(), v.to_str().expect("ASCII header value.").to_string())))
}

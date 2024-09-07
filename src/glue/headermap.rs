//! Provides serialization and deserialization functions for [`HeaderMap`].

use std::collections::HashMap;

use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
use reqwest::header::HeaderMap;
#[allow(unused_imports, reason = "Used in a doc comment.")] // [`HeaderValue`] is imported for [`serialize`]'s documentation.
use reqwest::header::HeaderValue;

/// Deserializes a [`HeaderMap`]
/// # Errors
/// If one of the keys or values aren't a valid header key or value, this function errors.
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderMap, D::Error> {
    let temp: HashMap<String, String> = Deserialize::deserialize(d)?;
    (&temp).try_into().map_err(D::Error::custom)
}

/// Serializes [`HeaderMap`].
/// # Errors
/// When the call to [`HeaderValue::to_str`] returns an error, that error is returned.
pub fn serialize<S: Serializer>(x: &HeaderMap, s: S) -> Result<S::Ok, S::Error> {
    s.collect_map(x.into_iter().map(|(k, v)| v.to_str().map(|v| (k.as_str().to_string(), v.to_string()))).collect::<Result<HashMap<_, _>, _>>().map_err(S::Error::custom)?)
}

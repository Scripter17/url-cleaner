//! Serde glue for [`HeaderMap`].

use std::collections::HashMap;

use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
use reqwest::header::HeaderMap;

/// Deserializes a [`HeaderMap`].
/// # Errors
/// If the value isn't a map or isn't a valid [`HeaderMap`], returns an error.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderMap, D::Error> {
    let temp: HashMap<String, String> = Deserialize::deserialize(d)?;
    (&temp).try_into().map_err(D::Error::custom)
}

/// Serializes a [`HeaderMap`].
/// # Errors
/// Never returns an error.
pub(crate) fn serialize<S: Serializer>(x: &HeaderMap, s: S) -> Result<S::Ok, S::Error> {
    s.collect_map(x.into_iter().map(|(k, v)| v.to_str().map(|v| (k.as_str().to_string(), v.to_string()))).collect::<Result<HashMap<_, _>, _>>().map_err(S::Error::custom)?)
}


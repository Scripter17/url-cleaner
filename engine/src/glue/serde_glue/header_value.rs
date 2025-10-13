//! Serde glue for [`HeaderValue`].

use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
use reqwest::header::HeaderValue;

/// Deserializes a [`HeaderValue`].
/// # Errors
/// If the value isn't a string or isn't a valid [`HeaderValue`], returns an error.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderValue, D::Error> {
    let temp: String = Deserialize::deserialize(d)?;
    temp.try_into().map_err(D::Error::custom)
}

/// Serializes a [`HeaderValue`].
/// # Errors
/// If the call to [`HeaderValue::to_str`] returns an error, that error is returned.
pub(crate) fn serialize<S: Serializer>(x: &HeaderValue, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(x.to_str().map_err(S::Error::custom)?)
}


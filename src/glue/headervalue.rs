//! Provides serialization and deserialization functions for [`HeaderValue`].

use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
#[allow(unused_imports)] // [`HeaderValue`] is imported for [`serialize`]'s documentation.
use reqwest::header::HeaderValue;

/// Deserializes a [`HeaderValue`]
/// # Errors
/// If one of the keys or values aren't a valid header key or value, this function errors.
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderValue, D::Error> {
    let temp: String = Deserialize::deserialize(d)?;
    temp.try_into().map_err(D::Error::custom)
}

/// Serializes [`HeaderValue`].
/// # Errors
/// When the call to [`HeaderValue::to_str`] returns an error, that error is returned.
pub fn serialize<S: Serializer>(x: &HeaderValue, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(x.to_str().map_err(S::Error::custom)?)
}

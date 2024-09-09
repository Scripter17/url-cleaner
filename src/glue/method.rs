//! Provides serialization and deserialization functions for [`Method`].

use std::str::FromStr;

use serde::{Deserialize, ser::Serializer, de::{Deserializer, Error as _}};
use reqwest::Method;

/// Deserializes a [`Method`].
/// # Errors
/// If the call to [`Method::from_str`] returns an error, that error is returned.
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
    Method::from_str(Deserialize::deserialize(d)?).map_err(D::Error::custom)
}

/// Serializes [`Method`].
pub fn serialize<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(method.as_str())
}

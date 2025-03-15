//! More granular error handling.

use serde::{Serialize, Deserialize, ser::Serializer, de::{Visitor, Deserializer, Error}};

use crate::types::*;
use crate::util::*;

/// Allows treating specific errors as passes/fails without ignoring all error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Suitability)]
pub enum IfError {
    /// Maps [`Condition`] errors to passes.
    Pass,
    /// Maps [`Condition`] errors to fails.
    Fail,
    /// Leaves [`Condition`] errors as-is.
    #[default]
    Error
}

impl IfError {
    /// If `x` is [`Err`], returns the value specified by `self`.
    /// # Errors
    /// If `x` is [`Err`] and `self is [`IfError::Error`], the error is returned.
    pub fn apply<T>(self, x: Result<bool, T>) -> Result<bool, T> {
        match (self, x) {
            (_          , Ok (x)) => Ok(x),
            (Self::Pass , Err(_)) => Ok(true),
            (Self::Fail , Err(_)) => Ok(false),
            (Self::Error, Err(e)) => Err(e)
        }
    }
}

/// [`Visitor`] to [`Deserialize`] [`IfError`]
struct IfErrorVisitor;

impl Visitor<'_> for IfErrorVisitor {
    type Value = IfError;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "Pass"  => Ok(Self::Value::Pass),
            "Fail"  => Ok(Self::Value::Fail),
            "Error" => Ok(Self::Value::Error),
            _ => Err(E::custom("Invalid string value"))
        }
    }
}

impl<'de> Deserialize<'de> for IfError {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(IfErrorVisitor)
    }
}

impl Serialize for IfError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pass  => serializer.serialize_str("Pass"),
            Self::Fail  => serializer.serialize_str("Fail"),
            Self::Error => serializer.serialize_str("Error")
        }
    }
}

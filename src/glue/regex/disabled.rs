use std::borrow::Cow;

use serde::{
    ser::{Error as SerError, Serialize, Serializer},
    de::{Error as DeError, Deserialize, Deserializer}
};

/// The disabled form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// This is the result of the default `regex` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
/// Calling any provided method on this will panic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexWrapper;

/// The disabled form of RegexParts.
/// This is the result of the default `regex` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
/// Calling any provided method on this will panic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexParts;

#[allow(dead_code)]
impl RegexParts {
    pub fn new         (_pattern: &str) -> Self  {panic!()}
    pub fn set_flags   (&mut self, _flags: &str) {panic!()}
    pub fn add_flags   (&mut self, _flags: &str) {panic!()}
    pub fn remove_flags(&mut self, _flags: &str) {panic!()}
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without the `regex` feature."))
    }
}

impl Serialize for RegexWrapper {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without the `regex` feature."))
    }
}

impl<'de> Deserialize<'de> for RegexParts {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without the `regex` feature."))
    }
}

impl Serialize for RegexParts {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without the `regex` feature."))
    }
}

impl RegexWrapper {
    /// The disabled version of the wrapper for `regex::Regex::is_match`.
    /// This version will always panic.
    pub fn is_match(&self, _str: &str) -> bool {
        panic!("URL Cleaner was compiled without the `regex` feature.")
    }

    /// The disabled version of the wrapper for `regex::Regex::replace`.
    /// This version will always panic.
    pub fn replace<'h, T>(&self, _haystack: &str, _rep: T) -> Cow<'h, str> {
        panic!("URL Cleaner was compiled without the `regex` feature.")
    }
}

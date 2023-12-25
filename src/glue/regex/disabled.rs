use std::borrow::Cow;

use serde::ser::{Error as SerError, Serialize, Serializer};
use serde::de::{Error as DeError, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq)]
/// The disabled form of the wrapper around [`regex::Regex`].
/// This is the result of the default `regex` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
pub struct RegexWrapper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexParts;

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without support for regex."))
    }
}

impl Serialize for RegexWrapper {
    fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without support for regex."))
    }
}

impl RegexWrapper {
    pub fn is_match(&self, _str: &str) -> bool {
        panic!()
    }

    pub fn replace<'h, T>(&self, _haystack: &str, _rep: T) -> Cow<'h, str> {
        panic!()
    }
}

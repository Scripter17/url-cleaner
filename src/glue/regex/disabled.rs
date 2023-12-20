use std::borrow::Cow;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone)]
/// The disabled form of the wrapper around [`regex::Regex`].
/// This is the result of the default `regex` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
pub struct RegexWrapper;

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Err(D::Error::custom("Url-cleaner was compiled without support for regex"))
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

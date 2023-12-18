use std::borrow::Cow;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Regex;

pub fn deserialize_regex<'de, D>(_deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>
{
    Err(D::Error::custom("Url-cleaner was compiled without support for regex"))
}

impl Regex {
    pub fn is_match(&self, _str: &str) -> bool {
        panic!()
    }

    pub fn replace<'h, T>(&self, _haystack: &str, _rep: T) -> Cow<'h, str> {
        panic!()
    }
}

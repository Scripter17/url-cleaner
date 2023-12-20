use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Glob;

impl<'de> Deserialize<'de> for Glob {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Err(D::Error::custom("Url-cleaner was compiled without support for glob"))
    }
}

impl Glob {
    pub fn matches(&self, _str: &str) -> bool {
        panic!()
    }
}
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone)]
/// The disabled form of the wrapper around [`glob::Glob`].
/// This is the result of the default `glob` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
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

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone)]
/// The disabled form of the wrapper around [`glob::Glob`].
/// This is the result of the default `glob` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
pub struct GlobWrapper;

impl<'de> Deserialize<'de> for GlobWrapper {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without support for glob."))
    }
}

impl GlobWrapper {
    pub fn matches(&self, _str: &str) -> bool {
        panic!()
    }
}

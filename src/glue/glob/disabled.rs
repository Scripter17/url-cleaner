use serde::{
    Serialize, Deserialize,
    ser::{Error as _, Serializer},
    de::{Error as _, Deserializer}
};

/// The disabled version of the wrapper around [`glob::Glob`] and [`glob::MatchOptions`].
/// This is the result of the default `glob` feature being disabled at compile time.
/// This version cannot be deserialized, which may or may not be the best way to handle this.
/// Calling any provided method on this will panic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobWrapper;

impl<'de> Deserialize<'de> for GlobWrapper {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without the `glob` feature."))
    }
}

impl Serialize for GlobWrapper {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without the `glob` feature."))
    }
}

impl GlobWrapper {
    /// The disabled version of [`GlobWrapper::matches`].
    /// # Panics
    /// This version always panics.
    pub fn matches(&self, _str: &str) -> bool {
        panic!("URL Cleaner was compiled without the `glob` feature.")
    }
}

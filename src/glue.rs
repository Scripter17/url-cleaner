//! Various "glue"s to integrate with other crates for advanced features.

#[cfg(feature = "regex"   )] pub mod regex;
#[cfg(feature = "regex"   )] pub use regex::*;
#[cfg(feature = "glob"    )] pub mod glob;
#[cfg(feature = "glob"    )] pub use glob::*;
#[cfg(feature = "commands")] pub mod command;
#[cfg(feature = "commands")] pub use command::*;
#[cfg(feature = "http"    )] pub mod http_client_config;
#[cfg(feature = "http"    )] pub use http_client_config::*;
#[cfg(feature = "http"    )] pub mod proxy;
#[cfg(feature = "http"    )] pub use proxy::*;
#[cfg(feature = "http"    )] pub mod http;
#[cfg(feature = "http"    )] pub use http::*;
#[cfg(feature = "http"    )] pub mod json;
#[cfg(feature = "http"    )] pub use json::*;
#[cfg(feature = "cache"   )] pub mod caching;
#[cfg(feature = "cache"   )] pub use caching::*;
#[cfg(feature = "base64"  )] pub mod base64;
#[cfg(feature = "base64"  )] pub use base64::*;

pub mod parse;

/// Glue to allow [`reqwest::header::HeaderValue`] to be used with serde.
#[cfg(feature = "http")]
pub(crate) mod serde_headervalue {
    use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
    use reqwest::header::HeaderValue;

    /// Deserializes a [`HeaderValue`].
    /// # Errors
    /// If the value isn't a string or isn't a valid [`HeaderValue`], returns an error.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderValue, D::Error> {
        let temp: String = Deserialize::deserialize(d)?;
        temp.try_into().map_err(D::Error::custom)
    }

    /// Serializes a [`HeaderValue`].
    /// # Errors
    /// If the call to [`HeaderValue::to_str`] returns an error, that error is returned.
    pub fn serialize<S: Serializer>(x: &HeaderValue, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(x.to_str().map_err(S::Error::custom)?)
    }
}

/// Glue to allow [`reqwest::header::HeaderMap`] to be used with serde.
#[cfg(feature = "http")]
pub(crate) mod serde_headermap {
    use std::collections::HashMap;

    use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
    use reqwest::header::HeaderMap;

    /// Deserializes a [`HeaderMap`].
    /// # Errors
    /// If the value isn't a map or isn't a valid [`HeaderMap`], returns an error
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderMap, D::Error> {
        let temp: HashMap<String, String> = Deserialize::deserialize(d)?;
        (&temp).try_into().map_err(D::Error::custom)
    }

    /// Serializes a [`HeaderMap`].
    /// # Errors
    /// Never returns an error.
    pub fn serialize<S: Serializer>(x: &HeaderMap, s: S) -> Result<S::Ok, S::Error> {
        s.collect_map(x.into_iter().map(|(k, v)| v.to_str().map(|v| (k.as_str().to_string(), v.to_string()))).collect::<Result<HashMap<_, _>, _>>().map_err(S::Error::custom)?)
    }
}

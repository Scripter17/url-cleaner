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
#[cfg(feature = "custom"  )] pub mod fn_wrapper;
#[cfg(feature = "custom"  )] pub use fn_wrapper::*;

pub mod unescape_js;
pub use unescape_js::*;

/// Glue to allow [`reqwest::Method`] to be used with serde.
#[cfg(feature = "http")]
#[allow(clippy::missing_errors_doc, reason = "Who cares?")]
pub mod serde_method {
    use std::str::FromStr;

    use serde::{Deserialize, ser::Serializer, de::{Deserializer, Error as _}};
    use reqwest::Method;

    /// Deserializes a [`Method`].
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
        Method::from_str(Deserialize::deserialize(d)?).map_err(D::Error::custom)
    }

    /// Serializes a [`Method`].
    pub fn serialize<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(method.as_str())
    }
}

/// Glue to allow [`reqwest::header::HeaderValue`] to be used with serde.
#[cfg(feature = "http")]
#[allow(clippy::missing_errors_doc, reason = "Who cares?")]
pub mod serde_headervalue {
    use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
    use reqwest::header::HeaderValue;

    /// Deserializes a [`HeaderValue`].
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderValue, D::Error> {
        let temp: String = Deserialize::deserialize(d)?;
        temp.try_into().map_err(D::Error::custom)
    }

    /// Serializes a [`HeaderValue`].
    pub fn serialize<S: Serializer>(x: &HeaderValue, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(x.to_str().map_err(S::Error::custom)?)
    }
}

/// Glue to allow [`reqwest::header::HeaderMap`] to be used with serde.
#[cfg(feature = "http")]
#[allow(clippy::missing_errors_doc, reason = "Who cares?")]
pub mod serde_headermap {
    use std::collections::HashMap;

    use serde::{Deserialize, ser::{Serializer, Error as _}, de::{Deserializer, Error as _}};
    use reqwest::header::HeaderMap;
    #[expect(unused_imports, reason = "Used in a doc comment.")] // [`HeaderValue`] is imported for [`serialize`]'s documentation.
    use reqwest::header::HeaderValue;

    /// Deserializes a [`HeaderMap`].
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HeaderMap, D::Error> {
        let temp: HashMap<String, String> = Deserialize::deserialize(d)?;
        (&temp).try_into().map_err(D::Error::custom)
    }

    /// Serializes a [`HeaderMap`].
    pub fn serialize<S: Serializer>(x: &HeaderMap, s: S) -> Result<S::Ok, S::Error> {
        s.collect_map(x.into_iter().map(|(k, v)| v.to_str().map(|v| (k.as_str().to_string(), v.to_string()))).collect::<Result<HashMap<_, _>, _>>().map_err(S::Error::custom)?)
    }
}

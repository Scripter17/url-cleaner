//! "Glue" to make working with types from other crates easier.

#[cfg(feature = "regex"            )] mod regex;
#[cfg(feature = "regex"            )] pub use regex::*;
#[cfg(feature = "glob"             )] mod glob;
#[cfg(feature = "glob"             )] pub use glob::*;
#[cfg(feature = "commands"         )] mod command;
#[cfg(feature = "commands"         )] pub use command::*;
#[cfg(feature = "advanced-requests")] mod advanced_requests;
#[cfg(feature = "advanced-requests")] pub use advanced_requests::*;
#[cfg(feature = "http"             )] mod http_client_config;
#[cfg(feature = "http"             )] pub use http_client_config::*;
#[cfg(feature = "http"             )] pub mod proxy;
#[cfg(feature = "http"             )] pub use proxy::*;
#[cfg(feature = "http"             )] pub(crate) mod headermap;
#[cfg(feature = "http"             )] pub(crate) mod headervalue;
#[cfg(feature = "http"             )] pub(crate) mod method;
#[cfg(feature = "cache"            )] mod caching;
#[cfg(feature = "cache"            )] pub use caching::*;
#[cfg(feature = "base64"           )] mod base64;
#[cfg(feature = "base64"           )] pub use base64::*;

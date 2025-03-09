//! "Glue" APIs that allow working with select other crates.

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
#[cfg(feature = "http"    )] pub(crate) mod headermap;
#[cfg(feature = "http"    )] pub(crate) mod headervalue;
#[cfg(feature = "http"    )] pub(crate) mod method;
#[cfg(feature = "http"    )] pub mod advanced_http;
#[cfg(feature = "http"    )] pub use advanced_http::*;
#[cfg(feature = "cache"   )] pub mod caching;
#[cfg(feature = "cache"   )] pub use caching::*;
#[cfg(feature = "base64"  )] pub mod base64;
#[cfg(feature = "base64"  )] pub use base64::*;
#[cfg(feature = "custom"  )] pub mod fn_wrapper;
#[cfg(feature = "custom"  )] pub use fn_wrapper::*;

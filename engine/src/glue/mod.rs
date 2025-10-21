//! Various "glue"s to integrate with other crates for advanced features.

pub mod parse;
pub mod percent_encoding;
pub mod json;
pub mod better_url;

#[cfg(feature = "regex"  )] pub mod regex;
#[cfg(feature = "command")] pub mod command;
#[cfg(feature = "http"   )] pub mod http;
#[cfg(feature = "cache"  )] pub mod cache;
#[cfg(feature = "base64" )] pub mod base64;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::parse;
    pub use super::percent_encoding::*;
    pub use super::json::*;
    pub use super::better_url::*;

    #[cfg(feature = "regex"  )] pub use super::regex::prelude::*;
    #[cfg(feature = "command")] pub use super::command::*;
    #[cfg(feature = "http"   )] pub use super::http::prelude::*;
    #[cfg(feature = "cache"  )] pub use super::cache::prelude::*;
    #[cfg(feature = "base64" )] pub use super::base64::prelude::*;
}

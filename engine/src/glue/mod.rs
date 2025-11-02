//! Various "glue"s to integrate with other crates for advanced features.

pub mod parse;
pub mod percent_encoding;
pub mod json;
pub mod better_url;
pub mod regex;
pub mod base64;

#[cfg(feature = "http"   )] pub mod http;
#[cfg(feature = "cache"  )] pub mod cache;
#[cfg(feature = "command")] pub mod command;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::parse;
    pub use super::percent_encoding::*;
    pub use super::json::*;
    pub use super::better_url::*;
    pub use super::regex::prelude::*;
    pub use super::base64::prelude::*;

    #[cfg(feature = "http"   )] pub use super::http::prelude::*;
    #[cfg(feature = "cache"  )] pub use super::cache::prelude::*;
    #[cfg(feature = "command")] pub use super::command::*;
}

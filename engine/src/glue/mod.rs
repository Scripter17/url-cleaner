//! Various "glue"s to integrate with other crates for advanced features.

#[cfg(feature = "regex"  )] pub mod regex;
#[cfg(feature = "command")] pub mod command;
#[cfg(feature = "http"   )] pub mod http;
#[cfg(feature = "cache"  )] pub mod cache;
#[cfg(feature = "base64" )] pub mod base64;

pub mod parse;
pub mod percent_encoding;
pub mod url_position;
pub mod json;

pub(crate) mod serde_glue;

/// Allows importing all glue stuff without the problematic module names.
pub mod prelude {
    #[cfg(feature = "regex"  )] pub use super::regex::*;
    #[cfg(feature = "command")] pub use super::command::*;
    #[cfg(feature = "http"   )] pub use super::http::*;
    #[cfg(feature = "cache"  )] pub use super::cache::*;
    #[cfg(feature = "base64" )] pub use super::base64::*;
    pub use super::parse;
    pub use super::percent_encoding::*;
    pub use super::url_position::*;
    pub use super::json::*;

    pub(crate) use super::serde_glue;
}

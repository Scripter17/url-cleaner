//! "Glue" to make working with types from other crates easier.

#[cfg(feature = "regex"   )] mod regex;
#[cfg(feature = "regex"   )] pub use regex::*;
#[cfg(feature = "glob"    )] mod glob;
#[cfg(feature = "glob"    )] pub use glob::*;
#[cfg(feature = "commands")] mod command;
#[cfg(feature = "commands")] pub use command::*;
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub mod proxy;
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub use proxy::*;
/// Serializing and deserializing [`reqwest::header::HeaderMap`].
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub(crate) mod headermap;
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub(crate) mod headervalue;
#[cfg(all(feature = "http", not(target_family = "wasm")))] pub(crate) mod method;
#[cfg(feature = "cache")] mod caching;
#[cfg(feature = "cache")] pub use caching::*;

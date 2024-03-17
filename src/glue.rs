#[cfg(feature = "regex"   )] mod regex;
#[cfg(feature = "regex"   )] pub use regex::*;
#[cfg(feature = "glob"    )] mod glob;
#[cfg(feature = "glob"    )] pub use glob::*;
#[cfg(feature = "commands")] mod command;
#[cfg(feature = "commands")] pub use command::*;
/// Serializing and deserializing [`reqwest::header::HeaderMap`].
#[cfg(all(feature = "http", not(target_family = "wasm")))]
pub mod headermap;

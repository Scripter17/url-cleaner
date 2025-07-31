//! # [Glue](crate::glue)
//!
//! URL Cleaner Engine has various "glues" for networking, caching, regex, base64, percent encoding/decoding, and commands.
//!
//! Each of the glues listed can be disabled at compile time except for commands, which is disabled by default and has to be explicitly enabled at compile time.
//!
//! While there are some other things in the [glue](crate::glue) module, those are either things with no better place to be or new glues I forgot to list above.
//!
//! The default cleaner requires the networking, caching, regex, and base64 glues are enabled.

pub(crate) use super::*;

pub mod parsing;
pub(crate) use parsing::*;
#[cfg(feature = "cache")]
pub mod caching;
#[cfg(feature = "cache")]
pub(crate) use caching::*;

//! The engine of URL Cleaner.

pub mod data;
pub mod job;
pub mod cleaner;
pub mod function;
pub mod components;
pub mod regex;
pub mod base64;
pub mod parsing;
pub mod errors;

#[cfg(feature = "http" )] pub mod http;
#[cfg(feature = "cache")] pub mod cache;

pub(crate) mod util;
pub(crate) mod debug;

/// A prelude module to make importing all the various types nicer.
///
/// Generally not meant for external use.
pub mod prelude {
    pub use super::data::*;
    pub use super::job::*;
    pub use super::cleaner::*;
    pub use super::function::*;
    pub use super::components::*;
    pub use super::regex::*;
    pub use super::base64::*;
    pub use super::parsing::*;
    pub use super::errors::*;

    #[cfg(feature = "http" )] pub use super::http::*;
    #[cfg(feature = "cache")] pub use super::cache::*;

    pub use better_url::prelude::*;
    pub(crate) use better_url::util::*;

    pub(crate) use super::util::*;
    pub(crate) use super::debug::debug;

    pub(crate) use std::borrow::{Borrow, Cow};
    pub(crate) use std::collections::{HashMap, HashSet};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::ops::{RangeBounds, Bound, Range, Deref, DerefMut};
    pub(crate) use std::hash::Hash;
    pub(crate) use std::io;
    pub(crate) use std::num::NonZero;

    pub(crate) use serde::{Serialize, Deserialize, ser::{Serializer, SerializeSeq, SerializeMap}, de::{self, Deserializer, Visitor, MapAccess, SeqAccess, Error as _}};
    pub(crate) use thiserror::Error;
}

//! The engine of URL Cleaner.

pub mod job;
pub mod cleaner;
pub mod function;
pub mod profiled_cleaner;
pub mod components;

#[cfg(feature = "http" )] pub mod http;
#[cfg(feature = "cache")] pub mod cache;

pub(crate) mod util;
pub(crate) mod debug;

/// A prelude module to make importing all the various types nicer.
///
/// Generally not meant for external use.
pub mod prelude {
    pub use super::job::*;
    pub use super::cleaner::*;
    pub use super::function::*;
    pub use super::profiled_cleaner::*;
    pub use super::components::*;

    #[cfg(feature = "http" )] pub use super::http::*;
    #[cfg(feature = "cache")] pub use super::cache::prelude::*;

    pub use better_url::prelude::*;
    pub(crate) use better_url::util::*;

    pub(crate) use super::util::*;
    pub(crate) use super::debug::debug;

    pub(crate) use serde::{Serialize, Deserialize, ser::{Serializer, SerializeSeq, SerializeMap}, de::{self, Deserializer, Visitor, MapAccess, SeqAccess, Error as _}};
    pub(crate) use std::borrow::Cow;
    pub(crate) use thiserror::Error;
    pub(crate) use std::collections::{HashMap, HashSet};
    pub(crate) use serde_with::{serde_as, MapPreventDuplicates};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::ops::{RangeBounds, Bound, Range};
}

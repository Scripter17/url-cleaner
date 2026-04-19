//! A wrapper around the [`url`](::url) crate that provides higher level operations.

mod url;
pub mod parts;
pub mod details;
pub mod errors;
pub mod util;

pub use url::*;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::url::*;
    pub use super::parts::*;
    pub use super::details::*;
    pub use super::errors::*;
    pub(crate) use super::util::*;

    pub(crate) use std::borrow::{Borrow, Cow};
    pub(crate) use std::ops::{Range, Bound, RangeBounds, Deref};
    pub(crate) use std::cmp::Ordering;
    pub(crate) use std::hash::{Hash, Hasher};
    pub(crate) use std::fmt::{Display, Formatter};
    pub(crate) use std::str::FromStr;
    #[cfg(feature = "serde")]
    pub(crate) use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error as _}};
}

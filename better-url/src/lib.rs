//! [`BetterUrl`] is a wrapper around [`::url::Url`] aiming to provide as nice an API as URLs can be forced into.

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
    pub(crate) use std::num::NonZero;
    #[cfg(feature = "serde")]
    pub(crate) use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor, Error as _}};
}

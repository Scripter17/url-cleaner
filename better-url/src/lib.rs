//! A URL crate that's better than the one provided by Servo.
//!
//! Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
//!
//! <https://www.gnu.org/licenses/agpl-3.0.html>
//!
//! ## Performance
//!
//! See [`better-url-bench`](https://github.com/Scripter17/url-cleaner/tree/main/better-url-bench) for performance details.

mod url;
pub mod parts;
pub mod details;
pub mod errors;
pub mod util;

pub use url::*;

#[cfg(test)]
mod tests;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::url::*;
    pub use super::parts::*;
    pub use super::details::*;
    pub use super::errors::*;
    pub(crate) use super::util::*;

    pub(crate) use std::borrow::{Borrow, Cow};
    pub(crate) use std::ops::{Range, Bound, RangeBounds};
    pub(crate) use std::cmp::Ordering;
    pub(crate) use std::hash::{Hash, Hasher};
    pub(crate) use std::fmt::{Display, Formatter};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::num::NonZero;
    #[cfg(any(feature = "serde", test))]
    pub(crate) use serde::{Deserialize, de::{Deserializer, Error as _}};
    #[cfg(feature = "serde")]
    pub(crate) use serde::{Serialize, ser::Serializer, de::Visitor};
}

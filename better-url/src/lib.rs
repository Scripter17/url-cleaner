//! A URL crate that's better than the one provided by Servo.
//!
//! Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
//!
//! <https://www.gnu.org/licenses/agpl-3.0.html>
//!
//! ## Performance
//!
//! See [`better-url-bench`](https://github.com/Scripter17/url-cleaner/tree/main/better-url-bench) for performance details.
//!
//! TL;DR: Better URL blows Servo's URL crate (`url`) out of the water, at least as of their 2.5.8.
//!
//! ## Parsing from bytes
//!
//! [`BetterUrl`] as well as the various [`parts`] types have from not just strings but also arbitrary byte sequences.
//!
//! For byte sequences that happen to be UTF-8, the behavior is identical, but for byte sequences that aren't UTF-8, *UTF-8 validity is not checked* unless and until needed.
//!
//! In the future I may add a feature flag to enforce UTF-8 validity in bytes constructors.
//!
//! ```
//! use better_url::prelude::*;
//!
//! assert_eq!(Ipv4Host::new(b"127.0.0.%E2\x91%A0").unwrap(), "127.0.0.1");
//!
//! assert_eq!(SpecialQuery::new(b"\xff"), "%FF");
//!
//! assert_eq!(BetterUrl::new(b"https://example.com/\xff?\xff#\xff").unwrap(), "https://example.com/%FF?%FF#%FF");
//! ```

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

    pub(crate) use std::borrow::{Borrow, BorrowMut, Cow};
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

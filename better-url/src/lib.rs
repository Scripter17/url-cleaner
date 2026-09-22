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
//! Please note that in addition to parsing [`BetterUrl`] and the various [`parts`] types from strings, you are also able to parse them from bytes.
//!
//! When doing so, invalid UTF-8 will be ignored.
//!
//! For most parts, this either has no effect (such as for [`Scheme`](parts::Scheme) and [`Ipv6Host`](parts::Ipv6Host)) or will percent encode the non-ASCII bytes as usual (such as with the many [`Query`](parts::Query) types).
//!
//! However, for [`DomainHost`](parts::DomainHost) and [`Ipv4Host`](parts::Ipv4Host) specifically, this has the weird effect that partially percent encoded UTF-8 multibyte sequences will be percent decoded into UTF-8 and then accepted.
//!
//! ```
//! use better_url::prelude::*;
//!
//! assert_eq!(BetterUrl::new(b"https://%C2\xA1.com").unwrap(), "https://xn--7a.com/");
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

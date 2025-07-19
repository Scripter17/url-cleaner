//! General utility functions.

use std::borrow::Cow;

mod macros;
pub(crate) use macros::*;
mod suitability;
pub(crate) use suitability::*;
#[cfg(feature = "debug")] mod debug;
#[cfg(feature = "debug")] pub(crate) use debug::*;

/// Dud debug macro.
#[cfg(not(feature = "debug"))]
macro_rules! debug {($($_:tt)*) => {}}
#[cfg(not(feature = "debug"))]
pub(crate) use debug;

mod serde_helpers;
pub(crate) use serde_helpers::*;
mod indexing;
pub(crate) use indexing::*;
mod segments;
pub(crate) use segments::*;

/// Percent encoding helper.
pub(crate) fn peh(s: &str) -> Cow<'_, str> {
    percent_encoding::percent_decode_str(s).decode_utf8_lossy()
}

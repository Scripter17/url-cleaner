//! General utility functions.

use std::ops::Bound;
use std::borrow::Cow;

mod segments;
pub(crate) use segments::*;
mod indexing;
pub(crate) use indexing::*;

/// Converts an `end` bound to a [`Bound`].
///
/// Specifically, if `i` is [`Some`], return [`Bound::Excluded`] or [`Bound::Unbounded`] if it's [`None`].
pub(crate) fn exorub(i: Option<usize>) -> Bound<usize> {
    match i {
        Some(i) => Bound::Excluded(i),
        None => Bound::Unbounded
    }
}

/// Percent encoding helper.
pub(crate) fn peh(s: &str) -> Cow<'_, str> {
    percent_encoding::percent_decode_str(s).decode_utf8_lossy()
}

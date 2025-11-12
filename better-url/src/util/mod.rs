//! General utility functions.

use std::ops::Bound;
use std::borrow::Cow;

mod segments;
pub use segments::*;
mod indexing;
pub use indexing::*;

/// Converts an `end` bound to a [`Bound`].
///
/// Specifically, if `i` is [`Some`], return [`Bound::Excluded`] or [`Bound::Unbounded`] if it's [`None`].
pub(crate) fn exorub(i: Option<usize>) -> Bound<usize> {
    match i {
        Some(i) => Bound::Excluded(i),
        None => Bound::Unbounded
    }
}

/// Percent decoding helper.
pub(crate) fn pdh(s: &str) -> Cow<'_, str> {
    percent_encoding::percent_decode_str(s).decode_utf8_lossy()
}

/// As defined in [the URL spec](https://url.spec.whatwg.org/#percent-encoded-bytes).
const QUERY_PERCENT_ENCODE_SET: percent_encoding::AsciiSet = percent_encoding::CONTROLS  .add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

pub(crate) fn qpeh(s: &str) -> Cow<'_, str> {
    percent_encoding::percent_encode(s.as_bytes(), &QUERY_PERCENT_ENCODE_SET).into()
}

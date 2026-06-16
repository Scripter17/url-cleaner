//! General hosts.

use crate::prelude::*;

/// If it [ends in a number](https://url.spec.whatwg.org/#ends-in-a-number-checker).
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert!(ends_in_a_number("123"));
/// assert!(ends_in_a_number("123."));
/// assert!(ends_in_a_number("a.123"));
/// assert!(ends_in_a_number("a.123."));
///
/// assert!(ends_in_a_number("0x1f"));
/// assert!(ends_in_a_number("0x1f."));
/// assert!(ends_in_a_number("a.0x1f"));
/// assert!(ends_in_a_number("a.0x1f."));
///
/// assert!(ends_in_a_number("01"));
/// assert!(ends_in_a_number("01."));
/// assert!(ends_in_a_number("a.01"));
/// assert!(ends_in_a_number("a.01."));
/// assert!(ends_in_a_number("09"));
///
/// assert!(!ends_in_a_number("0xZ"));
/// assert!(!ends_in_a_number("a"));
/// assert!(!ends_in_a_number("a."));
/// assert!(!ends_in_a_number(""));
/// ```
pub fn ends_in_a_number(value: &str) -> bool {
    last_is_a_number(value.my_trim_suffix("."))
}

/// If the last segment [`is_a_number`].
pub fn last_is_a_number(value: &str) -> bool {
    value.split('.').next_back().is_some_and(is_a_number)
}

/// If `value` would trigger [`ends_in_a_number`].
pub fn is_a_number(value: &str) -> bool {
    match value.as_bytes() {
        [] => false,
        [b'0', b'x' | b'X', x @ ..] => x.iter().all(u8::is_ascii_hexdigit),
        x => x.iter().all(u8::is_ascii_digit)
    }
}

/// Like [`ends_in_a_number`] but checks for the empty segment instead.
pub fn ends_in_empty(value: &str) -> bool {
    last_is_empty(value.my_trim_suffix("."))
}

/// Like [`last_is_a_number`] but checks for the empty segment instead.
pub fn last_is_empty(value: &str) -> bool {
    value.is_empty() || value.ends_with('.')
}

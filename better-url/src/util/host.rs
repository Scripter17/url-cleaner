//! Host stuff.

use crate::prelude::*;

/// Checks if a byte cannot appear in a percent-decoded and punicoded domain host.
///
/// Basically, if it can appear in a [parsed domain host](https://url.spec.whatwg.org/#host-parsing).
pub fn invalid_domain_byte(b: u8) -> bool {
    matches!(b, ..b'\x20' | b' ' | b'#' | b'/' | b':' | b'<' | b'>' | b'?' | b'@' | b'[' | b'\\' | b']' | b'^' | b'|' | b'%' | b'\x7f'..)
}

/// Checks if a host [ends in a number](https://url.spec.whatwg.org/#ends-in-a-number-checker).
/// # Examples
/// ```
/// use better_url::prelude::*;
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
///
/// assert!(!ends_in_a_number("09"));
/// assert!(!ends_in_a_number("0xZ"));
/// assert!(!ends_in_a_number("a"));
/// assert!(!ends_in_a_number("a."));
/// assert!(!ends_in_a_number(""));
/// ```
#[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
pub fn ends_in_a_number(s: &str) -> bool {
    let last = s.my_trim_suffix(".").rsplit('.').next().expect("???");

    match last.as_bytes() {
        b""                         => false,
        [b'0', b'x' | b'X', x @ ..] => x.iter().all(u8::is_ascii_hexdigit),
        [b'0', x @ ..]              => x.iter().all(|b| matches!(b, b'0'..=b'7')),
        x                           => x.iter().all(u8::is_ascii_digit),
    }
}

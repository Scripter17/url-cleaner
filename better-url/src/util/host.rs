//! Host stuff.

use crate::prelude::*;

/// <https://url.spec.whatwg.org/#forbidden-domain-code-point>.
pub(crate) const FORBIDDEN_HOST_BYTE: AsciiSet = AsciiSet(0).add_many(b"\x00\t\n\r #/:<>?@[\\]^|");

/// <https://url.spec.whatwg.org/#forbidden-domain-code-point>.
pub(crate) const FORBIDDEN_DOMAIN_BYTE: AsciiSet = AsciiSet(FORBIDDEN_HOST_BYTE.0 | C0.0).add(b'%').add(b'\x7f');

/// If it's a [forbidden host byte](https://url.spec.whatwg.org/#forbidden-domain-code-point).
pub fn invalid_host_byte(b: u8) -> bool {
    FORBIDDEN_HOST_BYTE.contains(b)
}

/// If it's a [forbidden domain byte](https://url.spec.whatwg.org/#forbidden-domain-code-point).
pub fn invalid_domain_byte(b: u8) -> bool {
    FORBIDDEN_DOMAIN_BYTE.contains(b)
}

/// Checks if a host [ends in a number](https://url.spec.whatwg.org/#ends-in-a-number-checker).
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
///
/// assert!(!ends_in_a_number("09"));
/// assert!(!ends_in_a_number("0xZ"));
/// assert!(!ends_in_a_number("a"));
/// assert!(!ends_in_a_number("a."));
/// assert!(!ends_in_a_number(""));
/// ```
#[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
pub fn ends_in_a_number(s: &str) -> bool {
    parse_ipv4_num(s.my_trim_suffix(".").split('.').next_back().expect("???")).is_some()
}

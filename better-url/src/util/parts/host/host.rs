//! General hosts.

/// [`bytes_ends_in_a_number`].
pub fn ends_in_a_number(value: &str) -> bool {
    bytes_ends_in_a_number(value.as_bytes())
}

/// [`bytes_ends_in_empty`].
pub fn ends_in_empty(value: &str) -> bool {
    bytes_ends_in_empty(value.as_bytes())
}

/// [`bytes_last_is_a_number`].
pub fn last_is_a_number(value: &str) -> bool {
    bytes_last_is_a_number(value.as_bytes())
}

/// [`bytes_last_is_empty`].
pub fn last_is_empty(value: &str) -> bool {
    bytes_last_is_empty(value.as_bytes())
}

/// [`bytes_is_a_number`].
pub fn is_a_number(value: &str) -> bool {
    bytes_is_a_number(value.as_bytes())
}



/// If the percent decoded and UTS46 mapped and normalized input [ends in a number](https://url.spec.whatwg.org/#ends-in-a-number-checker).
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert!( bytes_ends_in_a_number(b"123"));
/// assert!( bytes_ends_in_a_number(b"123."));
/// assert!( bytes_ends_in_a_number(b"a.123"));
/// assert!( bytes_ends_in_a_number(b"a.123."));
///
/// assert!( bytes_ends_in_a_number(b"0x1f"));
/// assert!( bytes_ends_in_a_number(b"0x1f."));
/// assert!( bytes_ends_in_a_number(b"a.0x1f"));
/// assert!( bytes_ends_in_a_number(b"a.0x1f."));
///
/// assert!( bytes_ends_in_a_number(b"01"));
/// assert!( bytes_ends_in_a_number(b"01."));
/// assert!( bytes_ends_in_a_number(b"a.01"));
/// assert!( bytes_ends_in_a_number(b"a.01."));
/// assert!( bytes_ends_in_a_number(b"09"));
///
/// assert!(!bytes_ends_in_a_number(b"0xZ"));
/// assert!(!bytes_ends_in_a_number(b"a"));
/// assert!(!bytes_ends_in_a_number(b"a."));
/// assert!(!bytes_ends_in_a_number(b""));
/// ```
pub fn bytes_ends_in_a_number(value: &[u8]) -> bool {
    match value {
        [value @ .., b'.'] | value => bytes_last_is_a_number(value)
    }
}

/// If the percent decoded and UTS46 mapped and normalized input ends in an empty segment.
pub fn bytes_ends_in_empty(value: &[u8]) -> bool {
    match value {
        [value @ .., b'.'] | value => bytes_last_is_empty(value)
    }
}

use crate::prelude::*;

/// Possible last bytes for [`bytes_last_is_a_number`] to return [`true`].
const THING: ByteSet = ByteSet::new().add_many(b"0123456789abcdefx");

/// If the percent decoded and UTS46 mapped and normalized input's last segment is a number.
pub fn bytes_last_is_a_number(value: &[u8]) -> bool {
    match value {
        &[.., b] if THING.contains(b) => {
            let i = value.iter().rposition(|&b| b == b'.').map_or(0, |i| i + 1);
            bytes_is_a_number(unsafe {value.get_unchecked(i..)})
        },
        _ => false
    }
}

/// If the percent decoded and UTS46 mapped and normalized input's last segment is empty.
pub fn bytes_last_is_empty(value: &[u8]) -> bool {
    matches!(value, [] | [.., b'.'])
}

/// If the percent decoded and UTS46 mapped and normalized input's is a number that triggers the IPv4 host parser.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert!(!bytes_is_a_number(b""             ));
/// assert!( bytes_is_a_number(b"0"            ));
/// assert!( bytes_is_a_number(b"10"           ));
/// assert!( bytes_is_a_number(b"1000000000000"));
/// assert!(!bytes_is_a_number(b"a0"           ));
/// assert!(!bytes_is_a_number(b"a000000000000"));
///
/// assert!( bytes_is_a_number(b"0x"             ));
/// assert!( bytes_is_a_number(b"0x0"            ));
/// assert!( bytes_is_a_number(b"0x10"           ));
/// assert!( bytes_is_a_number(b"0x1000000000000"));
/// assert!( bytes_is_a_number(b"0xa0"           ));
/// assert!( bytes_is_a_number(b"0xa000000000000"));
/// ```
pub fn bytes_is_a_number(value: &[u8]) -> bool {
    match value {
        [                  ] => false,
        [b'0', b'x', x @ ..] => x.iter().all(u8::is_ascii_hexdigit),
        x                    => x.iter().all(u8::is_ascii_digit   ),
    }
}

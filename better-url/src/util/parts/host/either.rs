//! General hosts.

use crate::prelude::*;

/// Possible last bytes for a domain segment that [`is_a_number`].
const NUM_SEGMENT_LASTS: ByteSet = ByteSet::new().add_many(b"0123456789abcdefx");

/** [`bytes_ends_in_a_number`]. **/ pub fn ends_in_a_number(value: &str) -> bool {bytes_ends_in_a_number(value.as_bytes())}
/** [`bytes_ends_in_empty`].    **/ pub fn ends_in_empty   (value: &str) -> bool {bytes_ends_in_empty   (value.as_bytes())}
/** [`bytes_last_is_a_number`]. **/ pub fn last_is_a_number(value: &str) -> bool {bytes_last_is_a_number(value.as_bytes())}
/** [`bytes_last_is_empty`].    **/ pub fn last_is_empty   (value: &str) -> bool {bytes_last_is_empty   (value.as_bytes())}
/** [`bytes_is_a_number`].      **/ pub fn is_a_number     (value: &str) -> bool {bytes_is_a_number     (value.as_bytes())}



/// If, ignoring a trailing dot, the percent decoded and UTS46 mapped and normalized input's last dot delimited segment is a number.
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
/// assert!( bytes_ends_in_a_number(b"0x"));
/// assert!( bytes_ends_in_a_number(b"0x."));
/// assert!( bytes_ends_in_a_number(b"a.0x"));
/// assert!( bytes_ends_in_a_number(b"a.0x."));
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

/// If, ignoring a trailing dot, the percent decoded and UTS46 mapped and normalized input's last dot delimited segment is empty.
pub fn bytes_ends_in_empty(value: &[u8]) -> bool {
    match value {
        [value @ .., b'.'] | value => bytes_last_is_empty(value)
    }
}

/// If the percent decoded and UTS46 mapped and normalized input's last dot delimited segment is a number.
pub fn bytes_last_is_a_number(value: &[u8]) -> bool {
    match value {
        &[.., b] if NUM_SEGMENT_LASTS.contains(b) => {
            let i = value.memrchr(b'.').map_or(0, |i| i + 1);
            bytes_is_a_number(unsafe {value.get_unchecked(i..)})
        },
        _ => false
    }
}

/// If the percent decoded and UTS46 mapped and normalized input's last dot delimited segment is empty.
pub fn bytes_last_is_empty(value: &[u8]) -> bool {
    matches!(value, [] | [.., b'.'])
}

/// If the value "is a number" for purposes of distinguishing between IPv4 hosts and domain hosts.
///
/// Assumes the input has already been percent decoded and UTS46 normalized.
///
/// Specifically, `^0x[0-9a-f]|[0-9]+$`
///
/// Per the spec, even if a number is too big to be an IPv4 segment (such as 99999999999999999), it still counts as a number for IPv4 parsing.
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

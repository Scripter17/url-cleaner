//! IPv4 stuff.

use std::net::Ipv4Addr;

use crate::prelude::*;

/// Parse an IPv4 number.
pub fn parse_ipv4_num(s: &str) -> Option<u32> {
    match s.as_bytes() {
        b"0x" | b"0X" | b"0"    => Some(0),
        [b'0', b'x' | b'X', ..] => u32::from_str_radix(&s[2..], 16).ok(),
        [b'0',  ..]             => u32::from_str_radix(&s[1..],  8).ok(),
        _                       => s.parse().ok()
    }
}

/// Parse an IPv4 byte.
pub fn parse_ipv4_byte(s: &str) -> Option<u8> {
    match s.as_bytes() {
        b"0x" | b"0X" | b"0"    => Some(0),
        [b'0', b'x' | b'X', ..] => u8::from_str_radix(&s[2..], 16).ok(),
        [b'0',  ..]             => u8::from_str_radix(&s[1..],  8).ok(),
        _                       => s.parse().ok()
    }
}

/// [Parse an IPv4 host](https://url.spec.whatwg.org/#concept-ipv4-parser).
/// # Errors
/// If parsing fails, returns the error [`InvalidIpv4Host`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// let tests = [
///     "1.2.3.4", "255.255.255.255",
///     "0x1.0x2.0X3.0X4", "0xff.0xfF.0xFf.0xFF",
///     "01.02.03.04", "0400.0400.0400.0400",
///
///     "1", "1.2", "1.2.3", "1.2.3.4", "1.2.3.4.5",
///     "0x1", "0x1.0x2", "0x1.0x2.0x3", "0x1.0x2.0x3.0x4", "0x1.0x2.0x3.0x4.0x5",
///     "01", "01.02", "01.02.03", "01.02.03.04", "01.02.03.04.05",
///
///     "", ".", "1..2",
///
///     "100000000000000000",
///     "100000000000000000.100000000000000000",
///     "100000000000000000.100000000000000000.100000000000000000",
///     "100000000000000000.100000000000000000.100000000000000000.100000000000000000",
/// ];
///
/// for s in tests {
///     match (parse_ipv4_host(s), url::Host::parse(s)) {
///         (Some(a), Ok(url::Host::Ipv4(b))) => assert_eq!(a, b, "{s}"),
///         (None, Err(_) | Ok(url::Host::Domain(_) | url::Host::Ipv6(_))) => {},
///         (a, b) => panic!("{s} -> {a:?} {b:?}")
///     }
/// }
/// ```
pub fn parse_ipv4_host(s: &str) -> Option<Ipv4Addr> {
    let mut parts = s.my_trim_suffix(".").split('.');

    let mut x = parts.next_back().and_then(parse_ipv4_num)?;

    if let Some(l) = parts.next() {x += (parse_ipv4_byte(l)? as u32) << 24;}
    if let Some(l) = parts.next() {x += (parse_ipv4_byte(l)? as u32) << 16;}
    if let Some(l) = parts.next() {x += (parse_ipv4_byte(l)? as u32) <<  8;}

    parts.next().is_none().then_some(x.into())
}

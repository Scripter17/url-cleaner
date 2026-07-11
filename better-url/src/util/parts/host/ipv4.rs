//! IPv4 hosts.

use std::fmt::Write;
use std::net::Ipv4Addr;

use crate::prelude::*;

/// [Parse an IPv4 number](https://url.spec.whatwg.org/#ipv4-number-parser).
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(parse_ipv4_num("0"), Some(0));
/// assert_eq!(parse_ipv4_num("9"), Some(9));
///
/// assert_eq!(parse_ipv4_num("a"  ), None    );
/// assert_eq!(parse_ipv4_num("A"  ), None    );
/// assert_eq!(parse_ipv4_num("0x" ), Some( 0));
/// assert_eq!(parse_ipv4_num("0X" ), Some( 0));
/// assert_eq!(parse_ipv4_num("0xa"), Some(10));
/// assert_eq!(parse_ipv4_num("0xA"), Some(10));
///
/// assert_eq!(parse_ipv4_num("07"), Some(7));
/// assert_eq!(parse_ipv4_num("08"), None   );
/// ```
pub fn parse_ipv4_num(value: &str) -> Option<u32> {
    match value.as_bytes() {
        b"0x" | b"0X" | b"0"    => Some(0),
        [b'0', b'x' | b'X', ..] => u32::from_str_radix(&value[2..], 16).ok(),
        [b'0', ..]              => u32::from_str_radix(&value[1..],  8).ok(),
        _                       => value.parse().ok()
    }
}

/// [Parse an IPv4 host](https://url.spec.whatwg.org/#concept-ipv4-parser).
///
/// See [`make_ipv4_host`] to also get a string.
pub fn parse_ipv4_host(value: &str) -> Option<std::net::Ipv4Addr> {
    let mut parts = SplitDots(Some(value.my_trim_suffix(".")));

    let last = parse_ipv4_num(parts.next_back()?)?;

    let mut ret = last;

    if let Some(l) = parts.next() {if last > 0xffffff {None?;} else {ret += (u8::try_from(parse_ipv4_num(l)?).ok()? as u32) << 24;}}
    if let Some(l) = parts.next() {if last > 0xffff   {None?;} else {ret += (u8::try_from(parse_ipv4_num(l)?).ok()? as u32) << 16;}}
    if let Some(l) = parts.next() {if last > 0xff     {None?;} else {ret += (u8::try_from(parse_ipv4_num(l)?).ok()? as u32) <<  8;}}

    if parts.next().is_some() {
        None?;
    }

    Some(ret.into())
}

/// Parse and normalize an IPv4 host.
/// # Errors
/// If `value` is not a valid IPv4 host, retruns the error [`InvalidIpv4Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// // Normalized inputs aren't allocated.
///
/// let (changed, _, host) = make_ipv4_host("127.0.0.1").unwrap();
///
/// assert!(!changed);
/// assert_matches!(host, Cow::Borrowed("127.0.0.1"));
///
/// // Only unnormalized inputs are allocated.
///
/// let (changed, _, host) = make_ipv4_host("127.0.1").unwrap();
///
/// assert!(changed);
/// assert_matches!(host, Cow::Owned(x) if x == "127.0.0.1");
/// ```
pub fn make_ipv4_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Ipv4Addr, Cow<'a, str>), InvalidIpv4Host> {
    let value = value.into();

    let addr = parse_ipv4_host(&value).ok_or(InvalidIpv4Host)?;

    let mut normalizer = Normalizer::new(value);
    write!(normalizer, "{addr}").expect("???");

    let (changed, host) = normalizer.done();

    Ok((changed, addr, host))
}

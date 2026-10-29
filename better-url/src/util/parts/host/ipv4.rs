//! IPv4 hosts.

use std::fmt::Write;
use std::net::Ipv4Addr;

use crate::prelude::*;

/// Make an [`Ipv4Host`].
///
/// Please note that you can also make an [`Ipv4Host`] directly from an existing [`Ipv4Addr`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidIpv4Host`].
///
/// If [`parse_ipv4_addr`] returns an error, returns the error [`InvalidIpv4Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// assert_matches!(make_ipv4_host(    "127.0.0.1").unwrap(), (false, Cow::Borrowed(x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host(        "127.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host(     "0177.0x1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host(     "0x7f.0X1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host(   "0x7f000001").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host(     "0x7%46.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host("%E2%91%a027.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// ```
pub fn make_ipv4_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, Ipv4HostDetails), InvalidIpv4Host> {
    let (a, value) = try_percent_decode (value).map_err(|_| InvalidIpv4Host)?;
    let (b, value) = uts46_map_normalize(value);

    let addr = parse_ipv4_addr(&value).ok_or(InvalidIpv4Host)?;

    let mut normalizer = Normalizer::new(value);
    write!(normalizer, "{addr}").expect("???");

    let (c, value) = normalizer.done();

    Ok((a || b || c, value, addr.into()))
}

/// Make an [`Ipv4Host`] from bytes.
///
/// Please note that you can also make an [`Ipv4Host`] directly from an existing [`Ipv4Addr`].
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, returns the error [`InvalidIpv4Host`].
///
/// If [`parse_ipv4_addr`] returns an error, returns the error [`InvalidIpv4Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// assert_matches!(make_ipv4_host_bytes(    b"127.0.0.1").unwrap(), (false, Cow::Borrowed(x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(        b"127.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(     b"0177.0x1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(     b"0x7f.0X1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(   b"0x7f000001").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(     b"0x7%46.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// assert_matches!(make_ipv4_host_bytes(b"%E2%91%a027.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
///
/// // Not valid UTF-8, but when percent decoded it becomes valid.
/// assert_matches!(make_ipv4_host_bytes(b"%E2\x91%a027.1").unwrap(), (true , Cow::Owned   (x), _) if x == "127.0.0.1");
/// ```
pub fn make_ipv4_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, Ipv4HostDetails), InvalidIpv4Host> {
    let (a, value) = try_percent_decode_bytes(value).map_err(|_| InvalidIpv4Host)?;
    let (b, value) = uts46_map_normalize     (value);

    let addr = parse_ipv4_addr(&value).ok_or(InvalidIpv4Host)?;

    let mut normalizer = Normalizer::new(value);
    write!(normalizer, "{addr}").expect("???");

    let (c, value) = normalizer.done();

    Ok((a || b || c, value, addr.into()))
}



/// [The IPv4 parser](https://url.spec.whatwg.org/#concept-ipv4-parser).
///
/// Expects the input to be percent decoded and UTS46 mapped and normalized, but not necessarily an [`Ipv4Host`] literal.
///
/// See [`make_ipv4_host`] to make an [`Ipv4Host`].
pub fn parse_ipv4_addr(value: &str) -> Option<Ipv4Addr> {
    let mut parts = SplitDots(Some(value.my_trim_suffix(".")));

    let last = parse_ipv4_u32(parts.next_back()?)?;

    let mut ret = last;

    if let Some(l) = parts.next() {if last > 0xffffff {None?;} else {ret += (parse_ipv4_u8(l)? as u32) << 24;}}
    if let Some(l) = parts.next() {if last > 0xffff   {None?;} else {ret += (parse_ipv4_u8(l)? as u32) << 16;}}
    if let Some(l) = parts.next() {if last > 0xff     {None?;} else {ret += (parse_ipv4_u8(l)? as u32) <<  8;}}

    if parts.next().is_some() {
        None?;
    }

    Some(ret.into())
}



/// [Parse an IPv4 number](https://url.spec.whatwg.org/#ipv4-number-parser).
///
/// For segments other than the last one, see [`parse_ipv4_u8`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(parse_ipv4_u32( "0"  ), Some( 0));
/// assert_eq!(parse_ipv4_u32( "9"  ), Some( 9));
/// assert_eq!(parse_ipv4_u32("+9"  ), None    );
///
/// assert_eq!(parse_ipv4_u32("a"   ), None    );
/// assert_eq!(parse_ipv4_u32("A"   ), None    );
/// assert_eq!(parse_ipv4_u32("0x"  ), Some( 0));
/// assert_eq!(parse_ipv4_u32("0X"  ), Some( 0));
/// assert_eq!(parse_ipv4_u32("0xa" ), Some(10));
/// assert_eq!(parse_ipv4_u32("0xA" ), Some(10));
/// assert_eq!(parse_ipv4_u32("+0xA"), None    );
/// assert_eq!(parse_ipv4_u32("0x+A"), None    );
///
/// assert_eq!(parse_ipv4_u32("07"  ), Some( 7));
/// assert_eq!(parse_ipv4_u32("08"  ), None    );
/// assert_eq!(parse_ipv4_u32("+07" ), None    );
/// assert_eq!(parse_ipv4_u32("0+7" ), None    );
/// ```
pub fn parse_ipv4_u32(value: &str) -> Option<u32> {
    if value.contains('+') {
        return None;
    }

    match value.as_bytes() {
        b"0x" | b"0X" | b"0"    => Some(0),
        [b'0', b'x' | b'X', ..] => u32::from_str_radix(&value[2..], 16).ok(),
        [b'0', ..]              => u32::from_str_radix(&value[1..],  8).ok(),
        _                       => value.parse().ok(),
    }
}

/// [`parse_ipv4_u32`] but returns a [`u8`] for the non-last segments.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(parse_ipv4_u8( "0"  ), Some( 0));
/// assert_eq!(parse_ipv4_u8( "9"  ), Some( 9));
/// assert_eq!(parse_ipv4_u8("+9"  ), None    );
///
/// assert_eq!(parse_ipv4_u8("a"   ), None    );
/// assert_eq!(parse_ipv4_u8("A"   ), None    );
/// assert_eq!(parse_ipv4_u8("0x"  ), Some( 0));
/// assert_eq!(parse_ipv4_u8("0X"  ), Some( 0));
/// assert_eq!(parse_ipv4_u8("0xa" ), Some(10));
/// assert_eq!(parse_ipv4_u8("0xA" ), Some(10));
/// assert_eq!(parse_ipv4_u8("+0xA"), None    );
/// assert_eq!(parse_ipv4_u8("0x+A"), None    );
///
/// assert_eq!(parse_ipv4_u8("07"  ), Some( 7));
/// assert_eq!(parse_ipv4_u8("08"  ), None    );
/// assert_eq!(parse_ipv4_u8("+07" ), None    );
/// assert_eq!(parse_ipv4_u8("0+7" ), None    );
/// ```
pub fn parse_ipv4_u8(value: &str) -> Option<u8> {
    if value.contains('+') {
        return None;
    }

    match value.as_bytes() {
        b"0x" | b"0X" | b"0"    => Some(0),
        [b'0', b'x' | b'X', ..] => u8::from_str_radix(&value[2..], 16).ok(),
        [b'0', ..]              => u8::from_str_radix(&value[1..],  8).ok(),
        _                       => value.parse().ok()
    }
}


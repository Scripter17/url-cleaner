//! [`Ipv6Host`] stuff.

use std::fmt::Write;
use std::net::Ipv6Addr;

use crate::prelude::*;

/// Parse and normalize an IPv6 host.
/// # Errors
/// If `value` is not a valid IPv6 host, returns the error [`InvalidIpv6Host`].
/// # Examples
/// ```
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// // Normalized inputs aren't allocated.
///
/// let (changed, _, host) = make_ipv6_host("[1:2::3]").unwrap();
///
/// assert!(!changed);
/// assert_matches!(host, Cow::Borrowed("[1:2::3]"));
///
/// // Only unnormalized inputs are allocated.
///
/// let (changed, _, host) = make_ipv6_host("[1:2:0::3]").unwrap();
///
/// assert!(changed);
/// assert_matches!(host, Cow::Owned(x) if x == "[1:2::3]");
/// ```
pub fn make_ipv6_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Ipv6Addr, Cow<'a, str>), InvalidIpv6Host> {
    let value = value.into();

    let addr = value.strip_prefix('[').ok_or(InvalidIpv6Host)?.strip_suffix(']').ok_or(InvalidIpv6Host)?.parse().map_err(|_| InvalidIpv6Host)?;

    let mut normalizer = Normalizer::new(value);
    write!(normalizer, "[{addr}]").expect("???");

    let (changed, host) = normalizer.done();

    Ok((changed, addr, host))
}

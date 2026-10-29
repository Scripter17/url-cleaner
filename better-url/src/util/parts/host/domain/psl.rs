//! [`get_domain_suffix`].

use psl::Psl;

use crate::prelude::*;

/// Get the suffix of a [`DomainHost`] literal.
///
/// Doesn't actually check if `value` a [`DomainHost`] literal and will give nonsense results if it isn't.
///
/// # Examples
///
/// ```
/// use better_url::util::*;
///
/// assert_eq!(get_domain_suffix("example.com"    ), "com"  );
/// assert_eq!(get_domain_suffix("example.com."   ), "com"  );
/// assert_eq!(get_domain_suffix("example.com.."  ), ""     );
/// assert_eq!(get_domain_suffix("example.co.uk"  ), "co.uk");
/// assert_eq!(get_domain_suffix("example.co.uk." ), "co.uk");
/// assert_eq!(get_domain_suffix("example.co.uk.."), ""     );
/// ```
pub fn get_domain_suffix(value: &str) -> &str {
    let value = value.my_trim_suffix(".");

    let len = psl::List.find(SplitDots(Some(value)).rev().map(str::as_bytes)).len;

    unsafe {value.get_unchecked(value.len() - len ..)}
}

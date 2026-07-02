//! Encoding.

use crate::prelude::*;

/// Percent decode, UTS46 normalize, and encode a domain.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_to_ascii`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`percent_decoded_domain_to_ascii`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(domain_to_ascii("abc.com"         ).unwrap().1, "abc.com"                         );
/// assert_eq!(domain_to_ascii("abc.com."        ).unwrap().1, "abc.com."                        );
/// assert_eq!(domain_to_ascii("Αθήνα.abc.Αθήνα" ).unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a" );
/// assert_eq!(domain_to_ascii("Αθήνα.abc.Αθήνα.").unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a.");
///
/// domain_to_ascii("abc.123").unwrap_err();
/// ```
pub fn domain_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = percent_decoded_domain_to_ascii(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and encode a percent decoded domain.
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_to_ascii`].
/// # Errors
/// If the call to [`normalized_domain_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 normalized domain.
/// # Errors
/// If the call to [`normalized_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
///
/// If the call to [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
pub fn normalized_domain_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = normalized_domain_segments_to_ascii(value)?;

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?
    }

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    Ok((changed, domain))
}

/// Encode a percent decoded and UTS46 normalized domain.
/// # Errors
/// If the call to [`strict_normalized_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
///
/// If the call to [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
pub fn strict_normalized_domain_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = strict_normalized_domain_segments_to_ascii(value)?;

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?
    }

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    Ok((changed, domain))
}

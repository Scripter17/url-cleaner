//! Decoding.

use crate::prelude::*;

/// Percent decode, UTS46 normalize, and decode a domain.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_to_unicode`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, returns the error [`InvalidDomainHost`].
///
/// If the call to [`normalized_domain_to_unicode`] returns an error, that error is returned.
pub fn domain_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = percent_decoded_domain_to_unicode(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and encode a percent decoded domain.
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_to_unicode`].
/// # Errors
/// If the call to [`normalized_domain_to_unicode`] returns an error, that error is returned.
pub fn percent_decoded_domain_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_to_unicode(value)?;
    Ok((a || b, value))
}

/// Decode a percent encoded and UTS46 normalized domain.
/// # Errors
/// If the call to [`normalized_domain_segments_to_unicode`] returns an error, that error is returned.
///
/// If the call to [`ends_in_a_number`] returns [`true`], reutrns the error [`InvalidDomainHost`].
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
pub fn normalized_domain_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = normalized_domain_segments_to_unicode(value)?;

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?;
    }

    Ok((changed, domain))
}

/// Strictly decode a UTS46 normalized domain.
/// # Errors
/// If the call to [`strict_normalized_domain_segments_to_unicode`] returns an error, that error is returned.
///
/// If the call to [`ends_in_a_number`] returns [`true`], reutrns the error [`InvalidDomainHost`].
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
pub fn strict_normalized_domain_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = strict_normalized_domain_segments_to_unicode(value)?;

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?;
    }

    Ok((changed, domain))
}

/// [`unchecked_normalized_domain_segments_to_unicode`]
/// # Panics
/// If the call to [`unchecked_normalized_domain_segments_to_unicode`] panics, that panic is continued.
pub fn unchecked_normalized_domain_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unchecked_normalized_domain_segments_to_unicode(value)
}

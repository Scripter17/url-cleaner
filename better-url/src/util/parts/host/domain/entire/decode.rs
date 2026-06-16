//! Decoding.

use crate::prelude::*;

/// [`uts46_normalize`] + [`decode_normalized_domain`].
/// # Errors
/// If the call to [`decode_normalized_domain`] returns an error, that error is returned.
pub fn decode_domain<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainHost> {
    let (a, value              ) = uts46_normalize(value);
    let (b, value, bidi_details) = decode_normalized_domain(value)?;
    Ok((a || b, value, bidi_details))
}

/// Decode a UTS46 normalized domain.
/// # Errors
/// If the call to [`decode_normalized_domain_segments`] returns an error, that error is returned.
///
/// If the call to [`ends_in_a_number`] returns [`true`], reutrns the error [`InvalidDomainHost`].
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
pub fn decode_normalized_domain<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainHost> {
    let (changed, domain, mut bidi_details) = decode_normalized_domain_segments(value)?;

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?;
    }

    if domain.ends_with('.') {
        bidi_details.pop();
    }

    Ok((changed, domain, bidi_details))
}

/// [`decode_normalized_domain_segments_unchecked`]
///
/// Notably also skips making a [`BidiDetails`].
/// # Panics
/// If the call to [`decode_normalized_domain_segments_unchecked`] panics, that panic is continued.
pub fn decode_normalized_domain_unchecked<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    decode_normalized_domain_segments_unchecked(value)
}

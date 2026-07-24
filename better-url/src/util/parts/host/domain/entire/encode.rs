//! Encoding.

use crate::prelude::*;

/// Encode a domain host.
///
/// If you know your input will be percent decoded, see [`encode_percent_decoded_domain_host`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, returns the error [`InvalidDomainHost`].
///
/// If the call to [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = encode_percent_decoded_domain_host(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded domain host.
///
/// If you know your input will be UTS46 mapped and normalized, see [`encode_normalized_domain_host`].
/// # Errors
/// If the call to [`encode_normalized_domain_segments`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = encode_normalized_domain_host(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 mapped and normalized domain host.
/// # Errors
/// If the call to [`encode_normalized_domain_segments`] returns an error, that error is returned.
///
/// If the resulting domain is empty or larger than [`u32::MAX`] bytes, returns the error [`InvalidDomainHost`].
///
/// If the resulting domain [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
pub fn encode_normalized_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = encode_normalized_domain_segments(value)?;

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?
    }

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    Ok((changed, domain))
}

//! Encoding.

use crate::prelude::*;

/// Encode a domain host.
///
/// If you know your input will be percent decoded, see [`encode_percent_decoded_domain_host`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidDomainHost`].
///
/// If [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = encode_percent_decoded_domain_host(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded domain host.
///
/// If you know your input will be UTS46 mapped and normalized, see [`encode_normalized_domain_host`].
/// # Errors
/// If [`encode_normalized_domain_segments`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = encode_normalized_domain_host(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 mapped and normalized domain host.
/// # Errors
/// If [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
pub fn encode_normalized_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();

    if ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    encode_not_eian_domain_host(value)
}

/// Encode a percent decoded and UTS46 mapped and normalized domain host that does not [`ends_in_a_number`].
/// # Errors
/// If [`encode_normalized_domain_segments`] returns an error, that error is returned.
///
/// If the resulting domain is empty or larger than [`u32::MAX`] bytes, returns the error [`InvalidDomainHost`].
///
/// If the resulting domain [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
pub fn encode_not_eian_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = encode_normalized_domain_segments(value)?;

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?
    }

    Ok((changed, domain))
}

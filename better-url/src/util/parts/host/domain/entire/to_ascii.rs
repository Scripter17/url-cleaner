//! Encoding.

use crate::prelude::*;

/// Encode a domain host.
///
/// If you know your input will be percent decoded, see [`percent_decoded_domain_host_to_ascii`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidDomainHost`].
///
/// If [`percent_decoded_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = percent_decoded_domain_host_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a domain host from bytes.
///
/// If you know your input will be percent decoded, see [`percent_decoded_domain_host_to_ascii`].
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, returns the error [`InvalidDomainHost`].
///
/// If [`percent_decoded_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_host_bytes_to_ascii<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode_bytes(value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = percent_decoded_domain_host_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded domain host.
///
/// If you know your input will be UTS46 mapped and normalized, see [`normalized_domain_host_to_ascii`].
/// # Errors
/// If [`normalized_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_host_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 mapped and normalized domain host.
/// # Errors
/// If [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
pub fn normalized_domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();

    if ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    not_eian_domain_host_to_ascii(value)
}

/// Encode a percent decoded and UTS46 mapped and normalized domain host that does not [`ends_in_a_number`].
/// # Errors
/// If [`normalized_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If the resulting domain [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
pub fn not_eian_domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (changed, domain) = normalized_domain_segments_to_ascii(value)?;

    if domain.is_empty() {
        Err(InvalidDomainHost)?
    }

    Ok((changed, domain))
}

//! Decoding.

use crate::prelude::*;

/// Decode a domain host literal.
/// # Errors
/// If `value` is empty or larger than [`u32::MAX`] bytes, returns the error [`InvalidDomainHost`].
///
/// If `value` [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
///
/// If the call to [`decode_domain_segments`] returns an error, that error is returned.
pub fn decode_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();

    if value.is_empty() || value.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?;
    }

    if ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    let (changed, domain) = decode_domain_segments(value)?;

    Ok((changed, domain))
}

/// Decode a domain host literal without any validity checks.
pub fn unchecked_decode_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unchecked_decode_domain_segments(value)
}

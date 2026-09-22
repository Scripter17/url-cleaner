//! Decoding.

use crate::prelude::*;

/// Decode a domain host literal.
/// # Errors
/// If `value` is empty or larger than [`u32::MAX`] bytes, returns the error [`InvalidDomainHost`].
///
/// If `value` [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
///
/// If [`domain_segments_to_unicode`] returns an error, that error is returned.
pub fn domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();

    if value.is_empty() || value.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?;
    }

    if ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    let (changed, domain) = domain_segments_to_unicode(value)?;

    Ok((changed, domain))
}

/// Decode a domain host literal without any validity checks.
pub fn unchecked_domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unchecked_domain_segments_to_unicode(value)
}

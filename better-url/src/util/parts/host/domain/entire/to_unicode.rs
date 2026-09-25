//! Decoding.

use crate::prelude::*;

/// Decode a domain host literal.
/// # Errors
/// If `value` [`ends_in_a_number`], returns the error [`InvalidDomainHost`].
///
/// If [`domain_segments_to_unicode`] returns an error, that error is returned.
pub fn domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let value = value.into();

    if value.is_empty() {
        Err(InvalidDomainHost)?;
    }

    if ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    Ok(domain_segments_to_unicode(value)?)
}

/// Decode a domain host literal without any validity checks.
pub fn unchecked_domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unchecked_domain_segments_to_unicode(value)
}

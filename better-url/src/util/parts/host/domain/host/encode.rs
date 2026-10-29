//! Encoding.

use crate::prelude::*;

/// Make a [`DomainHost`].
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`uts46_domain_host_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainHost`], returns the error [`InvalidDomainHost`].
pub fn make_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, DomainHostDetails), InvalidDomainHost> {
    let (a, value       ) = try_percent_decode          (value).map_err(|_| InvalidDomainHost)?;
    let (b, value, class) = uts46_classify_map_normalize(value);

    // Pretty sure non-empty input can become empty after [`uts46_classify_map_normalize`].
    if value.is_empty() {
        Err(InvalidDomainHost)?;
    }

    // If the input contained a [`FORBIDDEN_DOMAIN_HOST_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_HOST_INPUT`].
    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainHost)?;
    }

    // If the input contained Unicode and/or a number, check if the output [`ends_in_a_number`].
    if class & 0b0000_0110 != 0b0000_0000 && ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    // If the output contained no [`INVALID_DOMAIN_HOST_INPUT`], no Unicode, and no ASCII numbers, and it's not empty (checked above), then it's a valid [`DomainHost`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        let details = DomainHostDetails::parse_unchecked(&value);

        return Ok((a || b, value, details));
    }

    let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

    let c = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // If the input wasn't UTS46, it may in theory be possible for the output to contain a [`FORBIDDEN_DOMAIN_HOST_LITERAL`] byte not caught by the classification.
    if b && value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
        Err(InvalidDomainHost)?;
    }

    let details = DomainHostDetails::parse_unchecked(&value);

    Ok((a || b || c, value, details))
}

/// Make a [`DomainHost`] from bytes.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`uts46_domain_host_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainHost`], returns the error [`InvalidDomainHost`].
pub fn make_domain_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, DomainHostDetails), InvalidDomainHost> {
    let (a, value       ) = try_percent_decode_bytes    (value).map_err(|_| InvalidDomainHost)?;
    let (b, value, class) = uts46_classify_map_normalize(value);

    // Pretty sure non-empty input can become empty after [`uts46_classify_map_normalize`].
    if value.is_empty() {
        Err(InvalidDomainHost)?;
    }

    // If the input contained a [`FORBIDDEN_DOMAIN_HOST_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_HOST_INPUT`].
    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainHost)?;
    }

    // If the input contained Unicode and/or a number, check if the output [`ends_in_a_number`].
    if class & 0b0000_0110 != 0b0000_0000 && ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    // If the output contained no [`INVALID_DOMAIN_HOST_INPUT`], no Unicode, and no ASCII numbers, and it's not empty (checked above), then it's a valid [`DomainHost`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        let details = DomainHostDetails::parse_unchecked(&value);

        return Ok((a || b, value, details));
    }

    let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

    let c = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // If the input wasn't UTS46, it may in theory be possible for the output to contain a [`FORBIDDEN_DOMAIN_HOST_LITERAL`] byte not caught by the classification.
    if b && value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
        Err(InvalidDomainHost)?;
    }

    let details = DomainHostDetails::parse_unchecked(&value);

    Ok((a || b || c, value, details))
}

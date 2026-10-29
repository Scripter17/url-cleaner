//! Encoding.

use crate::prelude::*;

/// Make a [`DomainSegments`].
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`uts46_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainSegments`], returns the error [`InvalidDomainSegments`]
pub fn make_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value       ) = try_percent_decode          (value).map_err(|_| InvalidDomainSegments)?;
    let (b, value, class) = uts46_classify_map_normalize(value);

    // If the input contained a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`].
    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainSegments)?;
    }

    // If the output contained no [`INVALID_DOMAIN_SEGMENTS_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegments`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        return Ok((a || b, value));
    }

    let (raw, ascii) = uts46_domain_segments_to_ascii(value, false)?;

    let c = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // If the input wasn't UTS46, it may in theory be possible for the output to contain a [`FORBIDDEN_DOMAIN_SEGMENTS_LITERAL`] byte not caught by the classification.
    if b && value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENTS_LITERAL.contains(b)) {
        Err(InvalidDomainSegments)?;
    }

    Ok((a || b || c, value))
}

/// Make a [`DomainSegments`] from bytes.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`uts46_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainSegments`], returns the error [`InvalidDomainSegments`]
pub fn make_domain_segments_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value       ) = try_percent_decode_bytes    (value).map_err(|_| InvalidDomainSegments)?;
    let (b, value, class) = uts46_classify_map_normalize(value);

    // If the input contained a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`].
    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainSegments)?;
    }

    // If the output contained no [`INVALID_DOMAIN_SEGMENTS_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegments`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        return Ok((a || b, value));
    }

    let (raw, ascii) = uts46_domain_segments_to_ascii(value, false)?;

    let c = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // If the input wasn't UTS46, it may in theory be possible for the output to contain a [`FORBIDDEN_DOMAIN_SEGMENTS_LITERAL`] byte not caught by the classification.
    if b && value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENTS_LITERAL.contains(b)) {
        Err(InvalidDomainSegments)?;
    }

    Ok((a || b || c, value))
}

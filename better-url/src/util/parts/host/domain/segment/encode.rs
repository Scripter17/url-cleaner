//! Encoding.

use crate::prelude::*;

/// Encode a domain segment.
///
/// If you know your input will be percent decoded, see [`encode_percent_decoded_domain_segment`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`encode_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn encode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = encode_percent_decoded_domain_segment(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded domain segment.
///
/// If you know your input will be UTS46 mapped and normalized, see [`encode_normalized_domain_segment`].
/// # Errors
/// If the call to [`encode_normalized_domain_segment`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = encode_normalized_domain_segment(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 mapped and normalized domain segment.
/// # Errors
/// If `value` contains any ASCII byte in [`FORBIDDEN_DOMAIN_SEGMENT`], returns the error [`InvalidDomainSegment`].
///
/// If `value` is not ASCII:
///
/// - If the call to [`mostly_validate_domain_segment_unicode`] returns [`false`], returns the error [`InvalidDomainSegment`].
///
/// - If the call to [`BidiDetail::parse`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegment`].
///
/// - If the call to [`encode_punycode`] returns an error, returns the error [`InvalidDomainSegment`].
pub fn encode_normalized_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let value = value.into();

    if value.bytes().any(|b| b.is_ascii() && FORBIDDEN_DOMAIN_SEGMENT.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    if value.is_ascii() {
        return Ok((false, value));
    }

    if !mostly_validate_domain_segment_unicode(&value) {
        Err(InvalidDomainSegment)?;
    }

    if BidiDetail::parse(&value) == BidiDetail::ForceAscii {
        Err(InvalidDomainSegment)?;
    }

    let mut ret = encode_punycode(value).map_err(|_| InvalidDomainSegment)?;

    ret.to_mut().insert_str(0, "xn--");

    Ok((true, ret))
}

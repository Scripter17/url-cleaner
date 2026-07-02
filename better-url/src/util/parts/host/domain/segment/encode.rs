//! Encoding.

use crate::prelude::*;

/// Percent decode, UTS46 normalize, and encode a domain segment.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_segment_to_ascii`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`percent_decoded_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = percent_decoded_domain_segment_to_ascii(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and encode a percent decoded domain segment
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_segment_to_ascii`].
/// # Errors
/// If the call to [`normalized_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_segment_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 normalized domain segment.
/// # Errors
/// If the call to [`normalized_domain_segment_to_unicode`] returns an error, that error is returned.
pub fn normalized_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let value = value.into();

    if has_forbidden_domain_segment_byte(&value) {
        Err(InvalidDomainSegment)?;
    }

    if value.is_ascii() {
        Ok((false, value))
    } else {
        let (_, decoded) = normalized_domain_segment_to_unicode(value)?;
        let mut ret = encode_punycode(decoded).map_err(|TooLong| InvalidDomainSegment)?;
        ret.to_mut().insert_str(0, "xn--");
        Ok((true, ret))
    }
}



/// Strictly encode a percent decoded and UTS46 normalized domain segment.
/// # Errors
/// If the call to [`strict_normalized_domain_segment_to_unicode`] returns an error, that error is returned.
pub fn strict_normalized_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let value = value.into();

    if has_forbidden_domain_segment_byte(&value) {
        Err(InvalidDomainSegment)?;
    }

    if value.is_ascii() {
        let (_, _, bidi_detail) = strict_normalized_domain_segment_to_unicode(&*value)?;
        Ok((false, value, bidi_detail))
    } else {
        let (_, decoded, bidi_detail) = strict_normalized_domain_segment_to_unicode(value)?;
        let mut ret = encode_punycode(decoded).map_err(|TooLong| InvalidDomainSegment)?;
        ret.to_mut().insert_str(0, "xn--");
        Ok((true, ret, bidi_detail))
    }
}

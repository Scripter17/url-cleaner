//! Encoding.

use crate::prelude::*;

/// [`try_percent_decode`] + [`encode_percent_decoded_domain_segment`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`encode_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn encode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let (a, value             ) = try_percent_decode(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value, bidi_detail) = encode_percent_decoded_domain_segment(value)?;
    Ok((a || b, value, bidi_detail))
}

/// [`uts46_normalize`] + [`encode_normalized_domain_segment`]
/// # Errors
/// If the call to [`encode_normalized_domain_segment`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let (a, value             ) = uts46_normalize(value);
    let (b, value, bidi_detail) = encode_normalized_domain_segment(value)?;
    Ok((a || b, value, bidi_detail))
}

/// Encode a UTS46 normalize domain segment.
/// # Errors
/// If the call to [`decode_normalized_domain_segment`] returns an error, that error is returned.
pub fn encode_normalized_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let value = value.into();

    if value.is_ascii() {
        let (_, _, bidi_detail) = decode_normalized_domain_segment(&*value)?;
        // If `value` is ASCII and decoded properly then it's valid.
        Ok((false, value, bidi_detail))
    } else {
        let (_, decoded, bidi_detail) = decode_normalized_domain_segment(value)?;
        let mut ret = encode_punycode(decoded).map_err(|TooLong| InvalidDomainSegment)?;
        ret.to_mut().insert_str(0, "xn--");
        Ok((true, ret, bidi_detail))
    }
}

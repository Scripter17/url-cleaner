//! Encoding.

use crate::prelude::*;

/// Get [`DATA`].
const fn get_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        if FORBIDDEN_DOMAIN_SEGMENT_LITERAL.contains(i as u8) {
            ret[i as usize] = 1;
        }

        if FORBIDDEN_DOMAIN_SEGMENT_INPUT.contains(i as u8) {
            ret[i as usize] = 2;
        }

        i += 1;
    }

    ret
}

/// For each byte, `0` if a valid domain segment literal, `1` if a valid domain segment input, and `2` if an invalid domain segment input.
const DATA: [u8; 256] = get_data();

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
/// If `value` contains any ASCII byte in [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`], returns the error [`InvalidDomainSegment`].
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

    match value.bytes().fold(0, |acc, b| acc | DATA[b as usize]) {
        0 => Ok((false, value)),
        1 => {
            if !mostly_validate_domain_segment_unicode(&value) {
                Err(InvalidDomainSegment)?;
            }

            if BidiDetail::parse(&value) == BidiDetail::ForceAscii {
                Err(InvalidDomainSegment)?;
            }

            let mut ret = "xn--".to_string();

            encode_punycode_into(value.chars(), &mut ret).map_err(|_| InvalidDomainSegment)?;

            Ok((true, ret.into()))
        },
        _ => Err(InvalidDomainSegment)?
    }
}

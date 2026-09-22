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
/// If you know your input will be percent decoded, see [`percent_decoded_domain_segment_to_ascii`].
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If [`percent_decoded_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = percent_decoded_domain_segment_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a domain segment from bytes.
///
/// If you know your input will be percent decoded, see [`percent_decoded_domain_segment_to_ascii`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If [`percent_decoded_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_segment_bytes_to_ascii<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode_bytes(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = percent_decoded_domain_segment_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded domain segment.
///
/// If you know your input will be UTS46 mapped and normalized, see [`normalized_domain_segment_to_ascii`].
/// # Errors
/// If [`normalized_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value, class) = uts46_classify_map_normalize(value);

    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainSegment)?
    } else if class & 0b0000_0100 == 0b0000_0000 {
        Ok((a, value))
    } else {
        let (b, value) = normalized_domain_segment_to_ascii(value)?;
        Ok((a || b, value))
    }
}

/// Encode a percent decoded and UTS46 mapped and normalized domain segment.
/// # Errors
/// If `value` contains any ASCII byte in [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`], returns the error [`InvalidDomainSegment`].
///
/// If `value` is not ASCII:
///
/// - If [`mostly_validate_domain_segment_unicode`] returns [`false`], returns the error [`InvalidDomainSegment`].
///
/// - If [`BidiDetail::parse`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegment`].
///
/// - If [`encode_punycode`] returns an error, returns the error [`InvalidDomainSegment`].
pub fn normalized_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
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

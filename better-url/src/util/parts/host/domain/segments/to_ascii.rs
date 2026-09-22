//! Encoding.

use std::fmt::Write;

use crate::prelude::*;

/// Get [`DATA`].
const fn get_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        if FORBIDDEN_DOMAIN_SEGMENTS_LITERAL.contains(i as u8) {
            ret[i as usize] = 1;
        }

        if FORBIDDEN_DOMAIN_SEGMENTS_INPUT.contains(i as u8) {
            ret[i as usize] = 2;
        }

        i += 1;
    }

    ret
}

/// For each byte, `0` if a valid domain segments literal, `1` if a valid domain segments input, and `2` if an invalid domain segments input.
const DATA: [u8; 256] = get_data();

/// Encode domain segments.
///
/// If you know your input will be percent decoded, see [`percent_decoded_domain_segments_to_ascii`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If [`percent_decoded_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = percent_decoded_domain_segments_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode domain segments from bytes.
///
/// If you know your input will be percent decoded, see [`percent_decoded_domain_segments_to_ascii`].
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If [`percent_decoded_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_segments_bytes_to_ascii<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode_bytes(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = percent_decoded_domain_segments_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode percent decoded domain segments.
///
/// If you know your input will be UTS46 mapped and normalized, see [`normalized_domain_segments_to_ascii`].
/// # Errors
/// If [`normalized_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value, class) = uts46_classify_map_normalize(value);

    if class & 0b0010_0100 == 0b0010_0000 {
        Err(InvalidDomainSegments)?
    } else if class & 0b0000_0100 == 0b0000_0000 {
        Ok((a, value))
    } else {
        let (b, value) = normalized_domain_segment_to_ascii(value)?;
        Ok((a || b, value))
    }
}

/// Encode percent decoded and UTS46 mapped and normalized domain segments.
/// # Errors
/// If `value` contains any ASCII bytes in [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`], returns the error [`InvalidDomainSegments`].
///
/// If `value` is not ASCII:
///
/// - If any call to [`BidiDetail::parse`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegments`].
///
/// - If any call to [`BidiDetail::parse`] returns [`BidiDetail::ForceLtr`] and any other call to [`BidiDetail::parse`] returns [`BidiDetail::Rtl`], returns the error [`InvalidDomainSegments`].
///
/// - If any call to [`domain_segment_to_ascii`] returns an error, that error is returned.
#[expect(clippy::missing_panics_doc, reason = "Normalizer::write_str can't panic (unless String::push_str panics (at which point there's nothing to do.).).")]
pub fn normalized_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let mut value = value.into();

    match value.bytes().fold(0, |acc, b| acc | DATA[b as usize]) {
        0 => Ok((false, value)),
        1 => {
            let mut ret = Normalizer::new(&*value);

            let mut segments = value.split('.').peekable();

            let mut force_ltr = false;
            let mut rtl       = false;

            while let Some(segment) = segments.next() {
                let (_, segment, bidi_detail) = match segment.starts_with("xn--") {
                    true  => domain_segment_to_unicode(segment)?,
                    false => (false, segment.into(), segment.parse()?),
                };

                match bidi_detail {
                    BidiDetail::Ltr        => {},
                    BidiDetail::ForceLtr   => force_ltr = true,
                    BidiDetail::Rtl        => rtl       = true,
                    BidiDetail::ForceAscii => Err(InvalidDomainSegments)?
                }

                if force_ltr && rtl {
                    Err(InvalidDomainSegments)?;
                }

                let (_, encoded) = normalized_domain_segment_to_ascii(segment)?;

                ret.write_str(&encoded).expect("???");

                if segments.peek().is_some() {
                    ret.write_str(".").expect("???");
                }
            }

            Ok(match ret.done() {
                (changed, Cow::Owned   (x)) => (changed, x.into()),
                (changed, Cow::Borrowed(x)) => {
                    unsafe {
                        value.truncate_unchecked(x.len());
                    }
                    (changed, value)
                }
            })
        },
        _ => Err(InvalidDomainSegments)?,
    }
}

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
/// If you know your input will be percent decoded, see [`encode_percent_decoded_domain_segments`].
/// # Errors
/// If [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = encode_percent_decoded_domain_segments(value)?;
    Ok((a || b, value))
}

/// Encode percent decoded domain segments.
///
/// If you know your input will be UTS46 mapped and normalized, see [`encode_normalized_domain_segments`].
/// # Errors
/// If [`encode_normalized_domain_segments`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = encode_normalized_domain_segments(value)?;
    Ok((a || b, value))
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
/// - If any call to [`encode_domain_segment`] returns an error, that error is returned.
#[expect(clippy::missing_panics_doc, reason = "Normalizer::write_str can't panic (unless String::push_str panics (at which point there's nothing to do.).).")]
pub fn encode_normalized_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
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
                    true  => decode_domain_segment(segment)?,
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

                let (_, encoded) = encode_normalized_domain_segment(segment)?;

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

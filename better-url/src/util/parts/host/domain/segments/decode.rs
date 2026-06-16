//! Decoding.

use crate::prelude::*;

/// [`uts46_normalize`] + [`decode_normalized_domain_segments`].
/// # Errors
/// If the call to [`decode_normalized_domain_segments`] returns an error, that error is returned.
pub fn decode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainSegments> {
    let (a, value) = uts46_normalize(value);
    let (b, value, bidi_details) = decode_normalized_domain_segments(value)?;
    Ok((a || b, value, bidi_details))
}

/// Applies [`decode_normalized_domain_segment`] to each segment.
/// # Errors
/// If any call to [`decode_normalized_domain_segment`] returns an error, returns the error [`InvalidDomainSegments`].
pub fn decode_normalized_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainSegments> {
    let value = value.into();

    let mut bytes = value.bytes().peekable();

    'a: {if matches!(bytes.next(), None | Some(b'a'..=b'z')) {
        let mut segments = 1;

        while let Some(b) = bytes.next() {
            match b {
                b'a'..=b'z' | b'0'..=b'9' => {},
                b'.' if matches!(bytes.peek(), None | Some(b'a'..=b'z' | b'.')) => segments += 1,
                _ => break 'a
            }
        }

        return Ok((false, value, BidiDetails(SmallBitVec::from_elem(segments + 1, false))));
    }}

    let mut segments = value.split('.');
    let mut bidi_details = BidiDetails::default();

    while let Some(segment) = segments.next() {
        let (changed, decoded, bidi_detail) = decode_normalized_domain_segment(segment)?;
        bidi_details.try_push(bidi_detail)?;
        if changed {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&decoded);
            for segment in segments {
                let (_, segment, bidi_detail) = decode_normalized_domain_segment(segment)?;
                bidi_details.try_push(bidi_detail)?;
                ret.push('.');
                ret.push_str(&segment);
            }
            return Ok((true, ret.into(), bidi_details));
        }
    }

    Ok((false, value, bidi_details))
}

/// Applies [`decode_normalized_domain_segment_unchecked`] to each segment.
///
/// Notably also skips making a [`BidiDetails`].
/// # Panics
/// If any call to [`decode_normalized_domain_segment_unchecked`] panics, that panic is continued.
pub fn decode_normalized_domain_segments_unchecked<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let mut bytes = value.bytes().peekable();

    'a: {if matches!(bytes.next(), None | Some(b'a'..=b'z')) {
        while let Some(b) = bytes.next() {
            match b {
                b'a'..=b'z' | b'0'..=b'9' => {},
                b'.' if matches!(bytes.peek(), None | Some(b'a'..=b'z' | b'.')) => {},
                _ => break 'a
            }
        }

        return (false, value);
    }}

    let mut segments = value.split('.');

    while let Some(segment) = segments.next() {
        if let (true, decoded) = decode_normalized_domain_segment_unchecked(segment) {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&decoded);
            for segment in segments {
                ret.push('.');
                ret.push_str(&decode_normalized_domain_segment_unchecked(segment).1);
            }
            return (true, ret.into());
        }
    }

    (false, value)
}

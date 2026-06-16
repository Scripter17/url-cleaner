//! Encoding.

use crate::prelude::*;

/// [`try_percent_decode`] + [`encode_percent_decoded_domain_segments`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_domain_segments("abc.com"         ).unwrap().1, "abc.com"                         );
/// assert_eq!(encode_domain_segments("abc.com."        ).unwrap().1, "abc.com."                        );
/// assert_eq!(encode_domain_segments("Αθήνα.abc.Αθήνα" ).unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a" );
/// assert_eq!(encode_domain_segments("Αθήνα.abc.Αθήνα.").unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a.");
///
/// assert_eq!(encode_domain_segments("abc.123").unwrap().1, "abc.123");
/// ```
pub fn encode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainSegments> {
    let (a, value              ) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value, bidi_details) = encode_percent_decoded_domain_segments(value)?;
    Ok((a || b, value, bidi_details))
}

/// [`uts46_normalize`] + [`encode_normalized_domain_segments`].
/// # Errors
/// If the call to [`encode_normalized_domain_segments`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainSegments> {
    let (a, value         ) = uts46_normalize(value);
    let (b, value, details) = encode_normalized_domain_segments(value)?;
    Ok((a || b, value, details))
}

/// Encode a UTS46 encoded string of domain segmnts.
/// # Errors
/// If any call to [`encode_normalized_domain_segment`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If any call to [`BidiDetails::try_push`] returns an error, that error is returned.
pub fn encode_normalized_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainSegments> {
    let value = value.into();

    let mut bytes = value.bytes().peekable();

    if matches!(bytes.next(), None | Some(b'a'..=b'z')) {
        let mut normal = true;
        let mut segments = 1;

        while let Some(b) = bytes.next() {
            match b {
                b'a'..=b'z' | b'0'..=b'9' => {},
                b'.' if matches!(bytes.peek(), None | Some(b'a'..=b'z')) => segments += 1,
                _ => {normal = false; break;}
            }
        }

        if normal {
            let bidi_details = BidiDetails(SmallBitVec::from_elem(segments + 1, false));
            return Ok((true, value, bidi_details));
        }
    }

    let mut segments = value.split('.');
    let mut bidi_details = BidiDetails::default();

    while let Some(segment) = segments.next() {
        let (changed, encoded, bidi_detail) = encode_normalized_domain_segment(segment)?;
        bidi_details.try_push(bidi_detail)?;
        if changed {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&encoded);
            for segment in segments {
                let (_, encoded, bidi_detail) = encode_normalized_domain_segment(segment)?;
                bidi_details.try_push(bidi_detail)?;
                ret.push('.');
                ret.push_str(&encoded);
            }
            return Ok((true, ret.into(), bidi_details));
        }
    }

    Ok((false, value, bidi_details))
}

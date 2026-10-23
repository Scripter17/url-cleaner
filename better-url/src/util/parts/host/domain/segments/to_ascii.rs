//! Encoding.

use crate::prelude::*;

/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm, but with a name that clarifies it's being used on just a list of domain segments rather than a full domain.
///
/// Please note that this may return values that are later rejected by [`make_domain_segments`].
/// # Errors
/// If [`uts46_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegments> {
    let (_, value, class) = uts46_classify_map_normalize(value);

    // If the output contained no [`INVALID_DOMAIN_SEGMENTS_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegments`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        return Ok((value, None));
    }

    uts46_domain_segments_to_ascii(value, be_strict)
}

/// [`domain_segments_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`make_domain_segments`].
/// # Errors
/// If `be_strict` is [`true`] and/or `value` is not ASCII:
///
/// - If [`uts46_domain_segment_to_unicode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// - If [`BidiDetail::join`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegments`].
pub fn uts46_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegments> {
    let value = value.into();

    if (be_strict && value.contains("xn--")) || !value.is_ascii() {
        let mut ret = String::with_capacity(value.len());
        let mut segments = SplitDots(Some(&value)).peekable();
        let mut sum_bidi_detail = BidiDetail::Ltr;

        while let Some(raw_segment) = segments.next() {
            let (raw_segment, unicode_segment, bidi_detail) = uts46_domain_segment_to_unicode(raw_segment).map_err(|_| InvalidDomainSegments)?;

            sum_bidi_detail = sum_bidi_detail.join(bidi_detail);

            if sum_bidi_detail == BidiDetail::ForceAscii {
                Err(InvalidDomainSegments)?;
            }

            if unicode_segment.is_some() || raw_segment.is_ascii() {
                ret.push_str(&raw_segment);
            } else {
                ret.push_str("xn--");

                encode_punycode_into(&unicode_segment.map_or(raw_segment, Into::into), &mut ret);
            }

            if segments.peek().is_some() {
                ret.push('.');
            }
        }

        Ok((value, Some(ret)))
    } else {
        Ok((value, None))
    }
}

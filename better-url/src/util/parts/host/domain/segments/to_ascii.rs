//! Encoding.

use crate::prelude::*;

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode                    (value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = encode_percent_decoded_domain_segments(value)?;

    Ok((a || b, value))
}

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_segments_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode_bytes              (value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = encode_percent_decoded_domain_segments(value)?;

    Ok((a || b, value))
}

/// Common code for both [`encode_domain_segments`] and [`encode_domain_segments_bytes`].
/// # Errors
/// If [`uts46_domain_segments_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainSegments`], returns the error [`InvalidDomainSegments`]
pub fn encode_percent_decoded_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value, class) = uts46_classify_map_normalize(value);

    // If the input contained a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`].
    if class & 0b0010_0100 == 0b0010_0000 {
        Err(InvalidDomainSegments)?;
    }

    // If the output contained no [`INVALID_DOMAIN_SEGMENTS_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegments`] literal.
    if class & 0b0010_0110 == 0b0000_0000 {
        return Ok((a, value));
    }

    let (raw, ascii) = uts46_domain_segments_to_ascii(value, false)?;

    let b = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // I don't think this is required since it'd only trigger if UTS46 mapping and normalizing could add a new [`FORBIDDEN_DOMAIN_SEGMENTS_LITERAL`], but better safe than sorry.
    if value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENTS_LITERAL.contains(b)) {
        Err(InvalidDomainSegments)?;
    }

    Ok((a || b, value))
}



/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm, but with a name that clarifies it's being used on just a list of domain segments rather than a full domain.
///
/// Please note that this may return values that are later rejected by [`encode_domain_segments`].
/// # Errors
/// If [`uts46_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegments> {
    let (_, value, class) = uts46_classify_map_normalize(value);

    // If the output contained no [`INVALID_DOMAIN_SEGMENTS_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegments`] literal.
    if class & 0b0010_0110 == 0b0000_0000 {
        return Ok((value, None));
    }

    uts46_domain_segments_to_ascii(value, be_strict)
}

/// [`domain_segments_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`encode_domain_segments`].
/// # Errors
/// If `be_strict` is [`true`] and/or `value` is not ASCII:
///
/// - If [`uts46_domain_segment_to_unicode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// - If [`BidiDetail::join`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegments`].
pub fn uts46_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegments> {
    let value = value.into();

    if be_strict || !value.is_ascii() {
        let mut ret = String::with_capacity(value.len());
        let mut segments = SplitDots(Some(&value)).peekable();
        let mut sum_bidi_detail = BidiDetail::Ltr;

        while let Some(raw_segment) = segments.next() {
            let (raw_segment, unicode_segment, bidi_detail) = uts46_domain_segment_to_unicode(raw_segment).map_err(|_| InvalidDomainSegments)?;

            sum_bidi_detail = sum_bidi_detail.join(bidi_detail);

            if sum_bidi_detail == BidiDetail::ForceAscii {
                Err(InvalidDomainSegments)?;
            }

            if raw_segment.is_ascii() {
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

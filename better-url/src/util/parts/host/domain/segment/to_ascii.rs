//! Encoding.

use crate::prelude::*;

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn encode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode                   (value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = encode_percent_decoded_domain_segment(value)?;

    Ok((a || b, value))
}

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn encode_domain_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode_bytes             (value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = encode_percent_decoded_domain_segment(value)?;

    Ok((a || b, value))
}

/// Common code for both [`encode_domain_segment`] and [`encode_domain_segment_bytes`].
/// # Errors
/// If [`uts46_domain_segment_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainSegment`], returns the error [`InvalidDomainSegment`]
pub fn encode_percent_decoded_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value, class) = uts46_classify_map_normalize(value);

    // If the input contained a [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`].
    if class & 0b0001_0100 == 0b0001_0000 {
        Err(InvalidDomainSegment)?;
    }

    // If the output contained no [`INVALID_DOMAIN_SEGMENT_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegemnt`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        return Ok((a, value));
    }

    let (raw, ascii) = uts46_domain_segment_to_ascii(value, false)?;

    let b = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // I don't think this is required since it'd only trigger if UTS46 mapping and normalizing could add a new [`FORBIDDEN_DOMAIN_SEGMENT_LITERAL`], but better safe than sorry.
    if value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENT_LITERAL.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    Ok((a || b, value))
}



/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm for a single segment.
///
/// Please note that this may return values that are later rejected by [`encode_domain_segment`].
/// # Errors
/// If [`uts46_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegment> {
    let (_, value, class) = uts46_classify_map_normalize(value);

    // If the output contained no [`INVALID_DOMAIN_SEGMENT_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegment`] literal.
    if class & 0b0001_0110 == 0b0000_0000 {
        return Ok((value, None));
    }

    uts46_domain_segment_to_ascii(value, be_strict)
}

/// [`domain_segment_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`encode_domain_segment`].
/// # Errors
/// If `be_strict`:
///
/// - If [`uts46_domain_segment_to_unicode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If `value` is either non-ASCII or was decoded into being non-ASCII:
///
/// - If [`mostly_validate_uts46_domain_segment_unicode`] returns [`false`], returns the error [`InvalidDomainSegment`].
pub fn uts46_domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegment> {
    let mut value = value.into();

    if be_strict {
        (value, _, _) = uts46_domain_segment_to_unicode(value).map_err(|_| InvalidDomainSegment)?;
    }

    if !value.is_ascii() {
        // If `be_strict`, `mostly_validate_domain_segment_unicode`/`mostly_validate_uts46_domain_segment_unicode` was already checked and early returned on the failure of.
        if !be_strict && !mostly_validate_uts46_domain_segment_unicode(&value) {
            Err(InvalidDomainSegment)?;
        }

        // TODO: Push version using caller-provided &mut String?

        let mut ret = "xn--".into();

        encode_punycode_into(&value, &mut ret);

        Ok((value, Some(ret)))
    } else {
        Ok((value, None))
    }
}

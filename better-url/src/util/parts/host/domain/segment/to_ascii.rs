//! Encoding.

use crate::prelude::*;

/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm for a single segment.
///
/// Please note that this may return values that are later rejected by [`make_domain_segment`].
/// # Errors
/// If [`uts46_domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_segment_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainSegment> {
    let (_, value, class) = uts46_classify_map_normalize(value);

    // If the output contained no [`INVALID_DOMAIN_SEGMENT_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegment`] literal.
    if class & 0b0000_1110 == 0b0000_0000 {
        return Ok((value, None));
    }

    uts46_domain_segment_to_ascii(value, be_strict)
}

/// [`domain_segment_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`make_domain_segment`].
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

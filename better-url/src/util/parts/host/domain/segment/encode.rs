//! Encoding.

use crate::prelude::*;

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`make_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn make_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode                   (value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = make_percent_decoded_domain_segment(value)?;

    Ok((a || b, value))
}

/// Steps 5, 6, 7, and 9 of the [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`make_percent_decoded_domain_segment`] returns an error, that error is returned.
pub fn make_domain_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode_bytes             (value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = make_percent_decoded_domain_segment(value)?;

    Ok((a || b, value))
}

/// Common code for both [`make_domain_segment`] and [`make_domain_segment_bytes`].
/// # Errors
/// If [`uts46_domain_segment_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainSegment`], returns the error [`InvalidDomainSegment`]
pub fn make_percent_decoded_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (wasnt_uts46, value, class) = uts46_classify_map_normalize(value);

    // If the input contained a [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`].
    if class & 0b0000_1100 == 0b0000_1000 {
        Err(InvalidDomainSegment)?;
    }

    // If the output contained no [`INVALID_DOMAIN_SEGMENT_INPUT`], no Unicode, and no ASCII numbers, then it's a valid [`DomainSegemnt`] literal.
    if class & 0b0000_1110 == 0b0000_0000 {
        return Ok((wasnt_uts46, value));
    }

    let (raw, ascii) = uts46_domain_segment_to_ascii(value, false)?;

    let wasnt_ascii = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // If the input wasn't UTS46, it may in theory be possible for the output to contain a [`FORBIDDEN_DOMAIN_SEGMENT_LITERAL`] byte not caught by the classification.
    if wasnt_uts46 && value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENT_LITERAL.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    Ok((wasnt_uts46 || wasnt_ascii, value))
}

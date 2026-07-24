//! Decoding.

use icu_properties::{CodePointMapDataBorrowed, props::{GeneralCategory, GeneralCategoryGroup}};
use icu_normalizer::ComposingNormalizerBorrowed;

use crate::prelude::*;

/// the general category getter.
static GC: CodePointMapDataBorrowed<GeneralCategory> = CodePointMapDataBorrowed::new();
/// The NFC checker.
static NFC: ComposingNormalizerBorrowed = ComposingNormalizerBorrowed::new_nfc();

/// Decode an encoded domain segment literal.
/// # Errors
/// If `value` contains any byte in [`FORBIDDEN_DOMAIN_SEGMENT`], returns the error [`InvalidDomainSegment`].
pub fn decode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let value = value.into();

    if value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENT.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    Ok(unchecked_decode_domain_segment(value))
}

/// Decode a domain segment literal without any validity checks.
pub fn unchecked_decode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, BidiDetail) {
    let value = value.into();

    match value.strip_prefix("xn--") {
        Some(punycode) => {
            match decode_domain_segment_punycode(punycode) {
                Ok(decoded) => match BidiDetail::parse(&decoded) {
                    BidiDetail::ForceAscii => (false, value         , BidiDetail::ForceAscii),
                    bidi_detail            => (true , decoded.into(), bidi_detail           ),
                },
                Err(_) => (false, value, BidiDetail::ForceAscii),
            }
        },
        None => (false, value, BidiDetail::Ltr),
    }
}


/// Decode a domain segment's punycode.
/// # Errors
/// If `value` is empty or ends with `-`, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`decode_punycode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`mostly_validate_domain_segment_unicode`] returns [`false`], returns the error [`InvalidDomainSegment`].
pub fn decode_domain_segment_punycode(value: &str) -> Result<String, InvalidDomainSegment> {
    // If `value` is non-ASCII, violating 4.1, [`decode_punycode`] will return an error.
    // If `value` is empty, its decoding will be empty, violating 4.3.
    // If `value` is non-ASCII and ends in a `-`, it violates 4.1 anyway, so a false positive for violating 4.3 is fine.
    // If `value` is ASCII and ends in a `-`, its output will be ASCII, violating 4.3.

    if value.is_empty() || value.ends_with('-') {
        Err(InvalidDomainSegment)?;
    }

    let decoded = decode_punycode(value).map_err(|InvalidPunycode| InvalidDomainSegment)?;

    if !mostly_validate_domain_segment_unicode(&decoded) {
        Err(InvalidDomainSegment)?;
    }

    Ok(decoded)
}

/// If `value` satisfies [validity criteria](https://www.unicode.org/reports/tr46/#Validity_Criteria) 2 through 8.
///
/// 9 is handled by [`BidiDetail`].
pub fn mostly_validate_domain_segment_unicode(value: &str) -> bool {
    NFC.is_normalized(value)
        && !matches!(value.as_bytes(), [b'x', b'n', b'-', b'-', ..])
        && value.memchr(b'.').is_none()
        && !value.starts_with(mark)
        && !value.contains(idna_invalid)
        && validate_domain_segment_joiners(value)
}

/// If `c` is [`GeneralCategoryGroup::Mark`].
fn mark(c: char) -> bool {GeneralCategoryGroup::Mark.contains(GC.get(c))}

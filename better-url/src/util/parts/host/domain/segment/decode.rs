//! Decoding.

use icu_normalizer::ComposingNormalizerBorrowed;
use icu_properties::{CodePointMapDataBorrowed, props::{GeneralCategory, GeneralCategoryGroup}};

use crate::prelude::*;

/// The NFC checker.
static NFC: ComposingNormalizerBorrowed = ComposingNormalizerBorrowed::new_nfc();
/// the general category getter.
static GC: CodePointMapDataBorrowed<GeneralCategory> = CodePointMapDataBorrowed::new();

/// If `c` is [`GeneralCategoryGroup::Mark`].
fn mark(c: char) -> bool {GeneralCategoryGroup::Mark.contains(GC.get(c))}

/// [`uts46_normalize`] + [`decode_normalized_domain_segment`].
/// # Errors
/// If the call to [`decode_normalized_domain_segment`] returns an error, that error is returned.
pub fn decode_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let (a, value             ) = uts46_normalize(value);
    let (b, value, bidi_detail) = decode_normalized_domain_segment(value)?;
    Ok((a || b, value, bidi_detail))
}

/// [UTS46 processing](https://www.unicode.org/reports/tr46/#Processing) step 4.
///
/// Steps 1 and 2 are done by [`uts46_normalize`] and step 3 is irrelevant because this handles only one segment.
/// # Errors
/// If `value` starts with `xn--`:
///
/// - If the call to [`decode_punycode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// - If the decoded value is either empty or all ASCII, returns the error [`InvalidDomainSegment`].
///
/// - If the decoded value contains a U+FFFD, returns the error [`InvalidDomainSegment`].
///
/// - If the decoded value is changed by UTS46 normalization, returns the error [`InvalidDomainSegment`].
///
/// If the decoded value starts with a [`GeneralCategoryGroup::Mark`], returns the error [`InvalidDomainSegment`].
///
/// If the call to [`validate_domain_segment_joiners`] returns [`false`], reutrns the error [`InvalidDomainSegment`].
///
/// If the call to [`BidiDetail::parse`] returns an error, that error is returend.
pub fn decode_normalized_domain_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let mut value = value.into();
    let mut changed = false;

    // All segments matching the regex `|[a-z][a-z0-9]*` are valid and LTR.
    // Domains consisting entirely of these segments are very common.

    let mut bytes = value.bytes();

    if matches!(bytes.next(), None | Some(b'a'..=b'z')) && bytes.all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9')) {
        return Ok((false, value, BidiDetail::Ltr));
    }

    if !validate_domain_segment_bytes(&value) {
        Err(InvalidDomainSegment)?;
    }

    if let Some(punycode) = value.strip_prefix("xn--") {
        // If `punycode` is non-ASCII, violating 4.1, [`decode_punycode`] will return an error.
        // If `punycode` is empty, its decoding will be empty, violating 4.3.
        // If `punycode` is non-ASCII and ends in a `-`, it violates 4.1 anyway, so a false positive for violating 4.3 is fine.
        // If `punycode` is ASCII and ends in a `-`, its output will be ASCII, violating 4.3.

        if punycode.ends_with('-') || punycode.is_empty() {
            Err(InvalidDomainSegment)?;
        }

        value = decode_punycode(punycode).map_err(|InvalidPunycode| InvalidDomainSegment)?.into();

        // TODO: Does every codepoint being [`idna_valid`] imply it's normalized?
        // If so, this can be removed.
        if !NFC.is_normalized(&value) {
            Err(InvalidDomainSegment)?;
        }

        if value.starts_with("xn--") {
            Err(InvalidDomainSegment)?;
        }

        changed = true;
    }

    // `changed` being [`true`] means punycode was decoded and thus `value` can't be ASCII.
    // `value` being ASCII means none of these can happen.
    if changed || !value.is_ascii() {
        if value.starts_with(mark) {
            Err(InvalidDomainSegment)?;
        }

        if value.contains(|c| !idna_valid(c)) {
            Err(InvalidDomainSegment)?;
        }

        if !validate_domain_segment_joiners(&value) {
            Err(InvalidDomainSegment)?;
        }
    }

    let bidi_detail = value.parse()?;

    Ok((changed, value, bidi_detail))
}

/// If every ASCII byte in `value` is not in [`FORBIDDEN_DOMAIN_HOST`].
fn validate_domain_segment_bytes(value: &str) -> bool {
    !value.bytes().any(|b| b.is_ascii() && FORBIDDEN_DOMAIN_SEGMENT.contains(b))
}

/// [`decode_normalized_domain_segment`] without doing any validity checks.
///
/// Notably also skips making a [`BidiDetail`].
/// # Panics
/// Currently panics if [`decode_punycode`] returns an error. TODO: Fix that.
pub fn decode_normalized_domain_segment_unchecked<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.strip_prefix("xn--") {
        Some(punycode) => (true , decode_punycode(punycode).expect("To be given valid punycode").into()),
        None           => (false, value)
    }
}

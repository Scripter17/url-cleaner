//! Decoding.

use icu_properties::{CodePointMapDataBorrowed, props::{GeneralCategory, GeneralCategoryGroup}};

use crate::prelude::*;

/// the general category getter.
static GC: CodePointMapDataBorrowed<GeneralCategory> = CodePointMapDataBorrowed::new();

/// Percent decode, UTS46 normalize, and decode a domain segment.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_segment_to_unicode`].
/// # Errors
/// If the call to [`try_percent_decode`], returns the error [`InvalidDomainSegment`].
///
/// If the call to [`normalized_domain_segment_to_unicode`] returns an error, that error is returned.
pub fn domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegment)?;
    let (b, value) = percent_decoded_domain_segment_to_unicode(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and decode a percent decoded domain segment.
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_segment_to_unicode`].
/// # Errors
/// If the call to [`normalized_domain_segment_to_unicode`] returns an error, that error is returned.
pub fn percent_decoded_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_segment_to_unicode(value)?;
    Ok((a || b, value))
}

/// Decode a percent decoded and UTS46 normalized domain segment.
///
/// Specifically, [UTS46 processing](https://www.unicode.org/reports/tr46/#Processing) step 4.
/// # Errors
/// If the call to [`has_forbidden_domain_segment_byte`] returns [`true`], returns the error [`InvalidDomainSegment`].
///
/// If `value` starts with `xn--` and the call to [`domain_segment_decode_punycode`] returns an error *and* `value` is not ASCII, that error is returned.
///
/// If `value` does not start with `xn--`, is not ASCII, and [`mostly_validate_domain_segment_unicode`] returns [`false`], returns the error [`InvalidDomainSegment`].
pub fn normalized_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegment> {
    let value = value.into();

    if !has_forbidden_domain_segment_byte(&value) {
        Err(InvalidDomainSegment)?;
    }

    if let Some(punycode) = value.strip_prefix("xn--") {
        match domain_segment_decode_punycode(punycode) {
            Ok (x)                     => Ok((true , x.into())),
            Err(_) if value.is_ascii() => Ok((false, value   )),
            Err(e)                     => Err(e)
        }
    } else if value.is_ascii() || mostly_validate_domain_segment_unicode(&value) {
        Ok((false, value))
    } else {
        Err(InvalidDomainSegment)
    }
}



/// Strictly decode a percent decoded and UTS46 normalized domain segment.
///
/// Specifically, [UTS46 processing](https://www.unicode.org/reports/tr46/#Processing) step 4.
///
/// Note that because [`BidiDetail::parse`] returning an error makes a segment invalid, this also returns any non-error result of [`BidiDetail::parse`].
/// # Errors
/// If the call to [`has_forbidden_domain_segment_byte`] returns [`true`], returns the error [`InvalidDomainSegment`].
///
/// If `value` starts with `xn--` and  the call to [`domain_segment_decode_punycode`] returns an error, that error is returned.
///
/// If `value` does not start with `xn--`, is not ASCII, and the call to [`mostly_validate_domain_segment_unicode`] returns false, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`BidiDetail::parse`] returns an error, that error is returned.
pub fn strict_normalized_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetail), InvalidDomainSegment> {
    let mut value = value.into();

    if has_forbidden_domain_segment_byte(&value) {
        Err(InvalidDomainSegment)?;
    }

    if let Some(punycode) = value.strip_prefix("xn--") {
        value = domain_segment_decode_punycode(punycode)?.into();
        let bidi_detail = value.parse()?;

        Ok((true, value, bidi_detail))
    } else if value.is_ascii() || mostly_validate_domain_segment_unicode(&value) {
        let bidi_detail = value.parse()?;

        Ok((false, value, bidi_detail))
    } else {
        Err(InvalidDomainSegment)
    }
}



/// Uncheckedly decode a percent decoded and normalized domain segment.
/// # Panics
/// If [`decode_punycode`] returns an error, panics.
pub fn unchecked_normalized_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.strip_prefix("xn--") {
        Some(punycode) => match decode_punycode(punycode) {
            Ok (x)                     => (true , x.into()),
            Err(_) if value.is_ascii() => (false, value   ),
            Err(_)                     => panic!("normalized_domain_segment_unchecked_to_unicode was given invalid punycode: {punycode:?}.")
        },
        None => (false, value)
    }
}



/// If every ASCII byte in `value` is not in [`FORBIDDEN_DOMAIN_SEGMENT`].
pub fn has_forbidden_domain_segment_byte(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii() && FORBIDDEN_DOMAIN_SEGMENT.contains(b))
}

/// [`decode_punycode`] with extra checks to ensure it's valid in a domain segment.
/// # Errors
/// If `value` ends in `-` or is empty, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`decode_punycode`] returns an error, returns the error [`InvalidDomainSegment`].
///
/// If the call to [`mostly_validate_domain_segment_unicode`] returns an error, returns the error [`InvalidDomainSegment`].
pub fn domain_segment_decode_punycode(value: &str) -> Result<String, InvalidDomainSegment> {
    // If `value` is non-ASCII, violating 4.1, [`decode_punycode`] will return an error.
    // If `value` is empty, its decoding will be empty, violating 4.3.
    // If `value` is non-ASCII and ends in a `-`, it violates 4.1 anyway, so a false positive for violating 4.3 is fine.
    // If `value` is ASCII and ends in a `-`, its output will be ASCII, violating 4.3.

    if matches!(value.as_bytes(), [] | [.., b'-']) {
        Err(InvalidDomainSegment)?;
    }

    let decoded = decode_punycode(value).map_err(|InvalidPunycode| InvalidDomainSegment)?;

    if !mostly_validate_domain_segment_unicode(&decoded) {
        Err(InvalidDomainSegment)?;
    }

    Ok(decoded)
}

/// If `value` is a valid unicode decoded domain segment.
///
/// Specifically, [validity criteria](https://www.unicode.org/reports/tr46/#Validity_Criteria) 1, 4, 5, 6, 7.2, and 8.
///
/// 9 is done by [`BidiDetail`] as needed.
///
/// See [`idna_invalid`] for details on 7.2 and [`validate_domain_segment_joiners`] for details on 8.
pub fn mostly_validate_domain_segment_unicode(value: &str) -> bool {
    // Note to self: NFC checking is removed because, I think, all characters being IDNA-Table valid/deviation implies NFKC, and NFKC implies NFC.
    // If I'm wrong then... fuck.

    !matches!(value.as_bytes(), [b'x', b'n', b'-', b'-', ..])
        && !value.bytes().any(|b| b == b'.')
        && !value.starts_with(mark)
        && !value.contains(idna_invalid)
        && validate_domain_segment_joiners(value)
}

/// If `c` is [`GeneralCategoryGroup::Mark`].
fn mark(c: char) -> bool {GeneralCategoryGroup::Mark.contains(GC.get(c))}

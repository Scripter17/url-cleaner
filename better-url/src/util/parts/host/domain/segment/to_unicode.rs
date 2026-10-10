//! Decoding.

use icu_properties::{CodePointMapDataBorrowed, props::{GeneralCategory, GeneralCategoryGroup}};
use icu_normalizer::ComposingNormalizerBorrowed;

use crate::prelude::*;

/// the general category getter.
static GC: CodePointMapDataBorrowed<GeneralCategory> = CodePointMapDataBorrowed::new();
/// The NFC checker.
static NFC: ComposingNormalizerBorrowed = ComposingNormalizerBorrowed::new_nfc();

/// A single segment of [Domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode).
///
/// If your input is in UTS46, see [`uts46_domain_segment_to_unicode`].
/// # Errors
/// If [`uts46_domain_segment_to_unicode`] returns an error, that error is returned.
///
/// Note that, per the spec, the decoded value is still returned. It's just done in a way where you can't ignore errors.
/// # Examples
/// ```
/// use better_url::prelude::*;
/// use better_url::util::*;
///
/// assert_eq!(domain_segment_to_unicode("abc"    ), Ok(("abc"    .into(), None             , BidiDetail::Ltr     )));
/// assert_eq!(domain_segment_to_unicode("123"    ), Ok(("123"    .into(), None             , BidiDetail::ForceLtr)));
/// assert_eq!(domain_segment_to_unicode("⭐"     ), Ok(("⭐"     .into(), None             , BidiDetail::ForceLtr)));
/// assert_eq!(domain_segment_to_unicode("xn--f7i"), Ok(("xn--f7i".into(), Some("⭐".into()), BidiDetail::ForceLtr)));
/// ```
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    let (_, value) = uts46_map_normalize(value);

    uts46_domain_segment_to_unicode(value)
}

/// [`domain_segment_to_unicode`], assuming the input has been through [`uts46_map_normalize`].
/// # Errors
/// Long story.
///
/// Note that, per the spec, the decoded value is still returned. It's just done in a way where you can't ignore errors.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn uts46_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    let value = value.into();

    match value.strip_prefix("xn--") {
        None => match BidiDetail::fast_parse_ascii(&value) {
            Some(bidi_detail) => Ok((value, None, bidi_detail)),
            None => match mostly_validate_uts46_domain_segment_unicode(&value) {
                true => match BidiDetail::parse_unicode(&value) {
                    BidiDetail::ForceAscii => Err((value, None             )),
                    bidi_detail            => Ok ((value, None, bidi_detail)),
                },
                false => Err((value, None)),
            },
        },
        Some(punycode) => match decode_punycode(punycode) {
            Ok(decoded) => match mostly_validate_domain_segment_unicode(&decoded) {
                true => match BidiDetail::parse_unicode(&decoded) {
                    BidiDetail::ForceAscii => Err((value, Some(decoded)             )),
                    bidi_detail            => Ok ((value, Some(decoded), bidi_detail)),
                },
                false => Err((value, Some(decoded))),
            },
            Err(InvalidPunycode) => Err((value, None)),
        }
    }
}

/// [`uts46_domain_segment_to_unicode`] without any validity checks.
///
/// Used during ToUnicoding multiple domain segments after validity already failed, resulting in no need for further checks.
pub fn unchecked_uts46_domain_segment_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (Cow<'a, str>, Option<String>) {
    let value = value.into();

    match value.strip_prefix("xn--") {
        None => (value, None),
        Some(punycode) => match decode_punycode(punycode) {
            Ok (decoded        ) => (value, Some(decoded)),
            Err(InvalidPunycode) => (value, None         ),
        }
    }
}



/// If `value` satisfies [validity criteria](https://www.unicode.org/reports/tr46/#Validity_Criteria) 1 through 8.
///
/// 9 is handled by [`BidiDetail`].
pub fn mostly_validate_domain_segment_unicode(value: &str) -> bool {
    NFC.is_normalized(value)
        && !value.starts_with("xn--")
        && !value.contains('.')
        && !value.starts_with(|c| GeneralCategoryGroup::Mark.contains(GC.get(c)))
        && !value.contains(idna_invalid)
        && validate_domain_segment_joiners(value)
}

/// [`mostly_validate_domain_segment_unicode`], assuming the input has been through [`uts46_map_normalize`].
pub fn mostly_validate_uts46_domain_segment_unicode(value: &str) -> bool {
    !value.starts_with("xn--")
        && !value.contains('.')
        && !value.starts_with(|c| GeneralCategoryGroup::Mark.contains(GC.get(c)))
        && !value.contains('\u{FFFD}')
        && validate_domain_segment_joiners(value)
}

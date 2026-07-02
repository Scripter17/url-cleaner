//! Decoding.

use crate::prelude::*;

/// Percent decode, UTS46 normalize, and decode a string of domain segments.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_segments_to_unicode`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If the call to [`normalized_domain_segments_to_unicode`] returns an error, that error is returned.
pub fn domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = percent_decoded_domain_segments_to_unicode(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and decode a percent decoded string of domain segments.
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_segments_to_unicode`].
/// # Errors
/// If the call to [`normalized_domain_segments_to_unicode`] returns an error, that error is returned.
pub fn percent_decoded_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_segments_to_unicode(value)?;
    Ok((a || b, value))
}

/// Decode a percent decoded and UTS46 normalized string of domain segments.
/// # Errors
/// If any call to [`strict_normalized_domain_segment_to_unicode`] returns an error, returns the error [`InvalidDomainSegments`].
pub fn normalized_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();
    let mut segments = SplitDots(Some(&value));
    let mut saw_inv = false;
    let mut saw_rtl = false;

    while let Some(segment) = segments.next() {
        let (changed, decoded, bidi_detail) = match strict_normalized_domain_segment_to_unicode(segment) {
            Ok (x) => x,
            Err(_) if value.is_ascii() => return Ok((false, value)),
            Err(e) => Err(e)?
        };

        match bidi_detail {
            BidiDetail::Ltr => {},
            BidiDetail::Inv => saw_inv = true,
            BidiDetail::Rtl => saw_rtl = true,
        }

        if saw_inv && saw_rtl {
            return Ok((false, value));
        }

        if changed {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&decoded);
            for segment in segments {
                let (_, decoded, bidi_detail) = match strict_normalized_domain_segment_to_unicode(segment) {
                    Ok (x) => x,
                    Err(_) if value.is_ascii() => return Ok((false, value)),
                    Err(e) => Err(e)?
                };

                match bidi_detail {
                    BidiDetail::Ltr => {},
                    BidiDetail::Inv => saw_inv = true,
                    BidiDetail::Rtl => saw_rtl = true,
                }

                if saw_inv && saw_rtl {
                    return Ok((false, value));
                }

                ret.extend([".", &decoded]);
            }
            return Ok((true, ret.into()));
        }
    }

    Ok((false, value))
}



/// Strictly decode a percent decoded and UTS46 normalized string of domain segments.
/// # Errors
/// If any call to [`strict_normalized_domain_segment_to_unicode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If any call to [`BidiDetail::parse`] returns an error, that error is returned.
pub fn strict_normalized_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();
    let mut segments = SplitDots(Some(&value));
    let mut saw_inv = false;
    let mut saw_rtl = false;

    while let Some(segment) = segments.next() {
        let (changed, decoded, bidi_detail) = strict_normalized_domain_segment_to_unicode(segment)?;

        match bidi_detail {
            BidiDetail::Ltr => {},
            BidiDetail::Inv => saw_inv = true,
            BidiDetail::Rtl => saw_rtl = true,
        }

        if saw_inv && saw_rtl {
            return Ok((false, value));
        }

        if changed {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&decoded);
            for segment in segments {
                let (_, decoded, bidi_detail) =  strict_normalized_domain_segment_to_unicode(segment)?;

                match bidi_detail {
                    BidiDetail::Ltr => {},
                    BidiDetail::Inv => saw_inv = true,
                    BidiDetail::Rtl => saw_rtl = true,
                }

                if saw_inv && saw_rtl {
                    return Ok((false, value));
                }

                ret.extend([".", &decoded]);
            }
            return Ok((true, ret.into()));
        }
    }

    Ok((false, value))
}



/// Uncheckedly deocde a percent decoded and UTS46 normalized string of domain segments.
/// # Panics
/// If any call to [`unchecked_normalized_domain_segment_to_unicode`] panics, that panic is continued.
pub fn unchecked_normalized_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();
    let mut segments = SplitDots(Some(&value));

    while let Some(segment) = segments.next() {
        if let (true, decoded) = unchecked_normalized_domain_segment_to_unicode(segment) {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&decoded);
            for segment in segments {
                let (_, decoded) = unchecked_normalized_domain_segment_to_unicode(segment);
                ret.extend([".", &decoded]);
            }
            return (true, ret.into());
        }
    }

    (false, value)
}

//! Encoding.

use crate::prelude::*;

/// Percent decode, UTS46 normalize, and encode a string of domain segments.
///
/// If you know the input is already percent decoded, see [`percent_decoded_domain_segments_to_ascii`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`percent_decoded_domain_segments_to_ascii`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(domain_segments_to_ascii("abc.com"         ).unwrap().1, "abc.com"                         );
/// assert_eq!(domain_segments_to_ascii("abc.com."        ).unwrap().1, "abc.com."                        );
/// assert_eq!(domain_segments_to_ascii("Αθήνα.abc.Αθήνα" ).unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a" );
/// assert_eq!(domain_segments_to_ascii("Αθήνα.abc.Αθήνα.").unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a.");
///
/// assert_eq!(domain_segments_to_ascii("abc.123").unwrap().1, "abc.123");
/// ```
pub fn domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = percent_decoded_domain_segments_to_ascii(value)?;
    Ok((a || b, value))
}

/// UTS46 normalize and encode a percent decoded string of domain segments.
///
/// If you know the input is already UTS46 normalized, see [`normalized_domain_segments_to_ascii`].
/// # Errors
/// If the call to [`normalized_domain_segments_to_ascii`] returns an error, that error is returned.
pub fn percent_decoded_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = normalized_domain_segments_to_ascii(value)?;
    Ok((a || b, value))
}

/// Encode a percent decoded and UTS46 encoded string of domain segmnts.
/// # Errors
/// If any call to [`normalized_domain_segment_to_ascii`] returns an error, returns the error [`InvalidDomainSegments`].
pub fn normalized_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();

    let mut fast = true;

    for b in value.bytes() {
        match b {
            b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' => {},
            b if b.is_ascii() && FORBIDDEN_DOMAIN.contains(b) => Err(InvalidDomainSegments)?,
            _ => {fast = false; break}
        }
    }

    if fast {
        return Ok((false, value));
    }

    let mut segments = SplitDots(Some(&value));
    let mut saw_inv = false;
    let mut saw_rtl = false;

    while let Some(segment) = segments.next() {
        let (changed, encoded, bidi_detail) = match strict_normalized_domain_segment_to_ascii(segment) {
            Ok (x) => x,
            Err(_) if value.is_ascii() => return Ok((false, value)),
            Err(e) => Err(e)?,
        };

        match bidi_detail {
            BidiDetail::Ltr => {},
            BidiDetail::Inv => saw_inv = true,
            BidiDetail::Rtl => saw_rtl = true,
        }

        if saw_inv && saw_rtl {
            if value.is_ascii() {
                return Ok((false, value));
            } else {
                Err(InvalidDomainSegments)?;
            }
        }

        if changed {
            let mut ret = value[..segment.addr() - value.addr()].to_string();
            ret.push_str(&encoded);
            for segment in segments {
                let (_, encoded, bidi_detail) = match strict_normalized_domain_segment_to_ascii(segment) {
                    Ok (x) => x,
                    Err(_) if value.is_ascii() => return Ok((false, value)),
                    Err(e) => Err(e)?,
                };

                match bidi_detail {
                    BidiDetail::Ltr => {},
                    BidiDetail::Inv => saw_inv = true,
                    BidiDetail::Rtl => saw_rtl = true,
                }

                if saw_inv && saw_rtl {
                    if value.is_ascii() {
                        return Ok((false, value));
                    } else {
                        Err(InvalidDomainSegments)?;
                    }
                }

                ret.extend([".", &encoded]);
            }
            return Ok((true, ret.into()));
        }
    }

    Ok((false, value))
}

/// Encode a UTS46 encoded string of domain segmnts.
/// # Errors
/// If any call to [`normalized_domain_segment_to_ascii`] returns an error, returns the error [`InvalidDomainSegments`].
pub fn strict_normalized_domain_segments_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();
    let mut segments = SplitDots(Some(&value));
    let mut saw_inv = false;
    let mut saw_rtl = false;

    while let Some(segment) = segments.next() {
        let (changed, encoded, bidi_detail) = strict_normalized_domain_segment_to_ascii(segment)?;

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
            ret.push_str(&encoded);
            for segment in segments {
                let (_, encoded, bidi_detail) = strict_normalized_domain_segment_to_ascii(segment)?;

                match bidi_detail {
                    BidiDetail::Ltr => {},
                    BidiDetail::Inv => saw_inv = true,
                    BidiDetail::Rtl => saw_rtl = true,
                }

                if saw_inv && saw_rtl {
                    return Ok((false, value));
                }

                ret.extend([".", &encoded]);
            }
            return Ok((true, ret.into()));
        }
    }

    Ok((false, value))
}

/// If every ASCII byte in `value` is not in [`FORBIDDEN_DOMAIN`].
pub fn has_forbidden_domain_byte(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii() && FORBIDDEN_DOMAIN.contains(b))
}

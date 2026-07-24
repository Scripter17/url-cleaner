//! Encoding.

use std::fmt::Write;

use crate::prelude::*;

/// Encode domain segments.
///
/// If you know your input will be percent decoded, see [`encode_percent_decoded_domain_segments`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, returns the error [`InvalidDomainSegments`].
///
/// If the call to [`encode_percent_decoded_domain_segments`] returns an error, that error is returned.
pub fn encode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = try_percent_decode(value).map_err(|_| InvalidDomainSegments)?;
    let (b, value) = encode_percent_decoded_domain_segments(value)?;
    Ok((a || b, value))
}

/// Encode percent decoded domain segments.
///
/// If you know your input will be UTS46 mapped and normalized, see [`encode_normalized_domain_segments`].
/// # Errors
/// If the call to [`encode_normalized_domain_segments`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let (a, value) = uts46_map_normalize(value);
    let (b, value) = encode_normalized_domain_segments(value)?;
    Ok((a || b, value))
}

/// Encode percent decoded and UTS46 mapped and normalized domain segments.
/// # Errors
/// If `value` contains any ASCII bytes in [`FORBIDDEN_DOMAIN_SEGMENTS`], returns the error [`InvalidDomainSegments`].
///
/// If `value` is not ASCII:
///
/// - If any call to [`BidiDetail::parse`] returns [`BidiDetail::ForceAscii`], returns the error [`InvalidDomainSegments`].
///
/// - If any call to [`BidiDetail::parse`] returns [`BidiDetail::ForceLtr`] and any other call to [`BidiDetail::parse`] returns [`BidiDetail::Rtl`], returns the error [`InvalidDomainSegments`].
///
/// - If any call to [`encode_domain_segment`] returns an error, that error is returned.
pub fn encode_normalized_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let mut value = value.into();

    if value.bytes().any(|b| b.is_ascii() && FORBIDDEN_DOMAIN_SEGMENTS.contains(b)) {
        Err(InvalidDomainSegments)?;
    }

    if value.is_ascii() {
        return Ok((false, value));
    }

    let mut ret = Normalizer::new(&*value);

    let mut segments = value.split('.').peekable();

    let mut force_ltr = false;
    let mut rtl       = false;

    while let Some(segment) = segments.next() {
        match BidiDetail::parse(segment) {
            BidiDetail::Ltr        => {},
            BidiDetail::ForceLtr   => force_ltr = true,
            BidiDetail::Rtl        => rtl       = true,
            BidiDetail::ForceAscii => Err(InvalidDomainSegments)?
        }

        if force_ltr && rtl {
            Err(InvalidDomainSegments)?;
        }

        let (_, encoded) = encode_domain_segment(segment)?;

        match segments.peek() {
            Some(_) => write!(ret, "{encoded}.").expect("???"),
            None    => write!(ret, "{encoded}" ).expect("???"),
        }
    }

    Ok(match ret.done() {
        (changed, Cow::Owned   (x)) => (changed, x.into()),
        (changed, Cow::Borrowed(x)) => {
            value.retain_substr(x);
            (changed, value)
        }
    })
}

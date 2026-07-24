//! Decoding.

use std::fmt::Write;

use crate::prelude::*;

/// Decode an encoded domain segments literal.
/// # Errors
/// If `value` contains any byte in [`FORBIDDEN_DOMAIN_SEGMENTS`], returns the error [`InvalidDomainSegments`].
pub fn decode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();

    if value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENTS.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    Ok(unchecked_decode_domain_segments(value))
}

/// Decode a domain segments literal without any validity checks.
pub fn unchecked_decode_domain_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    let mut segments = value.split('.').peekable();

    let mut force_ltr = false;
    let mut rtl       = false;

    let mut ret = Normalizer::new(&*value);

    while let Some(segment) = segments.next() {
        let (_, decoded, bidi_detail) = unchecked_decode_domain_segment(segment);

        match bidi_detail {
            BidiDetail::Ltr        => {},
            BidiDetail::ForceLtr   => force_ltr = true,
            BidiDetail::Rtl        => rtl       = true,
            BidiDetail::ForceAscii => return (false, value),
        }

        if force_ltr && rtl {
            return (false, value);
        }

        match segments.peek() {
            Some(_) => write!(ret, "{decoded}.").expect("???"),
            None    => write!(ret, "{decoded}" ).expect("???"),
        }
    }

    match ret.done() {
        (changed, Cow::Owned   (x)) => (changed, x.into()),
        (changed, Cow::Borrowed(x)) => {
            value.retain_substr(x);
            (changed, value)
        }
    }
}

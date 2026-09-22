//! Decoding.

use std::fmt::Write;

use crate::prelude::*;

/// Decode an encoded domain segments literal.
/// # Errors
/// If `value` contains any byte in [`FORBIDDEN_DOMAIN_SEGMENTS_LITERAL`], returns the error [`InvalidDomainSegments`].
pub fn domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainSegments> {
    let value = value.into();

    if value.bytes().any(|b| FORBIDDEN_DOMAIN_SEGMENTS_LITERAL.contains(b)) {
        Err(InvalidDomainSegment)?;
    }

    Ok(unchecked_domain_segments_to_unicode(value))
}

/// Decode a domain segments literal without any validity checks.
pub fn unchecked_domain_segments_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    let mut segments = value.split('.').peekable();

    let mut force_ltr = false;
    let mut rtl       = false;

    let mut ret = Normalizer::new(&*value);

    while let Some(segment) = segments.next() {
        let (_, decoded, bidi_detail) = unchecked_domain_segment_to_unicode(segment);

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
            unsafe {
                value.truncate_unchecked(x.len());
            }
            (changed, value)
        }
    }
}

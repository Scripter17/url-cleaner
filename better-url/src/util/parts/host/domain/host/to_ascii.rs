//! Encoding.

use crate::prelude::*;

/// The [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm with `isOpaque` set to false and non-domain outputs mapped to error.
/// # Errors
/// If [`try_percent_decode`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_host`] returns an error, that error is returned.
pub fn encode_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode                (value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = encode_percent_decoded_domain_host(value)?;

    Ok((a || b, value))
}

/// The [host parser](https://url.spec.whatwg.org/#host-parsing) algorithm with `isOpaque` set to false and non-domain outputs mapped to error.
/// # Errors
/// If [`try_percent_decode_bytes`] returns an error, that error is returned.
///
/// If [`encode_percent_decoded_domain_host`] returns an error, that error is returned.
pub fn encode_domain_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value) = try_percent_decode_bytes          (value).map_err(|_| InvalidDomainHost)?;
    let (b, value) = encode_percent_decoded_domain_host(value)?;

    Ok((a || b, value))
}

/// Common code for both [`encode_domain_host`] and [`encode_domain_host_bytes`].
/// # Errors
/// If [`uts46_domain_host_to_ascii`] returns an error, that error is returned.
///
/// If `value` is otherwise not a valid [`DomainHost`], returns the error [`InvalidDomainHost`]
pub fn encode_percent_decoded_domain_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidDomainHost> {
    let (a, value, class) = uts46_classify_map_normalize(value);

    // Pretty sure non-empty input can become empty after [`uts46_classify_map_normalize`].
    if value.is_empty() {
        Err(InvalidDomainHost)?;
    }

    // If the input contained a [`FORBIDDEN_DOMAIN_HOST_INPUT`] but no Unicode, then the output must also contain a [`FORBIDDEN_DOMAIN_HOST_INPUT`].
    if class & 0b0010_0100 == 0b0010_0000 {
        Err(InvalidDomainHost)?;
    }

    // If the input contained Unicode and/or a number, check if the output [`ends_in_a_number`].
    if class & 0b0000_0110 != 0b0000_0000 && ends_in_a_number(&value) {
        Err(InvalidDomainHost)?;
    }

    // If the output contained no [`INVALID_DOMAIN_HOST_INPUT`], no Unicode, and no ASCII numbers, and it's not empty, then it's a valid [`DomainHost`] literal.
    if class & 0b0010_0110 == 0b0000_0000 {
        if value.is_empty() {
            Err(InvalidDomainHost)?;
        }

        return Ok((a, value));
    }

    let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

    let b = ascii.is_some();

    let value = ascii.map_or(raw, Into::into);

    // I don't think this is required since it'd only trigger if UTS46 mapping and normalizing could add a new [`FORBIDDEN_DOMAIN_HOST_LITERAL`], but better safe than sorry.
    if value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
        Err(InvalidDomainHost)?;
    }

    Ok((a || b, value))
}



/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm.
///
/// Please note that this may return values that are later rejected by [`encode_domain_host`].
/// # Errors
/// If [`domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainHost> {
    Ok(domain_segments_to_ascii(value, be_strict)?)
}

/// [`domain_host_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`encode_domain_host`].
/// # Errors
/// If [`domain_segment_to_ascii`] returns an error, that error is returned.
pub fn uts46_domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainHost> {
    Ok(uts46_domain_segments_to_ascii(value, be_strict)?)
}

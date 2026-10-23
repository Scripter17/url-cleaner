//! Encoding.

use crate::prelude::*;

/// The URL spec's [domain parser ToASCII](https://url.spec.whatwg.org/#domain-parser-toascii) algorithm.
///
/// Please note that this may return values that are later rejected by [`make_domain_host`].
/// # Errors
/// If [`domain_segment_to_ascii`] returns an error, that error is returned.
pub fn domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainHost> {
    Ok(domain_segments_to_ascii(value, be_strict)?)
}

/// [`domain_host_to_ascii`] assuming `value` has already been through [`uts46_map_normalize`].
///
/// Please note that this may return values that are later rejected by [`make_domain_host`].
/// # Errors
/// If [`domain_segment_to_ascii`] returns an error, that error is returned.
pub fn uts46_domain_host_to_ascii<'a, T: Into<Cow<'a, str>>>(value: T, be_strict: bool) -> Result<(Cow<'a, str>, Option<String>), InvalidDomainHost> {
    Ok(uts46_domain_segments_to_ascii(value, be_strict)?)
}

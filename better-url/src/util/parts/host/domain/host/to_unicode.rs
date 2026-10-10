//! Decoding.

use crate::prelude::*;

/// [Domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode).
/// # Errors
/// If [`uts46_domain_host_to_unicode`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    let (_, value) = uts46_map_normalize(value);

    uts46_domain_host_to_unicode(value)
}

/// [`domain_host_to_unicode`], assuming the input has been through [`uts46_map_normalize`].
/// # Errors
/// If [`uts46_domain_segments_to_unicode`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn uts46_domain_host_to_unicode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
    uts46_domain_segments_to_unicode(value)
}

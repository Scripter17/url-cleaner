//! Encoding.

use crate::prelude::*;

/// [`try_percent_decode`] + [`encode_percent_decoded_domain`].
/// # Errors
/// If the call to [`try_percent_decode`] returns an error, that error is returned.
///
/// If the call to [`encode_percent_decoded_domain`] returns an error, that error is returned.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_domain("abc.com"         ).unwrap().1, "abc.com"                         );
/// assert_eq!(encode_domain("abc.com."        ).unwrap().1, "abc.com."                        );
/// assert_eq!(encode_domain("Αθήνα.abc.Αθήνα" ).unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a" );
/// assert_eq!(encode_domain("Αθήνα.abc.Αθήνα.").unwrap().1, "xn--jxafb0a0a.abc.xn--jxafb0a0a.");
///
/// encode_domain("abc.123").unwrap_err();
/// ```
pub fn encode_domain<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainHost> {
    let (a, value              ) = try_percent_decode(value).map_err(|_| InvalidDomainHost)?;
    let (b, value, bidi_details) = encode_percent_decoded_domain(value)?;
    Ok((a || b, value, bidi_details))
}

/// [`uts46_normalize`] + [`encode_normalized_domain`].
/// # Errors
/// If the call to [`encode_normalized_domain`] returns an error, that error is returned.
pub fn encode_percent_decoded_domain<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainHost> {
    let (a, value         ) = uts46_normalize(value);
    let (b, value, details) = encode_normalized_domain(value)?;
    Ok((a || b, value, details))
}

/// Encodes a UTS46 normalized domain.
/// # Errors
/// If the call to [`encode_normalized_domain_segments`] returns an error, that error is returned.
///
/// If the resulting domain has a length of 0 or more than [`u32::MAX`], returns the error [`InvalidDomainHost`].
///
/// If the call to [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
pub fn encode_normalized_domain<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, BidiDetails), InvalidDomainHost> {
    let (changed, domain, mut bidi_details) = encode_normalized_domain_segments(value)?;

    if domain.is_empty() || domain.len() > u32::MAX as usize {
        Err(InvalidDomainHost)?
    }

    if ends_in_a_number(&domain) {
        Err(InvalidDomainHost)?;
    }

    if domain.ends_with('.') {
        bidi_details.pop();
    }

    Ok((changed, domain, bidi_details))
}

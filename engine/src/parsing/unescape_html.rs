//! [`unescape_html`] and co.

use crate::prelude::*;

/// Unescapes HTML text. Probably very bad and buggy, but SHOULD give correct outputs for in-spec inputs.
/// # Errors
/// If an unknown character reference is found, returns the error [`UnescapeHtmlError::GetHtmlCharRefError`].
///
/// If a `&` is found without a `;` afterwards, returns the error [`UnescapeHtmlError::SyntaxError`].
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(unescape_html("a&amp;b" ).unwrap(), "a&b");
/// assert_eq!(unescape_html("a&#65;b" ).unwrap(), "aAb");
/// assert_eq!(unescape_html("a&#x41;b").unwrap(), "aAb");
/// ```
pub fn unescape_html<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<Cow<'a, str>, UnescapeHtmlError> {
    let value = value.into();

    if memchr::memchr(b'&', value.as_bytes()).is_none() {
        return Ok(value);
    }

    let mut ret = String::with_capacity(value.len());
    let mut rest = &*value;

    while let Some(i) = memchr::memchr(b'&', rest.as_bytes()) {
        let a = unsafe {rest.get_unchecked(..i)};
        let b = unsafe {rest.get_unchecked(i+1..)};

        ret.push_str(a);

        let j = memchr::memchr(b';', b.as_bytes()).ok_or(SyntaxError)?;

        let c = unsafe {b.get_unchecked(..j)};
        let d = unsafe {b.get_unchecked(j+1..)};

        ret.push_str(&get_html_char_ref(c)?);

        rest = d;
    }

    ret.push_str(rest);

    Ok(ret.into())
}

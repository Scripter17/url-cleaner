//! [`unescape_html`] and co.

use thiserror::Error;

use crate::prelude::*;

/// The enum of errors that can happen when unescaping HTML text.
#[derive(Debug, Error)]
pub enum UnescapeHtmlError {
    /// Returned when a syntax error is encountered.
    #[error("Syntax error.")]
    SyntaxError,
    /// Returned when an [`GetHtmlCharRefError`] is encountered.
    #[error(transparent)]
    GetHtmlCharRefError(#[from] GetHtmlCharRefError)
}

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
pub fn unescape_html(s: &str) -> Result<String, UnescapeHtmlError> {
    let mut ret = String::new();

    let mut first = true;

    for segment in s.split('&') {
        match (first, segment.split_once(';')) {
            (true , _                     ) => {ret.push_str(segment); first=false;}
            (false, Some((char_ref, rest))) => {ret.push_str(&get_html_char_ref(char_ref)?); ret.push_str(rest);},
            (false, None                  ) => Err(UnescapeHtmlError::SyntaxError)?
        }
    }

    Ok(ret)
}

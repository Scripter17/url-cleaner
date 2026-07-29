//! Parsing.

use crate::prelude::*;

/// The enum of errors [`get_html_attribute`] can return.
#[derive(Debug, Error)]
pub enum GetHtmlAttributeError {
    /** [`SyntaxError`].       **/ #[error(transparent)] SyntaxError      (#[from] SyntaxError      ),
    /** [`UnescapeHtmlError`]. **/ #[error(transparent)] UnescapeHtmlError(#[from] UnescapeHtmlError),
}

/// [`get_html_char_ref`].
#[derive(Debug, Error)]
pub enum GetHtmlCharRefError {
    /** [`std::num::ParseIntError`].     **/ #[error(transparent)] ParseIntError   (#[from] std::num::ParseIntError    ),
    /** [`std::char::CharTryFromError`]. **/ #[error(transparent)] CharTryFromError(#[from] std::char::CharTryFromError),

    /** Unknown char name. **/ #[error("Unknown char name.")] UnknownCharName,
    /** Invalid char ref.  **/ #[error("Invalid char ref." )] InvalidCharRef ,
}

/// [`unescape_html`].
#[derive(Debug, Error)]
pub enum UnescapeHtmlError {
    /** [`SyntaxError`].         **/ #[error(transparent)] SyntaxError        (#[from] SyntaxError        ),
    /** [`GetHtmlCharRefError`]. **/ #[error(transparent)] GetHtmlCharRefError(#[from] GetHtmlCharRefError),
}

/// [`get_js_string`].
#[derive(Debug, Error)]
pub enum GetJsStringError {
    /** [`SyntaxError`] **/ #[error(transparent)] SyntaxError(#[from] SyntaxError),
}

/// Returned when a syntax error is encountered.
#[derive(Debug, Error)]
#[error("Syntax error")]
pub struct SyntaxError;

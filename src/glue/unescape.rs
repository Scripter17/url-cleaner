//! "Glue" to unescape text.

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

pub mod js;
pub mod html;

/// The unescape mode to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum UnescapeMode {
    /// If the input starts with a javascript string literal (including the quotes), return the value of that string.
    ///
    /// Stuff after the string literal is ignored.
    /// # Errors
    /// If the call to [`unescape_javascript_string_literal_prefix`] returns an error, that error is returned.
    JavascriptStringLiteralPrefix,
    /// Unescape HTML character references like `&amp;` and `&#10;`.
    /// # Errors
    /// If the call to [`unescape_html_text`] returns an error, that error is returned.
    HtmlText
}

/// The enum of errors [`UnescapeMode::unescape`] can return.
#[derive(Debug, Error)]
pub enum UnescapeError {
    /// Returned when a [`js::StringLiteralPrefixError`] is encountered.
    #[error(transparent)]
    StringLiteralPrefixError(#[from] js::StringLiteralPrefixError),
    /// Returned when a [`html::HtmlTextError`] is encountered.
    #[error(transparent)]
    HtmlTextError(#[from] html::HtmlTextError)
}

impl UnescapeMode {
    /// Run the unescape.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn unescape(&self, s: &str) -> Result<String, UnescapeError> {
        Ok(match self {
            Self::JavascriptStringLiteralPrefix => js::string_literal_prefix(s)?,
            Self::HtmlText => html::text(s)?
        })
    }
}

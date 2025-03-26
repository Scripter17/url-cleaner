//! "Glue" to unescape text.

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

mod javascript;
pub use javascript::*;
mod html;
pub use html::*;

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
    /// Returned when a [`JSSLPError`] is encountered.
    #[error(transparent)]
    JSSLPError(#[from] JSSLPError),
    /// Returned when a [`HtmlTextError`] is encountered.
    #[error(transparent)]
    HtmlTextError(#[from] HtmlTextError)
}

impl UnescapeMode {
    /// Run the unescape.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn unescape(&self, s: &str) -> Result<String, UnescapeError> {
        Ok(match self {
            Self::JavascriptStringLiteralPrefix => unescape_javascript_string_literal_prefix(s)?,
            Self::HtmlText => unescape_html_text(s)?
        })
    }
}

//! [`BetterParseOptions`].

use url::{Url, EncodingOverride, SyntaxViolation, ParseOptions, ParseError};

use crate::prelude::*;

/// [`url::ParseOptions`] with public fields and [`std::fmt::Debug`].
#[derive(Clone, Copy, Default)]
pub struct BetterParseOptions<'a> {
    /// [`ParseOptions::base_url`].
    pub base_url: Option<&'a Url>,
    /// [`ParseOptions::encoding_override`].
    pub encoding_override: EncodingOverride<'a>,
    /// [`ParseOptions::syntax_violation_callback`].
    pub syntax_violation_callback: Option<&'a dyn Fn(SyntaxViolation)>
}

impl<'a> BetterParseOptions<'a> {
    /// Set [`Self::base_url`].
    pub fn base_url(mut self, base_url: Option<&'a Url>) -> Self {
        self.base_url = base_url;
        self
    }

    /// Set [`Self::encoding_override`].
    pub fn encoding_override(mut self, encoding_override: EncodingOverride<'a>) -> Self {
        self.encoding_override = encoding_override;
        self
    }

    /// Set [`Self::syntax_violation_callback`].
    pub fn syntax_violation_callback(mut self, syntax_violation_callback: Option<&'a dyn Fn(SyntaxViolation)>) -> Self {
        self.syntax_violation_callback = syntax_violation_callback;
        self
    }

    /// Make the equivalent [`ParseOptions`], parse `input`, and convert into a [`BetterUrl`].
    /// # Errors
    /// If the call to [`ParseOptions::parse`] returns an error, that error is returned.
    pub fn parse(self, input: &str) -> Result<BetterUrl, ParseError> {
        ParseOptions::from(self)
            .parse(input)
            .map(Into::into)
    }
}

impl<'a> From<BetterParseOptions<'a>> for ParseOptions<'a> {
    fn from(value: BetterParseOptions<'a>) -> Self {
        Url::options()
            .base_url(value.base_url)
            .encoding_override(value.encoding_override)
            .syntax_violation_callback(value.syntax_violation_callback)
    }
}

impl std::fmt::Debug for BetterParseOptions<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BetterParseOptions")
            .field("base_url", &self.base_url)
            .field("encoding_override", &match self.encoding_override {
                Some(_) => "Some(_)",
                None => "None"
            })
            .field("syntax_violation_callback", &match self.syntax_violation_callback {
                Some(_) => "Some(_)",
                None => "None"
            })
            .finish()
    }
}

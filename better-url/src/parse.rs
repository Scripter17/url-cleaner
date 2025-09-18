//! [`ParseOptions`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use url::{Url, EncodingOverride, SyntaxViolation, ParseOptions, ParseError};

use crate::*;

/// [`url::ParseOptions`] with public fields and [`std::fmt::Debug`].
#[derive(Clone, Copy, Default)]
pub struct BetterParseOptions<'a> {
    /// [`ParseOptions::base_url`].
    pub base_url: Option<&'a Url>,
    /// [`ParseOptions::encoding_override`].
    pub encoding_override: EncodingOverride<'a>,
    /// [`ParseOptions::violation_fn`].
    pub violation_fn: Option<&'a dyn Fn(SyntaxViolation)>
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

    /// Set [`Self::violation_fn`].
    pub fn violation_fn(mut self, violation_fn: Option<&'a dyn Fn(SyntaxViolation)>) -> Self {
        self.violation_fn = violation_fn;
        self
    }

    /// Make the equivalent [`ParseOptions`], parse `input`, and convert into a [`BetterUrl`].
    /// # Errors
    /// If the call to [`ParseOptions::parse`] returns an error, that error is returned.
    pub fn parse(self, input: &str) -> Result<BetterUrl, ParseError> {
        Url::options()
            .base_url(self.base_url)
            .encoding_override(self.encoding_override)
            .syntax_violation_callback(self.violation_fn)
            .parse(input)
            .map(Into::into)
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
            .field("violation_fn", &match self.violation_fn {
                Some(_) => "Some(_)",
                None => "None"
            })
            .finish()
    }
}

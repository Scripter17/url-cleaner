use std::borrow::Cow;

use serde::{de::Error, Deserialize, Deserializer};
pub use regex::{Regex, RegexBuilder, Replacer};

#[derive(Clone, Debug, Deserialize)]
/// The enabled form of the wrapper around [`regex::Regex`].
/// Only the necessary methods are exposed for the sake of simplicity.
/// Note that if the `regex` feature is disabled, this struct is empty.
pub struct RegexWrapper {
    #[serde(flatten, deserialize_with = "deserialize_regex")]
    pub inner: Regex
}

#[derive(Debug, Deserialize)]
struct RegexParts {
    pattern: String,
    #[serde(default)]                case_insensitive: bool,
    #[serde(default)]                crlf: bool,
    #[serde(default)]                dot_all: bool,
    #[serde(default)]                ignore_whitespace: bool,
    #[serde(default = "newline_u8")] line_terminator: u8,
    #[serde(default)]                multi_line: bool,
    #[serde(default)]                octal: bool,
    #[serde(default)]                swap_greed: bool,
    #[serde(default = "get_true")]   unicode: bool
}

fn newline_u8() -> u8 {'\n' as u8}
fn get_true() -> bool {true}

fn deserialize_regex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Regex, D::Error> {
    let regex_parts: RegexParts = Deserialize::deserialize(deserializer)?;
    RegexBuilder::new(&regex_parts.pattern)
        .case_insensitive(regex_parts.case_insensitive)
        .crlf(regex_parts.crlf)
        .dot_matches_new_line(regex_parts.dot_all)
        .ignore_whitespace(regex_parts.ignore_whitespace)
        .line_terminator(regex_parts.line_terminator)
        .multi_line(regex_parts.multi_line)
        .octal(regex_parts.octal)
        .swap_greed(regex_parts.swap_greed)
        .unicode(regex_parts.unicode)
        .build()
        .map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}.", regex_parts.pattern)))
}

impl RegexWrapper {
    /// Wrapper for `regex::Regex::is_match`.
    pub fn is_match(&self, str: &str) -> bool {
        self.inner.is_match(str)
    }

    /// Wrapper for `regex::Regex::replace`.
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.inner.replace(haystack, rep)
    }
}

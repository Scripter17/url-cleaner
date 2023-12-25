use std::borrow::Cow;

use serde::{
    Serialize,
    ser::Serializer,
    {de::Error as DeError, Deserialize, Deserializer}
};
pub use regex::{Regex, RegexBuilder, Replacer, Error as RegexError};

/// The enabled form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Clone, Debug)]
pub struct RegexWrapper {
    inner: Regex,
    parts: RegexParts
}

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &Self) -> bool {self.parts.eq(&other.parts)}
    fn ne(&self, other: &Self) -> bool {self.parts.ne(&other.parts)}
}
impl Eq for RegexWrapper {}

/// The enabled form of `RegexParts`.
/// Contains the rules for constructing a [`Regex`]
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegexParts {
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

#[allow(dead_code)]
impl RegexParts {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            case_insensitive: false,
            crlf: false,
            dot_all: false,
            ignore_whitespace: false,
            line_terminator: newline_u8(),
            multi_line: false,
            octal: false,
            swap_greed: false,
            unicode: true
        }
    }

    /// Sets each flag to `true` if its character is in `flags`, otherwise `false`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details.
    /// I have chosen to give the octal flag `'o'` because the regex crate forgot.
    pub fn set_flags(&mut self, flags: &str) {
        self.case_insensitive =flags.contains('i');
        self.crlf             =flags.contains('R');
        self.dot_all          =flags.contains('s');
        self.ignore_whitespace=flags.contains('x');
        self.multi_line       =flags.contains('m');
        self.octal            =flags.contains('o');
        self.swap_greed       =flags.contains('U');
        self.unicode          =flags.contains('u');
    }

    /// Sets each flag to `true` if its character is in `flags`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details.
    /// I have chosen to give the octal flag `'o'` because the regex crate forgot.
    pub fn add_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive =true};
        if flags.contains('R') {self.crlf             =true};
        if flags.contains('s') {self.dot_all          =true};
        if flags.contains('x') {self.ignore_whitespace=true};
        if flags.contains('m') {self.multi_line       =true};
        if flags.contains('o') {self.octal            =true};
        if flags.contains('U') {self.swap_greed       =true};
        if flags.contains('u') {self.unicode          =true};
    }

    /// Sets each flag to `false` if its character is in `flags`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details.
    /// I have chosen to give the octal flag `'o'` because the regex crate forgot.
    pub fn remove_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive =false};
        if flags.contains('R') {self.crlf             =false};
        if flags.contains('s') {self.dot_all          =false};
        if flags.contains('x') {self.ignore_whitespace=false};
        if flags.contains('m') {self.multi_line       =false};
        if flags.contains('o') {self.octal            =false};
        if flags.contains('U') {self.swap_greed       =false};
        if flags.contains('u') {self.unicode          =false};
    }
}

fn newline_u8() -> u8 {'\n' as u8}
fn get_true() -> bool {true}

impl TryInto<Regex> for RegexParts {
    type Error = RegexError;

    fn try_into(self) -> Result<Regex, Self::Error> {
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.case_insensitive)
            .crlf(self.crlf)
            .dot_matches_new_line(self.dot_all)
            .ignore_whitespace(self.ignore_whitespace)
            .line_terminator(self.line_terminator)
            .multi_line(self.multi_line)
            .octal(self.octal)
            .swap_greed(self.swap_greed)
            .unicode(self.unicode)
            .build()
    }
}

impl TryInto<RegexWrapper> for RegexParts {
    type Error = <RegexParts as TryInto<Regex>>::Error;

    fn try_into(self) -> Result<RegexWrapper, Self::Error> {
        Ok(RegexWrapper {
            inner: self.clone().try_into()?,
            parts: self
        })
    }
}

impl<'de> Deserialize<'de> for RegexWrapper  {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = Deserialize::deserialize(deserializer)?;
        Ok(RegexWrapper {
            inner: parts.clone().try_into().map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}.", parts.pattern)))?,
            parts
        })
    }
}

impl Serialize for RegexWrapper {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.parts.serialize(serializer)
    }
}

impl Into<Regex     > for RegexWrapper {fn into(self) -> Regex      {self.inner}}
impl Into<RegexParts> for RegexWrapper {fn into(self) -> RegexParts {self.parts}}

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

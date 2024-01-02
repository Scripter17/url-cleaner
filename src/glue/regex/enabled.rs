use std::borrow::Cow;
use std::str::FromStr;
use std::convert::Infallible;

use serde::{
    Serialize,
    {de::Error as _, Deserialize, Deserializer}
};
use regex::{Regex, RegexBuilder, Replacer, Error as RegexError};

/// The enabled form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
/// Because converting a [`Regex`] to [`RegexParts`] is way more complicated than it should be, various [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] conversions are defined instead of making the filds public.
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Clone, Debug, Serialize)]
#[serde(into = "RegexParts")]
pub struct RegexWrapper {
    /// The compiled [`regex::Regex`].
    inner: Regex,
    /// The [`RegexParts`] used to construct the above [`regex::Regex`].
    /// Exists here primarily for the sake of [`Clone`].
    /// Let's see YOU implement clone for [`regex::Regex`]. It's a mess.
    parts: RegexParts
}

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &Self) -> bool {self.parts.eq(&other.parts)}
}
impl Eq for RegexWrapper {}

/// The enabled form of `RegexParts`.
/// Contains the rules for constructing a [`Regex`]
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegexParts {
    pattern: String,
    #[serde(default               , skip_serializing_if = "is_false")] case_insensitive: bool,
    #[serde(default               , skip_serializing_if = "is_false")] crlf: bool,
    #[serde(default               , skip_serializing_if = "is_false")] dot_all: bool,
    #[serde(default               , skip_serializing_if = "is_false")] ignore_whitespace: bool,
    #[serde(default = "newline_u8", skip_serializing_if = "is_nlu8" )] line_terminator: u8,
    #[serde(default               , skip_serializing_if = "is_false")] multi_line: bool,
    #[serde(default               , skip_serializing_if = "is_false")] octal: bool,
    #[serde(default               , skip_serializing_if = "is_false")] swap_greed: bool,
    #[serde(default = "get_true"  , skip_serializing_if = "is_true" )] unicode: bool
}

/// Serde doesn't have an equivalent to Clap's `default_value_t`
const fn is_false(x: &bool) -> bool {!*x} // <&bool as std::ops::Not>::not is not const.
const fn is_true(x: &bool) -> bool {*x}
const fn is_nlu8(x: &u8) -> bool {*x==b'\n'}
const fn newline_u8() -> u8 {b'\n'}
const fn get_true() -> bool {true}

impl FromStr for RegexParts {
    type Err=Infallible;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(RegexParts::new(str))
    }
}

#[allow(dead_code)]
impl RegexParts {
    /// Creates a [`RegexParts`] with the provided pattern. All other fields are set to their default values.
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

impl TryFrom<RegexParts> for Regex {
    type Error = RegexError;

    fn try_from(value: RegexParts) -> Result<Self, Self::Error> {
        RegexBuilder::new(&value.pattern)
            .case_insensitive(value.case_insensitive)
            .crlf(value.crlf)
            .dot_matches_new_line(value.dot_all)
            .ignore_whitespace(value.ignore_whitespace)
            .line_terminator(value.line_terminator)
            .multi_line(value.multi_line)
            .octal(value.octal)
            .swap_greed(value.swap_greed)
            .unicode(value.unicode)
            .build()
    }
}

impl TryFrom<RegexParts> for RegexWrapper {
    type Error = <RegexParts as TryInto<Regex>>::Error;

    fn try_from(value: RegexParts) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value.clone().try_into()?,
            parts: value
        })
    }
}

impl From<RegexWrapper> for Regex      {fn from(value: RegexWrapper) -> Self {value.inner}}
impl From<RegexWrapper> for RegexParts {fn from(value: RegexWrapper) -> Self {value.parts}}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = crate::glue::string_or_struct(deserializer)?;
        Ok(RegexWrapper {
            inner: parts.clone().try_into().map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}.", parts.pattern)))?,
            parts
        })
    }
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

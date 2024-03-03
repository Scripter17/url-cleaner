use std::str::FromStr;

use serde::{Serialize, Deserialize};
use regex::{Regex, RegexBuilder, Error as RegexError};
use regex_syntax::{ParserBuilder, Error as RegexSyntaxError};

use crate::string_or_struct_magic;

/// The enabled form of `RegexParts`.
/// Contains the rules for constructing a [`Regex`].
/// The pattern can be invalid. It only needs to be valid when the [`super::RegexWrapper`] it turns into is created.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(remote = "Self")]
pub struct RegexParts {
    /// The pattern passed into [`RegexBuilder::new`].
    pattern: String,
    #[serde(flatten)]
    config: RegexConfig
}

string_or_struct_magic!(RegexParts);

impl AsRef<RegexConfig> for RegexParts {
    fn as_ref(&self) -> &RegexConfig {
        &self.config
    }
}

/// The configuration determining how a regular expression works.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegexConfig {
    /// The flag that decides if [`RegexBuilder::case_insensitive`] is set. Defaults to `false`. This flags character is `'i'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub case_insensitive: bool,
    /// The flag that decides if [`RegexBuilder::crlf`] is set. Defaults to `false`. This flags character is `'R'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub crlf: bool,
    /// The flag that decides if [`RegexBuilder::dot_matches_new_line`] is set. Defaults to `false`. This flags character is `'s'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub dot_matches_new_line: bool,
    /// The flag that decides if [`RegexBuilder::ignore_whitespace`] is set. Defaults to `false`. This flags character is `'x'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub ignore_whitespace: bool,
    /// The flag that decides if [`RegexBuilder::line_terminator`] is set. Defaults to `b'\n'` (`10`).
    #[serde(default = "newline_u8", skip_serializing_if = "is_nlu8" )] pub line_terminator: u8,
    /// The flag that decides if [`RegexBuilder::multi_line`] is set. Defaults to `false`. This flags character is `'m'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub multi_line: bool,
    /// The flag that decides if [`RegexBuilder::octal`] is set. Defaults to `false`. This flags character is `'o'` because the `regex` crate forgot and I said so.
    #[serde(default               , skip_serializing_if = "is_false")] pub octal: bool,
    /// The flag that decides if [`RegexBuilder::swap_greed`] is set. Defaults to `false`. This flags character is `'U'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub swap_greed: bool,
    /// The flag that decides if [`RegexBuilder::unicode`] is set. Defaults to `true`. This flags character is `'u'`.
    #[serde(default = "get_true"  , skip_serializing_if = "is_true" )] pub unicode: bool
}

// Serde helper functions
const fn is_false(x: &bool) -> bool {!*x} // <&bool as std::ops::Not>::not is not const.
const fn is_true(x: &bool) -> bool {*x}
const fn is_nlu8(x: &u8) -> bool {*x==b'\n'}
const fn newline_u8() -> u8 {b'\n'}
const fn get_true() -> bool {true}

impl FromStr for RegexParts {
    type Err=Box<RegexSyntaxError>;

    /// Simply treats the string as a regex and defaults the config.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[allow(dead_code)]
impl RegexParts {
    /// Creates a [`RegexParts`] with the provided pattern. The config is set to its default value.
    /// # Errors
    /// If the pattern is invalid, the error encountered by the parser is returned.
    /// The error is boxed because it's massive.
    pub fn new(pattern: &str) -> Result<Self, Box<RegexSyntaxError>> {
        Self::new_with_config(pattern, RegexConfig::default())
    }

    /// Creates a [`RegexParts`] with the provided pattern and config.
    /// # Errors
    /// If the pattern is invalid, the error encountered by the parser is returned.
    /// The error is boxed because it's massive.
    pub fn new_with_config(pattern: &str, config: RegexConfig) -> Result<Self, Box<RegexSyntaxError>> {
        let ret=Self {
            pattern: pattern.to_string(),
            config
        };
        Into::<ParserBuilder>::into(&ret.config).build().parse(&ret.pattern).map_err(Box::new)?;
        Ok(ret)
    }

    /// Creates the regex.
    /// # Errors
    /// If the regex is larger than the maximum DFA size, this will error.
    pub fn build(&self) -> Result<Regex, RegexError> {
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.config.case_insensitive)
            .crlf(self.config.crlf)
            .dot_matches_new_line(self.config.dot_matches_new_line)
            .ignore_whitespace(self.config.ignore_whitespace)
            .line_terminator(self.config.line_terminator)
            .multi_line(self.config.multi_line)
            .octal(self.config.octal)
            .swap_greed(self.config.swap_greed)
            .unicode(self.config.unicode)
            .build()
    }
}

impl From<RegexParts> for RegexBuilder {
    fn from(value: RegexParts) -> Self {
        (&value).into()
    }
}

impl From<&RegexParts> for RegexBuilder {
    fn from(value: &RegexParts) -> Self {
        let mut ret=Self::new(&value.pattern);
        ret.case_insensitive(value.config.case_insensitive)
            .crlf(value.config.crlf)
            .dot_matches_new_line(value.config.dot_matches_new_line)
            .ignore_whitespace(value.config.ignore_whitespace)
            .line_terminator(value.config.line_terminator)
            .multi_line(value.config.multi_line)
            .octal(value.config.octal)
            .swap_greed(value.config.swap_greed)
            .unicode(value.config.unicode);
        ret
    }
}

impl From<&RegexConfig> for ParserBuilder {
    fn from(value: &RegexConfig) -> Self {
        let mut ret=Self::new();
        ret.case_insensitive(value.case_insensitive)
            .crlf(value.crlf)
            .dot_matches_new_line(value.dot_matches_new_line)
            .ignore_whitespace(value.ignore_whitespace)
            .line_terminator(value.line_terminator)
            .multi_line(value.multi_line)
            .octal(value.octal)
            .swap_greed(value.swap_greed)
            .utf8(value.unicode);
        ret
    }
}

impl From<RegexConfig> for ParserBuilder {
    fn from(value: RegexConfig) -> Self {
        (&value).into()
    }
}

impl TryFrom<&RegexParts> for Regex {
    type Error = RegexError;
    
    fn try_from(value: &RegexParts) -> Result<Self, Self::Error> {
        value.build()
    }
}

impl TryFrom<RegexParts> for Regex {
    type Error = RegexError;
    
    fn try_from(value: RegexParts) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl Default for RegexConfig {
    fn default() -> Self {
        Self {
            case_insensitive    : false,
            crlf                : false,
            dot_matches_new_line: false,
            ignore_whitespace   : false,
            line_terminator     : b'\n',
            multi_line          : false,
            octal               : false,
            swap_greed          : false,
            unicode             : true
        }
    }
}

impl RegexConfig {
    /// Sets each flag to `true` if its character is in `flags`, otherwise `false`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details on which character is which flag.
    /// I have chosen to give the octal flag `'o'` because the `regex` crate forgot.
    pub fn set_flags(&mut self, flags: &str) {
        self.case_insensitive    =flags.contains('i');
        self.crlf                =flags.contains('R');
        self.dot_matches_new_line=flags.contains('s');
        self.ignore_whitespace   =flags.contains('x');
        self.multi_line          =flags.contains('m');
        self.octal               =flags.contains('o');
        self.swap_greed          =flags.contains('U');
        self.unicode             =flags.contains('u');
    }

    /// Sets each flag to `true` if its character is in `flags`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details on which character is which flag.
    /// I have chosen to give the octal flag `'o'` because the `regex` crate forgot.
    pub fn add_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive    =true;}
        if flags.contains('R') {self.crlf                =true;}
        if flags.contains('s') {self.dot_matches_new_line=true;}
        if flags.contains('x') {self.ignore_whitespace   =true;}
        if flags.contains('m') {self.multi_line          =true;}
        if flags.contains('o') {self.octal               =true;}
        if flags.contains('U') {self.swap_greed          =true;}
        if flags.contains('u') {self.unicode             =true;}
    }

    /// Sets each flag to `false` if its character is in `flags`.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details on which character is which flag.
    /// I have chosen to give the octal flag `'o'` because the `regex` crate forgot.
    pub fn remove_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive    =false;}
        if flags.contains('R') {self.crlf                =false;}
        if flags.contains('s') {self.dot_matches_new_line=false;}
        if flags.contains('x') {self.ignore_whitespace   =false;}
        if flags.contains('m') {self.multi_line          =false;}
        if flags.contains('o') {self.octal               =false;}
        if flags.contains('U') {self.swap_greed          =false;}
        if flags.contains('u') {self.unicode             =false;}
    }

    /// Returns the flags as a string. `regex_parts.set_flags(&regex_parts.get_flags())` does nothing.
    /// See [the `regex` crate's docs](https://docs.rs/regex/latest/regex/index.html#grouping-and-flags) for details on which character is which flag.
    /// I have chosen to give the octal flag `'o'` because the `regex` crate forgot.
    #[must_use]
    pub fn get_flags(&self) -> String {
        let mut ret=String::new();
        if self.case_insensitive    {ret.push('i');}
        if self.crlf                {ret.push('R');}
        if self.dot_matches_new_line{ret.push('s');}
        if self.ignore_whitespace   {ret.push('x');}
        if self.multi_line          {ret.push('m');}
        if self.octal               {ret.push('o');}
        if self.swap_greed          {ret.push('U');}
        if self.unicode             {ret.push('u');}
        ret
    }
}

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use regex::{Regex, RegexBuilder};
use regex_syntax::{ParserBuilder, Parser, Error as RegexSyntaxError};

/// Contains the rules for constructing a [`Regex`].
/// 
/// The pattern is guaranteed to be valid.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(remote = "Self")]
pub struct RegexParts {
    /// The pattern passed into [`RegexBuilder::new`].
    pattern: String,
    /// The configuration flags.
    #[serde(flatten)]
    config: RegexConfig
}

impl FromStr for RegexParts {
    type Err=Box<RegexSyntaxError>;

    /// Simply treats the string as a regex and defaults the config.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for RegexParts {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        Self::from_str(s)
    }
}

crate::util::string_or_struct_magic!(RegexParts);

/// The configuration determining how a regular expression works.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegexConfig {
    /// The value passed into [`RegexBuilder::case_insensitive`]. Defaults to `false`. This flags character is `'i'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub case_insensitive: bool,
    /// The value passed into [`RegexBuilder::crlf`]. Defaults to `false`. This flags character is `'R'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub crlf: bool,
    /// The value passed into [`RegexBuilder::dot_matches_new_line`]. Defaults to `false`. This flags character is `'s'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub dot_matches_new_line: bool,
    /// The value passed into [`RegexBuilder::ignore_whitespace`]. Defaults to `false`. This flags character is `'x'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub ignore_whitespace: bool,
    /// The value passed into [`RegexBuilder::line_terminator`]. Defaults to `b'\n'` (`10`).
    #[serde(default = "newline_u8", skip_serializing_if = "is_nlu8" )] pub line_terminator: u8,
    /// The value passed into [`RegexBuilder::multi_line`]. Defaults to `false`. This flags character is `'m'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub multi_line: bool,
    /// The value passed into [`RegexBuilder::octal`]. Defaults to `false`. This flags character is `'o'` because the `regex` crate forgot and I said so.
    #[serde(default               , skip_serializing_if = "is_false")] pub octal: bool,
    /// The value passed into [`RegexBuilder::swap_greed`]. Defaults to `false`. This flags character is `'U'`.
    #[serde(default               , skip_serializing_if = "is_false")] pub swap_greed: bool,
    /// The value passed into [`RegexBuilder::unicode`]. Defaults to `true`. This flags character is `'u'`.
    #[serde(default = "get_true"  , skip_serializing_if = "is_true" )] pub unicode: bool
}

// Serde helper functions
const fn is_false(x: &bool) -> bool {!*x} // <&bool as std::ops::Not>::not is not const.
const fn is_true(x: &bool) -> bool {*x}
const fn is_nlu8(x: &u8) -> bool {*x==b'\n'}
const fn newline_u8() -> u8 {b'\n'}
const fn get_true() -> bool {true}

#[allow(dead_code)]
impl RegexParts {
    /// Creates a [`RegexParts`] with the provided pattern. The config is set to its default value.
    /// # Errors
    /// If the pattern is invalid, the error encountered by the parser is returned.
    /// The error is boxed because it's massive.
    pub fn new(pattern: &str) -> Result<Self, Box<RegexSyntaxError>> {
        Self::new_with_config(pattern, RegexConfig::default())
    }

    /// Getter for the pattern.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Getter for the config.
    pub fn config(&self) -> &RegexConfig {
        &self.config
    }

    /// Creates a [`RegexParts`] with the provided pattern and config.
    /// # Errors
    /// If the pattern is invalid, the error encountered by the parser is returned.
    /// The error is boxed because it's massive.
    pub fn new_with_config(pattern: &str, config: RegexConfig) -> Result<Self, Box<RegexSyntaxError>> {
        config.build_parser().parse(pattern).map_err(Box::new)?;
        Ok(Self {
            pattern: pattern.to_string(),
            config
        })
    }

    fn make_builder(&self) -> RegexBuilder {
        let mut ret=RegexBuilder::new(&self.pattern);
        ret.case_insensitive(self.config.case_insensitive)
            .crlf(self.config.crlf)
            .dot_matches_new_line(self.config.dot_matches_new_line)
            .ignore_whitespace(self.config.ignore_whitespace)
            .line_terminator(self.config.line_terminator)
            .multi_line(self.config.multi_line)
            .octal(self.config.octal)
            .swap_greed(self.config.swap_greed)
            .unicode(self.config.unicode);
        ret
    }

    /// Creates the regex.
    /// # Errors
    /// If the regex is larger than the maximum DFA size, this will error.
    pub fn build(&self) -> Result<Regex, regex::Error> {
        self.make_builder().build()
    }
}

impl TryFrom<&RegexParts> for Regex {
    type Error = regex::Error;
    
    fn try_from(value: &RegexParts) -> Result<Self, Self::Error> {
        value.build()
    }
}

impl TryFrom<RegexParts> for Regex {
    type Error = regex::Error;
    
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

    fn make_parser_builder(&self) -> ParserBuilder {
        let mut ret=ParserBuilder::new();
        ret.case_insensitive(self.case_insensitive)
            .crlf(self.crlf)
            .dot_matches_new_line(self.dot_matches_new_line)
            .ignore_whitespace(self.ignore_whitespace)
            .line_terminator(self.line_terminator)
            .multi_line(self.multi_line)
            .octal(self.octal)
            .swap_greed(self.swap_greed)
            .utf8(self.unicode);
        ret
    }

    fn build_parser(&self) -> Parser {
        self.make_parser_builder().build()
    }
}

//! Parts of a [`Regex`] for easy portability and lazy compilation.

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use regex::{Regex, RegexBuilder};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use super::RegexWrapper;

use crate::types::*;
use crate::util::*;

/// Config on how to make a [`Regex`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct RegexParts {
    /// The regex pattern to use.
    ///
    /// For performance, validity is only checked at [`Self::build`] time.
    pub pattern: String,
    /// Config to pass into [`RegexBuilder`].
    #[serde(flatten)]
    pub config: RegexConfig
}

impl RegexParts {
    /// Compile the regex.
    /// # Errors
    /// If the call to [`RegexBuilder::build`] returns an error, that error is returned.
    pub fn build(&self) -> Result<Regex, regex::Error> {
        debug!(RegexParts::build, self);
        RegexBuilder::new(&self.pattern)
            .case_insensitive    (self.config.case_insensitive    )
            .crlf                (self.config.crlf                )
            .dot_matches_new_line(self.config.dot_matches_new_line)
            .ignore_whitespace   (self.config.ignore_whitespace   )
            .line_terminator     (self.config.line_terminator     )
            .multi_line          (self.config.multi_line          )
            .octal               (self.config.octal               )
            .swap_greed          (self.config.swap_greed          )
            .unicode             (self.config.unicode             )
            .build()
    }
}

crate::util::string_or_struct_magic!(RegexParts);

impl FromStr for RegexParts {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for RegexParts {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<String> for RegexParts {
    fn from(value: String) -> Self {
        Self {
            pattern: value,
            config: Default::default()
        }
    }
}

impl Suitability for RegexParts {
    fn assert_suitability(&self, config: &Cleaner) {
        self.build().unwrap_or_else(|_| panic!("Regex to be buildable: {self:?}"));
        self.config.assert_suitability(config);
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

/// Configuration given to [`RegexBuilder`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub struct RegexConfig {
    /// The value passed to [`RegexBuilder::case_insensitive`].
    ///
    /// The character for this flag is `i`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub case_insensitive    : bool,
    /// The value passed to [`RegexBuilder::crlf`].
    ///
    /// The character for this flag is `R`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub crlf                : bool,
    /// The value passed to [`RegexBuilder::dot_matches_new_line`].
    ///
    /// The character for this flag is `s`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub dot_matches_new_line: bool,
    /// The value passed to [`RegexBuilder::ignore_whitespace`].
    ///
    /// The character for this flag is `x`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub ignore_whitespace   : bool,
    /// The value passed to [`RegexBuilder::line_terminator`].
    ///
    /// Defaults to `b'\n'` (`10`).
    #[serde(default = "newline_u8", skip_serializing_if = "is_nlu8" )] pub line_terminator     : u8,
    /// The value passed to [`RegexBuilder::multi_line`].
    ///
    /// The character for this flag is `m`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub multi_line          : bool,
    /// The value passed to [`RegexBuilder::octal`].
    ///
    /// The character for this flag is `o`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub octal               : bool,
    /// The value passed to [`RegexBuilder::swap_greed`].
    ///
    /// The character for this flag is `U`.
    ///
    /// Defaults to [`false`].
    #[serde(default               , skip_serializing_if = "is_false")] pub swap_greed          : bool,
    /// The value passed to [`RegexBuilder::unicode`].
    ///
    /// The character for this flag is `u`.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true"  , skip_serializing_if = "is_true" )] pub unicode             : bool
}

/// Serde helper function.
const fn is_nlu8(x: &u8) -> bool {*x==b'\n'}
/// Serde helper function.
const fn newline_u8() -> u8 {b'\n'}

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
    /// Sets the flags whose characters are in `flags` and unsets the flags whose characters aren't in `flags`.
    ///
    /// Flags do not have to be "in order", as returned by [`Self::get_flags`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::prelude::*;
    ///
    /// let mut config = RegexConfig::default();
    ///
    /// config.set_flags("i");
    ///
    /// assert!( config.case_insensitive);
    /// assert!(!config.unicode); // Unicode is enabled by default, but was disabled.
    /// ```
    pub fn set_flags(&mut self, flags: &str) {
        self.case_insensitive     = flags.contains('i');
        self.crlf                 = flags.contains('R');
        self.dot_matches_new_line = flags.contains('s');
        self.ignore_whitespace    = flags.contains('x');
        self.multi_line           = flags.contains('m');
        self.octal                = flags.contains('o');
        self.swap_greed           = flags.contains('U');
        self.unicode              = flags.contains('u');
    }

    /// Sets the flags whose characters are in `flags` and leaves the others unchanged.
    ///
    /// Flags do not have to be "in order", as returned by [`Self::get_flags`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::prelude::*;
    ///
    /// let mut config = RegexConfig::default();
    ///
    /// config.add_flags("i");
    ///
    /// assert!(config.case_insensitive);
    /// assert!(config.unicode); // Was enabled by default and left unchanged.
    /// ```
    pub fn add_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive     = true;}
        if flags.contains('R') {self.crlf                 = true;}
        if flags.contains('s') {self.dot_matches_new_line = true;}
        if flags.contains('x') {self.ignore_whitespace    = true;}
        if flags.contains('m') {self.multi_line           = true;}
        if flags.contains('o') {self.octal                = true;}
        if flags.contains('U') {self.swap_greed           = true;}
        if flags.contains('u') {self.unicode              = true;}
    }

    /// Unsets the flags whose characters are in `flags` and leaves the others unchanged.
    ///
    /// Flags do not have to be "in order", as returned by [`Self::get_flags`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::prelude::*;
    ///
    /// let mut config = RegexConfig::default();
    ///
    /// config.add_flags("i");
    /// config.remove_flags("u");
    ///
    /// assert!( config.case_insensitive); // Set by the call to [`Self::add_flags`] and left unchanged by the call to [`Self::remove_flags`].
    /// assert!(!config.unicode); // Set by default, left unchanged by the call to [`Self::add_flags`], and unset by the call to [`Self::remove_flags`].
    /// ```
    pub fn remove_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive     = false;}
        if flags.contains('R') {self.crlf                 = false;}
        if flags.contains('s') {self.dot_matches_new_line = false;}
        if flags.contains('x') {self.ignore_whitespace    = false;}
        if flags.contains('m') {self.multi_line           = false;}
        if flags.contains('o') {self.octal                = false;}
        if flags.contains('U') {self.swap_greed           = false;}
        if flags.contains('u') {self.unicode              = false;}
    }

    /// Gets the set flags.
    ///
    /// Exact order is not officially stable, but is unlikely to ever be changed from `iRsxmoUu` and VERY unlikely to ever be changed after that.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::prelude::*;
    ///
    /// let mut config = RegexConfig::default();
    ///
    /// assert_eq!(config.get_flags(), "u");
    ///
    /// config.case_insensitive = true;
    ///
    /// assert_eq!(config.get_flags(), "iu");
    /// ```
    #[must_use]
    pub fn get_flags(&self) -> String {
        let mut ret=String::new();
        if self.case_insensitive     {ret.push('i');}
        if self.crlf                 {ret.push('R');}
        if self.dot_matches_new_line {ret.push('s');}
        if self.ignore_whitespace    {ret.push('x');}
        if self.multi_line           {ret.push('m');}
        if self.octal                {ret.push('o');}
        if self.swap_greed           {ret.push('U');}
        if self.unicode              {ret.push('u');}
        ret
    }
}

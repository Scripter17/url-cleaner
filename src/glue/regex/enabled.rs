use std::borrow::Cow;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::{de::Error, Deserialize, Deserializer};
pub use regex::{Regex, RegexBuilder, Replacer, Error as RegexError};

/// The enabled form of the wrapper around [`regex::Regex`].
/// Only the necessary methods are exposed for the sake of simplicity.
/// Note that if the `regex` feature is disabled, this struct is empty.
#[derive(Clone, Debug)]
pub struct RegexWrapper {
    inner: Regex,
    parts: RegexParts
}

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }

    fn ne(&self, other: &Self) -> bool {
        self.parts.ne(&other.parts)
    }
}

impl Eq for RegexWrapper {}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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
    pub fn set_flags(&mut self, flags: &str) {
        self.case_insensitive =flags.contains('i');
        self.crlf             =flags.contains('R');
        self.dot_all          =flags.contains('s');
        self.ignore_whitespace=flags.contains('x');
        self.multi_line       =flags.contains('m');
        self.swap_greed       =flags.contains('U');
        self.unicode          =flags.contains('u');
    }
    pub fn add_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive =true};
        if flags.contains('R') {self.crlf             =true};
        if flags.contains('s') {self.dot_all          =true};
        if flags.contains('x') {self.ignore_whitespace=true};
        if flags.contains('m') {self.multi_line       =true};
        if flags.contains('U') {self.swap_greed       =true};
        if flags.contains('u') {self.unicode          =true};
    }
    pub fn remove_flags(&mut self, flags: &str) {
        if flags.contains('i') {self.case_insensitive =false};
        if flags.contains('R') {self.crlf             =false};
        if flags.contains('s') {self.dot_all          =false};
        if flags.contains('x') {self.ignore_whitespace=false};
        if flags.contains('m') {self.multi_line       =false};
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
        let inner=parts.clone().try_into().map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}.", parts.pattern)))?;
        Ok(RegexWrapper {
            inner: inner,
            parts: parts
        })
    }
}

impl Serialize for RegexWrapper {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Regex", 10)?;
        state.serialize_field("pattern"          , &self.parts.pattern          )?;
        state.serialize_field("case_insensitive" , &self.parts.case_insensitive )?;
        state.serialize_field("crlf"             , &self.parts.crlf             )?;
        state.serialize_field("dot_all"          , &self.parts.dot_all          )?;
        state.serialize_field("ignore_whitespace", &self.parts.ignore_whitespace)?;
        state.serialize_field("line_terminator"  , &self.parts.line_terminator  )?;
        state.serialize_field("multi_line"       , &self.parts.multi_line       )?;
        state.serialize_field("octal"            , &self.parts.octal            )?;
        state.serialize_field("swap_greed"       , &self.parts.swap_greed       )?;
        state.serialize_field("unicode"          , &self.parts.unicode          )?;
        state.end()
    }
}

impl Into<Regex> for RegexWrapper {
    fn into(self) -> Regex {
        self.inner
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

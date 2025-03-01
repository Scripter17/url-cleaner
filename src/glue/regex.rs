//! Provides [`RegexWrapper`], a lazy, serializable/deserializable, and deconstructable wrapper around [`Regex`].
//! 
//! Enabled by the `regex` feature flag.

use std::str::FromStr;
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::types::*;

mod regex_parts;
pub use regex_parts::*;

/// A wrapper around both a [`OnceLock`] of a [`Regex`] and a [`RegexParts`].
/// 
/// Both are included to allow both lazy compilation and turning a [`Self`] back into a [`RegexParts`].
/// Unfortunately, as they need to always be the same value, the fields of this struct are private.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "RegexParts", into = "RegexParts")]
pub struct RegexWrapper {
    /// Allows the [`Regex`] to only be constructed when needed.
    regex: OnceLock<Regex>,
    /// Instructions for how to create the [`Regex`] to put in [`Self::regex`].
    parts: RegexParts
}

impl From<RegexParts> for RegexWrapper {
    fn from(parts: RegexParts) -> Self {
        Self {
            regex: OnceLock::new(),
            parts
        }
    }
}

impl FromStr for RegexWrapper {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegexParts::from(s).into())
    }
}

impl From<&str> for RegexWrapper {
    fn from(s: &str) -> Self {
        RegexParts::from(s).into()
    }
}

impl PartialEq for RegexWrapper {
    /// Simply calls [`RegexParts::eq`].
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }
}
impl Eq for RegexWrapper {}

impl From<RegexWrapper> for RegexParts {
    fn from(value: RegexWrapper) -> Self {
        value.parts
    }
}

impl AsRef<RegexParts> for RegexWrapper {
    fn as_ref(&self) -> &RegexParts {
        &self.parts
    }
}

impl TryFrom<&RegexWrapper> for Regex {
    type Error = regex::Error;

    /// [`RegexParts::build`].
    fn try_from(value: &RegexWrapper) -> Result<Self, Self::Error> {
        value.get_regex().cloned()
    }
}

impl TryFrom<RegexWrapper> for Regex {
    type Error = regex::Error;

    /// [`RegexParts::build`].
    fn try_from(value: RegexWrapper) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl RegexWrapper {
    /// Gets the cached compiled regex and compiles it first if it's not already cached.
    /// # Errors
    /// Although regexes are ensured to be syntactically valid when a [`Self`] is created, it is possible for actually compiling a regex to result in a DFA bigger than the default limit in the [`regex`] crate which causes an error.
    /// 
    /// For details, please see the regex crate's documentation on [untrusted patterns](https://docs.rs/regex/latest/regex/index.html#untrusted-patterns) for details.
    pub fn get_regex(&self) -> Result<&Regex, regex::Error> {
        if let Some(regex) = self.regex.get() {
            Ok(regex)
        } else {
            let temp = self.parts.build()?;
            Ok(self.regex.get_or_init(|| temp))
        }
    }

    /// Verifies at suitability test time that the regex actually compiles.
    /// # Panics
    /// If the regex doesn't compile, panics.
    pub fn is_suitable_for_release(&self, _config: &Config) -> bool {
        self.get_regex().expect("The regex to compile.");
        true
    }
}

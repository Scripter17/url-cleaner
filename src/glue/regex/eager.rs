use std::borrow::Cow;
use std::str::FromStr;
use std::fmt;

use serde::{
    Serialize,
    de::Error as _, Deserialize, Deserializer
};

use regex::{Regex, Replacer, Error as RegexError};
use super::RegexParts;

/// The enabled and not lazy form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
/// Because converting a [`Regex`] to [`RegexParts`] is way more complicated than it should be, various [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] conversions are defined instead of making the fields public.
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Clone, Serialize)]
#[serde(into = "RegexParts")]
pub struct RegexWrapper {
    regex: Regex,
    parts: RegexParts
}

impl RegexWrapper {
    /// Wrapper for `Regex::is_match`.
    pub fn is_match(&self, str: &str) -> bool {
        self.regex.is_match(str)
    }

    /// Wrapper for `Regex::replace`.
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.regex.replace(haystack, rep)
    }
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = crate::glue::string_or_struct(deserializer)?;
        Ok(RegexWrapper {
            regex: parts.clone().try_into().map_err(|_| D::Error::custom(format!("Invalid regex pattern: {:?}.", parts.pattern)))?,
            parts
        })
    }
}

impl TryFrom<RegexParts> for RegexWrapper {
    type Error = <RegexParts as TryInto<Regex>>::Error;

    fn try_from(parts: RegexParts) -> Result<Self, Self::Error> {
        Ok(Self {
            regex: parts.clone().try_into()?,
            parts
        })
    }
}

impl AsRef<Regex> for RegexWrapper {
    fn as_ref(&self) -> &Regex {
        &self.regex
    }
}

impl FromStr for RegexWrapper {
    type Err = RegexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegexParts::new(s).try_into()
    }
}

impl PartialEq for RegexWrapper {
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

impl fmt::Debug for RegexWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(<RegexWrapper as AsRef<Regex>>::as_ref(self), f)
    }
}

impl fmt::Display for RegexWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(<RegexWrapper as AsRef<Regex>>::as_ref(self), f)
    }
}

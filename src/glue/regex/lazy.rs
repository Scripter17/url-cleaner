use std::borrow::Cow;
use std::sync::Arc;
use std::str::FromStr;
use std::fmt;

use serde::{
    Serialize,
    de::{Error as _, Deserialize, Deserializer}
};
use regex::{Regex, Replacer, Error as RegexError};
use oncemutex::OnceMutex;

use super::RegexParts;

/// The enabled and lazy form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// The contained [`RegexParts`] is only turned into a [`Regex`] and cached when needed.
/// Idea (and various implementation details) stolen from the [`regex_cache`](https://docs.rs/regex-cache/latest/regex_cache/) crate.
/// Note that if the `regex` feature is disabled, this struct is empty and all provided functions will always panic.
/// Because converting a [`Regex`] to [`RegexParts`] is way more complicated than it should be, various [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] conversions are defined instead of making the fields public.
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Clone, Serialize)]
#[serde(into = "RegexParts")]
pub struct RegexWrapper {
    regex: Arc<OnceMutex<Option<Regex>>>,
    parts: RegexParts
}

impl RegexWrapper {
    /// Wrapper for `Regex::is_match`.
    pub fn is_match(&self, str: &str) -> bool {
        self.as_ref().is_match(str)
    }

    /// Wrapper for `Regex::replace`.
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.as_ref().replace(haystack, rep)
    }
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = crate::glue::string_or_struct(deserializer)?;
        if parts.validate() {
            Ok(Self {
                parts,
                regex: Arc::new(OnceMutex::new(None))
            })
        } else {
            Err(D::Error::custom(format!("Invalid regex pattern: {:?}.", parts.pattern)))
        }
    }
}

impl TryFrom<RegexParts> for RegexWrapper {
    type Error = RegexError;
    
    fn try_from(parts: RegexParts) -> Result<Self, Self::Error> {
        if parts.validate() {
            Ok(Self {
                parts,
                regex: Arc::new(OnceMutex::new(None))
            })
        } else {
            parts.try_into() // Yeah sure that works.
        }
    }
}

impl AsRef<Regex> for RegexWrapper {
    fn as_ref(&self) -> &Regex {
        if let Some(mut guard) = self.regex.lock() {
            *guard = Some(self.parts.build().expect("The contained RegexParts's pattern to be confirmed valid during `<RegexWrapper as TryFrom<RegexPaers>>:try_from.`"));
        }
        (*self.regex).as_ref().expect("The contained Option<Regex> to have just been set to Some.")
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

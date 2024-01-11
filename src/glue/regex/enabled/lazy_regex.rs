//! Idea (and various implementation details) stolen from the [`regex_cache`](https://docs.rs/regex-cache/latest/regex_cache/) crate.
//! Reimplemented here primarily for the sake of supporting more recent regex flags.

use std::sync::Arc;
use std::ops::Deref;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::{
    Serialize,
    de::{Error as _, Deserialize, Deserializer}
};
use regex::{Regex, Error as RegexError};
use oncemutex::OnceMutex;

use super::RegexParts;

/// A Regex that is only compiled when needed.
/// Idea (and various implementation details) stolen from the [`regex_cache`](https://docs.rs/regex-cache/latest/regex_cache/) crate.
#[derive(Clone, Serialize)]
#[serde(into = "RegexParts")]
pub struct LazyRegex {
    #[serde(flatten)]
    parts: RegexParts,
    #[serde(skip)]
    regex: Arc<OnceMutex<Option<Regex>>>
}

impl<'de> Deserialize<'de> for LazyRegex {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = crate::glue::string_or_struct(deserializer)?;
        match parts.validate() {
            Ok(_) => Ok(Self {
                parts,
                regex: Arc::new(OnceMutex::new(None))
            }),
            Err(err) => Err(D::Error::custom(err))
        }
    }
}

impl TryFrom<RegexParts> for LazyRegex {
    type Error = RegexError;
    
    fn try_from(parts: RegexParts) -> Result<Self, Self::Error> {
        match parts.validate() {
            Ok(_) => Ok(Self {
                parts,
                regex: Arc::new(OnceMutex::new(None))
            }),
            Err(err) => Err(Self::Error::Syntax(err))
        }
    }
}

impl Deref for LazyRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<Regex> for LazyRegex {
    fn as_ref(&self) -> &Regex {
        if let Some(mut guard) = self.regex.lock() {
            *guard = Some(self.parts.build().expect("The contained RegexParts's pattern to be confirmed valid during `<LazyRegex as TryFrom<RegexPaers>>:try_from.`"));
        }
        (*self.regex).as_ref().expect("The contained Option<Regex> to have just been set to Some.")
    }
}


impl Debug for LazyRegex {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl Display for LazyRegex {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&**self, f)
    }
}

impl FromStr for LazyRegex {
    type Err = RegexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegexParts::new(s).try_into()
    }
}

impl PartialEq for LazyRegex {
    fn eq(&self, other: &Self) -> bool {
        self.parts.eq(&other.parts)
    }
}
impl Eq for LazyRegex {}

impl From<LazyRegex> for RegexParts {
    fn from(value: LazyRegex) -> Self {
        value.parts
    }
}

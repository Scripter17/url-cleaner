mod regex_parts;
pub use regex_parts::*;

use std::str::FromStr;
use std::sync::OnceLock;
use std::ops::Deref;

use serde::{Serialize, Deserialize, Deserializer};

use regex_syntax::Error as RegexSyntaxError;
use regex::Regex;

/// The enabled and not lazy form of the wrapper around [`regex::Regex`] and [`RegexParts`].
/// Because converting a [`Regex`] to [`RegexParts`] is way more complicated than it should be, various [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] conversions are defined instead of making the fields public.
#[derive(Clone, Debug, Serialize)]
#[serde(into = "RegexParts")]
pub struct RegexWrapper {
    regex: OnceLock<Regex>,
    parts: RegexParts
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let parts: RegexParts = crate::glue::string_or_struct(deserializer)?;
        Ok(Self::from(parts))
    }
}

impl From<RegexParts> for RegexWrapper {
    fn from(parts: RegexParts) -> Self {
        Self {
            regex: OnceLock::new(),
            parts
        }
    }
}

impl Deref for RegexWrapper {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        self.regex.get_or_init(|| (&self.parts).into())
    }
}

impl AsRef<Regex> for RegexWrapper {
    fn as_ref(&self) -> &Regex {
        self
    }
}

impl FromStr for RegexWrapper {
    type Err = Box<RegexSyntaxError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegexParts::new(s).map(Into::into)
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

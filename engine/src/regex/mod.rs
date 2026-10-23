//! [`LazyRegex`] and co.

use std::sync::OnceLock;

use regex::Regex;

use crate::prelude::*;

mod parts   ; pub use parts   ::*;
mod config  ; pub use config  ::*;
mod flags   ; pub use flags   ::*;
mod expander; pub use expander::*;

/// A lazily compiled [`Regex`], moving the cost of deserialization to the first usage.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// let regex = LazyRegex::from("abc");
///
/// assert!(regex.cached().is_none());
///
/// assert!(regex.get().unwrap().is_match("abc"));
///
/// assert!(regex.cached().is_some());
/// ```
#[derive(Clone, Debug, Suitability)]
pub struct LazyRegex {
    /// The lazily initialized [`Regex`].
    #[suitable(always)]
    regex: OnceLock<Result<Regex, regex::Error>>,
    /// The [`RegexParts`] to compile [`Self::regex`] from.
    parts: RegexParts
}

impl Serialize for LazyRegex {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.parts.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LazyRegex {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <RegexParts as Deserialize<'de>>::deserialize(deserializer).map(Into::into)
    }
}

impl LazyRegex {
    /// Get a reference to the cached result of [`RegexParts::build`] if it's present.
    pub fn cached(&self) -> Option<&Result<Regex, regex::Error>> {
        self.regex.get()
    }

    /// Get the [`RegexParts`] this uses to compile its [`Regex`].
    pub fn parts(&self) -> &RegexParts {
        &self.parts
    }

    /// Returns the cached [`Regex`], compiling it first if needed.
    /// # Errors
    /// If the cached value is an error or [`RegexParts::build`] returns an error, that error is cloned and returned.
    pub fn get(&self) -> Result<&Regex, regex::Error> {
        match self.regex.get_or_init(|| self.parts.build()) {
            Ok(x) => Ok(x),
            Err(e) => Err(e.clone())
        }
    }
}

impl From<RegexParts> for LazyRegex {
    fn from(parts: RegexParts) -> Self {
        Self {
            regex: OnceLock::new(),
            parts,
        }
    }
}

impl FromStr for LazyRegex {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegexParts::from(s).into())
    }
}

impl From<&str> for LazyRegex {

    fn from(value: &str) -> Self {
        RegexParts::from(value).into()
    }
}

impl From<String> for LazyRegex {
    fn from(value: String) -> Self {
        RegexParts::from(value).into()
    }
}

impl PartialEq for LazyRegex {
    /// Whether or not `self` and/or `other` have their [`Regex`] cached is not considered.
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

impl AsRef<RegexParts> for LazyRegex {
    fn as_ref(&self) -> &RegexParts {
        &self.parts
    }
}

impl TryFrom<LazyRegex> for Regex {
    type Error = regex::Error;

    fn try_from(value: LazyRegex) -> Result<Self, Self::Error> {
        value.regex.into_inner().unwrap_or_else(|| value.parts.build())
    }
}

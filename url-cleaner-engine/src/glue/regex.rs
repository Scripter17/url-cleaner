//! A lazily compiled wrapper around [`Regex`].

use std::str::FromStr;
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::util::*;

pub mod regex_parts;
pub use regex_parts::*;

/// A lazily compiled [`Regex`].
/// # Examples
/// ```
/// use url_cleaner_engine::glue::*;
///
/// let regex = RegexWrapper::from("abc");
/// assert!(regex.get().unwrap().is_match("abc"));
///
/// // Trying to set flags like `/.../i` isn't supported by the regex crate, and so isn't supported here.
/// assert_eq!(RegexWrapper::from("/abc/i").parts().pattern, "/abc/i");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, Suitability)]
#[serde(from = "RegexParts", into = "RegexParts")]
pub struct RegexWrapper {
    /// The lazily initialized [`Regex`].
    #[suitable(always)]
    regex: OnceLock<Regex>,
    /// The [`RegexParts`] to compile [`Self::regex`] from.
    parts: RegexParts
}

impl RegexWrapper {
    /// Gets the [`RegexParts`] this uses to compile its [`Regex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::*;
    ///
    /// let regex = RegexWrapper::from(".*=.*");
    /// assert_eq!(regex.parts(), &RegexParts {pattern: ".*=.*".into(), config: Default::default()});
    /// ```
    pub fn parts(&self) -> &RegexParts {
        &self.parts
    }

    /// Get the compiled [`Regex`] if it's been compiled.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::*;
    ///
    /// let regex = RegexWrapper::from(".*=.*");
    /// assert!(regex.get_no_compile().is_none());
    /// regex.get().unwrap();
    /// assert!(regex.get_no_compile().is_some());
    /// ```
    pub fn get_no_compile(&self) -> Option<&Regex> {
        self.regex.get()
    }

    /// Gets the compiled [`Regex`] or, if it hasn't been compiled, compiles it.
    ///
    /// Currently, if a call to [`Self::get`] returns an error, the next call will attempt to compile the [`Regex`] *again*.
    ///
    /// Given this should be pretty rare and the cost of storing the error is pretty high, I choose to consider a reasonable tradeoff.
    /// # Errors
    /// If the cache is unset and the call to [`RegexParts::build`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::glue::*;
    ///
    /// let regex = RegexWrapper::from(".*=.*");
    /// assert!(regex.get_no_compile().is_none());
    /// regex.get().unwrap();
    /// assert!(regex.get_no_compile().is_some());
    /// ```
    pub fn get(&self) -> Result<&Regex, regex::Error> {
        if let Some(regex) = self.regex.get() {
            Ok(regex)
        } else {
            let temp = self.parts.build()?;
            Ok(self.regex.get_or_init(|| temp))
        }
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

impl FromStr for RegexWrapper {
    type Err = std::convert::Infallible;

    /// Creates a new [`RegexParts`] and uses that.
    ///
    /// The regex is only compiled later.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegexParts::from(s).into())
    }
}

impl From<&str> for RegexWrapper {

    /// Creates a new [`RegexParts`] and uses that.
    ///
    /// The regex is only compiled later.
    fn from(s: &str) -> Self {
        RegexParts::from(s).into()
    }
}

impl PartialEq for RegexWrapper {
    /// Whether or not `self` and/or `other` have their [`Regex`] cached is not considered.
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

impl TryFrom<RegexWrapper> for Regex {
    type Error = regex::Error;

    /// Does not re-compile or clone the [`Regex`] if it's already cached. It simply takes it.
    fn try_from(value: RegexWrapper) -> Result<Self, Self::Error> {
        if let Some(regex) = value.regex.into_inner() {
            Ok(regex)
        } else {
            value.parts.build()
        }
    }
}

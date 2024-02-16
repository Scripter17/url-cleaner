use serde::{Serialize, Deserialize};
use thiserror::Error;

use super::{StringLocation, StringError};
use crate::glue::{RegexWrapper, GlobWrapper, string_or_struct};

/// A general API for matching strings with a variety of methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringMatcher {
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// assert!(StringMatcher::StringLocation {location: StringLocation::Start, value: "utm_".to_string()}.matches("utm_abc").is_ok_and(|x| x==true));
    /// ```
    StringLocation {
        /// The location to check for `value` at.
        location: StringLocation,
        /// The value to look for.
        value: String
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// # use url_cleaner::glue::RegexParts;
    /// assert!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).matches("axc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "regex")]
    Regex(#[serde(deserialize_with = "string_or_struct")] RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use std::str::FromStr;
    /// assert!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).matches("aabcc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "glob")]
    Glob(#[serde(deserialize_with = "string_or_struct")] GlobWrapper)
}

/// Enum containing all possible errors [`StringMatcher::matches`] can return.
#[derive(Debug, Error)]
pub enum MatcherError {
    /// Returned when [`StringLocation::satisfied_by`] errors.
    #[error(transparent)]
    StringError(#[from] StringError)
}

impl StringMatcher {
    /// # Errors
    /// If `self` is [`Self::StringLocation`] and the call to [`StringLocation::satisfied_by`] errors, returns that error.
    pub fn matches(&self, haystack: &str) -> Result<bool, MatcherError> {
        Ok(match self {
            Self::StringLocation {location, value} => location.satisfied_by(haystack, value)?,
            #[cfg(feature = "regex")]
            Self::Regex(regex) => regex.is_match(haystack),
            #[cfg(feature = "glob")]
            Self::Glob(glob) => glob.matches(haystack)
        })
    }
}

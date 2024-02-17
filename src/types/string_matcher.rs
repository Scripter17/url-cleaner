use serde::{Serialize, Deserialize};
use thiserror::Error;

use super::{StringLocation, StringLocationError, StringError};
#[cfg(feature = "regex")]
use crate::glue::RegexWrapper;
#[cfg(feature = "glob")]
use crate::glue::GlobWrapper;
use crate::glue::string_or_struct;

/// A general API for matching strings with a variety of methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringMatcher {
    Always,
    Never,
    Error,
    /// # Errors
    /// If the contained [`Self`] errors, returns that error.
    Debug(Box<Self>),
    TreatErrorAsPass(Box<Self>),
    TreatErrorAsFail(Box<Self>),
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    /// # Errors
    /// If any of the contained [`Self`]s error, returns that error.
    All(Vec<Self>),
    /// # Errors
    /// If any of the contained [`Self`]s error, returns that error.
    Any(Vec<Self>),
    /// # Errors
    /// If the contained [`Self`] errors, returns that error.
    Not(Box<Self>),
    /// Uses a [`StringLocation`].
    /// # Errors
    /// If the call to [`StringLocation::satisfied_by`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// assert!(StringMatcher::StringLocation {location: StringLocation::Start, value: "utm_".to_string()}.satisfied_by("utm_abc").is_ok_and(|x| x==true));
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
    /// assert!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).satisfied_by("axc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "regex")]
    Regex(#[serde(deserialize_with = "string_or_struct")] RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use std::str::FromStr;
    /// assert!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).satisfied_by("aabcc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "glob")]
    Glob(#[serde(deserialize_with = "string_or_struct")] GlobWrapper)
}

/// Enum containing all possible errors [`StringMatcher::matches`] can return.
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when [`StringLocation::satisfied_by`] errors.
    #[error(transparent)]
    StringError(#[from] StringError),
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    #[error("StringMatcher::Error was used.")]
    ExplicitError
}

impl StringMatcher {
    /// # Errors
    /// See [`Self`]'s documentation for details.
    pub fn satisfied_by(&self, haystack: &str) -> Result<bool, StringMatcherError> {
        Ok(match self {
            Self::StringLocation {location, value} => location.satisfied_by(haystack, value)?,
            #[cfg(feature = "regex")]
            Self::Regex(regex) => regex.is_match(haystack),
            #[cfg(feature = "glob")]
            Self::Glob(glob) => glob.matches(haystack),

            // Random stuff
            
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.satisfied_by(haystack)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.satisfied_by(haystack)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.satisfied_by(haystack)?,
            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(haystack).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(haystack).unwrap_or(false),

            // Debug
            
            Self::Always => true,
            Self::Never => false,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.satisfied_by(haystack);
                eprintln!("=== Debug StringMatcher ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },
            Self::TryElse{r#try, r#else}  => r#try.satisfied_by(haystack).or_else(|_| r#else.satisfied_by(haystack))?,
            Self::Error => Err(StringMatcherError::ExplicitError)?
        })
    }
}

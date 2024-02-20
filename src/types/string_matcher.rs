use serde::{Serialize, Deserialize};
use thiserror::Error;

use super::{StringLocation, StringLocationError, StringCmp, StringError};
#[cfg(feature = "regex")]
use crate::glue::RegexWrapper;
#[cfg(feature = "glob")]
use crate::glue::GlobWrapper;
use crate::glue::string_or_struct;

/// A general API for matching strings with a variety of methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringMatcher {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`StringMatcherError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringMatcherError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] errors, returns that error.
    Debug(Box<Self>),
    /// If the contained [`Self`] returns an error, treat it as a pass.
    TreatErrorAsPass(Box<Self>),
    /// If the contained [`Self`] returns an error, treat it as a fail.
    TreatErrorAsFail(Box<Self>),
    /// If `try` returns an error, `else` is executed.
    /// If `try` does not return an error, `else` is not executed.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Passes if all of the included [`Self`]s pass.
    /// Like [`Iterator::all`], an empty list passes.
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, that error is returned.
    All(Vec<Self>),
    /// Passes if any of the included [`Self`]s pass.
    /// Like [`Iterator::any`], an empty list fails.
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, that error is returned.
    Any(Vec<Self>),
    /// Passes if the included [`Self`] doesn't and vice-versa.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned.
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
    /// # use url_cleaner::types::StringMatcher;
    /// # use url_cleaner::glue::RegexParts;
    /// assert!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).satisfied_by("axc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "regex")]
    Regex(#[serde(deserialize_with = "string_or_struct")] RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringMatcher;
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use std::str::FromStr;
    /// assert!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).satisfied_by("aabcc").is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "glob")]
    Glob(#[serde(deserialize_with = "string_or_struct")] GlobWrapper),
    StringCmp {
        cmp: StringCmp,
        r: String
    }
}

/// An enum of all possible errors a [`StringMatcher`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// A generic string error.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// Returned by [`StringMatcher::StringLocation`].
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Always returned by [`StringMatcher::Error`].
    #[error("StringMatcher::Error was used.")]
    ExplicitError
}

impl StringMatcher {
    /// # Errors
    /// See the documentation for [`Self`]'s variants for details.
    pub fn satisfied_by(&self, haystack: &str) -> Result<bool, StringMatcherError> {
        #[cfg(feature = "debug")]
        println!("Matcher: {self:?}");
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
                eprintln!("=== StringMatcher::Debug ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },
            Self::TryElse{r#try, r#else}  => r#try.satisfied_by(haystack).or_else(|_| r#else.satisfied_by(haystack))?,
            Self::Error => Err(StringMatcherError::ExplicitError)?,
            Self::StringCmp {cmp, r} => cmp.satisfied_by(haystack, r)
        })
    }
}

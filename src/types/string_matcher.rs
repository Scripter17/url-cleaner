//! Provides [`StringMatcher`] which allows for testing if a [`str`] matches a certain rule.

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

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

    // Logic

    /// If `r#if` passes, return the result of `then`, otherwise return the value of `r#else`.
    /// # Errors
    /// If `r#if` returns an error, that error is returned.
    /// 
    /// If `r#if` passes and `then` returns an error, that error is returned.
    /// 
    /// If `r#if` fails and `r#else` returns an error, that error is returned.
    If {
        /// The [`Self`] that decides if `then` or `r#else` is used.
        r#if: Box<Self>,
        /// The [`Self`] to use if `r#if` passes.
        then: Box<Self>,
        /// The [`Self`] to use if `r#if` fails.
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

    // Error handling.

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
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    FirstNotError(Vec<Self>),

    // Other.

    /// Passes if the provided string is contained in the specified [`HashSet`].
    InHashSet(HashSet<String>),
    /// Uses a [`StringLocation`].
    /// # Errors
    /// If the call to [`StringLocation::satisfied_by`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// # use url_cleaner::types::Params;
    /// # use url::Url;
    /// assert_eq!(StringMatcher::Contains {r#where: StringLocation::Start, value: "utm_".into()}.satisfied_by("utm_abc", &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true);
    /// ```
    #[cfg(feature = "string-location")]
    Contains {
        /// The location to check for `value` at.
        r#where: StringLocation,
        /// The value to look for.
        #[cfg(feature = "string-source")]
        value: StringSource,
        /// The value to look for.
        #[cfg(not(feature = "string-source"))]
        value: String
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringMatcher;
    /// # use url_cleaner::glue::RegexParts;
    /// # use url_cleaner::types::Params;
    /// # use url::Url;
    /// assert_eq!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).satisfied_by("axc", &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true);
    /// ```
    #[cfg(feature = "regex")]
    Regex(RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringMatcher;
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use url_cleaner::types::Params;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// assert_eq!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).satisfied_by("aabcc", &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true);
    /// ```
    #[cfg(feature = "glob")]
    Glob(GlobWrapper),
    /// Modifies the provided string then matches it.
    #[cfg(feature = "string-modification")]
    Modified {
        /// The modification to apply.
        modification: StringModification,
        /// The matcher to test the modified string with.
        matcher: Box<Self>
    },
    /// Passes if the provided string only contains the specified [`char`]s.
    OnlyTheseChars(Vec<char>),
    /// [`str::is_ascii`].
    IsAscii
}

/// The enum of all possible errors [`StringMatcher::satisfied_by`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when [`StringMatcher::Error`] is used.
    #[error("StringMatcher::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringLocationError`] is encountered.
    #[cfg(feature = "string-location")]
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Returned when both the `try` and `else` of a [`StringMatcher::TryElse`] both return errors.
    #[error("A `StringMatcher::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`StringMatcher::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`StringMatcher::TryElse::else`],
        else_error: Box<Self>
    }
}

impl StringMatcher {
    /// # Errors
    /// See the documentation for [`Self`]'s variants for details.
    pub fn satisfied_by(&self, haystack: &str, url: &Url, params: &Params) -> Result<bool, StringMatcherError> {
        #[cfg(feature = "debug")]
        println!("Matcher: {self:?}");
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(StringMatcherError::ExplicitError)?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.satisfied_by(haystack, url, params);
                eprintln!("=== StringMatcher::Debug ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\nURL: {url:?}\nParams: {params:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(haystack, url, params)? {then} else {r#else}.satisfied_by(haystack, url, params)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.satisfied_by(haystack, url, params)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.satisfied_by(haystack, url, params)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.satisfied_by(haystack, url, params)?,

            // Error handling.

            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(haystack, url, params).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(haystack, url, params).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(haystack, url, params).or_else(|try_error| r#else.satisfied_by(haystack, url, params).map_err(|else_error| StringMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut result = Ok(false); // The initial value doesn't mean anything.
                for matcher in matchers {
                    result = matcher.satisfied_by(haystack, url, params);
                    if result.is_ok() {return result;}
                }
                result?
            }

            // Other.

            Self::InHashSet(hash_set) => hash_set.contains(haystack),
            #[cfg(feature = "string-location"    )] Self::Contains {r#where, value} => r#where.satisfied_by(haystack, get_string!(value, url, params, StringMatcherError))?,
            #[cfg(feature = "regex"              )] Self::Regex(regex) => regex.get_regex()?.is_match(haystack),
            #[cfg(feature = "glob"               )] Self::Glob(glob) => glob.matches(haystack),
            #[cfg(feature = "string-modification")] Self::Modified {modification, matcher} => matcher.satisfied_by(&{let mut temp=haystack.to_string(); modification.apply(&mut temp, params)?; temp}, url, params)?,
            Self::OnlyTheseChars(chars) => haystack.trim_start_matches(&**chars)=="",
            Self::IsAscii => haystack.is_ascii()
        })
    }
}

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;

use super::*;
#[cfg(feature = "regex")]
use crate::glue::RegexWrapper;
#[cfg(feature = "glob")]
use crate::glue::GlobWrapper;
use crate::glue::string_or_struct;
use crate::config::Params;

/// A general API for matching strings with a variety of methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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



    /// Passes if the provided string is contained in the specified [`HashSet`].
    InHashSet(HashSet<String>),
    /// Uses a [`StringLocation`].
    /// # Errors
    /// If the call to [`StringLocation::satisfied_by`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::{StringMatcher, StringLocation};
    /// # use url_cleaner::config::Params;
    /// assert!(StringMatcher::StringLocation {location: StringLocation::Start, value: "utm_".to_string()}.satisfied_by("utm_abc", &Params::default()).is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "string-location")]
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
    /// # use url_cleaner::config::Params;
    /// assert!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).satisfied_by("axc", &Params::default()).is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "regex")]
    Regex(#[serde(deserialize_with = "string_or_struct")] RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringMatcher;
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use url_cleaner::config::Params;
    /// # use std::str::FromStr;
    /// assert!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).satisfied_by("aabcc", &Params::default()).is_ok_and(|x| x==true));
    /// ```
    #[cfg(feature = "glob")]
    Glob(#[serde(deserialize_with = "string_or_struct")] GlobWrapper),
    /// Compares the provided string as the left hand side of the specified [`StringCmp`]
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, returns that error.
    /// If the call to [`StringSource::get`] returns `None` and `none_to_empty_string` is `false`, returns the error [`StringMatcherError::StringSourceIsNone`].
    #[cfg(all(feature = "string-cmp", feature = "string-source"))]
    StringCmp {
        /// The string comparison to use.
        cmp: StringCmp,
        /// If `cmp` returns `None`, this decides whether or not to treat it as an empty string.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The right hand side of the comparison.
        #[serde(deserialize_with = "string_or_struct")]
        r: StringSource
    },
    /// Compares the provided string as the left hand side of the specified [`StringCmp`]
    #[cfg(all(feature = "string-cmp", not(feature = "string-source")))]
    StringCmp {
        /// The string comparison to use.
        cmp: StringCmp,
        /// The right hand side of the comparison.
        r: String
    },
    /// Modifies the provided string then matches it.
    #[cfg(feature = "string-modification")]
    Modified {
        /// THe modification to apply.
        modification: StringModification,
        /// The matcher to test the modified string with.
        matcher: Box<Self>
    }
}

const fn get_true() -> bool {true}

/// The enum of all possible errors [`StringMatcher::satisfied_by`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when [`StringMatcher::Error`] is used.
    #[error("StringMatcher::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringError`] is encountered.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// Returned wehn a [`StringLocationError`] is encountered.
    #[cfg(feature = "string-location")]
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[cfg(feature = "string-modification")]
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone
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
            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(haystack, url, params).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(haystack, url, params).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(haystack, url, params).or_else(|_| r#else.satisfied_by(haystack, url, params))?,
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

            Self::InHashSet(hash_set) => hash_set.contains(haystack),
            #[cfg(feature = "string-location"    )] Self::StringLocation {location, value} => location.satisfied_by(haystack, value)?,
            #[cfg(feature = "regex"              )] Self::Regex(regex) => regex.is_match(haystack),
            #[cfg(feature = "glob"               )] Self::Glob(glob) => glob.matches(haystack),
            #[cfg(all(feature = "string-cmp", feature = "string-source"))] Self::StringCmp {cmp, none_to_empty_string, r} => cmp.satisfied_by(haystack, &r.get(url, params, *none_to_empty_string)?.ok_or(StringMatcherError::StringSourceIsNone)?),
            #[cfg(all(feature = "string-cmp", not(feature = "string-source")))] Self::StringCmp {cmp, r} => cmp.satisfied_by(haystack, r),
            #[cfg(feature = "string-modification")] Self::Modified {modification, matcher} => matcher.satisfied_by(&{let mut temp=haystack.to_string(); modification.apply(&mut temp, params)?; temp}, url, params)?
        })
    }
}

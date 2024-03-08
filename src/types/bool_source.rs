use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;

use crate::types::*;

/// Various possible ways to get a boolean value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoolSource {
    // Debug/constants.

    /// Always returns `true`.
    Always,
    /// Always returns `false`.
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

    // Conditional.

    /// If `r#if` passes, return the result of `then`, otherwise return the value of `r#else`.
    /// # Errors
    /// If `r#if` returns an error, that error is returned.
    /// If `r#if` passes and `then` returns an error, that error is returned.
    /// If `r#if` fails and `r#else` returns an error, that error is returned.
    If {
        /// The [`Self`] that decides if `then` or `r#else` is used.
        r#if: Box<Self>,
        /// The [`Self`] to use if `r#if` passes.
        then: Box<Self>,
        /// The [`Self`] to use if `r#if` fails.
        r#else: Box<Self>
    },
    /// Passes if the included [`Self`] doesn't and vice-versa.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned.
    Not(Box<Self>),
    /// Passes if all of the included [`Self`]s pass.
    /// Like [`Iterator::all`], an empty list passes..
    /// # Errors
    /// If any contained [`Self`] returns an error, that error is returned.
    All(Vec<Self>),
    /// Passes if any of the included [`Self`]s pass.
    /// Like [`Iterator::any`], an empty list fails..
    /// # Errors
    /// If any contained [`Self`] returns an error, that error is returned.
    Any(Vec<Self>),

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

    // Non-meta.

    /// Checks if `needle` exists in `haystack` according to `location`.
    /// # Errors
    /// If either `haystack`'s or `needle`;s call to [`StringSource::get`] returns `None`, returns the error [`BoolSourceError::StringSourceIsNone`].
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    #[cfg(all(feature = "string-source", feature = "string-location"))]
    StringLocation {
        /// The haystack to search for `needle` in.
        haystack: StringSource,
        /// The needle to search for in `haystack`.
        needle: StringSource,
        /// The location to search for `needle` at in `haystack`.
        location: StringLocation
    },
    /// Checks if `string` matches `matcher`.
    /// # Errors
    /// If `string`'s call to [`StringSource::get`] returns `None`, returns the error [`BoolSourceError::StringSourceIsNone`].
    #[cfg(all(feature = "string-source", feature = "string-matcher"))]
    StringMatcher {
        /// The string to match against.
        string: StringSource,
        /// The matcher to check `string` against.
        matcher: StringMatcher
    },
    /// Checks if the specified flag is set.
    #[cfg(feature = "string-source")]
    FlagIsSet(StringSource),
    /// Checks if the specified flag is set.
    #[cfg(not(feature = "string-source"))]
    FlagIsSet(String),
    /// Checks if the specified variable's value is the specified value.
    #[cfg(feature = "string-source")]
    VarIs {
        /// The name of the variable to check.
        name: StringSource,
        /// The expected value of the variable.
        value: Option<StringSource>
    },
    /// Checks if the specified variable's value is the specified value.
    #[cfg(not(feature = "string-source"))]
    VarIs {
        /// The name of the variable
        name: String,
        /// The expected value of the variable.
        value: Option<String>
    },
    /// Returns [`true`] if [`Url::parse`] returns [`Ok`].
    #[cfg(feature = "string-source")]
    IsValidUrl(StringSource),
    /// Returns [`true`] if [`Url::parse`] returns [`Ok`].
    #[cfg(not(feature = "string-source"))]
    IsValidUrl(String)
}

/// The enum of all possible errors [`BoolSource::get`] can return.
#[derive(Debug, Error)]
pub enum BoolSourceError {
    /// Returned when [`BoolSource::Error`] is used.
    #[error("BoolSource::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[cfg(feature = "string-location")]
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringMatcherError`] is encountered.
    #[cfg(feature = "string-matcher")]
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[cfg(feature = "string-source")]
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone
}

impl BoolSource {
    /// # Errors
    /// See [`Self`]'s documentation for details.
    pub fn get(&self, url: &Url, params: &Params) -> Result<bool, BoolSourceError> {
        Ok(match self {
            // Debug/constants.

            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(BoolSourceError::ExplicitError)?,
            Self::Debug(bool_source) => {
                let ret=bool_source.get(url, params);
                eprintln!("=== BoolSource::Debug ===\nBoolSource: {bool_source:?}\nURL: {url:?}\nParams: {params:?}\nRet: {ret:?}");
                ret?
            },

            // Conditional.

            Self::If {r#if, then, r#else} => if r#if.get(url, params)? {then} else {r#else}.get(url, params)?,
            Self::Not(bool_source) => !bool_source.get(url, params)?,
            Self::All(bool_sources) => {
                for bool_source in bool_sources {
                    if !bool_source.get(url, params)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(bool_sources) => {
                for bool_source in bool_sources {
                    if bool_source.get(url, params)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Error handling.

            Self::TreatErrorAsPass(bool_source) => bool_source.get(url, params).unwrap_or(true),
            Self::TreatErrorAsFail(bool_source) => bool_source.get(url, params).unwrap_or(false),
            Self::TryElse {r#try, r#else} => r#try.get(url, params).or_else(|_| r#else.get(url, params))?,

            // Non-meta.

            #[cfg(all(feature = "string-source", feature = "string-location"))]
            Self::StringLocation {haystack, needle, location} => location.satisfied_by(
                &haystack.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?,
                &needle  .get(url, params  )?.ok_or(BoolSourceError::StringSourceIsNone)?
            )?,
            #[cfg(all(feature = "string-source", feature = "string-matcher"))]
            Self::StringMatcher {string, matcher} => matcher.satisfied_by(
                &string.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?,
                url, params
            )?,
            #[cfg(feature = "string-source")]
            Self::FlagIsSet(name) => params.flags.contains(&name.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?.into_owned()),
            #[cfg(not(feature = "string-source"))]
            Self::FlagIsSet(name) => params.flags.contains(name),

            #[cfg(feature = "string-source")]
            Self::VarIs {name, value} => match value.as_ref() {
                Some(source) => params.vars.get(&name.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?.to_string()).map(|x| &**x)==source.get(url, params)?.as_deref(),
                None => params.vars.get(&name.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?.to_string()).is_none()
            },
            #[cfg(not(feature = "string-source"))]
            Self::VarIs {name: _, value} => params.vars.get(name).map(|x| &**x)==value.as_deref(),

            #[cfg(feature = "string-source")]
            Self::IsValidUrl(source) => Url::parse(&source.get(url, params)?.ok_or(BoolSourceError::StringSourceIsNone)?).is_ok(),
            #[cfg(not(feature = "string-source"))]
            Self::IsValidUrl(string) => Url::parse(string).is_ok()
        })
    }
}

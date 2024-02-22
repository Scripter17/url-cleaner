use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;

use super::*;
use crate::glue::string_or_struct;
use crate::config::Params;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoolSource {
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
    /// Like [`Iterator::all`], an empty list passes..
    /// # Errors
    /// If any contained [`Self`] returns an error, that error is returned.
    All(Vec<Self>),
    /// Passes if any of the included [`Self`]s pass.
    /// Like [`Iterator::any`], an empty list fails..
    /// # Errors
    /// If any contained [`Self`] returns an error, that error is returned.
    Any(Vec<Self>),
    /// Passes if the included [`Self`] doesn't and vice-versa.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned.
    Not(Box<Self>),



    #[cfg(all(feature = "string-source", feature = "string-cmp"))]
    StringCmp {
        #[serde(deserialize_with = "string_or_struct")]
        l: StringSource,
        #[serde(deserialize_with = "string_or_struct")]
        r: StringSource,
        #[serde(default = "get_true")]
        l_none_to_empty_string: bool,
        #[serde(default = "get_true")]
        r_none_to_empty_string: bool,
        cmp: StringCmp
    },
    #[cfg(all(feature = "string-source", feature = "string-location"))]
    StringLocation {
        #[serde(deserialize_with = "string_or_struct")]
        haystack: StringSource,
        #[serde(deserialize_with = "string_or_struct")]
        needle: StringSource,
        #[serde(default = "get_true")]
        haystack_none_to_empty_string: bool,
        #[serde(default = "get_true")]
        needle_none_to_empty_string: bool,
        location: StringLocation
    },
    #[cfg(all(feature = "string-source", feature = "string-matcher"))]
    StringMatcher {
        #[serde(deserialize_with = "string_or_struct")]
        string: StringSource,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        matcher: StringMatcher
    },
    #[cfg(feature = "string-source")]
    FlagIsSet {
        #[serde(deserialize_with = "string_or_struct")]
        name: StringSource,
        #[serde(default)]
        none_to_empty_string: bool
    },
    #[cfg(not(feature = "string-source"))]
    FlagIsSet {
        name: String
    }
}

const fn get_true() -> bool {true}

#[derive(Debug, Error)]
pub enum BoolSourceError {
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error(transparent)]
    #[cfg(feature = "string-location")]
    StringLocationError(#[from] StringLocationError),
    #[error(transparent)]
    #[cfg(feature = "string-matcher")]
    StringMatcherError(#[from] StringMatcherError),
    #[cfg(feature = "string-source")]
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    #[error("BoolSource::Error was used.")]
    ExplicitError
}

impl BoolSource {
    /// # Errors
    /// See [`Self`]'s documentation for details.
    pub fn get(&self, url: &Url, params: &Params) -> Result<bool, BoolSourceError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(BoolSourceError::ExplicitError)?,
            Self::Debug(bool_source) => {
                let ret=bool_source.get(url, params);
                eprintln!("=== BoolSource::Debug ===\nBoolSource: {bool_source:?}\nURL: {url:?}\nParams: {params:?}\nRet: {ret:?}");
                ret?
            },
            Self::TreatErrorAsPass(bool_source) => bool_source.get(url, params).unwrap_or(true),
            Self::TreatErrorAsFail(bool_source) => bool_source.get(url, params).unwrap_or(false),
            Self::TryElse {r#try, r#else} => r#try.get(url, params).or_else(|_| r#else.get(url, params))?,
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
            Self::Not(bool_source) => !bool_source.get(url, params)?,

            #[cfg(feature = "string-source")]
            Self::StringCmp {l, r, l_none_to_empty_string, r_none_to_empty_string, cmp} => cmp.satisfied_by(
                &l.get(url, params, *l_none_to_empty_string)?.ok_or(BoolSourceError::StringSourceIsNone)?,
                &r.get(url, params, *r_none_to_empty_string)?.ok_or(BoolSourceError::StringSourceIsNone)?
            ),
            #[cfg(feature = "string-location")]
            Self::StringLocation {haystack, needle, haystack_none_to_empty_string, needle_none_to_empty_string, location} => location.satisfied_by(
                &haystack.get(url, params, *haystack_none_to_empty_string)?.ok_or(BoolSourceError::StringSourceIsNone)?,
                &needle  .get(url, params, *needle_none_to_empty_string  )?.ok_or(BoolSourceError::StringSourceIsNone)?
            )?,
            #[cfg(feature = "string-matcher")]
            Self::StringMatcher {string, none_to_empty_string, matcher} => matcher.satisfied_by(
                &string.get(url, params, *none_to_empty_string)?.ok_or(BoolSourceError::StringSourceIsNone)?,
                params
            )?,
            #[cfg(feature = "string-source")]
            Self::FlagIsSet {name, none_to_empty_string} => params.flags.contains(&name.get(url, params, *none_to_empty_string)?.ok_or(BoolSourceError::StringSourceIsNone)?.into_owned()),
            #[cfg(not(feature = "string-source"))]
            Self::FlagIsSet {name} => params.flags.contains(name)
        })
    }
}

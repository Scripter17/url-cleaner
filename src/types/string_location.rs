//! Rules for looking for a string in another string.

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
pub enum StringLocation {
    Always,
    Never,
    Error(String),
    #[suitable(never)]
    Debug(Box<Self>),

    If {
        r#if: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    All(Vec<Self>),
    Any(Vec<Self>),
    Not(Box<Self>),

    TreatErrorAsPass(Box<Self>),
    TreatErrorAsFail(Box<Self>),
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    FirstNotError(Vec<Self>),

    Anywhere,
    Start,
    End,
    StartsAt(isize),
    EndsAt(isize),
    After(isize),
    Before(isize),
    Equals,
    Range {
        start: isize,
        end: Option<isize>,
        location: Box<Self>
    },
    AnySegment {
        split: String,
        location: Box<Self>
    },
    NthSegment {
        split: String,
        n: isize,
        location: Box<Self>
    }
}

// The [`Default`] derive macro doesn't say which enum the default is.
#[allow(clippy::derivable_impls, reason = "The derive for [`Default`] doesn't state the default value.")] 
impl Default for StringLocation {
    fn default() -> Self {
        Self::Anywhere
    }
}
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringLocationError {
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    #[error("The requested slice was either not on a UTF-8 boundary or was out of bounds.")]
    InvalidSlice,
    #[error("The requested index was either not on a UTF-8 boundary or was out of bounds.")]
    InvalidIndex,
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    #[error("A StringLocation::TryElse had both its `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    }
}

impl StringLocation {
    /// Checks if `needle` occurs in `haystack` at the specified location.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, StringLocationError> {
        debug!(StringLocation::satisfied_by, self, haystack, needle);
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(StringLocationError::ExplicitError(msg.clone()))?,
            Self::Debug(location) => {
                let is_satisfied=location.satisfied_by(haystack, needle);
                eprintln!("=== StringLocation::Debug ===\nLocation: {location:?}\nHaystack: {haystack:?}\nNeedle: {needle:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(haystack, needle)? {then} else {r#else}.satisfied_by(haystack, needle)?,
            Self::All(locations) => {
                for location in locations {
                    if !location.satisfied_by(haystack, needle)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(locations) => {
                for location in locations {
                    if location.satisfied_by(haystack, needle)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(location) => !location.satisfied_by(haystack, needle)?,

            // Error handling.

            Self::TreatErrorAsPass(location) => location.satisfied_by(haystack, needle).unwrap_or(true),
            Self::TreatErrorAsFail(location) => location.satisfied_by(haystack, needle).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(haystack, needle).or_else(|try_error| r#else.satisfied_by(haystack, needle).map_err(|else_error| StringLocationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(locations) => {
                let mut result = Ok(false); // Initial value doesn't mean anything.
                for location in locations {
                    result = location.satisfied_by(haystack, needle);
                    if result.is_ok() {return result}
                }
                result?
            }

            // Other.

            Self::Start                => haystack.starts_with(needle),
            Self::End                  => haystack.ends_with  (needle),
            Self::Anywhere             => haystack.contains   (needle),

            Self::StartsAt (start     ) => haystack.get(  neg_range_boundary(*start,       haystack.len()).ok_or(StringLocationError::InvalidIndex)?..).ok_or(StringLocationError::InvalidSlice)?.starts_with(needle),
            Self::EndsAt   (       end) => haystack.get(..neg_range_boundary(        *end, haystack.len()).ok_or(StringLocationError::InvalidIndex)?  ).ok_or(StringLocationError::InvalidSlice)?.ends_with(needle),

            Self::After    (start     ) => haystack.get(  neg_range_boundary(*start,       haystack.len()).ok_or(StringLocationError::InvalidIndex)?..).ok_or(StringLocationError::InvalidSlice)?.contains(needle),
            Self::Before   (       end) => haystack.get(..neg_range_boundary(        *end, haystack.len()).ok_or(StringLocationError::InvalidIndex)?  ).ok_or(StringLocationError::InvalidSlice)?.contains(needle),

            Self::Range {start, end, location} => location.satisfied_by(
                haystack.get(neg_range(*start, *end, haystack.len()).ok_or(StringLocationError::InvalidSlice)?).ok_or(StringLocationError::InvalidSlice)?,
                needle
            )?,

            Self::Equals               => haystack==needle,
            Self::AnySegment {split, location} => {
                for segment in haystack.split(split) {
                    if location.satisfied_by(segment, needle)? {
                        return Ok(true)
                    }
                }
                return Ok(false)
            },
            Self::NthSegment {split, n, location} => location.satisfied_by(neg_nth(haystack.split(split), *n).ok_or(StringLocationError::SegmentNotFound)?, needle)?
        })
    }
}

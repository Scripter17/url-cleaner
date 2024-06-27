//! Provides [`StringLocation`] which allows for testing if part of a [`str`] matches a certain rule.

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::util::*;

/// A wrapper around [`str`]'s various substring searching functions.
/// 
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringLocation {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`StringLocationError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringLocationError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// 
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),

    // Logic.

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

    /// Checks if an instance of the needle exists anywhere in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::Anywhere.satisfied_by("abcdef", "cde").unwrap(), true );
    /// assert_eq!(StringLocation::Anywhere.satisfied_by("abcdef", "efg").unwrap(), false);
    /// ```
    Anywhere,
    /// Checks if an instance of the needle exists at the start of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::Start.satisfied_by("abcdef", "abc").unwrap(), true );
    /// assert_eq!(StringLocation::Start.satisfied_by("abcdef", "bcd").unwrap(), false);
    /// ```
    Start,
    /// Checks if an instance of the needle exists at the end of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::End.satisfied_by("abcdef", "def").unwrap(), true );
    /// assert_eq!(StringLocation::End.satisfied_by("abcdef", "cde").unwrap(), false);
    /// ```
    End,
    /// Checks if an instance of the needle starts at the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::StartsAt( 0).satisfied_by("abcdef", "abc").unwrap(), true );
    /// assert_eq!(StringLocation::StartsAt( 1).satisfied_by("abcdef", "bcd").unwrap(), true );
    /// assert_eq!(StringLocation::StartsAt( 5).satisfied_by("abcdef", "f"  ).unwrap(), true );
    /// assert_eq!(StringLocation::StartsAt( 0).satisfied_by("abcdef", "bcd").unwrap(), false);
    /// assert_eq!(StringLocation::StartsAt( 1).satisfied_by("abcdef", "cde").unwrap(), false);
    /// assert_eq!(StringLocation::StartsAt( 5).satisfied_by("abcdef", "def").unwrap(), false);
    ///
    /// assert_eq!(StringLocation::StartsAt(-2).satisfied_by("abcdef", "ef" ).unwrap(), true);
    /// ```
    StartsAt(isize),
    /// Checks if an instance of the needle ends at the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::EndsAt(3).satisfied_by("abcdef", "abc").unwrap(), true );
    /// assert_eq!(StringLocation::EndsAt(4).satisfied_by("abcdef", "bcd").unwrap(), true );
    /// assert_eq!(StringLocation::EndsAt(6).satisfied_by("abcdef", "def").unwrap(), true );
    /// assert_eq!(StringLocation::EndsAt(6).satisfied_by("abcdef", "f"  ).unwrap(), true );
    /// assert_eq!(StringLocation::EndsAt(3).satisfied_by("abcdef", "bcd").unwrap(), false);
    /// assert_eq!(StringLocation::EndsAt(4).satisfied_by("abcdef", "cde").unwrap(), false);
    /// ```
    EndsAt(isize),
    /// Checks if an instance of the needle exists after the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::After(0).satisfied_by("abcdef", "abcdef").unwrap(), true );
    /// assert_eq!(StringLocation::After(1).satisfied_by("abcdef", "bcdef" ).unwrap(), true );
    /// assert_eq!(StringLocation::After(1).satisfied_by("abcdef", "1"     ).unwrap(), false);
    /// assert_eq!(StringLocation::After(6).satisfied_by("abcdef", "f"     ).unwrap(), false);
    /// StringLocation::After(7).satisfied_by("abcdef", ""      ).unwrap_err();
    /// ```
    After(isize),
    /// Checks if an instance of the needle exists before the specified point in the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// assert_eq!(StringLocation::Before(0).satisfied_by("abcdef", ""   ).unwrap(), true );
    /// assert_eq!(StringLocation::Before(1).satisfied_by("abcdef", "a"  ).unwrap(), true );
    /// assert_eq!(StringLocation::Before(6).satisfied_by("abcdef", "a"  ).unwrap(), true );
    /// assert_eq!(StringLocation::Before(4).satisfied_by("abcdef", "def").unwrap(), false);
    /// StringLocation::Before(7).satisfied_by("abcdef", "a"  ).unwrap_err();
    /// ```
    Before(isize),
    /// Checks equality.
    /// 
    /// Meant primarily for use with [`Self::AnySegment`] and [`Self::NthSegment`].
    Equals,
    /// Checks if an instance of the needle exists within the specified range of the haystack.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// ```
    Range {
        /// The start of the range to check.
        start: Option<isize>,
        /// The end of the range to check.
        end: Option<isize>,
        /// Where to look in the range
        location: Box<Self>
    },
    /// Splits the haystack at every instance of `split` and check if any segment satisfies `location`.
    /// # Errors
    /// If `location` returns an error on any segment, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// StringLocation::AnySegment {split: "/".to_string(), location: Box::new(StringLocation::Start)}.satisfied_by("abc/def/ghi", "d").unwrap()==true;
    /// ```
    AnySegment {
        /// The string to split by.
        split: String,
        /// The location of each segment to look for `needle` in.
        location: Box<Self>
    },
    /// Splits the haystack at every instance of `split` and check if the `n`th segment satisfies `location`.
    /// # Errors
    /// If the `n`th segment doesn't exist, returns the error [`StringLocationError::SegmentNotFound`].
    /// 
    /// If `location` returns an error on any segment, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringLocation;
    /// StringLocation::NthSegment {split: "/".to_string(), n:0, location: Box::new(StringLocation::Start)}.satisfied_by("abc/def/ghi", "d").unwrap()==false;
    /// StringLocation::NthSegment {split: "/".to_string(), n:1, location: Box::new(StringLocation::Start)}.satisfied_by("abc/def/ghi", "d").unwrap()==true;
    /// StringLocation::NthSegment {split: "/".to_string(), n:2, location: Box::new(StringLocation::Start)}.satisfied_by("abc/def/ghi", "d").unwrap()==false;
    /// StringLocation::NthSegment {split: "/".to_string(), n:3, location: Box::new(StringLocation::Start)}.satisfied_by("abc/def/ghi", "d").unwrap_err();
    /// ```
    NthSegment {
        /// The string to split by.
        split: String,
        /// The index of the segment to search in.
        n: isize,
        /// The location of the `n`th segment to look for `needle` in.
        location: Box<Self>
    }
}

// The [`Default`] derive macro doesn't say which enum the default is.
#[allow(clippy::derivable_impls)] // The derive for [`Default`] doesn't state the default value.
impl Default for StringLocation {
    /// [`Self::Anywhere`].
    fn default() -> Self {
        Self::Anywhere
    }
}

/// The enum of all possible errors [`StringLocation::satisfied_by`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringLocationError {
    /// Returned when [`StringLocation::Error`] is used.
    #[error("StringLocation::Error was used.")]
    ExplicitError,
    /// Returned when the requested slice is either not on a UTF-8 boundary or is out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundary or was out of bounds.")]
    InvalidSlice,
    /// Returned when the requested index is either not on a UTF-8 boundary or is out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or was out of bounds.")]
    InvalidIndex,
    /// Returned when the requested segment is not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// Returned wjem a [`StringLocation::TryElse`] has both its `try` and `else` return an error.
    #[error("A StringLocation::TryElse had both its `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`StringLocation::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`StringLocation::TryElse::else`],
        else_error: Box<Self>
    }
}

impl StringLocation {
    /// Checks if `needle` exists in `haystack` according to `self`'s rules.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, StringLocationError> {
        #[cfg(feature = "debug")]
        println!("Location: {self:?}");
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(StringLocationError::ExplicitError)?,
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

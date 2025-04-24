//! Rules for looking for a string in another string.

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::util::*;

/// Search a string for a substring in one of various ways.
///
/// Defaults to [`Self::Anywhere`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Suitability)]
pub enum StringLocation {
    /// Always passes.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::Always.satisfied_by("a", "b").unwrap());
    /// ```
    Always,
    /// Always fails.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::Never.satisfied_by("a", "a").unwrap());
    /// ```
    Never,
    /// Always returns the error [`StringLocationError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringLocationError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// StringLocation::Error("aaa".to_string()).satisfied_by("a", "a").unwrap_err();
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If [`Self::IfContains::where`]'s call to [`Self::satisfied_by`] passes, return the value of [`Self::IfContains::then`].
    ///
    /// If [`Self::IfContains::where`]'s call to [`Self::satisfied_by`] fails, return the value of [`Self::IfContains::else`].
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::IfContains {
    ///     r#where: Box::new(StringLocation::Always),
    ///     then   : Box::new(StringLocation::Always),
    ///     r#else : Box::new(StringLocation::Never)
    /// }.satisfied_by("a", "a").unwrap());
    ///
    /// assert!(!StringLocation::IfContains {
    ///     r#where: Box::new(StringLocation::Never),
    ///     then   : Box::new(StringLocation::Always),
    ///     r#else : Box::new(StringLocation::Never)
    /// }.satisfied_by("a", "a").unwrap());
    /// ```
    IfContains {
        /// The [`Self`] to decide between [`Self::IfContains::then`] and [`Self::IfContains::else`].
        r#where: Box<Self>,
        /// The [`Self`] to use if [`Self::IfContains::where`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::IfContains::where`] fails.
        r#else: Box<Self>
    },
    /// If the call to [`Self::satisfied_by`] passes or fails, invert it into failing or passing.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::Not(Box::new(StringLocation::Anywhere)).satisfied_by("abc", "a").unwrap());
    /// ```
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::All(vec![
    ///     StringLocation::Start,
    ///     StringLocation::End
    /// ]).satisfied_by("abcba", "a").unwrap());
    /// ```
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::Any(vec![
    ///     StringLocation::Start,
    ///     StringLocation::End
    /// ]).satisfied_by("cba", "a").unwrap());
    /// ```
    Any(Vec<Self>),

    /// If the call to [`Self::satisfied_by`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::TreatErrorAsPass(Box::new(StringLocation::Error("".to_string()))).satisfied_by("a", "a").unwrap());
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::satisfied_by`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::TreatErrorAsFail(Box::new(StringLocation::Error("".to_string()))).satisfied_by("a", "a").unwrap());
    /// ```
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::satisfied_by`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::satisfied_by`] return errors, both errors are returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::TryElse {
    ///     r#try : Box::new(StringLocation::Error("".to_string())),
    ///     r#else: Box::new(StringLocation::Always)
    /// }.satisfied_by("a", "a").unwrap());
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try'] returns an error.
        r#else: Box<Self>
    },
    /// Return the first non-error value.
    /// # Errors
    /// If all calls to [`Self::satisfied_by`] return errors, the last error is returned. In the future this should be changed to return all errors.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::FirstNotError(vec![
    ///     StringLocation::Error("".to_string()),
    ///     StringLocation::Error("".to_string()),
    ///     StringLocation::Always
    /// ]).satisfied_by("a", "a").unwrap());
    /// ```
    FirstNotError(Vec<Self>),

    /// Passes if the needle exists anywhere in the haystack.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::Anywhere.satisfied_by("abc", "a").unwrap());
    /// assert!(StringLocation::Anywhere.satisfied_by("cba", "a").unwrap());
    /// assert!(StringLocation::Anywhere.satisfied_by("bca", "a").unwrap());
    /// ```
    #[default]
    Anywhere,
    /// Passes if the haystack begins with the needle.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!( StringLocation::Start.satisfied_by("abc", "a").unwrap());
    /// assert!(!StringLocation::Start.satisfied_by("cac", "a").unwrap());
    /// assert!(!StringLocation::Start.satisfied_by("bca", "a").unwrap());
    /// ```
    Start,
    /// Passes if the haystack ends with the needle.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::End.satisfied_by("abc", "a").unwrap());
    /// assert!(!StringLocation::End.satisfied_by("cac", "a").unwrap());
    /// assert!( StringLocation::End.satisfied_by("bca", "a").unwrap());
    /// ```
    End,
    /// Passes if the needle starts at the specified location in the haystack.
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::StartsAt(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::StartsAt(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    StartsAt(isize),
    /// Passes if the needle ends at the specified location in the haystack.
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::EndsAt(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::EndsAt(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    EndsAt(isize),
    /// Passes if the haystack contains the needle at or after the specified index.
    ///
    /// If you want to only pass when the needle is strictly after the specified index, see [`Self::After`].
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!( StringLocation::AtOrAfter(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::AtOrAfter(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    AtOrAfter(isize),
    /// Passes if the haystack contains the needle before or at the specified index.
    ///
    /// If you want to only pass wehn the needle is strictly before the specified index, see [`Self::Before`].
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::BeforeOrAt(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::BeforeOrAt(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::BeforeOrAt(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::BeforeOrAt(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::BeforeOrAt(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    BeforeOrAt(isize),
    /// Passes if the haystack contains the needle after the specified index.
    ///
    /// If you want to also pass when the needle is at the specified index, see [`Self::AtOrAfter`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!( StringLocation::After(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    After(isize),
    /// Passes if the haystack contains the needle before the specified index.
    ///
    /// If you want to also pass when the needle is at the specified index, see [`Self::BeforeOrAt`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::Before(0).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(1).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(2).satisfied_by("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(3).satisfied_by("#abc#", "abc").unwrap());
    /// assert!( StringLocation::Before(4).satisfied_by("#abc#", "abc").unwrap());
    /// ```
    Before(isize),
    /// Passes if the haystack is equal to the needle.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!( StringLocation::Equals.satisfied_by( "abc" , "abc").unwrap());
    /// assert!(!StringLocation::Equals.satisfied_by("#abc" , "abc").unwrap());
    /// assert!(!StringLocation::Equals.satisfied_by( "abc#", "abc").unwrap());
    /// assert!(!StringLocation::Equals.satisfied_by("#abc#", "abc").unwrap());
    /// ```
    Equals,
    /// Passes if the haystack contains the needle in the specified range at [`Self::Range::where`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidSlice`].
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::Range {
    ///     start: 0,
    ///     end: Some(3),
    ///     r#where: Box::new(StringLocation::Equals)
    /// }.satisfied_by("abc##", "abc").unwrap());
    /// ```
    Range {
        /// The start of the range to search in.
        start: isize,
        /// The end of the range to search in, exclusive.
        ///
        /// Set to [`None`] to keep it unbounded.
        end: Option<isize>,
        /// The [`Self`] to search for the needle in the range.
        r#where: Box<Self>
    },
    /// Passes if any segment of the haystack, split by [`Self::AnySegment::split`], contains the needle at [`Self::AnySegment::where`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(StringLocation::AnySegment {
    ///     split: "/".to_string(),
    ///     r#where: Box::new(StringLocation::Equals)
    /// }.satisfied_by("123/abc/456", "abc").unwrap());
    /// ```
    AnySegment {
        /// The string to split the haystack with.
        split: String,
        /// The [`Self`] to search for the needle in each segment.
        r#where: Box<Self>
    },
    /// Passes if the [`Self::NthSegment::n`]th segment of the haystack, split by [`Self::NthSegment::split`], contains the needle at [`Self::NthSegment::where`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert!(!StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n: 0,
    ///     r#where: Box::new(StringLocation::Equals)
    /// }.satisfied_by("123/abc/456", "abc").unwrap());
    ///
    /// assert!(StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n: 1,
    ///     r#where: Box::new(StringLocation::Equals)
    /// }.satisfied_by("123/abc/456", "abc").unwrap());
    ///
    /// assert!(!StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n: 2,
    ///     r#where: Box::new(StringLocation::Equals)
    /// }.satisfied_by("123/abc/456", "abc").unwrap());
    /// ```
    NthSegment {
        /// The string to split the haystack with.
        split: String,
        /// The index of the segment to search in.
        n: isize,
        /// The [`Self`] to search for the needle in the [`Self::NthSegment::n`]th segment.
        r#where: Box<Self>
    }
}

/// The enum of errors [`StringLocation::satisfied_by`] can return.
#[derive(Debug, Error)]
pub enum StringLocationError {
    /// Returned when a [`StringLocation::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when a slice is either not on UTF-8 boundaries or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundaries or out of bounds.")]
    InvalidSlice,
    /// Returned when an index is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound,
    /// Returned when both [`StringLocation`]s in a [`StringLocation::TryElse`] return errors.
    #[error("Both StringLocations in a StringLocation::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringLocation::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`StringLocation::TryElse::else`]. 
        else_error: Box<Self>
    },
}

impl StringLocation {
    /// Checks if `needle` occurs in `haystack` at the specified location.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn satisfied_by(&self, haystack: &str, needle: &str) -> Result<bool, StringLocationError> {
        debug!(self, StringLocation::satisfied_by, self, haystack, needle);
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

            Self::IfContains {r#where, then, r#else} => if r#where.satisfied_by(haystack, needle)? {then} else {r#else}.satisfied_by(haystack, needle)?,
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

            Self::StartsAt  (start     ) => haystack.get(   neg_index                 (*start,       haystack.len()   ).ok_or(StringLocationError::InvalidIndex)?..).ok_or(StringLocationError::InvalidSlice)?.starts_with(needle),
            Self::EndsAt    (       end) => haystack.get(..=neg_index                 (        *end, haystack.len()   ).ok_or(StringLocationError::InvalidIndex)?  ).ok_or(StringLocationError::InvalidSlice)?.ends_with(needle),

            Self::AtOrAfter (start     ) => haystack.get(   neg_index                 (*start,       haystack.len()   ).ok_or(StringLocationError::InvalidIndex)?..).ok_or(StringLocationError::InvalidSlice)?.contains(needle),
            Self::BeforeOrAt(       end) => haystack.get(..=neg_index                 (        *end, haystack.len()   ).ok_or(StringLocationError::InvalidIndex)?  ).ok_or(StringLocationError::InvalidSlice)?.contains(needle),

            Self::After     (start     ) => haystack.get(   neg_shifted_range_boundary(*start,       haystack.len(), 1).ok_or(StringLocationError::InvalidIndex)?..).ok_or(StringLocationError::InvalidSlice)?.contains(needle),
            Self::Before    (       end) => haystack.get(.. neg_index                 (        *end, haystack.len()   ).ok_or(StringLocationError::InvalidIndex)?  ).ok_or(StringLocationError::InvalidSlice)?.contains(needle),

            Self::Range {start, end, r#where} => r#where.satisfied_by(
                haystack.get(neg_range(*start, *end, haystack.len()).ok_or(StringLocationError::InvalidSlice)?).ok_or(StringLocationError::InvalidSlice)?,
                needle
            )?,

            Self::Equals               => haystack==needle,
            Self::AnySegment {split, r#where} => {
                for segment in haystack.split(split) {
                    if r#where.satisfied_by(segment, needle)? {
                        return Ok(true)
                    }
                }
                return Ok(false)
            },
            Self::NthSegment {split, n, r#where} => r#where.satisfied_by(neg_nth(haystack.split(split), *n).ok_or(StringLocationError::SegmentNotFound)?, needle)?
        })
    }
}

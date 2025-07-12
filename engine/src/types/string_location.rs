//! Rules for looking for a string in another string.

use std::ops::Bound;

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
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::Always.check("a", "b").unwrap());
    /// ```
    Always,
    /// Always fails.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::Never.check("a", "a").unwrap());
    /// ```
    Never,
    /// Always returns the error [`StringLocationError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringLocationError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// StringLocation::Error("aaa".to_string()).check("a", "a").unwrap_err();
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),



    /// Swap the haystack and the needle.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    Swap(Box<Self>),



    /// If [`Self::IfContains::at`]'s call to [`Self::check`] passes, return the value of [`Self::IfContains::then`].
    ///
    /// If [`Self::IfContains::at`]'s call to [`Self::check`] fails, return the value of [`Self::IfContains::else`].
    /// # Errors
    /// If any call to [`Self::check`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::IfContains {
    ///     at: Box::new(StringLocation::Always),
    ///     then   : Box::new(StringLocation::Always),
    ///     r#else : Box::new(StringLocation::Never)
    /// }.check("a", "a").unwrap());
    ///
    /// assert!(!StringLocation::IfContains {
    ///     at: Box::new(StringLocation::Never),
    ///     then   : Box::new(StringLocation::Always),
    ///     r#else : Box::new(StringLocation::Never)
    /// }.check("a", "a").unwrap());
    /// ```
    IfContains {
        /// The [`Self`] to decide between [`Self::IfContains::then`] and [`Self::IfContains::else`].
        at: Box<Self>,
        /// The [`Self`] to use if [`Self::IfContains::at`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::IfContains::at`] fails.
        r#else: Box<Self>
    },
    /// If the call to [`Self::check`] passes or fails, invert it into failing or passing.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::Not(Box::new(StringLocation::Anywhere)).check("abc", "a").unwrap());
    /// ```
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    /// If any call to [`Self::check`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::All(vec![
    ///     StringLocation::Start,
    ///     StringLocation::End
    /// ]).check("abcba", "a").unwrap());
    /// ```
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    /// If any call to [`Self::check`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::Any(vec![
    ///     StringLocation::Start,
    ///     StringLocation::End
    /// ]).check("cba", "a").unwrap());
    /// ```
    Any(Vec<Self>),

    /// If the call to [`Self::check`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::TreatErrorAsPass(Box::new(StringLocation::Error("".to_string()))).check("a", "a").unwrap());
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::check`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::TreatErrorAsFail(Box::new(StringLocation::Error("".to_string()))).check("a", "a").unwrap());
    /// ```
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::check`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::check`] return errors, both errors are returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::TryElse {
    ///     r#try : Box::new(StringLocation::Error("".to_string())),
    ///     r#else: Box::new(StringLocation::Always)
    /// }.check("a", "a").unwrap());
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try'] returns an error.
        r#else: Box<Self>
    },
    /// Return the first non-error value.
    /// # Errors
    /// If all calls to [`Self::check`] return errors, the last error is returned. In the future this should be changed to return all errors.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::FirstNotError(vec![
    ///     StringLocation::Error("".to_string()),
    ///     StringLocation::Error("".to_string()),
    ///     StringLocation::Always
    /// ]).check("a", "a").unwrap());
    /// ```
    FirstNotError(Vec<Self>),

    /// Passes if the needle exists anywhere in the haystack.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::Anywhere.check("abc", "a").unwrap());
    /// assert!(StringLocation::Anywhere.check("cba", "a").unwrap());
    /// assert!(StringLocation::Anywhere.check("bca", "a").unwrap());
    /// ```
    #[default]
    Anywhere,
    /// Passes if the haystack begins with the needle.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!( StringLocation::Start.check("abc", "a").unwrap());
    /// assert!(!StringLocation::Start.check("cac", "a").unwrap());
    /// assert!(!StringLocation::Start.check("bca", "a").unwrap());
    /// ```
    Start,
    /// Passes if the haystack ends with the needle.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::End.check("abc", "a").unwrap());
    /// assert!(!StringLocation::End.check("cac", "a").unwrap());
    /// assert!( StringLocation::End.check("bca", "a").unwrap());
    /// ```
    End,
    /// Passes if the needle starts at the specified location in the haystack.
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::StartsAt(0).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::StartsAt(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(2).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(3).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::StartsAt(4).check("#abc#", "abc").unwrap());
    /// ```
    StartsAt(isize),
    /// Passes if the needle ends at the specified location in the haystack.
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::EndsAt(0).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(2).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::EndsAt(3).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::EndsAt(4).check("#abc#", "abc").unwrap());
    /// ```
    EndsAt(isize),
    /// Passes if the haystack contains the needle at or after the specified index.
    ///
    /// If you want to only pass when the needle is strictly after the specified index, see [`Self::After`].
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!( StringLocation::AtOrAfter(0).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::AtOrAfter(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(2).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(3).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::AtOrAfter(4).check("#abc#", "abc").unwrap());
    /// ```
    AtOrAfter(isize),
    /// Passes if the haystack contains the needle before or at the specified index.
    ///
    /// If you want to only pass wehn the needle is strictly before the specified index, see [`Self::Before`].
    /// # Errors
    /// If the specified index is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::BeforeOrAt(0).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::BeforeOrAt(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::BeforeOrAt(2).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::BeforeOrAt(3).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::BeforeOrAt(4).check("#abc#", "abc").unwrap());
    /// ```
    BeforeOrAt(isize),
    /// Passes if the haystack contains the needle after the specified index.
    ///
    /// If you want to also pass when the needle is at the specified index, see [`Self::AtOrAfter`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!( StringLocation::After(0).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(2).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(3).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::After(4).check("#abc#", "abc").unwrap());
    /// ```
    After(isize),
    /// Passes if the haystack contains the needle before the specified index.
    ///
    /// If you want to also pass when the needle is at the specified index, see [`Self::BeforeOrAt`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::Before(0).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(1).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(2).check("#abc#", "abc").unwrap());
    /// assert!(!StringLocation::Before(3).check("#abc#", "abc").unwrap());
    /// assert!( StringLocation::Before(4).check("#abc#", "abc").unwrap());
    /// ```
    Before(isize),
    /// Passes if the haystack is equal to the needle.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!( StringLocation::Equals.check( "abc" , "abc").unwrap());
    /// assert!(!StringLocation::Equals.check("#abc" , "abc").unwrap());
    /// assert!(!StringLocation::Equals.check( "abc#", "abc").unwrap());
    /// assert!(!StringLocation::Equals.check("#abc#", "abc").unwrap());
    /// ```
    Equals,
    /// Passes if the haystack contains the needle in the specified range at [`Self::InRange::at`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringLocationError::InvalidSlice`].
    ///
    /// If the call to [`Self::check`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::InRange {
    ///     start: 0,
    ///     end  : Some(3),
    ///     at   : Box::new(StringLocation::Equals)
    /// }.check("abc##", "abc").unwrap());
    /// ```
    InRange {
        /// The start of the range to search in.
        start: isize,
        /// The end of the range to search in, exclusive.
        ///
        /// Set to [`None`] to keep it unbounded.
        end: Option<isize>,
        /// The [`Self`] to search for the needle in the range.
        at: Box<Self>
    },
    /// Passes if any segment of the haystack, split by [`Self::AnySegment::split`], contains the needle at [`Self::AnySegment::at`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(StringLocation::AnySegment {
    ///     split: "/".to_string(),
    ///     at   : Box::new(StringLocation::Equals)
    /// }.check("123/abc/456", "abc").unwrap());
    /// ```
    AnySegment {
        /// The string to split the haystack with.
        split: String,
        /// The [`Self`] to search for the needle in each segment.
        at: Box<Self>
    },
    /// Passes if the [`Self::NthSegment::n`]th segment of the haystack, split by [`Self::NthSegment::split`], contains the needle at [`Self::NthSegment::at`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert!(!StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n    : 0,
    ///     at   : Box::new(StringLocation::Equals)
    /// }.check("123/abc/456", "abc").unwrap());
    ///
    /// assert!(StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n    : 1,
    ///     at   : Box::new(StringLocation::Equals)
    /// }.check("123/abc/456", "abc").unwrap());
    ///
    /// assert!(!StringLocation::NthSegment {
    ///     split: "/".to_string(),
    ///     n    : 2,
    ///     at   : Box::new(StringLocation::Equals)
    /// }.check("123/abc/456", "abc").unwrap());
    /// ```
    NthSegment {
        /// The string to split the haystack with.
        split: String,
        /// The index of the segment to search in.
        n: isize,
        /// The [`Self`] to search for the needle in the [`Self::NthSegment::n`]th segment.
        at: Box<Self>
    }
}

/// The enum of errors [`StringLocation::check`] can return.
#[derive(Debug, Error)]
pub enum StringLocationError {
    /// Returned when a [`StringLocation::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`StringLocation`]s in a [`StringLocation::TryElse`] return errors.
    #[error("Both StringLocations in a StringLocation::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringLocation::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`StringLocation::TryElse::else`].
        else_error: Box<Self>
    },

    /// Returned when a slice is either not on UTF-8 boundaries or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundaries or out of bounds.")]
    InvalidSlice,
    /// Returned when an index is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound
}

impl StringLocation {
    /// Checks if `needle` occurs in `haystack` at the specified location.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check(&self, haystack: &str, needle: &str) -> Result<bool, StringLocationError> {
        debug!(StringLocation::check, self, haystack, needle);
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(StringLocationError::ExplicitError(msg.clone()))?,
            Self::Debug(location) => {
                let is_satisfied=location.check(haystack, needle);
                eprintln!("=== StringLocation::Debug ===\nLocation: {location:?}\nHaystack: {haystack:?}\nNeedle: {needle:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            Self::Swap(location) => location.check(needle, haystack)?,

            // Logic.

            Self::IfContains {at, then, r#else} => if at.check(haystack, needle)? {then} else {r#else}.check(haystack, needle)?,
            Self::All(locations) => {
                for location in locations {
                    if !location.check(haystack, needle)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(locations) => {
                for location in locations {
                    if location.check(haystack, needle)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(location) => !location.check(haystack, needle)?,

            // Error handling.

            Self::TreatErrorAsPass(location) => location.check(haystack, needle).unwrap_or(true),
            Self::TreatErrorAsFail(location) => location.check(haystack, needle).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.check(haystack, needle).or_else(|try_error| r#else.check(haystack, needle).map_err(|else_error| StringLocationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(locations) => {
                let mut result = Ok(false); // Initial value doesn't mean anything.
                for location in locations {
                    result = location.check(haystack, needle);
                    if result.is_ok() {return result}
                }
                result?
            }

            // Other.

            Self::Anywhere => haystack.contains(needle),

            Self::Start            => haystack.starts_with(needle),
            Self::StartsAt (start) => haystack.get((Bound::Included(neg_index(*start, haystack.len()).ok_or(StringLocationError::InvalidIndex)?), Bound::Unbounded)).ok_or(StringLocationError::InvalidSlice)?.starts_with(needle),
            Self::AtOrAfter(start) => haystack.get((Bound::Included(neg_index(*start, haystack.len()).ok_or(StringLocationError::InvalidIndex)?), Bound::Unbounded)).ok_or(StringLocationError::InvalidSlice)?.contains(needle),
            Self::After    (start) => haystack.get((Bound::Excluded(neg_index(*start, haystack.len()).ok_or(StringLocationError::InvalidIndex)?), Bound::Unbounded)).ok_or(StringLocationError::InvalidSlice)?.contains(needle),

            Self::End              => haystack.ends_with(needle),
            Self::EndsAt    (end)  => haystack.get((Bound::Unbounded, Bound::Included(neg_index(*end, haystack.len()).ok_or(StringLocationError::InvalidIndex)?))).ok_or(StringLocationError::InvalidSlice)?.ends_with(needle),
            Self::BeforeOrAt(end)  => haystack.get((Bound::Unbounded, Bound::Included(neg_index(*end, haystack.len()).ok_or(StringLocationError::InvalidIndex)?))).ok_or(StringLocationError::InvalidSlice)?.contains(needle),
            Self::Before    (end)  => haystack.get((Bound::Unbounded, Bound::Excluded(neg_index(*end, haystack.len()).ok_or(StringLocationError::InvalidIndex)?))).ok_or(StringLocationError::InvalidSlice)?.contains(needle),

            Self::InRange {start, end, at} => at.check(
                haystack.get(neg_range(*start, *end, haystack.len()).ok_or(StringLocationError::InvalidSlice)?).ok_or(StringLocationError::InvalidSlice)?,
                needle
            )?,

            Self::Equals => haystack==needle,
            Self::AnySegment {split, at} => {
                for segment in haystack.split(split) {
                    if at.check(segment, needle)? {
                        return Ok(true)
                    }
                }
                return Ok(false)
            },
            Self::NthSegment {split, n, at} => at.check(neg_nth(haystack.split(split), *n).ok_or(StringLocationError::SegmentNotFound)?, needle)?
        })
    }
}

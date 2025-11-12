//! [`CharMatcher`].

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// Match a [`char`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum CharMatcher {
    /// Always satisfied.
    Always,
    /// Never satisfied.
    Never,
    /// Always returns the error [`CharMatcherError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`CharMatcherError::ExplicitError`].
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    /// If [`Self::If::if`] is satisfied, return the value of [`Self::If::then`].
    ///
    /// If [`Self::If::if`] is unsatisfied, return the value of [`Self::If::else`].
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] is satisfied.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] is unsatisfied.
        r#else: Box<Self>
    },
    /// Inverts the satisfaction of the contained [`Self`].
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned.
    Not(Box<Self>),
    /// Satisfied if all contained [`Self`]s are satisfied.
    /// # Errors
    /// If any call to [`Self::check`] returns an error, that error is returned.
    All(Vec<Self>),
    /// Satisfied if any contained [`Self`] is satisfied.
    /// # Errors
    /// If any call to [`Self::check`] returns an error, that error is returned.
    Any(Vec<Self>),

    /// Satisfied if the contained [`Self`] is satisfied or errors.
    ErrorToSatisfied(Box<Self>),
    /// Satisfied if the contained [`Self`] is satisfied.
    ///
    /// Unsatisfied if the contained [`Self`] errors.
    ErrorToUnsatisfied(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::check`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::check`] return errors, both errors are returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },
    /// Checks the contained [`Self`]s in order, stopping as soon as a call to [`Self::check`] doesn't return an error.
    /// # Errors
    /// If all calls to [`Self::check`] return errors, the last error is returned. In the future this should be changed to return all errors.
    FirstNotError(Vec<Self>),

    /// Satisfied if the [`char`] is the specified [`char`].
    Is(char),
    /// Satisfied if the [`char`] is between [`Self::Between::min`] and [`Self::Between::max`] (inclusive).
    Between {
        /// The lower bound of [`char`]s to pass for.
        min: char,
        /// The upper bound of [`char`]s to pass for.
        max: char
    },
    /// Satisfied if the [`char`] is in the specified [`HashSet`].
    IsOneOf(HashSet<char>),
    /// [`char::is_alphabetic`].
    IsAlphabetic,
    /// [`char::is_alphanumeric`].
    IsAlphanumeric,
    /// [`char::is_ascii`].
    IsAscii,
    /// [`char::is_ascii_alphabetic`].
    IsAsciiAlphabetic,
    /// [`char::is_ascii_alphanumeric`].
    IsAsciiAlphanumeric,
    /// [`char::is_ascii_control`].
    IsAsciiControl,
    /// [`char::is_ascii_digit`].
    IsAsciiDigit,
    /// [`char::is_ascii_graphic`].
    IsAsciiGraphic,
    /// [`char::is_ascii_hexdigit`].
    IsAsciiHexdigit,
    /// [`char::is_ascii_lowercase`].
    IsAsciiLowercase,
    /// [`char::is_ascii_octdigit`].
    IsAsciiOctdigit,
    /// [`char::is_ascii_punctuation`].
    IsAsciiPunctuation,
    /// [`char::is_ascii_uppercase`].
    IsAsciiUppercase,
    /// [`char::is_ascii_whitespace`].
    IsAsciiWhitespace,
    /// [`char::is_control`].
    IsControl,
    /// [`char::is_digit`] with the radix `10`.
    IsDigit,
    /// [`char::is_digit`].
    /// # Errors
    /// If the radix is greater than 36, returns the error [`CharMatcherError::InvalidRadix`].
    IsDigitRadix(u32),
    /// [`char::is_lowercase`].
    IsLowercase,
    /// [`char::is_numeric`].
    IsNumeric,
    /// [`char::is_uppercase`].
    IsUppercase,
    /// [`char::is_whitespace`].
    IsWhitespace
}

/// The enum of errors [`CharMatcher::check`] can return.
#[derive(Debug, Error)]
pub enum CharMatcherError {
    /// Returned when a [`StringMatcher::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`CharMatcher`]s in a [`CharMatcher::TryElse`] return errors.
    #[error("Both CharMatchers in a CharMatcher::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`CharMatcher::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`CharMatcher::TryElse::else`].
        else_error: Box<Self>
    },

    /// Returned when attempting to use [`CharMatcher::IsDigitRadix`] with an invalid radix (above 36).
    #[error("Attempted to use CharMatcher::IsDigitRadix with an invalid radix ({0}).")]
    InvalidRadix(u32)
}

impl CharMatcher {
    /// Return [`true`] if `c` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check(&self, c: char) -> Result<bool, CharMatcherError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(s) => Err(CharMatcherError::ExplicitError(s.clone()))?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.check(c);
                eprintln!("=== CharMatcher::Debug ===\nMatcher: {matcher:?}\nC: {c:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.check(c)? {then} else {r#else}.check(c)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.check(c)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.check(c)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.check(c)?,

            // Error handling.

            Self::ErrorToSatisfied  (matcher) => matcher.check(c).unwrap_or(true),
            Self::ErrorToUnsatisfied(matcher) => matcher.check(c).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.check(c).or_else(|try_error| r#else.check(c).map_err(|else_error| CharMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut result = Ok(false); // The initial value doesn't mean anything.
                for matcher in matchers {
                    result = matcher.check(c);
                    if result.is_ok() {return result;}
                }
                result?
            },


            Self::Is(r#char)          => c == *r#char,
            Self::Between {min, max}  => *min <= c && c <= *max,
            Self::IsOneOf(chars)      => chars.contains(&c),



            Self::IsAlphabetic        => c.is_alphabetic(),
            Self::IsAlphanumeric      => c.is_alphanumeric(),
            Self::IsAscii             => c.is_ascii(),
            Self::IsAsciiAlphabetic   => c.is_ascii_alphabetic(),
            Self::IsAsciiAlphanumeric => c.is_ascii_alphanumeric(),
            Self::IsAsciiControl      => c.is_ascii_control(),
            Self::IsAsciiDigit        => c.is_ascii_digit(),
            Self::IsAsciiGraphic      => c.is_ascii_graphic(),
            Self::IsAsciiHexdigit     => c.is_ascii_hexdigit(),
            Self::IsAsciiLowercase    => c.is_ascii_lowercase(),
            Self::IsAsciiOctdigit     => matches!(c, '0'..='7'),
            Self::IsAsciiPunctuation  => c.is_ascii_punctuation(),
            Self::IsAsciiUppercase    => c.is_ascii_uppercase(),
            Self::IsAsciiWhitespace   => c.is_ascii_whitespace(),
            Self::IsControl           => c.is_control(),
            Self::IsDigit             => c.is_ascii_digit(),
            Self::IsDigitRadix(radix) => if *radix <= 36 {c.is_digit(*radix)} else {Err(CharMatcherError::InvalidRadix(*radix))?},
            Self::IsLowercase         => c.is_lowercase(),
            Self::IsNumeric           => c.is_numeric(),
            Self::IsUppercase         => c.is_uppercase(),
            Self::IsWhitespace        => c.is_whitespace()
        })
    }
}

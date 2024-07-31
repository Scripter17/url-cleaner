//! Provides [`CharMatcher`] which allows for testing if a [`char`] matches a certain rule.

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// A general API for matching [`char`]s with a variety of methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharMatcher {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`CharMatcherError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`CharMatcherError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// 
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] errors, returns that error.
    Debug(Box<Self>),

    // Logic

    /// If `r#if` passes, return the result of `then`, otherwise return the result of `r#else`.
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


    /// Passes if the provided [`char`] equals the specified [`char`].
    Equals(char),
    /// Passes if the provided [`char`] is between [`Self::Between::min`] and [`Self::Between::max`], inclusive.
    Between {
        /// The lower bound.
        min: char,
        /// The upper bound.
        max: char
    },
    /// Passes if the provided [`char`] is in the specified [`HashSet`].
    IsOneOf(HashSet<char>),



    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    FirstNotError(Vec<Self>),
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
    // /// [`char::is_ascii_oct_digit`].
    // IsAsciiOctdigit,
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

/// The enum of all possible errors [`CharMatcher::satisfied_by`] can return.
#[derive(Debug, Error)]
pub enum CharMatcherError {
    /// Returned when [`CharMatcher::IsDigitRadix`] has a radix greater than 36 which would make [`char::is_digit`] panic.
    #[error("Invalid radix: {0}. Radi must be between 0 and 36 inclusive. See [`char::is_digit`] for details.")]
    InvalidRadix(u32),
    /// Returned when [`CharMatcher::Error`] is used.
    #[error("CharMatcher::Error was used.")]
    ExplicitError,
    /// Returned when both the `try` and `else` of a [`CharMatcher::TryElse`] both return errors.
    #[error("A `CharMatcher::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`CharMatcher::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`CharMatcher::TryElse::else`],
        else_error: Box<Self>
    },
}

impl CharMatcher {
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, c: char) -> Result<bool, CharMatcherError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(CharMatcherError::ExplicitError)?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.satisfied_by(c);
                eprintln!("=== CharMatcher::Debug ===\nMatcher: {matcher:?}\nC: {c:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(c)? {then} else {r#else}.satisfied_by(c)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.satisfied_by(c)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.satisfied_by(c)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.satisfied_by(c)?,

            // Error handling.

            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(c).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(c).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(c).or_else(|try_error| r#else.satisfied_by(c).map_err(|else_error| CharMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut result = Ok(false); // The initial value doesn't mean anything.
                for matcher in matchers {
                    result = matcher.satisfied_by(c);
                    if result.is_ok() {return result;}
                }
                result?
            },


            Self::Equals(r#char)      => c == *r#char,
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
            // Self::IsAsciiOctdigit     => c.is_ascii_octdigit(),
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

    /// Internal method to make sure I don't accidetnally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used)]
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        match self {
            Self::If {r#if, then, r#else} => r#if.is_suitable_for_release() && then.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::All(matchers) => matchers.iter().all(|matcher| matcher.is_suitable_for_release()),
            Self::Any(matchers) => matchers.iter().all(|matcher| matcher.is_suitable_for_release()),
            Self::Not(matcher) => matcher.is_suitable_for_release(),
            Self::TreatErrorAsPass(matcher) => matcher.is_suitable_for_release(),
            Self::TreatErrorAsFail(matcher) => matcher.is_suitable_for_release(),
            Self::TryElse {r#try, r#else} => r#try.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::Debug(_) => false,
            _ => true
        }
    }
}

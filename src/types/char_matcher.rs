//! Rules for matching a [`char`].

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum CharMatcher {
    Always,
    Never,
    Error,
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
    Equals(char),
    Between {
        min: char,
        max: char
    },
    IsOneOf(HashSet<char>),
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
#[derive(Debug, Error)]
pub enum CharMatcherError {
    #[error("Invalid radix: {0}. Radix must be between 0 and 36 inclusive. See [`char::is_digit`] for details.")]
    InvalidRadix(u32),
    #[error("CharMatcher::Error was used.")]
    ExplicitError,
    #[error("A `CharMatcher::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    },
}

impl CharMatcher {
    /// Return [`true`] if `c` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
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
}

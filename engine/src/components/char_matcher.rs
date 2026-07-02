//! [`CharMatcher`].

use crate::prelude::*;

/// Match a [`char`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum CharMatcher {
    /// [`true`].
    Always,
    /// [`false`].
    Never,
    /// [`ExplicitError`].
    Error(String),

    /// If [`Self::If::if`], [`Self::If::then`], else [`Self::If::else`].
    If {
        /// The if.
        r#if: Box<Self>,
        /// The then.
        then: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// Invert.
    Not(Box<Self>),
    /// All.
    All(Vec<Self>),
    /// Any.
    Any(Vec<Self>),

    /// Map [`Err`] to [`true`].
    ErrorToSatisfied(Box<Self>),
    /// Map [`Err`] to [`false`].
    ErrorToUnsatisfied(Box<Self>),
    /// [`Self::TryElse::try`] or, if it's [`Err`], [`Self::TryElse::else`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// The first [`Self`] to return [`Ok`].
    FirstNotError(Vec<Self>),

    /// If the [`char`] is [`Self::Is::0`].
    Is(char),
    /// [`RangeBounds::contains`].
    Between {
        /// The start of the range.
        ///
        /// Defaulted to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        start: Bound<char>,
        /// The end of the range.
        ///
        /// Defaulted to [`Bound::Unbounded`].
        #[serde(default = "unbounded", skip_serializing_if = "is_unbounded")]
        end: Bound<char>,
    },
    /// Satisfied if the [`char`] is in the specified [`HashSet`].
    IsOneOf(HashSet<char>),

    /** [`char::is_alphabetic`].                **/ IsAlphabetic,
    /** [`char::is_alphanumeric`].              **/ IsAlphanumeric,
    /** [`char::is_ascii`].                     **/ IsAscii,
    /** [`char::is_ascii_alphabetic`].          **/ IsAsciiAlphabetic,
    /** [`char::is_ascii_alphanumeric`].        **/ IsAsciiAlphanumeric,
    /** [`char::is_ascii_control`].             **/ IsAsciiControl,
    /** [`char::is_ascii_digit`].               **/ IsAsciiDigit,
    /** [`char::is_ascii_graphic`].             **/ IsAsciiGraphic,
    /** [`char::is_ascii_hexdigit`].            **/ IsAsciiHexdigit,
    /** [`char::is_ascii_lowercase`].           **/ IsAsciiLowercase,
    /** [`char::is_ascii_octdigit`].            **/ IsAsciiOctdigit,
    /** [`char::is_ascii_punctuation`].         **/ IsAsciiPunctuation,
    /** [`char::is_ascii_uppercase`].           **/ IsAsciiUppercase,
    /** [`char::is_ascii_whitespace`].          **/ IsAsciiWhitespace,
    /** [`char::is_control`].                   **/ IsControl,
    /** [`char::is_digit`] with the radix `10`. **/ IsDigit,
    /** [`char::is_digit`].                     **/ IsDigitRadix(Radix),
    /** [`char::is_lowercase`].                 **/ IsLowercase,
    /** [`char::is_numeric`].                   **/ IsNumeric,
    /** [`char::is_uppercase`].                 **/ IsUppercase,
    /** [`char::is_whitespace`].                **/ IsWhitespace
}

impl CharMatcher {
    /// Return [`true`] if `c` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check(&self, c: char) -> Result<bool, CharMatcherError> {
        Ok(match self {
            Self::Always => true,
            Self::Never  => false,
            Self::Error(s) => Err(ExplicitError(s.clone()))?,

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
            Self::TryElse{r#try, r#else} => match r#try.check(c) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(c) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(matchers) => {
                let mut errors = Vec::new();
                for matcher in matchers {
                    match matcher.check(c) {
                        Ok(x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }
                Err(FirstNotErrorErrors(errors))?
            },


            Self::Is(r#char)           => c == *r#char,
            Self::Between {start, end} => (*start, *end).contains(&c),
            Self::IsOneOf(chars)       => chars.contains(&c),



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
            Self::IsDigitRadix(radix) => radix.char_is_digit(c),
            Self::IsLowercase         => c.is_lowercase(),
            Self::IsNumeric           => c.is_numeric(),
            Self::IsUppercase         => c.is_uppercase(),
            Self::IsWhitespace        => c.is_whitespace()
        })
    }
}

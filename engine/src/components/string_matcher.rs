//! [`StringMatcher`].

#![allow(unused_assignments, reason = "False positive.")]

#[expect(unused_imports, reason = "Used in docs.")]
use regex::Regex;

use crate::prelude::*;

/// Match a string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum StringMatcher {
    /// [`true`].
    Always,
    /// [`false`].
    ///
    /// The default.
    #[default]
    Never,
    /// [`ExplicitError`].
    Error(String),

    // Logic

    /// If [`Self::If::if`] then [`Self::If::then`], otherwise [`Self::If::else`].
    If {
        /// The if.
        r#if: Box<Self>,
        /// The then.
        then: Box<Self>,
        /// The else.
        ///
        /// Defaults to [`Self::Never`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
    },
    /// Invert.
    Not(Box<Self>),
    /// All.
    All(Vec<Self>),
    /// Any.
    Any(Vec<Self>),

    // Error handling

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

    // Equality

    /// [`StringSource::get`] + [`PartialEq::eq`].
    Is(StringSource),
    /// [`SetSource::get`] + [`Set::contains`].
    IsInSet(SetSource),

    // Containment

    /// [`StringSource::get`] + [`str::starts_with`].
    StartsWith(StringSource),
    /// [`StringSource::get`] + [`str::ends_with`].
    EndsWith(StringSource),
    /// [`StringSource::get`] + [`StringLocation::check`].
    Contains {
        /// The [`StringSource`].
        substr: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },

    /// [`StringSource::get`] + [`str::starts_with`].
    IsPrefixOf(StringSource),
    /// [`StringSource::get`] + [`str::ends_with`].
    IsSuffixOf(StringSource),
    /// Inverse of [`Self::Contains`].
    IsContainedIn {
        /// The [`StringSource`].
        container: StringSource,
        /// The [`StringLocation`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },

    // Char matching

    /// Passes if [`None`] or [`str::is_ascii`].
    IsAscii,

    /// Passes if [`None`] or every character is [`char::is_alphabetic`].
    AllAlphabetic,
    /// Passes if [`None`] or every character is [`char::is_alphanumeric`].
    AllAlphanumeric,
    /// Passes if [`None`] or every character is [`char::is_ascii_alphabetic`].
    AllAsciiAlphabetic,
    /// Passes if [`None`] or every character is [`char::is_ascii_alphanumeric`].
    AllAsciiAlphanumeric,
    /// Passes if [`None`] or every character is [`char::is_ascii_control`].
    AllAsciiControl,
    /// Passes if [`None`] or every character is [`char::is_ascii_digit`].
    AllAsciiDigit,
    /// Passes if [`None`] or every character is [`char::is_ascii_graphic`].
    AllAsciiGraphic,
    /// Passes if [`None`] or every character is [`char::is_ascii_hexdigit`].
    AllAsciiHexdigit,
    /// Passes if [`None`] or every character is [`char::is_ascii_lowercase`].
    AllAsciiLowercase,
    /// Passes if [`None`] or every character is [`char::is_ascii_octdigit`].
    AllAsciiOctdigit,
    /// Passes if [`None`] or every character is [`char::is_ascii_punctuation`].
    AllAsciiPunctuation,
    /// Passes if [`None`] or every character is [`char::is_ascii_uppercase`].
    AllAsciiUppercase,
    /// Passes if [`None`] or every character is [`char::is_ascii_whitespace`].
    AllAsciiWhitespace,
    /// Passes if [`None`] or every character is [`char::is_control`].
    AllControl,
    /// Passes if [`None`] or every character is [`char::is_digit`].
    AllDigit(Radix),
    /// Passes if [`None`] or every character is [`char::is_lowercase`].
    AllLowercase,
    /// Passes if [`None`] or every character is [`char::is_numeric`].
    AllNumeric,
    /// Passes if [`None`] or every character is [`char::is_uppercase`].
    AllUppercase,
    /// Passes if [`None`] or every character is [`char::is_whitespace`].
    AllWhitespace,
    /// Passes if [`None`] or all [`char`]s in the string are in the specified [`HashSet`].
    AllCharsAreOneOf(HashSet<char>),
    /// Passes if [`None`] or any of the [`char`]s in the string are in the specified [`HashSet`].
    AnyCharIsOneOf(HashSet<char>),
    /// Passes if [`None`] or all [`char`]s in the string satisfy the specified [`CharMatcher`].
    AllCharsMatch(CharMatcher),
    /// Passes if [`None`] or any [`char`]s in the string satisfy the specified [`CharMatcher`].
    AnyCharMatches(CharMatcher),

    // Other

    /// Satisfied if the length of the string is the specified value.
    /// # Errors
    /// If [`None`], returns the error [`SubjectIsNone`].
    LengthIs(usize),

    /// Applies [`Self::Modified::modification`] to a copy of the string, leaving the original unchanged, and returns the satisfaction of [`Self::Modified::matcher`] on that string.
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), checkerr(Self))]
    Modified {
        /// The [`StringModification`] to apply to the copy of the string.
        modification: Box<StringModification>,
        /// The [`Self`] to match the modified string with.
        matcher: Box<Self>
    },

    /// Satisfied if the string is [`Some`].
    IsSome,
    /// Satisfied if the string is [`None`].
    IsNone,
    /// Satisfied if the string is [`Some`] and [`Self::IsSomeAnd::0`] is satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    IsSomeAnd(Box<Self>),
    /// Satisfied if the string is [`None`] or [`Self::IsNoneOr::0`] is satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    IsNoneOr(Box<Self>),

    // Glue

    /// Satisfied if the call to [`Regex::is_match`] returns [`true`].
    /// # Errors
    /// If [`None`], returns the error [`SubjectIsNone`].
    ///
    #[doc = edoc!(geterr(LazyRegex))]
    Regex(LazyRegex),

    // Function/Extern

    /// Uses a [`Self`] from [`Cleaner::functions`].
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`FunctionArgs`].
    FunctionArg(StringSource),
    /// Calls the contained function.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    /// # Errors
    #[doc = edoc!(callerr(Self::Extern::0))]
    #[suitable(never)]
    #[serde(skip)]
    Extern(StringMatcherExtern)
}

impl From<LazyRegex> for StringMatcher {
    fn from(value: LazyRegex) -> Self {
        Self::Regex(value)
    }
}

impl StringMatcher {
    /// Returns [`true`] if `value` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>, value: Option<&str>) -> Result<bool, StringMatcherError> {
        debug!(StringMatcher::check, self, args, value; self._check(task_state, args, value))
    }

    /// [`Self::check`].
    fn _check<'j>(&'j self, task_state: &TaskState<'j>, args: Option<&'j FunctionArgs>, value: Option<&str>) -> Result<bool, StringMatcherError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Logic.

            Self::IsSome => value.is_some(),
            Self::IsNone => value.is_none(),
            Self::IsSomeAnd(matcher) => value.is_some() && matcher.check(task_state, args, value)?,
            Self::IsNoneOr (matcher) => value.is_none() || matcher.check(task_state, args, value)?,

            Self::If {r#if, then, r#else} => match r#if.check(task_state, args, value)? {
                true  => then  .check(task_state, args, value)?,
                false => r#else.check(task_state, args, value)?,
            },
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.check(task_state, args, value)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.check(task_state, args, value)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.check(task_state, args, value)?,

            // Error handling.

            Self::ErrorToSatisfied  (matcher) => matcher.check(task_state, args, value).unwrap_or(true ),
            Self::ErrorToUnsatisfied(matcher) => matcher.check(task_state, args, value).unwrap_or(false),
            Self::TryElse {r#try, r#else} => match r#try.check(task_state, args, value) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(task_state, args, value) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },
            Self::FirstNotError(matchers) => {
                let mut errors = Vec::new();

                for matcher in matchers {
                    match matcher.check(task_state, args, value) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },

            // Equality

            Self::Is     (check) => value == get!(?&check),
            Self::IsInSet(set  ) => get!(set).contains(value),

            // Containment

            Self::StartsWith(prefix    ) => value.ok_or(SubjectIsNone)?.starts_with(get!(&prefix)),
            Self::EndsWith  (suffix    ) => value.ok_or(SubjectIsNone)?.ends_with  (get!(&suffix)),
            Self::Contains  {substr, at} => at.check(value.ok_or(SubjectIsNone)?,   get!(&substr))?,

            Self::IsPrefixOf    (container    ) => get!(container).starts_with(value.ok_or(SubjectIsNone)?),
            Self::IsSuffixOf    (container    ) => get!(container).ends_with  (value.ok_or(SubjectIsNone)?),
            Self::IsContainedIn {container, at} => at.check(get!(&container),  value.ok_or(SubjectIsNone)?)?,

            // Char matcher

            Self::IsAscii => value.unwrap_or_default().is_ascii(),

            Self::AllAlphabetic           =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_alphabetic())),
            Self::AllAlphanumeric         =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_alphanumeric())),
            Self::AllAsciiAlphabetic      =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_alphabetic())),
            Self::AllAsciiAlphanumeric    =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_alphanumeric())),
            Self::AllAsciiControl         =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_control())),
            Self::AllAsciiDigit           =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_digit())),
            Self::AllAsciiGraphic         =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_graphic())),
            Self::AllAsciiHexdigit        =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_hexdigit())),
            Self::AllAsciiLowercase       =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_lowercase())),
            Self::AllAsciiOctdigit        =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_digit(8))),
            Self::AllAsciiPunctuation     =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_punctuation())),
            Self::AllAsciiUppercase       =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_uppercase())),
            Self::AllAsciiWhitespace      =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_ascii_whitespace())),
            Self::AllControl              =>  value.is_none_or (|value| value.bytes().all(|b| b.is_ascii() && (b as char).is_control())),
            Self::AllDigit(radix)         =>  value.is_none_or (|value| value.bytes().all(|b| radix.byte_is_digit(b))),
            Self::AllLowercase            =>  value.is_none_or (|value| value.chars().all(|c| c.is_lowercase())),
            Self::AllNumeric              =>  value.is_none_or (|value| value.chars().all(|c| c.is_numeric())),
            Self::AllUppercase            =>  value.is_none_or (|value| value.chars().all(|c| c.is_uppercase())),
            Self::AllWhitespace           =>  value.is_none_or (|value| value.chars().all(|c| c.is_whitespace())),
            Self::AllCharsAreOneOf(chars) =>  value.is_none_or (|value| value.chars().all(|c| chars.contains(&c))),
            Self::AnyCharIsOneOf  (chars) =>  value.is_none_or (|value| value.chars().any(|c| chars.contains(&c))),
            Self::AllCharsMatch(matcher) => {
                for c in value.unwrap_or_default().chars() {
                    if !matcher.check(c)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::AnyCharMatches(matcher) => {
                for c in value.unwrap_or_default().chars() {
                    if matcher.check(c)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Other

            Self::LengthIs(x) => value.ok_or(SubjectIsNone)?.len() == *x,

            Self::Modified {modification, matcher} => {
                let mut temp = value.map(Cow::Borrowed);
                modification.apply(task_state, args, &mut temp)?;
                matcher.check(task_state, args, temp.as_deref())?
            }

            // Glue

            Self::Regex(regex) => regex.get()?.is_match(value.ok_or(SubjectIsNone)?),

            // Misc

            Self::Function(call) => task_state.job.cleaner.functions.string_matchers
                .get(&call.name).ok_or(FunctionNotFound)?
                .check(task_state, Some(&call.args), value)?,

            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.string_matchers
                .get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?
                .check(task_state, args, value)?,

            Self::Extern(function) => function(task_state, args, value)?,
        })
    }
}

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
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it's [`Err`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// The first contained [`Self`] to return [`Ok`].
    /// # Errors
    /// If no contained [`Self`] returns [`Ok`], returns the error [`FirstNotErrorErrors`] containing every error.
    FirstNotError(Vec<Self>),

    // Equality

    /// [`StringSource::get`] + [`Eq::eq`].
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
        substring: StringSource,
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
    /// # Errors
    /// If the radix is less than 2 or greater than 36, returns the error [`StringMatcherError::InvalidRadix`].
    AllDigit(u32),
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
    /// Passes if [`None`] or none of the [`char`]s in the string are in the specified [`HashSet`].
    NoCharIsOneOf(HashSet<char>),
    /// Passes if [`None`] or all [`char`]s in the string satisfy the specified [`CharMatcher`].
    AllCharsMatch(CharMatcher),
    /// Passes if [`None`] or any [`char`]s in the string satisfy the specified [`CharMatcher`].
    AnyCharMatches(CharMatcher),

    // Other

    /// Satisfied if the length of the string is the specified value.
    /// # Errors
    #[doc = edoc!(stringisnone(StringMatcher))]
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
    #[doc = edoc!(stringisnone(StringMatcher), geterr(LazyRegex))]
    Regex(LazyRegex),

    // Function/Extern

    /// Uses a [`Self`] from [`Cleaner::functions`].
    /// # Errors
    #[doc = edoc!(functionnotfound(Self, StringMatcher), checkerr(Self))]
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    /// # Errors
    #[doc = edoc!(notinfunction(StringMatcher), callargfunctionnotfound(Self, StringMatcher), checkerr(Self))]
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

/// The enum of errors [`StringMatcher::check`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// [`ExplicitError`].
    #[error(transparent)]
    ExplicitError(#[from] ExplicitError),
    /// [`TryElseError`].
    #[error(transparent)]
    TryElseError(#[from] Box<TryElseError<Self>>),
    /// [`FirstNotErrorErrors`].
    #[error(transparent)]
    FirstNotErrorErrors(#[from] FirstNotErrorErrors<Self>),

    /// [`SubjectIsNone`]
    #[error(transparent)]
    SubjectIsNone(#[from] SubjectIsNone),

    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`CharMatcherError`] is encountered.
    #[error(transparent)]
    CharMatcherError(#[from] CharMatcherError),

    /// [`ListNotFound`].
    #[error(transparent)]
    ListNotFound(#[from] ListNotFound),
    /// Returned when a [`SetSourceError`] is encountered.
    #[error(transparent)]
    SetSourceError(#[from] SetSourceError),
    /// [`SetNotFound`].
    #[error(transparent)]
    SetNotFound(#[from] SetNotFound),

    /// Returned when attempting to use [`StringMatcher::IsNumberRadix`] with an invalid radix (above 36).
    #[error("Attempted to use StringMatcher::IsNumberRadix with an invalid radix ({0}).")]
    InvalidRadix(u32),

    /// [`regex::Error`].
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    /// [`FunctionNotFound`].
    #[error(transparent)]
    FunctionNotFound(#[from] FunctionNotFound),
    /// [`NotInFunction`].
    #[error(transparent)]
    NotInFunction(#[from] NotInFunction),
    /// [`FunctionArgFunctionNotFound`].
    #[error(transparent)]
    FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /// An arbitrary [`std::error::Error`] for use with [`StringMatcher::Extern`].
    #[error(transparent)]
    Extern(Box<dyn std::error::Error + Send + Sync>)
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

            Self::StartsWith(prefix       ) => value.ok_or(SubjectIsNone)?.starts_with(get!(&prefix   )),
            Self::EndsWith  (suffix       ) => value.ok_or(SubjectIsNone)?.ends_with  (get!(&suffix   )),
            Self::Contains  {substring, at} => at.check(value.ok_or(SubjectIsNone)?,   get!(&substring))?,

            Self::IsPrefixOf    (container    ) => get!(container).starts_with(value.ok_or(SubjectIsNone)?),
            Self::IsSuffixOf    (container    ) => get!(container).ends_with  (value.ok_or(SubjectIsNone)?),
            Self::IsContainedIn {container, at} => at.check(get!(&container),  value.ok_or(SubjectIsNone)?)?,

            // Char matcher

            Self::IsAscii => value.unwrap_or_default().is_ascii(),

            Self::AllAlphabetic        => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_alphabetic()),
            Self::AllAlphanumeric      => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_alphanumeric()),
            Self::AllAsciiAlphabetic   => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_alphabetic()),
            Self::AllAsciiAlphanumeric => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_alphanumeric()),
            Self::AllAsciiControl      => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_control()),
            Self::AllAsciiDigit        => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_digit()),
            Self::AllAsciiGraphic      => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_graphic()),
            Self::AllAsciiHexdigit     => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_hexdigit()),
            Self::AllAsciiLowercase    => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_lowercase()),
            Self::AllAsciiOctdigit     => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_digit(8)),
            Self::AllAsciiPunctuation  => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_punctuation()),
            Self::AllAsciiUppercase    => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_uppercase()),
            Self::AllAsciiWhitespace   => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_ascii_whitespace()),
            Self::AllControl           => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_control()),
            Self::AllDigit(radix)      => match radix {
                2..=36 => value.unwrap_or_default().bytes().all(|b| b.is_ascii() && (b as char).is_digit(*radix)),
                _ => Err(StringMatcherError::InvalidRadix(*radix))?
            },
            Self::AllLowercase            =>  value.unwrap_or_default().chars().all(|c| c.is_lowercase()),
            Self::AllNumeric              =>  value.unwrap_or_default().chars().all(|c| c.is_numeric()),
            Self::AllUppercase            =>  value.unwrap_or_default().chars().all(|c| c.is_uppercase()),
            Self::AllWhitespace           =>  value.unwrap_or_default().chars().all(|c| c.is_whitespace()),
            Self::AllCharsAreOneOf(chars) =>  value.unwrap_or_default().chars().all(|c| chars.contains(&c)),
            Self::AnyCharIsOneOf  (chars) =>  value.unwrap_or_default().chars().any(|c| chars.contains(&c)),
            Self::NoCharIsOneOf   (chars) => !value.unwrap_or_default().chars().any(|c| chars.contains(&c)),
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

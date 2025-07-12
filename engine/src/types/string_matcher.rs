//! Rules for matching a string.

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashMap;
use std::collections::HashSet;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
#[cfg(feature = "regex")]
#[expect(unused_imports, reason = "Used in docs.")]
use ::regex::Regex;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Check if a [`str`] matches a certain pattern/rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
pub enum StringMatcher {
    /// Always passes.
    Always,
    /// Always fails.
    Never,
    /// Always returns the error [`StringMatcherError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringMatcherError::ExplicitError`].
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    // Logic

    /// If the call to [`Self::If::if`] passes, return the value of [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails, return the value of [`Self::If::else`].
    /// # Errors
    #[doc = edoc!(checkerr(Self, 2))]
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] fails.
        r#else: Box<Self>
    },
    /// If the call to [`Self::check`] passes or fails, invert it into failing or passing.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    Any(Vec<Self>),

    /// If the call to [`Self::check`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::check`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::check`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(checkerrte(Self, StringMatcher))]
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try'] returns an error.
        r#else: Box<Self>
    },
    /// Calls [`Self::check`] on each contained [`Self`] in order, returning the first to return [`Ok`].
    /// # Errors
    #[doc = edoc!(checkerrfne(Self, StringMatcher))]
    FirstNotError(Vec<Self>),

    // Equality

    /// Passes if the string is equal to the value of the specified [`StringSource`].
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns [`false`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    Is(StringSource),
    /// Passes if the string is in the specified [`Set`].
    IsOneOf(Set<String>),
    /// Passes if the string is in the specified [`Params::sets`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, StringMatcher), notfound(Set, StringMatcher))]
    IsInSet(#[suitable(assert = "set_is_documented")] StringSource),

    // Containment

    /// Passes if the sring starts with the specified string.
    /// # Errors
    /// If the string is [`None`], returns the error [`StringMatcherError::StringIsNone`].
    StartsWith(String),
    /// Passes if the sring ends with the specified string.
    /// # Errors
    /// If the string is [`None`], returns the error [`StringMatcherError::StringIsNone`].
    EndsWith(String),
    /// Passes if the string is a prefix of the specified string.
    /// # Errors
    /// If the string is [`None`], returns the error [`StringMatcherError::StringIsNone`].
    IsPrefixOf(String),
    /// Passes if the sring is a suffix of the specified string.
    /// # Errors
    /// If the string is [`None`], returns the error [`StringMatcherError::StringIsNone`].
    IsSuffixOf(String),

    /// Passes if the string contains [`Self::Contains::value`] at [`Self::Contains::at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, StringMatcher), checkerr(StringLocation))]
    Contains {
        /// The value to look for at [`Self::Contains::at`].
        value: StringSource,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Effectively [`Self::Contains`] for each value in [`Self::ContainsAny::values`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 3), getnone(StringSource, StringMatcher, 3), checkerr(StringLocation, 3))]
    ContainsAny {
        /// The value to look for at [`Self::Contains::at`].
        values: Vec<StringSource>,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Effectively [`Self::ContainsAny`] for each value in the [`Params::lists`]s specified by [`Self::ContainsAnyInList::list`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, StringMatcher))]
    ///
    /// If no list with the specified name is found, returns the error [`StringMatcherError::ListNotFound`].
    ///
    #[doc = edoc!(checkerr(StringLocation, 3))]
    ContainsAnyInList {
        /// The name of the [`Params::lists`] whose values to look for.
        list: StringSource,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },

    // Char matching

    /// Passes if all [`char`]s in the string are in the specified [`HashSet`].
    AllCharsAreOneOf(HashSet<char>),
    /// Passes if any of the [`char`]s in the string are in the specified [`HashSet`].
    AnyCharIsOneOf(HashSet<char>),
    /// Passes if none of the [`char`]s in the string are in the specified [`HashSet`].
    NoCharIsOneOf(HashSet<char>),
    /// Passes if all [`char`]s in the string satisfies the specified [`CharMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(CharMatcher, 3))]
    AllCharsMatch(CharMatcher),
    /// Passes if any [`char`]s in the string satisfies the specified [`CharMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(CharMatcher, 3))]
    AnyCharMatches(CharMatcher),
    /// Passes if [`str::is_ascii`] returns [`true`].
    IsAscii,

    // Segments

    /// Splits the string with [`Self::NthSegmentMatches::split`], gets the [`Self::NthSegmentMatches::n`]th segment, and returns the satisfaction of [`Self::NthSegmentMatches::matcher`] of it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, StringMatcher))]
    ///
    /// If the segment isn't found, returns the error [`StringMatcherError::SegmentNotFound`].
    ///
    #[doc = edoc!(checkerr(Self))]
    NthSegmentMatches {
        /// The value to split the string with.
        split: StringSource,
        /// The index of the segment to set.
        n: isize,
        /// The [`Self`] to match the segment with.
        matcher: Box<Self>
    },
    /// Splits the string with [`Self::NthSegmentMatches::split`] and passes if any segment satisfies [`Self::AnySegmentMatches::matcher`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, StringMatcher), checkerr(Self))]
    AnySegmentMatches {
        /// The value to split the string with.
        split: StringSource,
        /// The [`Self`] to match the segments with.
        matcher: Box<Self>
    },
    /// Splits the string with [`Self::SegmentsStartWith::split`] and passes if the list of segments starts with the list of segments from splitting [`Self::SegmentsStartWith::value`] with [`Self::SegmentsStartWith::split`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, StringMatcher, 2))]
    SegmentsStartWith {
        /// The value to split the strings with.
        split: Box<StringSource>,
        /// The value to get the subsegments from.
        value: Box<StringSource>
    },
    /// Splits the string with [`Self::SegmentsEndWith::split`] and passes if the list of segments ends with the list of segments from splitting [`Self::SegmentsEndWith::value`] with [`Self::SegmentsEndWith::split`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, StringMatcher, 2))]
    SegmentsEndWith {
        /// The value to split the strings with.
        split: Box<StringSource>,
        /// The value to get the subsegments from.
        value: Box<StringSource>
    },

    // Other

    /// Passes if the length of the string is the specified value.
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

    /// Passes if the string is [`Some`].
    IsSome,
    /// Passes if the string is [`None`].
    IsNone,
    /// Passes if the string is [`Some`] and [`Self::IsSomeAnd::0`] passes.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    IsSomeAnd(Box<Self>),
    /// Passes if the string is [`None`] or [`Self::IsNoneOr::0`] passes.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    IsNoneOr(Box<Self>),

    // Glue

    /// Passes if the call to [`Regex::is_match`] returns [`true`].
    /// # Errors
    #[doc = edoc!(geterr(RegexWrapper))]
    #[cfg(feature = "regex")]
    Regex(RegexWrapper),

    // Common/Custom

    /// Gets a [`Self`] from [`TaskStateView::commons`]'s [`Commons::string_modifications`] and applies it.
    /// # Errors
    #[doc = edoc!(ageterr(StringSource, CommonCall::name), agetnone(StringSource, StringMatcher, CommonCall::name), commonnotfound(Self, StringMatcher), callerr(CommonCallArgsSource::build), checkerr(Self))]
    Common(CommonCall),
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::string_matchers`] and applies it.
    /// # Errors
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`StringMatcherError::NotInCommonContext`].
    ///
    #[doc = edoc!(commoncallargnotfound(Self, StringMatcher), checkerr(Self))]
    CommonCallArg(StringSource),
    /// Calls the contained function.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(Option<&str>, &TaskStateView) -> Result<bool, StringMatcherError>)
}

#[cfg(feature = "regex")]
impl From<RegexWrapper> for StringMatcher {
    fn from(value: RegexWrapper) -> Self {
        Self::Regex(value)
    }
}

/// The enum of errors [`StringMatcher::check`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when a [`StringMatcher::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`StringMatcher`]s in a [`StringMatcher::TryElse`] return errors.
    #[error("Both StringMatchers in a StringMatcher::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringMatcher::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`StringMatcher::TryElse::else`].
        else_error: Box<Self>
    },
    /// Returned when all [`StringMatcher`]s in a [`StringMatcher::FirstNotError`] error.
    #[error("All StringMatchers in a StringMatcher::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

    /// Returned when the string to match is [`None`] where it has to be [`Some`].
    #[error("The string to match was None where it had to be Some.")]
    StringIsNone,

    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`CharMatcherError`] is encountered.
    #[error(transparent)]
    CharMatcherError(#[from] CharMatcherError),

    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a list wasn't found.
    #[error("The requested list wasn't found.")]
    ListNotFound,
    /// Returned when a [`Set`] isn't found.
    #[error("The requested set wasn't found.")]
    SetNotFound,

    /// Returned when a [`::regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),

    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Returned when a [`StringMatcher`] with the specified name isn't found in the [`Commons::string_matchers`].
    #[error("A StringMatcher with the specified name wasn't found in the Commons::string_matchers.")]
    CommonStringMatcherNotFound,
    /// Returned when trying to use [`StringMatcher::CommonCallArg`] outside of a common context.
    #[error("Tried to use StringMatcher::CommonCallArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the [`StringMatcher`] requested from a [`StringMatcher::CommonCallArg`] isn't found.
    #[error("The StringMatcher requested from a StringMatcher::CommonCallArg wasn't found.")]
    CommonCallArgStringMatcherNotFound,

    /// An arbitrary [`std::error::Error`] for use with [`StringMatcher::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl StringMatcher {
    /// Returns [`true`] if `haystack` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check(&self, haystack: Option<&str>, task_state: &TaskStateView) -> Result<bool, StringMatcherError> {
        debug!(StringMatcher::check, self, haystack);
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(StringMatcherError::ExplicitError(msg.clone()))?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.check(haystack, task_state);
                eprintln!("=== StringMatcher::Debug ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\ntask_state: {task_state:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.check(haystack, task_state)? {then} else {r#else}.check(haystack, task_state)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.check(haystack, task_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.check(haystack, task_state)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.check(haystack, task_state)?,

            // Error handling.

            Self::TreatErrorAsPass(matcher) => matcher.check(haystack, task_state).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.check(haystack, task_state).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.check(haystack, task_state).or_else(|try_error| r#else.check(haystack, task_state).map_err(|else_error| StringMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut errors = Vec::new();
                for matcher in matchers {
                    match matcher.check(haystack, task_state) {
                        Ok(x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }
                Err(StringMatcherError::FirstNotErrorErrors(errors))?
            },

            // Equality

            Self::Is(value) => haystack == get_option_str!(value, task_state),
            Self::IsOneOf(hash_set) => hash_set.contains(haystack),
            Self::IsInSet(name) => task_state.params.sets.get(get_str!(name, task_state, StringMatcherError)).ok_or(StringMatcherError::SetNotFound)?.contains(haystack),

            // Containment

            Self::StartsWith(needle) => haystack.ok_or(StringMatcherError::StringIsNone)?.starts_with(needle),
            Self::EndsWith  (needle) => haystack.ok_or(StringMatcherError::StringIsNone)?.ends_with  (needle),
            Self::IsPrefixOf(needle) => needle.starts_with(haystack.ok_or(StringMatcherError::StringIsNone)?),
            Self::IsSuffixOf(needle) => needle.ends_with  (haystack.ok_or(StringMatcherError::StringIsNone)?),

            Self::Contains {at, value} => at.check(haystack.ok_or(StringMatcherError::StringIsNone)?, get_str!(value, task_state, StringMatcherError))?,
            Self::ContainsAny {values, at} => {
                let haystack = haystack.ok_or(StringMatcherError::StringIsNone)?;
                for value in values {
                    if at.check(haystack, get_str!(value, task_state, StringModificationError))? {
                        return Ok(true)
                    }
                }
                false
            },
            Self::ContainsAnyInList {at, list} => {
                let haystack = haystack.ok_or(StringMatcherError::StringIsNone)?;
                for x in task_state.params.lists.get(get_str!(list, task_state, StringMatcherError)).ok_or(StringMatcherError::ListNotFound)? {
                    if at.check(haystack, x)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Char matcher

            Self::AllCharsAreOneOf(chars) =>  haystack.ok_or(StringMatcherError::StringIsNone)?.chars().all(|c| chars.contains(&c)),
            Self::AnyCharIsOneOf  (chars) =>  haystack.ok_or(StringMatcherError::StringIsNone)?.chars().any(|c| chars.contains(&c)),
            Self::NoCharIsOneOf   (chars) => !haystack.ok_or(StringMatcherError::StringIsNone)?.chars().any(|c| chars.contains(&c)),
            Self::AllCharsMatch(matcher) => {
                for char in haystack.ok_or(StringMatcherError::StringIsNone)?.chars() {
                    if !matcher.check(char)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::AnyCharMatches(matcher) => {
                for char in haystack.ok_or(StringMatcherError::StringIsNone)?.chars() {
                    if matcher.check(char)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::IsAscii => haystack.ok_or(StringMatcherError::StringIsNone)?.is_ascii(),

            // Segments

            Self::NthSegmentMatches {n, split, matcher} => matcher.check(Some(neg_nth(haystack.ok_or(StringMatcherError::StringIsNone)?.split(get_str!(split, task_state, StringMatcherError)), *n).ok_or(StringMatcherError::SegmentNotFound)?), task_state)?,
            Self::AnySegmentMatches {split, matcher} => {
                for segment in haystack.ok_or(StringMatcherError::StringIsNone)?.split(get_str!(split, task_state, StringMatcherError)) {
                    if matcher.check(Some(segment), task_state)? {
                        return Ok(true);
                    }
                };
                return Ok(false);
            },
            Self::SegmentsEndWith { split, value } => {
                let split = get_str!(split, task_state, StringMatcherError);
                // haystack.split(split).collect::<Vec<_>>().into_iter().rev().zip(get_str!(value, task_state, StringMatcherError).split(split)).all(|(x, y)| x==y)
                haystack.ok_or(StringMatcherError::StringIsNone)?.strip_suffix(get_str!(value, task_state, StringMatcherError))
                    .is_some_and(|x| x.split(split).last()==Some(""))
            },
            Self::SegmentsStartWith { split, value } => {
                let split = get_str!(split, task_state, StringMatcherError);
                haystack.ok_or(StringMatcherError::StringIsNone)?.strip_prefix(get_str!(value, task_state, StringMatcherError))
                    .is_some_and(|x| x.strip_prefix(split).is_some())
            },

            // Other

            Self::LengthIs(x) => haystack.ok_or(StringMatcherError::StringIsNone)?.len() == *x,

            Self::Modified {modification, matcher} => {
                let mut temp = haystack.map(Cow::Borrowed);
                modification.apply(&mut temp, task_state)?;
                matcher.check(temp.as_deref(), task_state)?
            }

            Self::IsSome => haystack.is_some(),
            Self::IsNone => haystack.is_none(),
            Self::IsSomeAnd(matcher) => haystack.is_some() && matcher.check(haystack, task_state)?,
            Self::IsNoneOr(matcher) => haystack.is_none() || matcher.check(haystack, task_state)?,

            // Glue

            #[cfg(feature = "regex")] Self::Regex(regex) => regex.get()?.is_match(haystack.ok_or(StringMatcherError::StringIsNone)?),

            // Common/Custom

            Self::Common(common_call) => {
                task_state.commons.string_matchers.get(get_str!(common_call.name, task_state, StringSourceError)).ok_or(StringMatcherError::CommonStringMatcherNotFound)?.check(
                    haystack,
                    &TaskStateView {
                        common_args: Some(&common_call.args.build(task_state)?),
                        url        : task_state.url,
                        scratchpad : task_state.scratchpad,
                        context    : task_state.context,
                        job_context: task_state.job_context,
                        params     : task_state.params,
                        commons    : task_state.commons,
                        #[cfg(feature = "cache")]
                        cache      : task_state.cache,
                        unthreader : task_state.unthreader
                    }
                )?
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(StringMatcherError::NotInCommonContext)?.string_matchers.get(get_str!(name, task_state, StringMatcherError)).ok_or(StringMatcherError::CommonCallArgStringMatcherNotFound)?.check(haystack, task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(haystack, task_state)?,
        })
    }
}

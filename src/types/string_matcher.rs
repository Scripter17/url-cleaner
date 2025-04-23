//! Rules for matching a string.

use std::collections::HashSet;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashMap;

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
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] fails.
        r#else: Box<Self>
    },
    /// If the call to [`Self::satisfied_by`] passes or fails, invert it into failing or passing.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    Any(Vec<Self>),

    /// If the call to [`Self::satisfied_by`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::satisfied_by`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::satisfied_by`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::satisfied_by`] return errors, both errors are returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try'] returns an error.
        r#else: Box<Self>
    },
    /// Checks the contained [`Self`]s in order, stopping as soon as a call to [`Self::satisfied_by`] doesn't return an error.
    /// # Errors
    /// If all calls to [`Self::satisfied_by`] return errors, the last error is returned. In the future this should be changed to return all errors.
    FirstNotError(Vec<Self>),

    /// Passes if the string contains [`Self::Contains::value`] at [`Self::Contains::where`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    Contains {
        /// The value to look for at [`Self::Contains::where`].
        value: StringSource,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#where: StringLocation
    },
    /// Effectively [`Self::Contains`] for each value in [`Self::ContainsAny::values`].
    /// # Errors
    /// If any call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF any call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If any call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    ContainsAny {
        /// The value to look for at [`Self::Contains::where`].
        values: Vec<StringSource>,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#where: StringLocation
    },
    /// Effectively [`Self::ContainsAny`] for each value in the [`Params::lists`]s specified by [`Self::ContainsAnyInList::list`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If no list with the specified name is found, returns the error [`StringMatcherError::ListNotFound`].
    ///
    /// If any call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    ContainsAnyInList {
        /// The name of the [`Params::lists`] whose values to look for.
        list: StringSource,
        /// Where to look for [`Self::Contains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#where: StringLocation
    },
    /// Passes if the string is equal to the value of the specified [`StringSource`].
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns [`false`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Is(StringSource),
    /// Passes if the string is in the specified [`HashSet`].
    IsOneOf(HashSet<String>),
    /// Passes if the string is in the specified [`Params::sets`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    InSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Passes if the call to [`Regex::is_match`] returns [`true`].
    /// # Errors
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    #[cfg(feature = "regex")]
    Regex(RegexWrapper),
    /// Passes if the call to [`GlobWrapper::matches`] returns [`true`].
    #[cfg(feature = "glob")]
    Glob(GlobWrapper),
    /// Applies [`Self::Modified::modification`] to a copy of the string, leaving the original unchanged, and returns the satisfaction of [`Self::Modified::matcher`] on that string.
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    Modified {
        /// The [`StringModification`] to apply to the copy of the string.
        modification: StringModification,
        /// The [`Self`] to match the modified string with.
        matcher: Box<Self>
    },
    /// Passes if calling [`str::trim_start_matches`] with the specified [`char`]s returns an empty string.
    OnlyTheseChars(Vec<char>),
    /// Passes if all [`char`]s in the string satisfies the specified [`CharMatcher`].
    /// # Errors
    /// If any call to [`CharMatcher::satisfied_by`] returns an error, that error is returned.
    AllCharsMatch(CharMatcher),
    /// Passes if any [`char`]s in the string satisfies the specified [`CharMatcher`].
    /// # Errors
    /// If any call to [`CharMatcher::satisfied_by`] returns an error, that error is returned.
    AnyCharMatches(CharMatcher),
    /// Passes if [`str::is_ascii`] returns [`true`].
    IsAscii,
    /// Splits the string with [`Self::NthSegmentMatches::split`], gets the [`Self::NthSegmentMatches::n`]th segment, and returns the satisfaction of [`Self::NthSegmentMatches::matcher`] of it.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If the segment isn't found, returns the error [`StringMatcherError::SegmentNotFound`].
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
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
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    AnySegmentMatches {
        /// The value to split the string with.
        split: StringSource,
        /// The [`Self`] to match the segments with.
        matcher: Box<Self>
    },
    /// Passes if the length of the string is the specified value.
    LengthIs(usize),
    /// Splits the string with [`Self::SegmentsStartWith::split`] and passes if the list of segments starts with the list of segments from splitting [`Self::SegmentsStartWith::value`] with [`Self::SegmentsStartWith::split`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF either call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    SegmentsStartWith {
        /// The value to split the strings with.
        split: Box<StringSource>,
        /// The value to get the subsegments from.
        value: Box<StringSource>
    },
    /// Splits the string with [`Self::SegmentsEndWith::split`] and passes if the list of segments ends with the list of segments from splitting [`Self::SegmentsEndWith::value`] with [`Self::SegmentsEndWith::split`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// IF either call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    SegmentsEndWith {
        /// The value to split the strings with.
        split: Box<StringSource>,
        /// The value to get the subsegments from.
        value: Box<StringSource>
    },
    /// Gets a [`Self`] from [`Config::commons`]'s [`Commons::string_modifications`] and applies it.
    /// # Errors
    /// If [`CommonCall::name`]'s call to [`StringSource::get`] returns an error, returns the error [`StringMatcherError::StringSourceIsNone`].
    ///
    /// If no [`Self`] with the specified name is found, returns the error [`StringMatcherError::CommonStringMatcherNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    Common(CommonCall),
    /// Calls the contained function.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&str, &TaskStateView) -> Result<bool, StringMatcherError>)
}

#[cfg(feature = "regex")]
impl From<RegexWrapper> for StringMatcher {
    fn from(value: RegexWrapper) -> Self {
        Self::Regex(value)
    }
}

#[cfg(feature = "glob")]
impl From<GlobWrapper> for StringMatcher {
    fn from(value: GlobWrapper) -> Self {
        Self::Glob(value)
    }
}

/// The enum of errors [`StringMatcher::satisfied_by`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when a [`StringMatcher::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`::regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    /// Returned when both [`StringMatcher`]s in a [`StringMatcher::TryElse`] return errors.
    #[error("Both StringMatchers in a StringMatcher::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringMatcher::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`StringMatcher::TryElse::else`]. 
        else_error: Box<Self>
    },
    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound,
    /// The requested list wasn't found.
    #[error("The requested list wasn't found.")]
    ListNotFound,
    /// Returned when a [`CharMatcherError`] is encountered.
    #[error(transparent)]
    CharMatcherError(#[from] CharMatcherError),
    /// Returned when a [`StringMatcher`] with the specified name isn't found in the [`Commons::string_matchers`].
    #[error("A StringMatcher with the specified name wasn't found in the Commons::string_matchers.")]
    CommonStringMatcherNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// An arbitrary [`std::error::Error`] for use with [`StringMatcher::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl StringMatcher {
    /// Returns [`true`] if `haystack` satisfies `self`.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn satisfied_by(&self, haystack: &str, task_state: &TaskStateView) -> Result<bool, StringMatcherError> {
        debug!(StringMatcher::satisfied_by, self);
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(StringMatcherError::ExplicitError(msg.clone()))?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.satisfied_by(haystack, task_state);
                eprintln!("=== StringMatcher::Debug ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\ntask_state: {task_state:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(haystack, task_state)? {then} else {r#else}.satisfied_by(haystack, task_state)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.satisfied_by(haystack, task_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.satisfied_by(haystack, task_state)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.satisfied_by(haystack, task_state)?,

            // Error handling.

            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(haystack, task_state).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(haystack, task_state).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(haystack, task_state).or_else(|try_error| r#else.satisfied_by(haystack, task_state).map_err(|else_error| StringMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut result = Ok(false); // The initial value doesn't mean anything.
                for matcher in matchers {
                    result = matcher.satisfied_by(haystack, task_state);
                    if result.is_ok() {return result;}
                }
                result?
            },

            // Other.

            Self::IsOneOf(hash_set) => hash_set.contains(haystack),
            Self::Contains {r#where, value} => r#where.satisfied_by(haystack, get_str!(value, task_state, StringMatcherError))?,
            Self::ContainsAny {values, r#where} => {
                for value in values {
                    if r#where.satisfied_by(haystack, get_str!(value, task_state, StringModificationError))? {
                        return Ok(true)
                    }
                }
                false
            },
            // Cannot wait for [`Iterator::try_any`](https://github.com/rust-lang/rfcs/pull/3233)
            Self::ContainsAnyInList {r#where, list} => {
                for x in task_state.params.lists.get(get_str!(list, task_state, StringMatcherError)).ok_or(StringMatcherError::ListNotFound)? {
                    if r#where.satisfied_by(haystack, x)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Modified {modification, matcher} => matcher.satisfied_by(&{let mut temp=haystack.to_string(); modification.apply(&mut temp, task_state)?; temp}, task_state)?,
            #[cfg(feature = "regex")] Self::Regex(regex) => regex.get()?.is_match(haystack),
            #[cfg(feature = "glob" )] Self::Glob(glob) => glob.matches(haystack),
            Self::OnlyTheseChars(chars) => haystack.trim_start_matches(&**chars).is_empty(),
            Self::AllCharsMatch(matcher) => {
                for char in haystack.chars() {
                    if !matcher.satisfied_by(char)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::AnyCharMatches(matcher) => {
                for char in haystack.chars() {
                    if matcher.satisfied_by(char)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::IsAscii => haystack.is_ascii(),
            Self::NthSegmentMatches {n, split, matcher} => matcher.satisfied_by(neg_nth(haystack.split(get_str!(split, task_state, StringMatcherError)), *n).ok_or(StringMatcherError::SegmentNotFound)?, task_state)?,
            Self::AnySegmentMatches {split, matcher} => {
                for segment in haystack.split(get_str!(split, task_state, StringMatcherError)) {
                    if matcher.satisfied_by(segment, task_state)? {
                        return Ok(true);
                    }
                };
                return Ok(false);
            },
            Self::Is(source) => Some(haystack) == source.get(task_state)?.as_deref(),
            Self::InSet(name) => task_state.params.sets.get(get_str!(name, task_state, StringMatcherError)).is_some_and(|set| set.contains(haystack)),
            Self::LengthIs(x) => haystack.len() == *x,
            Self::SegmentsEndWith { split, value } => {
                let split = get_str!(split, task_state, StringMatcherError);
                // haystack.split(split).collect::<Vec<_>>().into_iter().rev().zip(get_str!(value, task_state, StringMatcherError).split(split)).all(|(x, y)| x==y)
                haystack.strip_suffix(get_str!(value, task_state, StringMatcherError))
                    .is_some_and(|x| x.split(split).last()==Some(""))
            },
            Self::SegmentsStartWith { split, value } => {
                let split = get_str!(split, task_state, StringMatcherError);
                haystack.strip_prefix(get_str!(value, task_state, StringMatcherError))
                    .is_some_and(|x| x.strip_prefix(split).is_some())
            },
            Self::Common(common_call) => {
                task_state.commons.string_matchers.get(get_str!(common_call.name, task_state, StringSourceError)).ok_or(StringMatcherError::CommonStringMatcherNotFound)?.satisfied_by(
                    haystack,
                    &TaskStateView {
                        url: task_state.url,
                        context: task_state.context,
                        params: task_state.params,
                        scratchpad: task_state.scratchpad,
                        #[cfg(feature = "cache")]
                        cache: task_state.cache,
                        commons: task_state.commons,
                        common_args: Some(&common_call.args.build(task_state)?),
                        job_context: task_state.job_context
                    }
                )?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(haystack, task_state)?,
        })
    }
}

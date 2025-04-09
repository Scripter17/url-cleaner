//! Rules for matching a string.

use std::collections::HashSet;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
pub enum StringMatcher {
    Always,
    Never,
    Error(String),
    #[suitable(never)]
    Debug(Box<Self>),

    If {
        r#if: Box<Self>,
        then: Box<Self>,
        r#else: Box<Self>
    },
    Not(Box<Self>),
    All(Vec<Self>),
    Any(Vec<Self>),

    TreatErrorAsPass(Box<Self>),
    TreatErrorAsFail(Box<Self>),
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    FirstNotError(Vec<Self>),

    Contains {
        value: StringSource,
        #[serde(default)]
        r#where: StringLocation
    },
    ContainsAny {
        values: Vec<StringSource>,
        r#where: StringLocation
    },
    ContainsAnyInList {
        r#where: StringLocation,
        list: StringSource
    },
    Is(StringSource),
    IsOneOf(HashSet<String>),
    InSet(#[suitable(assert = "set_is_documented")] StringSource),
    #[cfg(feature = "regex")]
    Regex(RegexWrapper),
    #[cfg(feature = "glob")]
    Glob(GlobWrapper),
    Modified {
        modification: StringModification,
        matcher: Box<Self>
    },
    OnlyTheseChars(Vec<char>),
    AllCharsMatch(CharMatcher),
    AnyCharMatches(CharMatcher),
    IsAscii,
    NthSegmentMatches {
        split: StringSource,
        n: isize,
        matcher: Box<Self>
    },
    AnySegmentMatches {
        split: StringSource,
        matcher: Box<Self>
    },
    LengthIs(usize),
    SegmentsStartWith {
        split: Box<StringSource>,
        value: Box<StringSource>
    },
    SegmentsEndWith {
        split: Box<StringSource>,
        value: Box<StringSource>
    },
    Common(CommonCall),
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    Custom(FnWrapper<fn(&str, &TaskStateView) -> Result<bool, StringMatcherError>>)
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
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    #[error("A `StringMatcher::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    },
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    #[error("The requested list was not found.")]
    ListNotFound,
    #[error(transparent)]
    CharMatcherError(#[from] CharMatcherError),
    #[error("The common StringMatcher was not found.")]
    CommonStringMatcherNotFound,
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl StringMatcher {
    /// Returns [`true`] if `haystack` satisifes `self`.
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
            Self::Is(source) => haystack == get_str!(source, task_state, StringMatcherError),
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

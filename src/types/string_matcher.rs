//! Provides [`StringMatcher`] which allows for testing if a [`str`] matches a certain rule.

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// A general API for matching [`str`]ings with a variety of methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringMatcher {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`StringMatcherError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringMatcherError::ExplicitError`].
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
    /// Passes if the included [`Self`] doesn't and vice-versa.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned.
    Not(Box<Self>),
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
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    FirstNotError(Vec<Self>),

    // Other.

    /// Uses a [`StringLocation`].
    /// # Errors
    /// If the call to [`StringLocation::satisfied_by`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(StringMatcher::Contains {r#where: StringLocation::Start, value: "utm_".into()}.satisfied_by("utm_abc", &job_state).unwrap(), true);
    /// ```
    Contains {
        /// The value to look for.
        value: StringSource,
        /// The location to check for `value` at.
        #[serde(default)]
        r#where: StringLocation
    },
    /// Passes if the string equals the specified value.
    Equals(StringSource),
    /// Passes if the provided string is contained in the specified [`HashSet`].
    IsOneOf(HashSet<String>),
    /// Passes if the string is in the specified [`Params::sets`] set.
    /// 
    /// See also: [`Self::IsOneOf`].
    InSet(StringSource),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::RegexParts;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(StringMatcher::Regex(RegexParts::new("a.c").unwrap().try_into().unwrap()).satisfied_by("axc", &job_state).unwrap(), true);
    /// ```
    #[cfg(feature = "regex")]
    Regex(RegexWrapper),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::GlobWrapper;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// assert_eq!(StringMatcher::Glob(GlobWrapper::from_str("a*c").unwrap()).satisfied_by("aabcc", &job_state).unwrap(), true);
    /// ```
    #[cfg(feature = "glob")]
    Glob(GlobWrapper),
    /// Modifies the provided string then matches it.
    Modified {
        /// The modification to apply.
        modification: StringModification,
        /// The matcher to test the modified string with.
        matcher: Box<Self>
    },
    /// Passes if the provided string only contains the specified [`char`]s.
    OnlyTheseChars(Vec<char>),
    /// Passes if the specified matcher passes for all characters in the haystack.
    /// # Errors
    /// If any call to [`CharMatcher::satisfied_by`] return an error, that error is returned.
    AllCharsMatch(CharMatcher),
    /// Passes if the specified matcher passes for any characters in the haystack.
    /// # Errors
    /// If any call to [`CharMatcher::satisfied_by`] return an error, that error is returned.
    AnyCharMatches(CharMatcher),
    /// [`str::is_ascii`].
    IsAscii,
    /// Passes if the `n`th segment of the string passes specified matcher.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the specified segment isn't found, returns the error [`StringMatcherError::SegmentNotFound`].
    /// 
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    NthSegmentMatches {
        /// The split used to segment the string.
        split: StringSource,
        /// The index of the segment to match.
        n: isize,
        /// The matcher to test the segment with.
        matcher: Box<Self>
    },
    /// Passes if any segment passes the specified matcher.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    AnySegmentMatches {
        /// The split used to segment the string.
        split: StringSource,
        /// The matcher to test each segment with.
        matcher: Box<Self>
    },
    /// Passes if the specified [`StringLocation`] is satisfied by any of the strings in [`Self::ContainsAnyInList::list`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    /// 
    /// If the call to [`HashMap::get`] to get the list from [`Params::lists`] returns [`None`] returns the error [`StringMatcherError::ListNotFound`].
    /// 
    /// If any of the calls to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    ContainsAnyInList {
        /// The location in `haystack` to look at.
        r#where: StringLocation,
        /// The name of the list of strings to look for.
        list: StringSource
    },
    /// Passes if the provided string's length is the specified value.
    LengthIs(usize),
    /// Like [`StringLocation::Start`] but works based on segments instead of characters.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// let matcher = StringMatcher::SegmentsStartWith {
    ///     split: Box::new("--".into()),
    ///     value: Box::new("abc--def".into())
    /// };
    /// assert_eq!(matcher.satisfied_by("abc--def--ghi"  , &job_state).unwrap(), true );
    /// assert_eq!(matcher.satisfied_by("abc--def----ghi", &job_state).unwrap(), true );
    /// assert_eq!(matcher.satisfied_by("abc--deff--ghi" , &job_state).unwrap(), false);
    /// ```
    SegmentsStartWith {
        /// The value to segment the haystack by.
        split: Box<StringSource>,
        /// The string of segments to search for in the haystack.
        value: Box<StringSource>
    },
    /// Like [`StringLocation::End`] but works based on segments instead of characters.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringMatcherError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let context = Default::default();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// let matcher = StringMatcher::SegmentsEndWith {
    ///     split: Box::new("--".into()),
    ///     value: Box::new("def--ghi".into())
    /// };
    /// assert_eq!(matcher.satisfied_by("abc--def--ghi"  , &job_state).unwrap(), true );
    /// assert_eq!(matcher.satisfied_by("abc----def--ghi", &job_state).unwrap(), true );
    /// assert_eq!(matcher.satisfied_by("abc--ddef--ghi" , &job_state).unwrap(), false);
    /// ```
    SegmentsEndWith {
        /// The value to segment the haystack by.
        split: Box<StringSource>,
        /// The string of segments to search for in the haystack.
        value: Box<StringSource>
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::string_matchers`].
    /// 
    /// Currently does not pass-in [`JobState::vars`] or preserve updates. This will eventually be changed.
    Common {
        /// The name of the [`Self`] to use.
        name: StringSource,
        /// The [`JobState::common_vars`] to pass.
        #[serde(default, skip_serializing_if = "is_default")]
        vars: HashMap<String, StringSource>
    }
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

/// The enum of all possible errors [`StringMatcher::satisfied_by`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringMatcherError {
    /// Returned when [`StringMatcher::Error`] is used.
    #[error("StringMatcher::Error was used.")]
    ExplicitError,
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Returned when both the `try` and `else` of a [`StringMatcher::TryElse`] both return errors.
    #[error("A `StringMatcher::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`StringMatcher::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`StringMatcher::TryElse::else`],
        else_error: Box<Self>
    },
    /// Returned when the requested segment is not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// Returned when the requested list is not found.
    #[error("The requested list was not found.")]
    ListNotFound,
    /// Returned when a [`CharMatcherError`] is encountered.
    #[error(transparent)]
    CharMatcherError(#[from] CharMatcherError),
    /// Returned when the common [`StringMatcher`] is not found.
    #[error("The common StringMatcher was not found.")]
    CommonStringMatcherNotFound,
}

impl StringMatcher {
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, haystack: &str, job_state: &JobState) -> Result<bool, StringMatcherError> {
        debug!(StringMatcher::satisfied_by, self);
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(StringMatcherError::ExplicitError)?,
            Self::Debug(matcher) => {
                let is_satisfied=matcher.satisfied_by(haystack, job_state);
                eprintln!("=== StringMatcher::Debug ===\nMatcher: {matcher:?}\nHaystack: {haystack:?}\nJob state: {job_state:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(haystack, job_state)? {then} else {r#else}.satisfied_by(haystack, job_state)?,
            Self::All(matchers) => {
                for matcher in matchers {
                    if !matcher.satisfied_by(haystack, job_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(matchers) => {
                for matcher in matchers {
                    if matcher.satisfied_by(haystack, job_state)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(matcher) => !matcher.satisfied_by(haystack, job_state)?,

            // Error handling.

            Self::TreatErrorAsPass(matcher) => matcher.satisfied_by(haystack, job_state).unwrap_or(true),
            Self::TreatErrorAsFail(matcher) => matcher.satisfied_by(haystack, job_state).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(haystack, job_state).or_else(|try_error| r#else.satisfied_by(haystack, job_state).map_err(|else_error| StringMatcherError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(matchers) => {
                let mut result = Ok(false); // The initial value doesn't mean anything.
                for matcher in matchers {
                    result = matcher.satisfied_by(haystack, job_state);
                    if result.is_ok() {return result;}
                }
                result?
            },

            // Other.

            Self::IsOneOf(hash_set) => hash_set.contains(haystack),
            Self::Contains {r#where, value} => r#where.satisfied_by(haystack, get_str!(value, job_state, StringMatcherError))?,
            Self::Modified {modification, matcher} => matcher.satisfied_by(&{let mut temp=haystack.to_string(); modification.apply(&mut temp, job_state)?; temp}, job_state)?,
            #[cfg(feature = "regex")] Self::Regex(regex) => regex.get_regex()?.is_match(haystack),
            #[cfg(feature = "glob" )] Self::Glob(glob) => glob.matches(haystack),
            Self::OnlyTheseChars(chars) => haystack.trim_start_matches(&**chars)=="",
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
            Self::NthSegmentMatches {n, split, matcher} => matcher.satisfied_by(neg_nth(haystack.split(get_str!(split, job_state, StringMatcherError)), *n).ok_or(StringMatcherError::SegmentNotFound)?, job_state)?,
            Self::AnySegmentMatches {split, matcher} => {
                for segment in haystack.split(get_str!(split, job_state, StringMatcherError)) {
                    if matcher.satisfied_by(segment, job_state)? {
                        return Ok(true);
                    }
                };
                return Ok(false);
            },
            Self::Equals(source) => haystack == get_str!(source, job_state, StringMatcherError),
            Self::InSet(name) => job_state.params.sets.get(get_str!(name, job_state, StringMatcherError)).is_some_and(|set| set.contains(haystack)),
            // Cannot wait for [`Iterator::try_any`] (https://github.com/rust-lang/rfcs/pull/3233)
            Self::ContainsAnyInList {r#where, list} => {
                for x in job_state.params.lists.get(get_str!(list, job_state, StringMatcherError)).ok_or(StringMatcherError::ListNotFound)? {
                    if r#where.satisfied_by(haystack, x)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::LengthIs(x) => haystack.len() == *x,
            Self::SegmentsEndWith { split, value } => {
                let split = get_str!(split, job_state, StringMatcherError);
                // haystack.split(split).collect::<Vec<_>>().into_iter().rev().zip(get_str!(value, job_state, StringMatcherError).split(split)).all(|(x, y)| x==y)
                haystack.strip_suffix(get_str!(value, job_state, StringMatcherError))
                    .is_some_and(|x| x.split(split).last()==Some(""))
            },
            Self::SegmentsStartWith { split, value } => {
                let split = get_str!(split, job_state, StringMatcherError);
                haystack.strip_prefix(get_str!(value, job_state, StringMatcherError))
                    .is_some_and(|x| x.strip_prefix(split).is_some())
            },
            Self::Common {name, vars} => {
                let common_vars = vars.iter().map(|(k, v)| Ok::<_, StringMatcherError>((k.clone(), get_string!(v, job_state, StringMatcherError)))).collect::<Result<HashMap<_, _>, _>>()?;
                let mut temp_url = job_state.url.clone();
                job_state.commons.string_matchers.get(get_str!(name, job_state, StringSourceError)).ok_or(StringMatcherError::CommonStringMatcherNotFound)?.satisfied_by(
                    haystack,
                    &JobState {
                        url: &mut temp_url,
                        context: job_state.context,
                        params: job_state.params,
                        vars: Default::default(),
                        #[cfg(feature = "cache")]
                        cache_handler: job_state.cache_handler,
                        commons: job_state.commons,
                        common_vars: Some(&common_vars)
                    }
                )?
            }
        })
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used, reason = "Private API, but they should be replaced by [`Option::is_none_or`] in 1.82.")]
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        match self {
            Self::If {r#if, then, r#else} => r#if.is_suitable_for_release() && then.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::Not(matcher) => matcher.is_suitable_for_release(),
            Self::All(matchers) => matchers.iter().all(|matcher| matcher.is_suitable_for_release()),
            Self::Any(matchers) => matchers.iter().all(|matcher| matcher.is_suitable_for_release()),
            Self::TreatErrorAsPass(matcher) => matcher.is_suitable_for_release(),
            Self::TreatErrorAsFail(matcher) => matcher.is_suitable_for_release(),
            Self::TryElse {r#try, r#else} => r#try.is_suitable_for_release() && r#else.is_suitable_for_release(),
            Self::FirstNotError(matchers) => matchers.iter().all(|matcher| matcher.is_suitable_for_release()),
            Self::Contains {value, r#where} => value.is_suitable_for_release() && r#where.is_suitable_for_release(),
            Self::Equals(value) => value.is_suitable_for_release(),
            Self::InSet(name) => name.is_suitable_for_release(),
            Self::Modified {modification, matcher} => modification.is_suitable_for_release() && matcher.is_suitable_for_release(),
            Self::AllCharsMatch(matcher) => matcher.is_suitable_for_release(),
            Self::AnyCharMatches(matcher) => matcher.is_suitable_for_release(),
            Self::NthSegmentMatches {split, matcher, ..} => split.is_suitable_for_release() && matcher.is_suitable_for_release(),
            Self::AnySegmentMatches {split, matcher} => split.is_suitable_for_release() && matcher.is_suitable_for_release(),
            Self::ContainsAnyInList {list, r#where} => list.is_suitable_for_release() && r#where.is_suitable_for_release(),
            Self::SegmentsStartWith {split, value} => split.is_suitable_for_release() && value.is_suitable_for_release(),
            Self::SegmentsEndWith {split, value} => split.is_suitable_for_release() && value.is_suitable_for_release(),
            Self::Debug(_) => false,
            #[cfg(feature = "regex")] Self::Regex(_) => true,
            #[cfg(feature = "glob")] Self::Glob(_) => true,
            Self::Always | Self::Never | Self::Error | Self::IsOneOf(_) | Self::OnlyTheseChars(_) | Self::IsAscii | Self::LengthIs(_) => true,
            Self::Common {name, vars} => name.is_suitable_for_release() && vars.iter().all(|(_, v)| v.is_suitable_for_release())
        }
    }
}

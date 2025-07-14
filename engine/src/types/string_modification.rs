//! Rules for modifying strings.

use std::borrow::Cow;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use ::percent_encoding::{percent_decode_str, utf8_percent_encode};
#[expect(unused_imports, reason = "Used in a doc comment.")]
#[cfg(feature = "regex")]
use ::regex::Regex;
#[cfg(feature = "base64")]
use ::base64::prelude::*;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Modify a string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub enum StringModification {
    /// Doesn't do any modification.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::None.apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("abc".into()));
    /// ```
    None,



    /// Print debug info about the contained [`Self`] and its call to [`Self::apply`].
    ///
    /// The exact info printed is unspecified and subject to change at any time for any reason.
    /// # Suitability
    /// Always unsuiable to be in the default config.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),



    /// Always returns the error [`StringModificationError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringModificationError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::Error("...".into()).apply(&mut to, &task_state_view).unwrap_err();
    ///
    /// assert_eq!(to, Some("abc".into()));
    /// ```
    Error(String),
    /// If the contained [`Self`] returns an error, ignore it.
    ///
    /// Does not revert any successful calls to [`Self::apply`]. For that, also use [`Self::RevertOnError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::IgnoreError(Box::new(StringModification::Error("...".into()))).apply(&mut to, &task_state_view).unwrap();
    /// ```
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the string to its previous value then return the error.
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::RevertOnError(Box::new(StringModification::All(vec![
    ///     StringModification::Set("def".into()),
    ///     StringModification::Error("...".into())
    /// ]))).apply(&mut to, &task_state_view).unwrap_err();
    ///
    /// assert_eq!(to, Some("abc".into()));
    /// ```
    RevertOnError(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::apply`] returns an error, apply [`Self::TryElse::else`].
    ///
    /// Does not revert on error. For that, see [`Self::RevertOnError`].
    /// # Errors
    #[doc = edoc!(applyerrte(Self, StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::TryElse {
    ///     r#try : Box::new(StringModification::Error("...".into())),
    ///     r#else: Box::new(StringModification::Set("def".into()))
    /// }.apply(&mut to, &task_state_view);
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },
    /// Applies the contained [`Self`]s in order.
    ///
    /// Does not revert on error. For that, see [`Self::RevertOnError`].
    /// # Errors
    #[doc = edoc!(applyerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::All(vec![
    ///     StringModification::Append("def".into()),
    ///     StringModification::Error("...".into())
    /// ]).apply(&mut to, &task_state_view).unwrap_err();
    ///
    /// assert_eq!(to, Some("abcdef".into()));
    /// ```
    All(Vec<Self>),
    /// Calls [`Self::apply`] on each contained [`Self`] in order, stopping once one returns [`Ok`].
    ///
    /// Does not revert on error. For that, see [`Self::RevertOnError`].
    /// # Errors
    #[doc = edoc!(applyerrfne(Self, StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::FirstNotError(vec![
    ///     StringModification::Append("def".into()),
    ///     StringModification::Error("...".into())
    /// ]).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("abcdef".into()));
    /// ```
    FirstNotError(Vec<Self>),



    /// Only apply the [`Self`] if the string is [`Some`].
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    IfSome(Box<Self>),
    /// If the string satisfies [`Self::IfMatches::matcher`], apply [`Self::IfMatches::then`].
    ///
    /// If the string does not satisfy [`Self::IfMatches::matcher`] and [`Self::IfMatches::else`] is [`Some`], apply [`Self::IfMatches::else`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher), applyerr(Self))]
    IfMatches {
        /// The [`StringMatcher`] to check if the string satisfies.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] is satisfied.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] isn't satisfied.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
    },
    /// If the string containes [`Self::IfContains::value`] at [`Self::IfContains::at`], apply [`Self::IfContains::then`], otherwise apply [`Self::IfContains::else`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModificationError), checkerr(StringLocation), applyerr(Self))]
    IfContains {
        /// The [`StringSource`] to look for in the string.
        value: StringSource,
        /// The [`StringLocation`] to look for [`Self::IfContains::value`] at.
        at: StringLocation,
        /// The [`Self`] to apply if [`Self::IfContains::value`] is found at [`Self::IfContains::at`].
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::IfContains::value`] is not found at [`Self::IfContains::at`].
        r#else: Option<Box<Self>>
    },
    /// If the string containes any value in [`Self::IfContainsAny::values`] at [`Self::IfContains::at`], apply [`Self::IfContains::then`], otherwise apply [`Self::IfContains::else`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 3), getnone(StringSource, StringModificationError, 3), checkerr(StringLocation, 3), applyerr(Self, 3))]
    IfContainsAny {
        /// The [`StringSource`]s to look for in the string.
        values: Vec<StringSource>,
        /// The [`StringLocation`] to look for [`Self::IfContainsAny::values`] at.
        at: StringLocation,
        /// The [`Self`] to apply if any value in [`Self::IfContainsAny::values`] is found at [`Self::IfContains::at`].
        then: Box<Self>,
        /// The [`Self`] to apply if no values in [`Self::IfContainsAny::values`] are found at [`Self::IfContains::at`].
        r#else: Option<Box<Self>>
    },
    /// Indexes [`Self::Map::map`] with [`Self::Map::value`] and, if a [`Self`] is found, applies it.
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), applyerr(Self))]
    Map {
        /// The value to index [`Self::Map::map`] with.
        value: Box<StringSource>,
        /// The [`Map`] to index with [`Self::Map::value`].
        #[serde(flatten)]
        map: Map<Self>,
    },



    /// Sets the string to the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    Set(StringSource),
    /// Appends the specified value.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    Append(StringSource),
    /// Prepends the specified value.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    Prepend(StringSource),
    /// Insert [`Self::Insert::value`] at [`Self::Insert::index`].
    ///
    /// See [`String::insert_str`] for details.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If [`Self::Insert::index`] isn't a [`str::is_char_boundary`], returns the error [`StringModificationError::InvalidIndex`].
    Insert {
        /// The value to insert at [`Self::Insert::index`].
        value: StringSource,
        /// The index to insert [`Self::Insert::value`] at.
        index: isize
    },
    /// Sets the string to lowercase.
    ///
    /// See [`str::to_lowercase`] for details.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    Lowercase,
    /// Sets the string to uppercase.
    ///
    /// See [`str::to_uppercase`] for details.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    Uppercase,



    /// Removes the specified prefix.
    ///
    /// If you want to not return an error when the string doesn't start with the prefix, see [`Self::StripMaybePrefix`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the string doesn't start with the specified prefix, returns the error [`StringModificationError::PrefixNotFound`].
    StripPrefix(StringSource),
    /// If the string starts with the specified value, remove it.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    StripMaybePrefix(StringSource),
    /// Removes the specified suffix.
    ///
    /// If you want to not return an error when the string doesn't start with the suffix, see [`Self::StripMaybeSuffix`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the string doesn't start with the specified suffix, returns the error [`StringModificationError::SuffixNotFound`].
    StripSuffix(StringSource),
    /// If the string ends with the specified value, remove it.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    StripMaybeSuffix(StringSource),
    /// Removes the [`char`] at the specified index.
    ///
    /// See [`String::remove`] for details.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    ///
    /// If the specified index isn't a [`str::is_char_boundary`], returns the error [`StringModificationError::InvalidIndex`].
    RemoveChar(isize),



    /// Finds the first instance of the specified substring and keeps only everything before it.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::KeepBefore("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("a".into()));
    /// ```
    KeepBefore(StringSource),
    /// Finds the first instance of the specified substring and removes everything before it.
    ///
    /// Useful in combination with [`Self::GetJsStringLiteralPrefix`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::StripBefore("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("bc".into()));
    /// ```
    StripBefore(StringSource),
    /// Effectively [`Self::KeepAfter`] with [`Self::KeepBetween::start`] and [`Self::KeepBefore`] with [`Self::KeepBetween::end`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 2), getnone(StringSource, StringModification, 2))]
    ///
    /// If the value of [`Self::KeepBetween::start`] isn't found in the string, returns the error [`StringModificationError::KeepBetweenStartNotFound`].
    ///
    /// If the value of [`Self::KeepBetween::end`] isn't found in the string after [`Self::KeepBetween::start`], returns the error [`StringModificationError::KeepBetweenEndNotFound`].
    KeepBetween {
        /// The value to [`Self::KeepAfter`].
        start: StringSource,
        /// The value to [`Self::KeepBefore`].
        end: StringSource
    },
    /// Finds the first instance of the specified substring and removes everything after it.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::StripAfter("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("ab".into()));
    /// ```
    StripAfter(StringSource),
    /// Finds the first instance of the specified substring and keeps only everything after it.
    ///
    /// Useful in combination with [`Self::GetHtmlAttribute`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::KeepAfter("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("c".into()));
    /// ```
    KeepAfter(StringSource),



    /// [`Self::KeepAfter`] but does nothing if the substring isn't found.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModificationError))]
    KeepMaybeBefore(StringSource),
    /// [`Self::StripBefore`] but does nothing if the substring isn't found.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModificationError))]
    StripMaybeBefore(StringSource),
    /// [`Self::KeepBetween`] but with [`Self::KeepAfter`] [`Self::KeepBefore`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 2), getnone(StringSource, StringModification, 2))]
    KeepMaybeBetween {
        /// The value to [`Self::KeepMaybeAfter`].
        start: StringSource,
        /// The value to [`Self::KeepMaybeBefore`].
        end: StringSource
    },
    /// [`Self::StripAfter`] but does nothing if the substring isn't found.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModificationError))]
    StripMaybeAfter(StringSource),
    /// [`Self::KeepAfter`] but does nothing if the substring isn't found.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModificationError))]
    KeepMaybeAfter(StringSource),



    /// Replace up to [`Self::Replacen::count`] instances of [`Self::Replacen::find`] with [`Self::Replacen::replace`].
    ///
    /// See [`str::replacen`] for details.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    Replacen {
        /// The value to replace with [`Self::Replacen::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::Replacen::find`] with.
        replace: StringSource,
        /// The maximum amount of instances to replace.
        count: usize
    },
    /// Replace all instances of [`Self::ReplaceAll::find`] with [`Self::ReplaceAll::replace`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 2), getnone(StringSource, StringModification, 2))]
    ReplaceAll {
        /// The value to replace with [`Self::ReplaceAll::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::ReplaceAll::find`] with.
        replace: StringSource
    },



    /// Replace the specified range with [`Self::ReplaceRange::replace`].
    /// # Errors
    /// If either [`Self::ReplaceRange::start`] and/or [`Self::ReplaceRange::end`] isn't a [`str::is_char_boundary`], returns the error [`StringModificationError::InvalidSlice`].
    ///
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ReplaceRange {
        /// The start of the range to replace.
        start: isize,
        /// The end of the range to replace.
        end: Option<isize>,
        /// The value to replace the range with.
        replace: StringSource
    },
    /// Removes everything outside the specified range.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    ///
    /// If either [`Self::KeepRange::start`] and/or [`Self::KeepRange::end`] isn't a [`str::is_char_boundary`], returns the error [`StringModificationError::InvalidSlice`].
    KeepRange {
        /// The start of the range to keep.
        start: isize,
        /// The end of the range to keep.
        end: Option<isize>
    },



    /// Split the string with [`Self::SetSegment::split`] and set the [`Self::SetSegment::index`] segment to [`Self::SetSegment::value`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 2))]
    ///
    /// If the segment isn't found, returns the error [`StringModificationError::SegmentNotFound`].
    SetSegment {
        /// The value to split the string with.
        split: StringSource,
        /// The index of the segment to set.
        index: isize,
        /// The value to set the segment to.
        value: StringSource
    },
    /// Split the string with [`Self::InsertSegment::split`] and inserts [`Self::InsertSegment::value`] before the [`Self::InsertSegment::index`] segment.
    ///
    /// If [`Self::InsertSegment::index`] is equal to the amount of segments, appends the new segment at the end.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource, 2))]
    ///
    /// If the segment boundary isn't found, returns the error [`StringModificationError::SegmentNotFound`].
    InsertSegment {
        /// The value to split the string with.
        split: StringSource,
        /// The location to insert the value at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// Split the string with [`Self::KeepNthSegment::split`] and keep only the [`Self::KeepNthSegment::index`] segment.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the segment isn't found, returns the error [`StringModificationError::SegmentNotFound`].
    KeepNthSegment {
        /// The value to split the string on.
        split: StringSource,
        /// The index of the segment to keep.
        index: isize
    },
    /// Split the string with [`Self::KeepSegmentRange`] and keep only the segments between [`Self::KeepSegmentRange::start`] (inclusive) and [`Self::KeepSegmentRange::end`] (exclusive).
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification))]
    ///
    /// If the range isn't found, returns the error [`StringModificationError::SegmentRangeNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state_view);
    ///
    /// let mut to = Some("a/b/c/d/e".into());
    ///
    /// StringModification::KeepSegmentRange {
    ///     split: "/".into(),
    ///     start: 1,
    ///     end: Some(4)
    /// }.apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("b/c/d".into()));
    ///
    ///
    /// StringModification::KeepSegmentRange {
    ///     split: "/".into(),
    ///     start: 0,
    ///     end: Some(-1)
    /// }.apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("b/c".into()));
    /// ```
    KeepSegmentRange {
        /// The value to split the string with.
        split: StringSource,
        /// The index of the first segment to keep.
        ///
        /// Defaults to `0`.
        #[serde(default, skip_serializing_if = "is_default")]
        start: isize,
        /// The index of the last segment to keep.
        ///
        /// Set to [`None`] to keep all segments after [`Self::KeepSegmentRange::start`].
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        end: Option<isize>
    },



    /// Parses the javascript string literal at the start of the string and returns its value.
    ///
    /// Useful in combination with [`Self::KeepAfter`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), callerr(parse::js::string_literal_prefix))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// let mut to = Some(r#"let destination = "https:\/\/example.com";"#.into());
    ///
    /// StringModification::All(vec![
    ///     StringModification::KeepAfter("let destination = ".into()),
    ///     StringModification::GetJsStringLiteralPrefix
    /// ]).apply(&mut to, &task_state).unwrap();;
    ///
    /// assert_eq!(to, Some("https://example.com".into()));
    /// ```
    GetJsStringLiteralPrefix,
    /// Processes HTML character references/escape codes like `&map;` into `&` and `&41;` into `A`.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), callerr(parse::html::unescape_text))]
    UnescapeHtmlText,
    /// Parses the HTML element at the start of the string and returns the [`Self::UnescapeHtmlText`]ed value of the last attribute with the specified name.
    ///
    /// Useful in combination with [`Self::StripBefore`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), callerr(parse::html::get_attribute_value))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// let mut to = Some(r#"Redirecting you to <a id="destination" href="https://example.com?a=2&amp;b=3">example.com</a>..."#.into());
    ///
    /// StringModification::All(vec![
    ///     StringModification::StripBefore(r#"<a id="destination""#.into()),
    ///     StringModification::GetHtmlAttribute("href".into())
    /// ]).apply(&mut to, &task_state).unwrap();
    ///
    /// assert_eq!(to, Some("https://example.com?a=2&b=3".into()));
    /// ```
    GetHtmlAttribute(StringSource),



    /// Calls [`Regex::find`] and returns its value.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(RegexWrapper), callnone(Regex::find, StringModificationError::RegexMatchNotFound))]
    #[cfg(feature = "regex")]
    RegexFind(RegexWrapper),
    /// Calls [`::regex::Regex::captures`] and returns the result of [`::regex::Captures::expand`]ing with [`Self::RegexSubstitute::replace`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification), callnone(Regex::captures, StringModificationError::RegexMatchNotFound))]
    #[cfg(feature = "regex")]
    RegexSubstitute {
        /// The [`RegexWrapper`] to capture with.
        regex: RegexWrapper,
        /// The format string to pass to [`::regex::Captures::expand`].
        replace: StringSource
    },
    /// [`::regex::Captures::expand`]s each [`::regex::Regex::captures_iter`] with [`Self::JoinAllRegexSubstitutions::replace`] and join them with [`Self::JoinAllRegexSubstitutions::join`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification), geterr(RegexWrapper))]
    #[cfg(feature = "regex")]
    JoinAllRegexSubstitutions {
        /// The [`RegexWrapper`] to capture with.
        regex: RegexWrapper,
        /// The format string to pass to [`::regex::Captures::expand`].
        replace: StringSource,
        /// The [`StringSource`] to join the expanded captures with.
        join: StringSource
    },
    /// [`Regex::replace`]s the first match of [`Self::RegexReplaceOne::regex`] with [`Self::RegexReplaceOne::replace`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification), geterr(RegexWrapper))]
    #[cfg(feature = "regex")]
    RegexReplaceOne {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The format string to expand the capture with.
        replace: StringSource
    },
    /// [`Regex::replace`]s the all matches of [`Self::RegexReplaceAll::regex`] with [`Self::RegexReplaceAll::replace`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification), geterr(RegexWrapper))]
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The format string to expand the captures with.
        replace: StringSource
    },
    /// [`Regex::replacen`]s the first [`Self::RegexReplacen::n`] of [`Self::RegexReplacen::regex`] with [`Self::RegexReplacen::replace`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), geterr(StringSource), getnone(StringSource, StringModification), geterr(RegexWrapper))]
    #[cfg(feature = "regex")]
    RegexReplacen {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The number of captures to find and replace.
        n: usize,
        /// The format string to expand the captures with.
        replace: StringSource
    },



    /// Parses the string as JSON and uses [`serde_json::Value::pointer_mut`] with the specified pointer.
    ///
    /// When extracting values from javascript, it's often faster to find the start of the desired string and use [`Self::GetJsStringLiteralPrefix`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), callerr(serde_json::from_str), geterr(StringSource), getnone(StringSource, StringModification), callnone(serde_json::Value::pointer_mut, StringModificationError::JsonValueNotFound))]
    ///
    /// If the call to [`serde_json::Value::pointer_mut`] doesn't return a [`serde_json::Value::String`], returns the error [`StringModificationError::JsonPointeeIsNotAString`].
    JsonPointer(StringSource),



    /// Percent encodes the string.
    ///
    /// Please note that this can be deserialized from `"PercentDecode"`, in which case the contained [`PercentEncodeAlphabet`] is defaulted.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""PercentEncode""#).unwrap(), StringModification::PercentEncode(Default::default()));
    /// ```
    PercentEncode(#[serde(default, skip_serializing_if = "is_default")] PercentEncodeAlphabet),
    /// Percent decodes the string.
    ///
    /// Unfortunately doesn't allow specifying a [`PercentEncodeAlphabet`] to keep certain values encoded due to limitations with the [`::percent_encoding`] API.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), callerr(::percent_encoding::PercentDecode::decode_utf8))]
    PercentDecode,
    /// [`Self::PercentDecode`] but replaces non-UTF-8 percent encoded byte equences with U+FFFD (�), the replacement character.
    ///
    /// Unfortunately doesn't allow specifying a [`PercentEncodeAlphabet`] to keep certain values encoded due to limitations with the [`::percent_encoding`] API.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    LossyPercentDecode,



    /// Base64 encodes the string.
    ///
    /// Please note that this can be deserialized from `"Base64Encode"`, in which case the contained [`Base64Config`] is defaulted.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""Base64Encode""#).unwrap(), StringModification::Base64Encode(Default::default()));
    /// ```
    #[cfg(feature = "base64")]
    Base64Encode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),
    /// Base64 decodes the string.
    ///
    /// Please note that this can be deserialized from `"Base64Decode"`, in which case the contained [`Base64Config`] is defaulted.
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), callerr(::base64::engine::GeneralPurpose::decode), callerr(String::from_utf8))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""Base64Decode""#).unwrap(), StringModification::Base64Decode(Default::default()));
    /// ```
    #[cfg(feature = "base64")]
    Base64Decode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),



    /// The same operation [`Action::RemoveQueryParamsMatching`] does to a [`UrlPart::Query`].
    /// # Errors
    /// If the string is [`None`], returns the error [`StringModificationError::StringIsNone`].
    ///
    #[doc = edoc!(stringisnone(StringModification), checkerr(StringMatcher, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// let mut to = Some("a=2&b=3&%61=4&c=5".into());
    ///
    /// StringModification::RemoveQueryParamsMatching(Box::new(StringMatcher::Is("a".into()))).apply(&mut to, &task_state).unwrap();
    /// assert_eq!(to, Some("b=3&c=5".into()));
    /// StringModification::RemoveQueryParamsMatching(Box::new(StringMatcher::Is("b".into()))).apply(&mut to, &task_state).unwrap();
    /// assert_eq!(to, Some("c=5".into()));
    /// StringModification::RemoveQueryParamsMatching(Box::new(StringMatcher::Is("c".into()))).apply(&mut to, &task_state).unwrap();
    /// assert_eq!(to, None);
    /// ```
    RemoveQueryParamsMatching(Box<StringMatcher>),
    /// The same operation [`Action::AllowQueryParamsMatching`] does to a [`UrlPart::Query`].
    /// # Errors
    #[doc = edoc!(stringisnone(StringModification), checkerr(StringMatcher, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// let mut to = Some("a=2&b=3&%61=4&c=5".into());
    ///
    /// StringModification::AllowQueryParamsMatching(Box::new(StringMatcher::Is("a".into()))).apply(&mut to, &task_state).unwrap();
    /// assert_eq!(to, Some("a=2&%61=4".into()));
    /// StringModification::AllowQueryParamsMatching(Box::new(StringMatcher::Is("b".into()))).apply(&mut to, &task_state).unwrap();
    /// assert_eq!(to, None);
    /// ```
    AllowQueryParamsMatching(Box<StringMatcher>),
    /// The same operation [`Action::RemoveQueryParamsInSetOrStartingWithAnyInList`] does to a [`UrlPart::Query`].
    /// # Errors
    #[doc = edoc!(notfound(Set, StringModification))]
    ///
    /// If the list isn't found, returns the error [`StringModificationError::ListNotFound`].
    RemoveQueryParamsInSetOrStartingWithAnyInList {
        /// The name of the [`Set`] in [`Params::sets`] to use.
        set: String,
        /// The name of the list in [`Params::lists`] to use.
        list: String
    },



    /// Gets a [`Self`] from [`TaskStateView::commons`]'s [`Commons::string_modifications`] and applies it.
    /// # Errors
    #[doc = edoc!(ageterr(StringSource, CommonCall::name), agetnone(StringSource, StringModification, CommonCall::name), commonnotfound(Self, StringModification), callerr(CommonCallArgsSource::build), applyerr(Self))]
    Common(CommonCall),
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::string_modifications`] and applies it.
    /// # Errors
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`StringModificationError::NotInCommonContext`].
    ///
    #[doc = edoc!(commoncallargnotfound(Self, StringModification), applyerr(Self))]
    CommonCallArg(StringSource),
    /// Calls the contained function.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut Option<Cow<'_, str>>, &TaskStateView) -> Result<(), StringModificationError>)
}

string_or_struct_magic!(StringModification);

/// The error returned when trying to deserialize a [`StringModification`] variant with fields that aren't all defaultable.
#[derive(Debug, Error)]
#[error("Tried deserializing undefaultable or unknown variant {0}.")]
pub struct NonDefaultableVariant(String);

impl FromStr for StringModification {
    type Err = NonDefaultableVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            #[cfg(feature = "base64")] "Base64Decode" => StringModification::Base64Decode(Default::default()),
            #[cfg(feature = "base64")] "Base64Encode" => StringModification::Base64Encode(Default::default()),
            "PercentEncode"            => StringModification::PercentEncode(Default::default()),
            "PercentDecode"            => StringModification::PercentDecode,
            "LossyPercentDecode"       => StringModification::LossyPercentDecode,
            "None"                     => StringModification::None,
            "Lowercase"                => StringModification::Lowercase,
            "Uppercase"                => StringModification::Uppercase,
            "GetJsStringLiteralPrefix" => StringModification::GetJsStringLiteralPrefix,
            "UnescapeHtmlText"         => StringModification::UnescapeHtmlText,
            _                          => Err(NonDefaultableVariant(s.into()))?
        })
    }
}

/// The enum of errors [`StringModification::apply`] can return.
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// Returned when a [`StringModification::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`StringModification`]s in a [`StringModification::TryElse`] return errors.
    #[error("Both StringModifications in a StringModification::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`StringModification::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`StringModification::TryElse::else`].
        else_error: Box<Self>
    },
    /// Returned when all [`StringModification`]s in a [`StringModification::FirstNotError`] error.
    #[error("All StringModifications in a StringModification::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when a [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),

    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a JSON value isn't found.
    #[error("The requested JSON value was not found.")]
    JsonValueNotFound,
    /// Returned when a JSON pointee isn't a string.
    #[error("The requested JSON pointee was not a string.")]
    JsonPointeeIsNotAString,

    /// Returned when a slice is either not on UTF-8 boundaries or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundaries or out of bounds.")]
    InvalidSlice,
    /// Returned when an index is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// Returned when a segment isn't found.
    #[error("The requested segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a segment range isn't found
    #[error("The requested segment range wasn't found.")]
    SegmentRangeNotFound,

    /// Returned when the string being modified doesn't start with the specified prefix.
    #[error("The string being modified didn't start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// Returned when the string being modified doesn't end with the specified suffix.
    #[error("The string being modified didn't end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,

    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`std::string::FromUtf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// Returned when the [`StringModification::KeepBetween::start`] isn't found in the string.
    #[error("The StringModification::KeepBetween::start isn't found in the string.")]
    KeepBetweenStartNotFound,
    /// Returned when the [`StringModification::KeepBetween::end`] isn't found in the string after the [`StringModification::KeepBetween::start`].
    #[error("The StringModification::KeepBetween::end isn't found in the string after the StringModification::KeepBetween::start.")]
    KeepBetweenEndNotFound,

    /// Returned when a [`parse::js::StringLiteralPrefixError`] is encountered.
    #[error(transparent)]
    JsStringLiteralPrefixError(#[from] parse::js::StringLiteralPrefixError),
    /// Returned when a [`parse::html::UnescapeTextError`] is encountered.
    #[error(transparent)]
    HtmlUnescapeTextError(#[from] parse::html::UnescapeTextError),
    /// Returned when a [`parse::html::GAVError`] is encountered.
    #[error(transparent)]
    HtmlGetAttributeValueError(#[from] parse::html::GAVError),
    /// Returned when the requested HTML attribute isn't found.
    #[error("The requested HTML attribute wasn't found.")]
    HtmlAttributeNotFound,
    /// Returned when the requested HTML attribute doesn't have a value.
    #[error("The requested HTML attribute doesn't have a value.")]
    HtmlAttributeHasNoValue,

    /// Returned when a substring isn't found in the string.
    #[error("The substring wasn't found in the string.")]
    SubstringNotFound,
    /// Returned when the string to modify is [`None`] where it has to be [`Some`].
    #[error("The string to modify was None where it had to be Some")]
    StringIsNone,
    /// Returned when a [`Set`] with the specified name isn't found.
    #[error("A Set with the specified name wasn't found.")]
    SetNotFound,
    /// Returned when a list with the specified name isn't found.
    #[error("A list with the specified name wasn't found.")]
    ListNotFound,

    /// Returned when a [`::regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    /// Returned when a [`Regex`] doesn't find any matches in the string.
    #[cfg(feature = "regex")]
    #[error("The regex didn't find any matches in the string.")]
    RegexMatchNotFound,
    /// Returned when a [`::base64::DecodeError`] is encountered.
    #[cfg(feature = "base64")]
    #[error(transparent)]
    Base64DecodeError(#[from] ::base64::DecodeError),

    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Returned when a [`StringModification`] with the specified name isn't found in the [`Commons::string_modifications`].
    #[error("A StringModification with the specified name wasn't found in the Commons::string_modifications.")]
    CommonStringModificationNotFound,
    /// Returned when trying to use [`StringModification::CommonCallArg`] outside of a common context.
    #[error("Tried to use StringModification::CommonCallArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the [`StringModification`] requested from a [`StringModification::CommonCallArg`] isn't found.
    #[error("The StringModification requested from a StringModification::CommonCallArg wasn't found.")]
    CommonCallArgStringModificationNotFound,

    /// An arbitrary [`std::error::Error`] for use with [`StringModification::Custom`].
    #[cfg(feature = "custom")]
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send>)
}

impl From<StringSourceError> for StringModificationError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl From<StringMatcherError> for StringModificationError {
    fn from(value: StringMatcherError) -> Self {
        Self::StringMatcherError(Box::new(value))
    }
}

impl StringModification {
    /// Modified a string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn apply(&self, to: &mut Option<Cow<'_, str>>, task_state: &TaskStateView) -> Result<(), StringModificationError> {
        debug!(StringModification::apply, self, to);
        match self {
            Self::None => {},
            Self::Error(msg) => Err(StringModificationError::ExplicitError(msg.clone()))?,
            Self::Debug(modification) => {
                let to_before_mapper=to.clone();
                let modification_result=modification.apply(to, task_state);
                eprintln!("=== StringModification::Debug ===\nModification: {modification:?}\ntask_state: {task_state:?}\nString before mapper: {to_before_mapper:?}\nModification return value: {modification_result:?}\nString after mapper: {to:?}");
                modification_result?;
            },
            Self::IgnoreError(modification) => {let _=modification.apply(to, task_state);},
            Self::RevertOnError(modification) => {
                let old_to = to.clone();
                if let Err(e) = modification.apply(to, task_state) {
                    *to = old_to;
                    Err(e)?
                }
            },
            Self::TryElse{r#try, r#else} => r#try.apply(to, task_state).or_else(|try_error| r#else.apply(to, task_state).map_err(|else_error| StringModificationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::All(modifications) => {
                for modification in modifications {
                    modification.apply(to, task_state)?;
                }
            },
            Self::FirstNotError(modifications) => {
                let mut errors = Vec::new();
                for modification in modifications {
                    match modification.apply(to, task_state) {
                        Ok(()) => return Ok(()),
                        Err(e) => errors.push(e)
                    }
                }
                Err(StringModificationError::FirstNotErrorErrors(errors))?
            },
            Self::IfSome(modification) => if to.is_some() {modification.apply(to, task_state)?;}
            Self::IfMatches {matcher, then, r#else} => if matcher.check(to.as_deref(), task_state)? {
                then.apply(to, task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?
            },
            Self::IfContains {value, at, then, r#else} => if at.check(to.as_deref().ok_or(StringModificationError::StringIsNone)?, &value.get(task_state)?.ok_or(StringModificationError::StringSourceIsNone)?)? {
                then.apply(to, task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?;
            },
            Self::IfContainsAny {values, at, then, r#else} => {
                for value in values {
                    if at.check(to.as_deref().ok_or(StringModificationError::StringIsNone)?, &value.get(task_state)?.ok_or(StringModificationError::StringSourceIsNone)?)? {
                        return then.apply(to, task_state);
                    }
                }
                if let Some(r#else) = r#else {
                    r#else.apply(to, task_state)?;
                }
            },
            Self::Map {value, map} => if let Some(x) = map.get(value.get(task_state)?) {x.apply(to, task_state)?;},



            Self::Set(value)     => *to = value.get(task_state)?.map(|x| Cow::Owned(x.into_owned())),
            Self::Append(value)  => to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut().push_str(get_str!(value, task_state, StringModificationError)),
            Self::Prepend(value) => {
                let suffix = to.as_deref().ok_or(StringModificationError::StringIsNone)?;
                let mut ret = get_string!(value, task_state, StringModificationError);
                ret.push_str(suffix);
                *to=Some(Cow::Owned(ret));
            },
            Self::Insert{index, value} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let index = neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?;
                if to.is_char_boundary(index) {
                    to.insert_str(index, get_str!(value, task_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidIndex)?;
                }
            },
            Self::Lowercase => *to = Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.to_lowercase())),
            Self::Uppercase => *to = Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.to_uppercase())),



            Self::StripPrefix(prefix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let prefix = get_str!(prefix, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => if inner.starts_with(prefix) {inner.drain(..prefix.len());} else {Err(StringModificationError::PrefixNotFound)?;},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(inner.strip_prefix(prefix).ok_or(StringModificationError::PrefixNotFound)?)
                }
            },
            Self::StripMaybePrefix(prefix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let prefix = get_str!(prefix, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => if inner.starts_with(prefix) {inner.drain(..prefix.len());},
                    Cow::Borrowed(inner) => if let Some(x) = inner.strip_prefix(prefix) {*to = Cow::Borrowed(x);}
                }
            },
            Self::StripSuffix(suffix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let suffix = get_str!(suffix, task_state, StringModificationError);
                #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
                match to {
                    Cow::Owned(inner) => if inner.ends_with(suffix) {inner.truncate(inner.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(inner.strip_suffix(suffix).ok_or(StringModificationError::SuffixNotFound)?)
                }
            },
            Self::StripMaybeSuffix(suffix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let suffix = get_str!(suffix, task_state, StringModificationError);
                #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
                match to {
                    Cow::Owned(inner) => if inner.ends_with(suffix) {inner.truncate(inner.len() - suffix.len());},
                    Cow::Borrowed(inner) => if let Some(x) = inner.strip_suffix(suffix) {*to = Cow::Borrowed(x);}
                }
            },
            Self::RemoveChar(index) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                if to.is_char_boundary(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?) {
                    to.remove(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?);
                } else {
                    Err(StringModificationError::InvalidIndex)?;
                }
            },



            Self::KeepBefore(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => {inner.drain(inner.find(s).ok_or(StringModificationError::SubstringNotFound)?..);},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[..inner.find(s).ok_or(StringModificationError::SubstringNotFound)?])
                }
            },
            Self::StripBefore(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => {inner.drain(..inner.find(s).ok_or(StringModificationError::SubstringNotFound)?);},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[inner.find(s).ok_or(StringModificationError::SubstringNotFound)?..])
                }
            },
            Self::KeepBetween {start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = match to {
                    Cow::Borrowed(inner) => Cow::Borrowed(inner
                        .split_once(get_str!(start, task_state, StringSourceError)).ok_or(StringModificationError::KeepBetweenStartNotFound)?.1
                        .split_once(get_str!(end  , task_state, StringSourceError)).ok_or(StringModificationError::KeepBetweenEndNotFound  )?.0
                    ),
                    Cow::Owned(inner) => Cow::Owned(inner
                        .split_once(get_str!(start, task_state, StringSourceError)).ok_or(StringModificationError::KeepBetweenStartNotFound)?.1
                        .split_once(get_str!(end  , task_state, StringSourceError)).ok_or(StringModificationError::KeepBetweenEndNotFound  )?.0
                        .to_string()
                    )
                }
            },
            Self::StripAfter(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                match to {
                    Cow::Owned(inner) => {inner.drain((inner.find(s).ok_or(StringModificationError::SubstringNotFound)? + s.len())..);},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[..inner.find(s).ok_or(StringModificationError::SubstringNotFound)? + s.len()])
                }
            },
            Self::KeepAfter(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                match to {
                    Cow::Owned(inner) => {inner.drain(..(inner.find(s).ok_or(StringModificationError::SubstringNotFound)? + s.len()));},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[inner.find(s).ok_or(StringModificationError::SubstringNotFound)? + s.len()..])
                }
            },



            Self::KeepMaybeBefore(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                if let Some(i) = to.find(s) {
                    match to {
                        Cow::Owned(inner) => {inner.drain(i..);},
                        Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[..i])
                    }
                }
            },
            Self::StripMaybeBefore(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                if let Some(i) = to.find(s) {
                    match to {
                        Cow::Owned(inner) => {inner.drain(..i);},
                        Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[i..])
                    }
                }
            },
            Self::KeepMaybeBetween {start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = match to {
                    Cow::Borrowed(inner) => {
                        let mut temp = &**inner;
                        temp = temp.split_once(get_str!(start, task_state, StringSourceError)).map_or(temp, |(_, x)| x);
                        temp = temp.split_once(get_str!(end  , task_state, StringSourceError)).map_or(temp, |(x, _)| x);
                        Cow::Borrowed(temp)
                    },
                    Cow::Owned(inner) => {
                        let mut temp = &**inner;
                        temp = temp.split_once(get_str!(start, task_state, StringSourceError)).map_or(temp, |(_, x)| x);
                        temp = temp.split_once(get_str!(end  , task_state, StringSourceError)).map_or(temp, |(x, _)| x);
                        Cow::Owned(temp.to_string())
                    }
                }
            },
            Self::StripMaybeAfter(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                if let Some(i) = to.find(s) {
                    #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                    match to {
                        Cow::Owned(inner) => {inner.drain((i + s.len())..);},
                        Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[..i + s.len()])
                    }
                }
            },
            Self::KeepMaybeAfter(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                if let Some(i) = to.find(s) {
                    #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                    match to {
                        Cow::Owned(inner) => {inner.drain(..(i + s.len()));},
                        Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[i + s.len()..])
                    }
                }
            },



            Self::Replacen{find, replace, count}     => *to=Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.replacen(get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError), *count))),
            Self::ReplaceAll{find, replace}             => *to=Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.replace (get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError)))),



            Self::ReplaceRange{start, end, replace}  => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let range=neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, get_str!(replace, task_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidSlice)?;
                }
            },
            Self::KeepRange{start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                match to {
                    Cow::Owned(inner) => *inner = inner.get(neg_range(*start, *end, inner.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
                    Cow::Borrowed(inner) => *inner = inner.get(neg_range(*start, *end, inner.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?
                }
            },



            Self::SetSegment {split, index, value} => {
                let split = get_cow!(split, task_state, StringModificationError);
                let mut segments = to.as_ref().ok_or(StringModificationError::StringIsNone)?.split(&*split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_index = neg_index(*index, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                match value.get(task_state)? {
                    Some(value) => *segments.get_mut(fixed_index).expect("StringModification::SetSegment to be implemented correctly") = value,
                    None => {segments.remove(fixed_index);}
                }
                *to = Some(Cow::Owned(segments.join(&*split)));
            },
            Self::InsertSegment {split, index, value} => {
                let split = get_cow!(split, task_state, StringModificationError);
                let mut segments = to.as_ref().ok_or(StringModificationError::StringIsNone)?.split(&*split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_index = neg_range_boundary(*index, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                if let Some(value) = value.get(task_state)? {
                    segments.insert(fixed_index, value);
                }
                *to = Some(Cow::Owned(segments.join(&*split)));
            },
            Self::KeepNthSegment {split, index} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = match to {
                    Cow::Owned(inner) => Cow::Owned(neg_nth(inner.split(get_str!(split, task_state, StringModificationError)), *index).ok_or(StringModificationError::SegmentNotFound)?.to_string()),
                    Cow::Borrowed(inner) => Cow::Borrowed(neg_nth(inner.split(get_str!(split, task_state, StringModificationError)), *index).ok_or(StringModificationError::SegmentNotFound)?),
                }
            },
            Self::KeepSegmentRange {split, start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let split = get_str!(split, task_state, StringModificationError);
                *to = neg_vec_keep(to.split(split), *start, *end).ok_or(StringModificationError::SegmentRangeNotFound)?.join(split);
            },



            Self::GetJsStringLiteralPrefix => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = Cow::Owned(parse::js::string_literal_prefix(to)?);
            },
            Self::UnescapeHtmlText => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = Cow::Owned(parse::html::unescape_text(to)?);
            },
            Self::GetHtmlAttribute(name) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = Cow::Owned(parse::html::get_attribute_value(to, get_str!(name, task_state, StringModificationError))?.ok_or(StringModificationError::HtmlAttributeNotFound)?.ok_or(StringModificationError::HtmlAttributeHasNoValue)?);
            },



            #[cfg(feature = "regex")]
            Self::RegexFind(regex) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = regex.get()?.find(to).ok_or(StringModificationError::RegexMatchNotFound)?.as_str().to_string();
            },
            #[cfg(feature = "regex")]
            Self::RegexSubstitute {regex, replace} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let replace = get_str!(replace, task_state, StringModificationError);
                let mut temp = "".to_string();
                regex.get()?.captures(to).ok_or(StringModificationError::RegexMatchNotFound)?.expand(replace, &mut temp);
                *to = temp;
            },
            #[cfg(feature = "regex")]
            Self::JoinAllRegexSubstitutions {regex, replace, join} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let replace = get_str!(replace, task_state, StringModificationError);
                let join = get_str!(join, task_state, StringModificationError);
                let regex = regex.get()?;
                let mut temp = "".to_string();
                if join.is_empty() {
                    for captures in regex.captures_iter(to) {
                        captures.expand(replace, &mut temp);
                    }
                } else {
                    let mut iter = regex.captures_iter(to).peekable();
                    while let Some(captures) = iter.next() {
                        captures.expand(replace, &mut temp);
                        if iter.peek().is_some() {temp.push_str(join);}
                    }
                }
                *to = temp;
            },
            #[cfg(feature = "regex")]
            Self::RegexReplaceOne {regex,replace} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = regex.get()?.replace(to,get_str!(replace, task_state, StringModificationError)).into_owned();
            },
            #[cfg(feature = "regex")]
            Self::RegexReplaceAll {regex,replace} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = regex.get()?.replace_all(to,get_str!(replace, task_state, StringModificationError)).into_owned();
            },
            #[cfg(feature = "regex")]
            Self::RegexReplacen {regex, n, replace} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = regex.get()?.replacen(to, *n, get_str!(replace, task_state, StringModificationError)).into_owned();
            },



            Self::JsonPointer(pointer) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                match serde_json::from_str::<serde_json::Value>(to)?.pointer_mut(get_str!(pointer, task_state, StringModificationError)).ok_or(StringModificationError::JsonValueNotFound)?.take() {
                    serde_json::Value::String(s) => *to = s,
                    _ => Err(StringModificationError::JsonPointeeIsNotAString)?
                }
            },



            Self::PercentEncode(alphabet) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = utf8_percent_encode(to, alphabet.get()).to_string();
            },
            Self::PercentDecode => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = percent_decode_str(to).decode_utf8()?.into_owned();
            },
            Self::LossyPercentDecode => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = percent_decode_str(to).decode_utf8_lossy().into_owned();
            },



            #[cfg(feature = "base64")]
            Self::Base64Encode(config) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = config.build().encode(to.as_bytes());
            },
            #[cfg(feature = "base64")]
            Self::Base64Decode(config) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = String::from_utf8(config.build().decode(to.as_bytes())?)?;
            },



            Self::RemoveQueryParamsMatching(matcher) => if let Some(inner) = to {
                let mut new = String::with_capacity(inner.len());
                for param in inner.split('&') {
                    if !matcher.check(Some(&peh(param.split('=').next().expect("The first segment to always exist"))), task_state)? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != inner.len() {
                    *to = Some(Cow::<str>::Owned(new)).filter(|new| !new.is_empty());
                }
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(inner) = to {
                let mut new = String::with_capacity(inner.len());
                for param in inner.split('&') {
                    if matcher.check(Some(&peh(param.split('=').next().expect("The first segment to always exist"))), task_state)? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != inner.len() {
                    *to = Some(Cow::<str>::Owned(new)).filter(|new| !new.is_empty());
                }
            },
            Self::RemoveQueryParamsInSetOrStartingWithAnyInList {set, list} => if let Some(inner) = to {
                let mut new = String::with_capacity(inner.len());
                let set = task_state.params.sets.get(set).ok_or(StringModificationError::SetNotFound)?;
                let list = task_state.params.lists.get(list).ok_or(StringModificationError::ListNotFound)?;
                for param in inner.split('&') {
                    let name = peh(param.split('=').next().expect("The first segment to always exist."));
                    if !(set.contains(Some(&*name)) || list.iter().any(|x| name.starts_with(x))) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != inner.len() {
                    *to = Some(Cow::<str>::Owned(new)).filter(|x| !x.is_empty());
                }
            },



            Self::Common(common_call) => {
                task_state.commons.string_modifications.get(get_str!(common_call.name, task_state, StringModificationError)).ok_or(StringModificationError::CommonStringModificationNotFound)?.apply(
                    to,
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
                )?;
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(StringModificationError::NotInCommonContext)?.string_modifications.get(get_str!(name, task_state, StringModificationError)).ok_or(StringModificationError::CommonCallArgStringModificationNotFound)?.apply(to, task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(to, task_state)?
        };
        Ok(())
    }
}

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

/// Modify a [`String`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringModification {
    /// Doesn't do any modification.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    #[suitable(never)]
    Debug(Box<Self>),



    /// Always returns the error [`StringModificationError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringModificationError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::IgnoreError(Box::new(StringModification::Error("...".into()))).apply(&mut to, &task_state_view).unwrap();
    /// ```
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the [`String`] to its previous value then return the error.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// If both calls to [`Self::apply`] return errors, both errors are returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// If any call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// If all calls to [`Self::apply`] error, the errors are returned in a [`StringModificationError::FirstNotErrorErrors`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
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



    /// Only apply [`Self::IfSome::0`] if the string is [`Some`].
    IfSome(Box<Self>),
    /// If the call to [`Self::If::condition`] passes, apply [`Self::If::then`].
    ///
    /// If the call to [`Self::If::condition`] fails and [`Self::If::else`] is [`Some`], apply [`Self::If::else`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    ///
    /// let mut to = Some("abc".into());
    /// StringModification::If {
    ///     condition: Box::new(Condition::Always),
    ///     then     : Box::new(StringModification::Set("def".into())),
    ///     r#else   : Some(Box::new(StringModification::None))
    /// }.apply(&mut to, &task_state_view).unwrap();
    /// assert_eq!(to, Some("def".into()));
    ///
    /// let mut to = Some("abc".into());
    /// StringModification::If {
    ///     condition: Box::new(Condition::Never),
    ///     then     : Box::new(StringModification::Set("def".into())),
    ///     r#else   : Some(Box::new(StringModification::None))
    /// }.apply(&mut to, &task_state_view).unwrap();
    /// assert_eq!(to, Some("abc".into()));
    /// ```
    If {
        /// The [`Condition`] to decide between [`Self::If::then`] and [`Self::If::else`].
        condition: Box<Condition>,
        /// The [`Self`] to apply if [`Self::If::condition`] passes.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::If::condition`] fails.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
    },
    /// If the [`String`] satisfies [`Self::IfMatches::matcher`], apply [`Self::If::then`].
    ///
    /// If the [`String`] does not satisfy [`Self::IfMatches::matcher`] and [`Self::IfMatches::else`] is [`Some`], apply [`Self::IfMatches::else`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    IfMatches {
        /// The [`StringMatcher`] to check if the [`String`] satisfies.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] is satisfied.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] isn't satisfied.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
    },



    /// Sets the [`String`] to the specified value.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Set(StringSource),
    /// Appends the specified value.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Append(StringSource),
    /// Prepends the specified value.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Prepend(StringSource),
    /// Replace all instances of [`Self::Replace::find`] with [`Self::Replace::replace`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    Replace {
        /// The value to replace with [`Self::Replace::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::Replace::find`] with.
        replace: StringSource
    },
    /// Replace the specified range with [`Self::ReplaceRange::replace`].
    /// # Errors
    /// If the specified range is either out of bounds or doesn't fall on UTF-8 character boundaries, returns the error [`StringModificationError::InvalidSlice`].
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ReplaceRange {
        /// The start of the range to replace.
        start: isize,
        /// The end of the range to replace.
        end: Option<isize>,
        /// The value to replace the range with.
        replace: StringSource
    },
    /// Sets the string to lowercase.
    ///
    /// See [`str::to_lowercase`] for details.
    Lowercase,
    /// Sets the string to uppercase.
    ///
    /// See [`str::to_uppercase`] for details.
    Uppercase,
    /// Removes the specified prefix.
    ///
    /// If you want to not return an error when the string doesn't start with the prefix, see [`Self::StripMaybePrefix`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the string doesn't start with the specified prefix, returns the error [`StringModificationError::PrefixNotFound`].
    StripPrefix(StringSource),
    /// Removes the specified suffix.
    ///
    /// If you want to not return an error when the string doesn't start with the suffix, see [`Self::StripMaybeSuffix`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the string doesn't start with the specified suffix, returns the error [`StringModificationError::SuffixNotFound`].
    StripSuffix(StringSource),
    /// If the string starts with the specified value, remove it.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    StripMaybePrefix(StringSource),
    /// If the string ends with the specified value, remove it.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    StripMaybeSuffix(StringSource),
    /// Replace up to [`Self::Replacen::count`] instances of [`Self::Replacen::find`] with [`Self::Replacen::replace`].
    ///
    /// See [`str::replacen`] for details.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    Replacen {
        /// The value to replace with [`Self::Replacen::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::Replacen::find`] with.
        replace: StringSource,
        /// The maximum amount of instances to replace.
        count: usize
    },
    /// Insert [`Self::Insert::value`] at [`Self::Insert::index`].
    ///
    /// See [`String::insert_str`] for details.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the call to [`String::insert_str`] would panic, returns the error [`StringModificationError::InvalidIndex`].
    Insert {
        /// The value to insert at [`Self::Insert::index`].
        value: StringSource,
        /// The index to insert [`Self::Insert::value`] at.
        index: isize
    },
    /// Removes the [`char`] at the specified index.
    ///
    /// See [`String::remove`] for details.
    /// # Errors
    /// If the call to [`String::remove`] would panic, returns the error [`StringModificationError::InvalidIndex`].
    RemoveChar(isize),
    /// Removes everything outside the specified range.
    /// # Errors
    /// If the range is out of bounds, returns the error [`StringModificationError::InvalidSlice`].
    ///
    /// If the call to [`str::get`] returns [`None`], returns the error [`StringModificationError::InvalidSlice`].
    KeepRange {
        /// The start of the range to keep.
        start: isize,
        /// The end of the range to keep.
        end: Option<isize>
    },



    /// Calls [`::regex::Regex::captures`] and returns the result of [`::regex::Captures::expand`]ing with [`Self::RegexCaptures::replace`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the call to [`Regex::captures`] returns [`None`], returns the error [`StringModificationError::RegexMatchNotFound`];
    #[cfg(feature = "regex")]
    RegexCaptures {
        /// The [`RegexWrapper`] to capture with.
        regex: RegexWrapper,
        /// The format string to pass to [`::regex::Captures::expand`].
        replace: StringSource
    },
    /// [`::regex::Captures::expand`] each [`::regex::Regex::captures_iter`] with [`Self::JoinAllRegexCaptures::replace`] and join them with [`Self::JoinAllRegexCaptures::join`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    #[cfg(feature = "regex")]
    JoinAllRegexCaptures {
        /// The [`RegexWrapper`] to capture with.
        regex: RegexWrapper,
        /// The format string to pass to [`::regex::Captures::expand`].
        replace: StringSource,
        /// The [`StringSource`] to join the expanded captures with.
        join: StringSource
    },
    /// Calls [`Regex::find`] and returns its value.
    /// # Errors
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Regex::find`] returns [`None`], returns the errorr [`StringModificationError::RegexMatchNotFound`].
    #[cfg(feature = "regex")]
    RegexFind(RegexWrapper),
    /// [`Regex::replace`]s the first match of [`Self::RegexReplace::regex`] with [`Self::RegexReplace::replace`].
    /// # Errors
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    #[cfg(feature = "regex")]
    RegexReplace {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The format string to expand the capture with.
        replace: StringSource
    },
    /// [`Regex::replace`]s the all matches of [`Self::RegexReplace::regex`] with [`Self::RegexReplace::replace`].
    /// # Errors
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The format string to expand the captures with.
        replace: StringSource
    },
    /// [`Regex::replacen`]s the first [`Self::RegexReplacen::n`] of [`Self::RegexReplace::regex`] with [`Self::RegexReplace::replace`].
    /// # Errors
    /// If the call to [`RegexWrapper::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    #[cfg(feature = "regex")]
    RegexReplacen {
        /// The [`RegexWrapper`] to search with.
        regex: RegexWrapper,
        /// The number of captures to find and replace.
        n: usize,
        /// The format string to expand the captures with.
        replace: StringSource
    },



    /// Percent encodes the string.
    ///
    /// Please note that this can be deserialized from `"PercentDecode"`, in which case the contained [`PercentEncodeAlphabet`] is defaulted.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""PercentEncode""#).unwrap(), StringModification::PercentEncode(Default::default()));
    /// ```
    PercentEncode(#[serde(default, skip_serializing_if = "is_default")] PercentEncodeAlphabet),
    /// Percent decodes the string.
    ///
    /// Unfortunately doesn't allow specifying a [`PercentEncodeAlphabet`] to keep certain values encoded due to limitations with the [`::percent_encoding`] API.
    /// # Errors
    /// If the call to [`::percent_encoding::PercentDecode::decode_utf8`] returns an error, that error is returned.
    PercentDecode,
    /// [`Self::PercentDecode`] but replaces non-UTF-8 percent encoded byte equences with U+FFFD (�), the replacement character.
    ///
    /// Unfortunately doesn't allow specifying a [`PercentEncodeAlphabet`] to keep certain values encoded due to limitations with the [`::percent_encoding`] API.
    LossyPercentDecode,
    /// Base64 encodes the string.
    ///
    /// Please note that this can be deserialized from `"Base64Encode"`, in which case the contained [`Base64Config`] is defaulted.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""Base64Encode""#).unwrap(), StringModification::Base64Encode(Default::default()));
    /// ```
    #[cfg(feature = "base64")]
    Base64Encode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),
    /// Base64 decodes the string.
    ///
    /// Please note that this can be deserialized from `"Base64Decode"`, in which case the contained [`Base64Config`] is defaulted.
    /// # Errors
    /// If the call to [`::base64::engine::GeneralPurpose::decode`] returns an error, that error is returned.
    ///
    /// If the call to [`String::from_utf8`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// assert_eq!(serde_json::from_str::<StringModification>(r#""Base64Decode""#).unwrap(), StringModification::Base64Decode(Default::default()));
    /// ```
    #[cfg(feature = "base64")]
    Base64Decode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),



    /// Parses the javascript string literal at the start of the string and returns its value.
    ///
    /// Useful in combination with [`Self::KeepAfter`].
    /// # Errors
    /// If the call to [`parse::js::string_literal_prefix()`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
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
    /// If the call to [`parse::html::unescape_text`] returns an error, that error is returned.
    UnescapeHtmlText,
    /// Parses the HTML element at the start of the string and returns the [`Self::UnescapeHtmlText`]ed value of the last attribute with the specified name.
    ///
    /// Useful in combination with [`Self::StripBefore`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`parse::html::get_attribute_value`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state);
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
    /// Parses the string as JSON and uses [`serde_json::Value::pointer`] with the specified pointer.
    ///
    /// When extracting values from javascript, it's often faster to find the start of the desired string and use [`Self::GetJsStringLiteralPrefix`].
    /// # Errors
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the call to [`serde_json::Value::pointer_mut`] returns [`None`], returns the error [`StringModificationError::JsonValueNotFound`].
    ///
    /// If the call to [`serde_json::Value::pointer_mut`] doesn't return a [`serde_json::Value::String`], returns the error [`StringModificationError::JsonPointeeIsNotAString`].
    JsonPointer(StringSource),



    /// Split the string on `&`, split each segment on `=`, and remove all segments whose first subsegment matches the specified [`StringMatcher`].
    ///
    /// Useful for websites that put tracking parameters in the [`UrlPart::Fragment`] of all places.
    ///
    /// Currently does not do percent decoding, so `%61=2` isn't normalized to `a=2`.
    /// # Errors
    /// If any call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    RemoveQueryParamsMatching(Box<StringMatcher>),
    /// Split the string on `&`, split each segment on `=`, and keep only segments whose first subsegment matches the specified [`StringMatcher`].
    ///
    /// Useful for websites that put tracking parameters in the [`UrlPart::Fragment`] of all places.
    ///
    /// Currently does not do percent decoding, so `%61=2` isn't normalized to `a=2`.
    /// # Errors
    /// If any call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    AllowQueryParamsMatching(Box<StringMatcher>),
    /// Finds the first instance of the specified substring and removes everything before it.
    ///
    /// Useful in combination with [`Self::GetJsStringLiteralPrefix`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::StripBefore("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("bc".into()));
    /// ```
    StripBefore(StringSource),
    /// Finds the first instance of the specified substring and removes everything after it.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::StripAfter("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("ab".into()));
    /// ```
    StripAfter(StringSource),
    /// Finds the first instance of the specified substring and keeps only everything before it.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::KeepBefore("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("a".into()));
    /// ```
    KeepBefore(StringSource),
    /// Finds the first instance of the specified substring and keeps only everything after it.
    ///
    /// Useful in combination with [`Self::GetHtmlAttribute`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the specified substring isn't found, returns the error [`StringModificationError::SubstringNotFound`]
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state_view!(task_state_view);
    /// let mut to = Some("abc".into());
    ///
    /// StringModification::KeepAfter("b".into()).apply(&mut to, &task_state_view).unwrap();
    ///
    /// assert_eq!(to, Some("c".into()));
    /// ```
    KeepAfter(StringSource),
    /// Effectively [`Self::KeepAfter`] with [`Self::ExtractBetween::start`] and [`Self::KeepBefore`] with [`Self::ExtractBetween::end`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the value of [`Self::ExtractBetween::start`] isn't found in the string, returns the error [`StringModificationError::ExtractBetweenStartNotFound`].
    ///
    /// If the value of [`Self::ExtractBetween::end`] isn't found in the string after [`Self::ExtractBetween::start`], returns the error [`StringModificationError::ExtractBetweenEndNotFound`].
    ExtractBetween {
        /// The value to [`Self::KeepAfter`].
        start: StringSource,
        /// The value to [`Self::KeepBefore`].
        end: StringSource
    },
    /// Split the string with [`Self::KeepNthSegment::split`] and keep only the [`Self::KeepNthSegment::index`] segment.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
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
    /// If the call to [`StringSource::get`] returns an error that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If the range isn't found, returns the error [`StringModificationError::SegmentRangeNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state_view);
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
    /// Split the string with [`Self::SetSegment::split`] and set the [`Self::SetSegment::index`] segment to [`Self::SetSegment::value`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If [`Self::SetSegment::split`]'s call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
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
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If [`Self::SetSegment::split`]'s call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
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
    /// Indexes [`Self::Map::map`] with [`Self::Map::value`] and, if a [`Self`] is found, applies it.
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is retutrned.
    Map {
        /// The value to index [`Self::Map::map`] with.
        value: Box<StringSource>,
        /// The [`Map`] to index with [`Self::Map::value`].
        #[serde(flatten)]
        map: Map<Self>,
    },
    /// Gets a [`Self`] from [`Cleaner::commons`]'s [`Commons::string_modifications`] and applies it.
    /// # Errors
    /// If [`CommonCall::name`]'s call to [`StringSource::get`] returns an error, returns the error [`StringModificationError::StringSourceIsNone`].
    ///
    /// If no [`Self`] with the specified name is found, returns the error [`StringModificationError::CommonStringModificationNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    Common(CommonCall),
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::string_modifications`] and applies it.
    /// # Errors
    /// If no [`Self`] with the specified name is found, returns the error [`StringModificationError::CommonCallArgStringModificationNotFound`].
    ///
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`StringModificationError::NotInCommonContext`].
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    CommonCallArg(StringSource),
    /// Calls the contained function.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "Who cares")]
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
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
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
    /// Returned when a [`Regex`] doesn't find any matches in the string.
    #[cfg(feature = "regex")]
    #[error("The regex didn't find any matches in the string.")]
    RegexMatchNotFound,
    /// Returned when a [`::regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
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
    /// Returned when a [`::base64::DecodeError`] is encountered.
    #[cfg(feature = "base64")]
    #[error(transparent)]
    Base64DecodeError(#[from] ::base64::DecodeError),
    /// Returned when a [`std::string::FromUtf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when a [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    /// Returned when the [`StringModification::ExtractBetween::start`] isn't found in the string.
    #[error("The StringModification::ExtractBetween::start isn't found in the string.")]
    ExtractBetweenStartNotFound,
    /// Returned when the [`StringModification::ExtractBetween::end`] isn't found in the string after the [`StringModification::ExtractBetween::start`].
    #[error("The StringModification::ExtractBetween::end isn't found in the string after the StringModification::ExtractBetween::start.")]
    ExtractBetweenEndNotFound,
    /// Returned when a [`StringModification`] with the specified name isn't found in the [`Commons::string_modifications`].
    #[error("A StringModification with the specified name wasn't found in the Commons::string_modifications.")]
    CommonStringModificationNotFound,
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] Box<ConditionError>),
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
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Returned when trying to use [`StringModification::CommonCallArg`] outside of a common context.
    #[error("Tried to use StringModification::CommonCallArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the [`StringModification`] requested from a [`StringModification::CommonCallArg`] isn't found.
    #[error("The StringModification requested from a StringModification::CommonCallArg wasn't found.")]
    CommonCallArgStringModificationNotFound,
    /// Returned when the string to modify is [`None`] where it has to be [`Some`].
    #[error("The string to modify was None where it had to be Some")]
    StringIsNone,
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

impl From<ConditionError> for StringModificationError {
    fn from(value: ConditionError) -> Self {
        Self::ConditionError(Box::new(value))
    }
}

impl StringModification {
    /// Modified a [`String`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn apply(&self, to: &mut Option<Cow<'_, str>>, task_state: &TaskStateView) -> Result<(), StringModificationError> {
        debug!(self, StringModification::apply, to, task_state);
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
            Self::If {condition, then, r#else} => if condition.satisfied_by(task_state)? {
                then.apply(to, task_state)?
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?
            },
            Self::IfMatches {matcher, then, r#else} => if matcher.satisfied_by(to.as_deref(), task_state)? {
                then.apply(to, task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?
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
            Self::Set(value)     => *to = value.get(task_state)?.map(|x| Cow::Owned(x.into_owned())),
            Self::Append(value)  => to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut().push_str(get_str!(value, task_state, StringModificationError)),
            Self::Prepend(value) => {
                let suffix = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let mut ret=get_string!(value, task_state, StringModificationError);
                ret.push_str(suffix);
                *to=Some(Cow::Owned(ret));
            },
            Self::Replace{find, replace}             => *to=Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.replace (get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError)))),
            Self::Replacen{find, replace, count}     => *to=Some(Cow::Owned(to.as_ref().ok_or(StringModificationError::StringIsNone)?.replacen(get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError), *count))),
            Self::ReplaceRange{start, end, replace}  => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let range=neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, get_str!(replace, task_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidSlice)?;
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
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripSuffix(suffix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let suffix = get_str!(suffix, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => if inner.ends_with(suffix) {inner.truncate(inner.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(inner.strip_suffix(suffix).ok_or(StringModificationError::SuffixNotFound)?)
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
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripMaybeSuffix(suffix) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let suffix = get_str!(suffix, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => if inner.ends_with(suffix) {inner.truncate(inner.len() - suffix.len());},
                    Cow::Borrowed(inner) => if let Some(x) = inner.strip_suffix(suffix) {*to = Cow::Borrowed(x);}
                }
            },
            Self::Insert{index, value} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                if to.is_char_boundary(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?) {
                    to.insert_str(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?, get_str!(value, task_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidIndex)?;
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
            Self::KeepRange{start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                match to {
                    Cow::Owned(inner) => *inner = inner.get(neg_range(*start, *end, inner.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
                    Cow::Borrowed(inner) => *inner = inner.get(neg_range(*start, *end, inner.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?
                }
            },
            #[cfg(feature = "regex")]
            Self::RegexCaptures {regex, replace} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                let replace = get_str!(replace, task_state, StringModificationError);
                let mut temp = "".to_string();
                regex.get()?.captures(to).ok_or(StringModificationError::RegexMatchNotFound)?.expand(replace, &mut temp);
                *to = temp;
            },
            #[cfg(feature = "regex")]
            Self::JoinAllRegexCaptures {regex, replace, join} => {
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
            Self::RegexFind(regex) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = regex.get()?.find(to).ok_or(StringModificationError::RegexMatchNotFound)?.as_str().to_string();
            },
            #[cfg(feature = "regex")]
            Self::RegexReplace {regex,replace} => {
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
            Self::PercentEncode(alphabet) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = utf8_percent_encode(to, alphabet.get()).to_string();
            },
            Self::PercentDecode => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = percent_decode_str (to).decode_utf8()?.into_owned();
            },
            Self::LossyPercentDecode => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = percent_decode_str (to).decode_utf8_lossy().into_owned();
            },
            #[cfg(feature = "base64")] Self::Base64Encode(config) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = config.build().encode(to.as_bytes());
            },
            #[cfg(feature = "base64")] Self::Base64Decode(config) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                *to = String::from_utf8(config.build().decode(to.as_bytes())?)?;
            },
            Self::JsonPointer(pointer) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?.to_mut();
                match serde_json::from_str::<serde_json::Value>(to)?.pointer_mut(get_str!(pointer, task_state, StringModificationError)).ok_or(StringModificationError::JsonValueNotFound)?.take() {
                    serde_json::Value::String(s) => *to = s,
                    _ => Err(StringModificationError::JsonPointeeIsNotAString)?
                }
            },


            Self::RemoveQueryParamsMatching(matcher) => {
                let inner = to.as_ref().ok_or(StringModificationError::StringIsNone)?;
                *to = Some(Cow::<str>::Owned(inner.split('&').filter_map(|kev|
                    matcher.satisfied_by(
                        Some(kev.split('=').next().expect("Why can't I #[allow] an .expect() here?")),
                        task_state
                    )
                    .map(|x| (!x).then_some(kev))
                    .transpose()
                ).collect::<Result<Vec<_>, _>>()?.join("&"))).filter(|x| !x.is_empty());
            },
            Self::AllowQueryParamsMatching(matcher) => {
                let inner = to.as_ref().ok_or(StringModificationError::StringIsNone)?;
                *to = Some(Cow::<str>::Owned(inner.split('&').filter_map(|kev|
                    matcher.satisfied_by(
                        Some(kev.split('=').next().expect("Why can't I #[allow] an .expect() here?")),
                        task_state
                    )
                    .map(|x| x.then_some(kev))
                    .transpose()
                ).collect::<Result<Vec<_>, _>>()?.join("&"))).filter(|x| !x.is_empty());
            },



            Self::StripBefore(s) => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                let s = get_str!(s, task_state, StringModificationError);
                match to {
                    Cow::Owned(inner) => {inner.drain(..inner.find(s).ok_or(StringModificationError::SubstringNotFound)?);},
                    Cow::Borrowed(inner) => *to = Cow::Borrowed(&inner[inner.find(s).ok_or(StringModificationError::SubstringNotFound)?..])
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
            Self::ExtractBetween {start, end} => {
                let to = to.as_mut().ok_or(StringModificationError::StringIsNone)?;
                *to = match to {
                    Cow::Borrowed(inner) => Cow::Borrowed(inner
                        .split_once(get_str!(start, task_state, StringSourceError)).ok_or(StringModificationError::ExtractBetweenStartNotFound)?.1
                        .split_once(get_str!(end  , task_state, StringSourceError)).ok_or(StringModificationError::ExtractBetweenEndNotFound  )?.0
                    ),
                    Cow::Owned(inner) => Cow::Owned(inner
                        .split_once(get_str!(start, task_state, StringSourceError)).ok_or(StringModificationError::ExtractBetweenStartNotFound)?.1
                        .split_once(get_str!(end  , task_state, StringSourceError)).ok_or(StringModificationError::ExtractBetweenEndNotFound  )?.0
                        .to_string()
                    )
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
                        cache      : task_state.cache
                    }
                )?;
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(StringModificationError::NotInCommonContext)?.string_modifications.get(get_str!(name, task_state, StringModificationError)).ok_or(StringModificationError::CommonCallArgStringModificationNotFound)?.apply(to, task_state)?,

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

            Self::Map {value, map} => if let Some(x) = map.get(value.get(task_state)?) {x.apply(to, task_state)?;},

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

            #[cfg(feature = "custom")]
            Self::Custom(function) => function(to, task_state)?
        };
        Ok(())
    }
}

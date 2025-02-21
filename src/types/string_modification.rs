//! Provides [`StringModification`] which provides an easy API for all the ways one might want to modify a [`String`].

use std::borrow::Cow;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC, AsciiSet};
#[expect(unused_imports, reason = "Used in a doc comment.")]
#[cfg(feature = "regex")]
use ::regex::Regex;
#[cfg(feature = "base64")]
use ::base64::prelude::*;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Various ways to modify a [`String`].
/// 
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum StringModification {
    /// Does nothing.
    None,
    /// Always returns the error [`StringModificationError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringModificationError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its application to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// If `matcher` passes, apply `modification`, otherwise apply `else_modification`.
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If either possible call to [`StringModification::apply`] returns an error, that error is returned.
    IfStringMatches {
        /// The [`StringMatcher`] that decides if `modification` or `else_modification` is used.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to use if `matcher` passes.
        modification: Box<Self>,
        /// The [`Self`] to use if `matcher` fails.
        /// 
        /// Defaults to [`None`].
        #[serde(default)]
        else_modification: Option<Box<Self>>
    },
    /// Effectively a [`Self::IfStringMatches`] where each subsequent link is put inside the previous link's [`Self::IfStringMatches::else_modification`].
    /// # Errors
    /// If a call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If a call to [`StringModification::apply`] returns an error, that error is returned.
    StringMatcherChain(Vec<StringMatcherChainLink>),
    /// Ignores any error the call to [`Self::apply`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
    /// 
    /// If `try` does not return an er
    /// # Errors
    /// If `else` returns an error, that error is returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If one of the calls to [`Self::apply`] return an error, the string is left unchanged and the error is returned.
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the string remains changed by the previous calls to [`Self::apply`] and the error is returned.
    /// 
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the calls to [`Self::apply`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the calls to [`Self::apply`] return an error, the string is left as whatever the previous contained mapper set it to and the error is returned.
    AllNoRevert(Vec<Self>),
    /// If any of the calls to [`Self::apply`] return an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// 
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    AllIgnoreError(Vec<Self>),
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If all calls to [`Self::apply`] return an error, returns the last error.
    FirstNotError(Vec<Self>),



    /// Replaces the entire target string to the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::Set("ghi".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "ghi");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Set(StringSource),
    /// Append the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::Append("ghi".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "abcdefghi");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Append(StringSource),
    /// Prepend the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::Prepend("ghi".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "ghiabcdef");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Prepend(StringSource),
    /// Replace all instances of `find` with `replace`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcabc".to_string();
    /// StringModification::Replace{find: "ab".into(), replace: "xy".into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "xycxyc");
    /// ```
    Replace {
        /// The value to look for.
        find: StringSource,
        /// The value to replace with.
        replace: StringSource
    },
    /// Replace the specified range with `replace`.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::ReplaceRange{start: Some( 6), end: Some( 7), replace: "123" .into()}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// assert_eq!(&x, "abcdef");
    /// StringModification::ReplaceRange{start: Some( 1), end: Some( 4), replace: "ab"  .into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "aabef");
    /// StringModification::ReplaceRange{start: Some(-3), end: Some(-1), replace: "abcd".into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "aaabcdf");
    /// StringModification::ReplaceRange{start: Some(-3), end: None    , replace: "efg" .into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "aaabefg");
    /// StringModification::ReplaceRange{start: Some(-8), end: None    , replace: "hij" .into()}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// assert_eq!(&x, "aaabefg");
    /// ```
    ReplaceRange {
        /// The start of the range to replace.
        start: Option<isize>,
        /// The end of the range to replace.
        end: Option<isize>,
        /// The value to replace the range with.
        replace: StringSource
    },
    /// [`str::to_lowercase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "ABCdef".to_string();
    /// StringModification::Lowercase.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "abcdef");
    /// ```
    Lowercase,
    /// [`str::to_uppercase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcDEF".to_string();
    /// StringModification::Uppercase.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "ABCDEF");
    /// ```
    Uppercase,
    /// Mimics [`str::strip_prefix`] using [`str::starts_with`] and [`String::drain`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't begin with the specified prefix, returns the error [`StringModificationError::PrefixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripPrefix("abc".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripPrefix("abc".into()).apply(&mut x, &job_state.to_view()).unwrap_err();
    /// assert_eq!(&x, "def");
    /// ```
    StripPrefix(StringSource),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't end with the specified suffix, returns the error [`StringModificationError::SuffixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripSuffix("def".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripSuffix("def".into()).apply(&mut x, &job_state.to_view()).unwrap_err();
    /// assert_eq!(&x, "abc");
    /// ```
    StripSuffix(StringSource),
    /// [`Self::StripPrefix`] but does nothing if the target string doesn't begin with the specified prefix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybePrefix("abc".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripMaybePrefix("abc".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "def");
    /// ```
    StripMaybePrefix(StringSource),
    /// [`Self::StripSuffix`] but does nothing if the target string doesn't end with the specified suffix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybeSuffix("def".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripMaybeSuffix("def".into()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "abc");
    /// ```
    StripMaybeSuffix(StringSource),
    /// [`str::replacen`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "aaaaa".to_string();
    /// StringModification::Replacen{find: "a" .into(), replace: "x".into(), count: 2}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "xxaaa");
    /// StringModification::Replacen{find: "xa".into(), replace: "x".into(), count: 2}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "xxaa");
    /// ```
    Replacen {
        /// The value to look for.
        find: StringSource,
        /// The value to replace with.
        replace: StringSource,
        /// The number of times to do the replacement.
        count: usize
    },
    /// [`String::insert_str`].
    /// # Errors
    /// If `where` is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abc".to_string();
    /// StringModification::Insert{r#where:  0, value: "def".into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "defabc");
    /// StringModification::Insert{r#where:  2, value: "ghi".into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "deghifabc");
    /// StringModification::Insert{r#where: -1, value: "jhk".into()}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "deghifabjhkc");
    /// ```
    Insert {
        /// The location to insert `value`.
        r#where: isize,
        /// The string to insert.
        value: StringSource
    },
    /// [`String::remove`].
    /// # Errors
    /// If the specified index is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdef".to_string();
    /// StringModification::Remove( 1).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "acdef");
    /// StringModification::Remove(-1).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "acde");
    /// ```
    Remove(isize),
    /// Discards everything outside the specified range.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abcdefghi".into();
    /// StringModification::KeepRange{start: Some( 1), end: Some( 8)}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "bcdefgh");
    /// StringModification::KeepRange{start: None    , end: Some( 6)}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "bcdefg");
    /// StringModification::KeepRange{start: Some(-3), end: None    }.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "efg");
    /// StringModification::KeepRange{start: Some(-3), end: Some(-1)}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "ef");
    /// ```
    KeepRange {
        /// The start of the range to keep.
        start: Option<isize>,
        /// The end of the range to keep.
        end: Option<isize>
    },
    /// [`Regex::captures`] and [`::regex::Captures::expand`].
    /// # Errors
    /// When the call to [`Regex::captures`] returns [`None`], returns the error [`StringModificationError::RegexMatchNotFound`]
    #[cfg(feature = "regex")]
    RegexCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`::regex::Captures::expand`] with.
        replace: StringSource
    },
    /// [`Regex::captures_iter`] and [`::regex::Captures::expand`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::*;
    /// # use std::str::FromStr;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "...a2..a3....a4".to_string();
    /// StringModification::JoinAllRegexCaptures {
    ///     regex: RegexWrapper::from_str(r"a(\d)").unwrap(),
    ///     replace: "A$1$1".into(),
    ///     join: "---".into()
    /// }.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "A22---A33---A44");
    /// ```
    /// # Errors
    /// If the call to [`RegexWrapper::get_regex`] returns an error, that error is returned,
    #[cfg(feature = "regex")]
    JoinAllRegexCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`::regex::Captures::expand`] with.
        replace: StringSource,
        /// The value to join the captures with. Does not default to anything.
        join: StringSource
    },
    /// [`Regex::find`].
    /// # Errors
    /// When the call to [`Regex::find`] returns [`None`], returns the error [`StringModificationError::RegexMatchNotFound`]
    #[cfg(feature = "regex")]
    RegexFind(RegexWrapper),
    /// [`Regex::replace`]
    /// Please note that this only does one replacement. See [`Self::RegexReplaceAll`] and [`Self::RegexReplacen`] for alternatives.
    #[cfg(feature = "regex")]
    RegexReplace {
        /// The regex to do replacement with.
        regex: RegexWrapper,
        /// The replacement string.
        replace: StringSource
    },
    /// [`Regex::replace_all`]
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        /// The regex to do replacement with.
        regex: RegexWrapper,
        /// The replacement string.
        replace: StringSource
    },
    /// [`Regex::replacen`]
    #[cfg(feature = "regex")]
    RegexReplacen {
        /// The regex to do replacement with.
        regex: RegexWrapper,
        /// The number of replacements to do.
        n: usize,
        /// The replacement string.
        replace: StringSource
    },
    /// Choose which string modification to apply based on of a flag is set.
    IfFlag {
        /// The flag to check the setness of.
        flag: StringSource,
        /// The string modification to apply if the flag is set.
        then: Box<Self>,
        /// The string modification to apply if the flag is not set.
        r#else: Box<Self>
    },
    /// Uses [`percent_encoding::utf8_percent_encode`] to percent encode all bytes that are not alphanumeric ASCII.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "a/b/c".to_string();
    /// StringModification::UrlEncode(Default::default()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a%2Fb%2Fc");
    /// ```
    UrlEncode(UrlEncodeAlphabet),
    /// [`percent_encoding::percent_decode_str`]
    /// # Errors
    /// If the call to [`percent_encoding::percent_decode_str`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "a%2fb%2Fc".to_string();
    /// StringModification::UrlDecode.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a/b/c");
    /// ```
    UrlDecode,
    /// Encode the string using [`::base64::prelude::BASE64_STANDARD`].
    #[cfg(feature = "base64")]
    Base64Encode(#[serde(default)] Base64Config),
    /// Decode the string using [`::base64::prelude::BASE64_STANDARD`].
    /// # Errors
    /// If the call to [`::base64::engine::Engine::decode`] returns an error, that error is returned.
    /// 
    /// If the call to [`String::from_utf8`] returns an error, that error is returned.
    #[cfg(feature = "base64")]
    Base64Decode(#[serde(default)] Base64Config),
    /// [`serde_json::Value::pointer`].
    /// Does not do any string conversions. I should probably add an option for that.
    /// # Errors
    /// If the pointer doesn't point to anything, returns the error [`StringModificationError::JsonValueNotFound`].
    /// 
    /// If the pointer points to a non-string value, returns the error [`StringModificationError::JsonValueIsNotAString`].
    JsonPointer(StringSource),
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abc xyz".to_string();
    /// StringModification::ModifyNthSegment {
    ///     split: " ".into(),
    ///     n: 1,
    ///     modification: Box::new(StringModification::Set("abc".into()))
    /// }.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "abc abc");
    /// ```
    /// # Errors
    /// If the segment is not found, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the modification returns an error, that error is returned.
    ModifyNthSegment {
        /// The value to split the siring by.
        split: StringSource,
        /// The index of the segment to modify.
        n: isize,
        /// The modification to apply.
        modification: Box<Self>
    },
    /// Splits the provided string by `split` and keeps only the `n`th segment.
    /// # Errors
    /// If the `n`th segment is not found, returns the error [`StringModificationError::SegmentNotFound`].
    KeepNthSegment {
        /// The value to split the string by.
        split: StringSource,
        /// The index of the segment to keep.
        n: isize
    },
    /// Splits the provided string by `split` and keeps only the segments in the specified range.
    /// # Errors
    /// If the segment range is not found, returns the error [`StringModificationError::SegmentRangeNotFound`].
    KeepSegmentRange {
        /// The value to split the string by.
        split: StringSource,
        /// The start of the range of segments to keep.
        start: Option<isize>,
        /// The end of the range of segments to keep.
        end: Option<isize>
    },
    /// Splits the provided string by `split`, replaces the `n`th segment with `value` or removes the segment if `value` is `None`, then joins the segments back together.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "a.b.c.d.e.f".to_string();
    /// StringModification::SetNthSegment{split: ".".into(), n:  1, value: Some( "1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.f");
    /// StringModification::SetNthSegment{split: ".".into(), n: -1, value: Some("-1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.-1");
    /// StringModification::SetNthSegment{split: ".".into(), n: -2, value: None}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.-1");
    /// StringModification::SetNthSegment{split: ".".into(), n:  5, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".into(), n: -6, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".into(), n: -5, value: Some("-5".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "-5.1.c.d.-1");
    /// ```
    SetNthSegment {
        /// The value to split the string by.
        split: StringSource,
        /// The index of the segment to modify.
        n: isize,
        /// The value to set. If `None` then the segment is removed.
        value: Option<StringSource>
    },
    /// Finds the `n`th segment matching `matcher` and sets it to `value`.
    /// # Errors
    /// If `n` is not in the range of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    SetNthMatchingSegment {
        /// The value to split the siring by.
        split: StringSource,
        /// The index of the segments to modify.
        n: isize,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>,
        /// The value to set. If `None` then the segment is removed.
        value: Option<StringSource>
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let modification = StringModification::SetAroundNthMatchingSegment {
    ///     split: " ".into(),
    ///     n: 0,
    ///     matcher: Box::new(StringMatcher::Equals("b".into())),
    ///     shift: 1,
    ///     value: None
    /// };
    /// 
    /// let mut x = "a b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "a b");
    /// 
    /// let mut x = "a b".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// 
    /// 
    /// 
    /// let modification = StringModification::SetAroundNthMatchingSegment {
    ///     split: " ".into(),
    ///     n: 0,
    ///     matcher: Box::new(StringMatcher::Equals("b".into())),
    ///     shift: -1,
    ///     value: None
    /// };
    /// 
    /// let mut x = "a b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "b c");
    /// 
    /// let mut x = "b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// ```
    SetAroundNthMatchingSegment {
        /// The value to split the siring by.
        split: StringSource,
        /// The index of the segments to modify.
        n: isize,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>,
        /// The offset of the segment to set.
        shift: isize,
        /// The value to set. If `None` then the segment is removed.
        value: Option<StringSource>
    },
    /// Splits the provided string by `split`, replaces the range of segments specified by `start` and `end` with `value`,  then joins the segments back together.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If there is no segment at `start` (or `0` if `start` is [`None`]), returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the segment range is not found, returns the error [`StringModificationError::SegmentRangeNotFound`].
    SetSegmentRange {
        /// The value to split the string by.
        split: StringSource,
        /// The start of the range of segments to replace.
        start: Option<isize>,
        /// The end of the range of segments to replace.
        end: Option<isize>,
        /// The value to replace the segments with.
        value: Option<StringSource>
    },
    /// Like [`Self::SetNthSegment`] except it inserts `value` before the `n`th segment instead of overwriting.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "a.b.c".to_string();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  1, value: Some( "1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.b.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -1, value: Some("-1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  4, value: Some( "4".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.4.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  6, value: Some( "6".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.4.c.6");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  8, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -8, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -7, value: Some("-7".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "-7.a.1.b.-1.4.c.6");
    /// ```
    InsertSegmentBefore {
        /// The value to split the string by.
        split: StringSource,
        /// The segment index to insert before.
        n: isize,
        /// The value to insert.
        value: Option<StringSource>
    },
    /// Like [`Self::SetNthSegment`] except it inserts `value` after the `n`th segment instead of overwriting.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "a.b.c".to_string();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  1, value: Some( "1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.b.1.c");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -1, value: Some("-1".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.b.1.c.-1");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  4, value: Some( "4".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.b.1.c.-1.4");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  6, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -7, value: Some( "E".into())}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -6, value: Some("-6".into())}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(&x, "a.-6.b.1.c.-1.4");
    /// ```
    InsertSegmentAfter {
        /// The value to split the string by.
        split: StringSource,
        /// The segment index to insert before.
        n: isize,
        /// The value to insert.
        value: Option<StringSource>
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abc def xyz".to_string();
    /// StringModification::ModifySegments {
    ///     split: " ".into(),
    ///     ns: vec![1, 2],
    ///     modification: Box::new(StringModification::Set("a b c".into()))
    /// }.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "abc a b c a b c");
    /// ```
    /// # Errors
    /// If a segment is not found, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the modification returns an error, that error is returned.
    ModifySegments {
        /// The value to split the siring by.
        split: StringSource,
        /// The indices of the segments to modify.
        ns: Vec<isize>,
        /// The modification to apply.
        modification: Box<Self>
    },
    /// Modifies the `n`th segment that matches `matcher`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "aaa aaaa aaaa".to_string();
    /// StringModification::ModifyNthMatchingSegment {
    ///     split: " ".into(),
    ///     n: 1,
    ///     matcher: Box::new(StringMatcher::LengthIs(4)),
    ///     modification: Box::new(StringModification::Set("zzzz".into()))
    /// }.apply(&mut x, &job_state.to_view()).unwrap();
    /// 
    /// assert_eq!(x, "aaa aaaa zzzz");
    /// ```
    ModifyNthMatchingSegment {
        /// The value to split the siring by.
        split: StringSource,
        /// The index of the segments to modify.
        n: isize,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to apply.
        modification: Box<Self>
    },
    /// For each `n` in `ns`, modifies the `n`th segment that matches `matcher`.
    ModifyMatchingSegments {
        /// The value to split the siring by.
        split: StringSource,
        /// The indices of the segments to modify.
        ns: Vec<isize>,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to apply.
        modification: Box<Self>
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let modification = StringModification::ModifyAroundNthMatchingSegment {
    ///     split: " ".into(),
    ///     n: 0,
    ///     matcher: Box::new(StringMatcher::Equals("b".into())),
    ///     shift: 1,
    ///     modification: Box::new(StringModification::Set("-".into()))
    /// };
    /// 
    /// let mut x = "a b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "a b -");
    /// 
    /// let mut x = "a b".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// 
    /// 
    /// 
    /// let modification = StringModification::ModifyAroundNthMatchingSegment {
    ///     split: " ".into(),
    ///     n: 0,
    ///     matcher: Box::new(StringMatcher::Equals("b".into())),
    ///     shift: -1,
    ///     modification: Box::new(StringModification::Set("-".into()))
    /// };
    /// 
    /// let mut x = "a b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "- b c");
    /// 
    /// let mut x = "b c".to_string();
    /// modification.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// ```
    ModifyAroundNthMatchingSegment {
        /// The value to split the siring by.
        split: StringSource,
        /// The index of the segments to modify.
        n: isize,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>,
        /// The offset of the segment to modify.
        shift: isize,
        /// The value to set. If `None` then the segment is removed.
        modification: Box<Self>
    },
    /// Finds matching segments and removes.
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "not_utp=true&utm_location=fragment".into();
    /// StringModification::RemoveMatchingSegments {
    ///     split: "&".into(),
    ///     matcher: Box::new(StringMatcher::Contains {value: "utm_".into(), r#where: StringLocation::Start})
    /// }.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "not_utp=true");
    /// ```
    RemoveMatchingSegments {
        /// The value to split the string by.
        split: StringSource,
        /// The [`StringMatcher`] to test each segment with.
        matcher: Box<StringMatcher>
    },
    /// [`Mapper::RemoveQueryParamsMatching`] but for strings because SOMEONE's been putting UTPs in their fragments.
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "not_utp=true&utm_location=fragment".into();
    /// StringModification::RemoveQueryParamsMatching(Box::new(StringMatcher::Contains {value: "utm_".into(), r#where: StringLocation::Start})).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "not_utp=true");
    /// ```
    RemoveQueryParamsMatching(Box<StringMatcher>),
    /// [`Mapper::AllowQueryParamsMatching`] but for strings because SOMEONE's been putting UTPs in their fragments.
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "this_param_is_important=true&utm_location=fragment&other_crap=2".into();
    /// StringModification::AllowQueryParamsMatching(Box::new(StringMatcher::Contains {value: "important".into(), r#where: StringLocation::Anywhere})).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "this_param_is_important=true");
    /// ```
    AllowQueryParamsMatching(Box<StringMatcher>),
    /// If the provided string is in the specified map, return the value of its corresponding [`StringSource`].
    /// # Errors
    /// If the provided string is not in the specified map, returns the error [`StringModificationError::StringNotInMap`].
    Map(HashMap<String, StringSource>),
    /// Extracts the substring of `source` found between the first `start` and the first subsequent `end`.
    /// 
    /// The same as [`StringSource::ExtractBetween`] but doesn't preserve borrowedness.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`StringModificationError::StringSourceIsNone`].
    /// 
    /// If `start` is not found in `source`, returns the error [`StringModificationError::ExtractBetweenStartNotFound`].
    /// 
    /// If `end` is not found in `source` after `start`, returns the error [`StringModificationError::ExtractBetweenEndNotFound`].
    ExtractBetween {
        /// The [`StringSource`] to look for before the substring.
        start: StringSource,
        /// The [`StringSource`] to look for after the substring.
        end: StringSource
    },
    /// Takes every [`char`] and maps it according to the specified map.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "abc".to_string();
    /// StringModification::MapChars {map: [('a', Some('A')), ('b', None)].into_iter().collect(), not_found_behavior: CharNotFoundBehavior::Nothing}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "Ac");
    /// 
    /// let mut x = "abc".to_string();
    /// StringModification::MapChars {map: [('a', Some('A')), ('b', None)].into_iter().collect(), not_found_behavior: CharNotFoundBehavior::Error}.apply(&mut x, &job_state.to_view()).unwrap_err();
    /// assert_eq!(x, "abc");
    /// let mut x = "abc".to_string();
    /// StringModification::MapChars {map: [('a', Some('A')), ('b', None), ('c', Some('c'))].into_iter().collect(), not_found_behavior: CharNotFoundBehavior::Error}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "Ac");
    /// 
    /// let mut x = "abc".to_string();
    /// StringModification::MapChars {map: [('a', Some('A')), ('b', None)].into_iter().collect(), not_found_behavior: CharNotFoundBehavior::Replace(None)}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "A");
    /// let mut x = "abc".to_string();
    /// StringModification::MapChars {map: [('a', Some('A')), ('b', None)].into_iter().collect(), not_found_behavior: CharNotFoundBehavior::Replace(Some('?'))}.apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "A?");
    /// ````
    MapChars {
        /// The map to map [`char`]s by
        map: HashMap<char, Option<char>>,
        /// What do do when a [`char`] that isn't in `map` is found.
        not_found_behavior: CharNotFoundBehavior
    },
    /// Be careful to make sure no element key is a prefix of any other element key.
    /// 
    /// The current implementation sucks and can't handle that.
    /// # Tests
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// let mut x = "\\/a\\n\\\\n".to_string();
    /// StringModification::RunEscapeCodes([
    ///     ("\\/" .to_string(), "/" .to_string()),
    ///     ("\\\\".to_string(), "\\".to_string()),
    ///     ("\\n" .to_string(), "\n".to_string())
    /// ].into_iter().collect()).apply(&mut x, &job_state.to_view()).unwrap();
    /// assert_eq!(x, "/a\n\\n");
    /// ```
    RunEscapeCodes(HashMap<String, String>),
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::string_modifications`].
    Common(CommonCall),
    /// Uses a function pointer.
    /// 
    /// Cannot be serialized or deserialized.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    Custom(FnWrapper<fn(&mut String, &JobStateView) -> Result<(), StringModificationError>>)
}

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent#description
const JS_ENCODE_URI_COMPONENT_ASCII_SET: AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'-').remove(b'_').remove(b'.')
    .remove(b'!').remove(b'~').remove(b'*')
    .remove(b'\'').remove(b'(').remove(b')');
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI#description
const JS_ENCODE_URI_ASCII_SET: AsciiSet = JS_ENCODE_URI_COMPONENT_ASCII_SET
    .remove(b';').remove(b'/').remove(b'?')
    .remove(b':').remove(b'@').remove(b'&')
    .remove(b'=').remove(b'+').remove(b'$')
    .remove(b',').remove(b'#');

/// Alphabets for [`StringModification::UrlEncode`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrlEncodeAlphabet {
    /// The alphabet defined by JavaScript's [`encodeURIComponent`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent#description)
    #[default]
    JsEncodeUriComponent,
    /// The alphabet defined by JavaScript's [`encodeURI`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI#description)
    JsEncodeUri,
    /// [`NON_ALPHANUMERIC`].
    NonAlphanumeric
}

impl UrlEncodeAlphabet {
    /// Gets the alphabet.
    ///
    /// For some reason, [`AsciiSet`], as of latest version when I write this (2.3.1), does not implment any non-auto/blanket traits.
    ///
    /// As I write this, there's a merged pull request for [`Debug`], [`PartialEq`], and [`Eq`] and an unmerged pull request for [`Clone`] and [`Copy`].
    pub fn get(&self) -> &'static AsciiSet {
        match self {
            Self::JsEncodeUriComponent => &JS_ENCODE_URI_COMPONENT_ASCII_SET,
            Self::JsEncodeUri          => &JS_ENCODE_URI_ASCII_SET,
            Self::NonAlphanumeric      => NON_ALPHANUMERIC
        }
    }
}

/// Tells [`StringModification::MapChars`] what to do when a [`char`] isn't found in the map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharNotFoundBehavior {
    /// Leave the [`char`] as-is.
    Nothing,
    /// Return an error.
    Error,
    /// Replace with the specified [`char`].
    Replace(Option<char>)
}

string_or_struct_magic!(StringModification);

/// Individual links in the [`StringModification::StringMatcherChain`] chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringMatcherChainLink {
    /// The [`StringMatcher`] to apply [`Self::modification`] under.
    matcher: StringMatcher,
    /// The [`StringModification`] to apply if [`Self::matcher`] is satisfied.
    modification: StringModification
}

/// Returned when trying to call [`StringModification::from_str`] with a variant name that has non-defaultable fields.
#[derive(Debug, Error)]
#[error("Tried deserializing a StringModification variant with non-defaultable fields from a string.")]
pub struct NonDefaultableVariant;

impl FromStr for StringModification {
    type Err = NonDefaultableVariant;

    /// Used for allowing deserializing [`Self::Base64Decode`] and [`Self::Base64Encode`] from strings using the default values for their fields.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            #[cfg(feature = "base64")] "Base64Decode" => StringModification::Base64Decode(Default::default()),
            #[cfg(feature = "base64")] "Base64Encode" => StringModification::Base64Encode(Default::default()),
            "UrlDecode" => StringModification::UrlDecode,
            "UrlEncode" => StringModification::UrlEncode(Default::default()),
            "None"      => StringModification::None,
            "Error"     => StringModification::Error,
            "Lowercase" => StringModification::Lowercase,
            "Uppercase" => StringModification::Uppercase,
            _           => Err(NonDefaultableVariant)?
        })
    }
}

/// The enum of all possible errors [`StringModification::apply`] can return.
#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// Returned when [`StringModification::Error`] is used.
    #[error("StringModification::Error was used.")]
    ExplicitError,
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`serde_json::Error`] is encountered.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when [`serde_json::Value::pointer`] returns [`None`].
    #[error("The requested JSON value was not found.")]
    JsonValueNotFound,
    /// Returned when [`serde_json::Value::pointer`] returns a value that is not a string.
    #[error("The requested JSON value was not a string.")]
    JsonValueIsNotAString,
    /// Returned when the requested slice is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundary or out of bounds.")]
    InvalidSlice,
    /// Returned when the requested index is either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// Returned when the requested segment is not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// Returned when the requested segments are not found.
    #[error("The requested segments were not found.")]
    SegmentRangeNotFound,
    /// Returned when the provided string does not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// Returned when the provided string does not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
    /// Returned when the requested regex pattern is not found in the provided string.
    #[error("The requested regex pattern was not found in the provided string.")]
    RegexMatchNotFound,
    /// Returned when a [`::regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    /// Returned when both the `try` and `else` of a [`StringModification::TryElse`] both return errors.
    #[error("A `StringModification::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`StringModification::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`StringModification::TryElse::else`],
        else_error: Box<Self>
    },
    /// Returned when a [`::base64::DecodeError`] is encountered.
    #[error(transparent)]
    #[cfg(feature = "base64")]
    Base64DecodeError(#[from] ::base64::DecodeError),
    /// Returned when a [`std::string::FromUtf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when the provided string is not in the specified map.
    #[error("The provided string was not in the specified map.")]
    StringNotInMap,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    /// Returned when a [`MakeBase64EngineError`] is encountered.
    #[error(transparent)]
    #[cfg(feature = "base64")]
    MakeBase64EngineError(#[from] MakeBase64EngineError),
    /// Returned when the `start` of a [`StringModification::ExtractBetween`] is not found in the provided string.
    #[error("The `start` of an `ExtractBetween` was not found in the provided string.")]
    ExtractBetweenStartNotFound,
    /// Returned when the `start` of a [`StringModification::ExtractBetween`] is not found in the provided string.
    #[error("The `end` of an `ExtractBetween` was not found in the provided string.")]
    ExtractBetweenEndNotFound,
    /// Returned by [`StringModification::MapChars`] when [`StringModification::MapChars::not_found_behavior`] is set to [`CharNotFoundBehavior::Error`] and a character not in [`StringModification::MapChars::map`] is encountered.
    #[error("Attempted to map a character not found in the mapping map.")]
    CharNotInMap,
    /// Returned when the common [`StringModification`] is not found.
    #[error("The common StringModification was not found.")]
    CommonStringModificationNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Custom error.
    #[error(transparent)]
    #[cfg(feature = "custom")]
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
    /// Apply the modification in-place using the provided [`Params`].
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, to: &mut String, job_state: &JobStateView) -> Result<(), StringModificationError> {
        debug!(StringModification::apply, self);
        match self {
            Self::None => {},
            Self::Error => Err(StringModificationError::ExplicitError)?,
            Self::Debug(modification) => {
                let to_before_mapper=to.clone();
                let modification_result=modification.apply(to, job_state);
                eprintln!("=== StringModification::Debug ===\nModification: {modification:?}\nJob state: {job_state:?}\nString before mapper: {to_before_mapper:?}\nModification return value: {modification_result:?}\nString after mapper: {to:?}");
                modification_result?;
            },
            Self::IfStringMatches {matcher, modification, else_modification} => if matcher.satisfied_by(to, job_state)? {
                modification.apply(to, job_state)?;
            } else if let Some(else_modification) = else_modification {
                else_modification.apply(to, job_state)?
            },
            Self::StringMatcherChain(chain) => for link in chain {
                if link.matcher.satisfied_by(to, job_state)? {
                    link.modification.apply(to, job_state)?;
                    break;
                }
            },
            Self::IgnoreError(modification) => {let _=modification.apply(to, job_state);},
            Self::TryElse{r#try, r#else} => r#try.apply(to, job_state).or_else(|try_error| r#else.apply(to, job_state).map_err(|else_error| StringModificationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::All(modifications) => {
                let mut temp_to=to.clone();
                for modification in modifications {
                    modification.apply(&mut temp_to, job_state)?;
                }
                *to=temp_to;
            }
            Self::AllNoRevert(modifications) => {
                for modification in modifications {
                    modification.apply(to, job_state)?;
                }
            },
            Self::AllIgnoreError(modifications) => {
                for modification in modifications {
                    let _=modification.apply(to, job_state);
                }
            },
            Self::FirstNotError(modifications) => {
                let mut error=Ok(());
                for modification in modifications {
                    error=modification.apply(to, job_state);
                    if error.is_ok() {break}
                }
                error?
            },
            Self::Set(value)                         => get_string!(value, job_state, StringModificationError).clone_into(to),
            Self::Append(value)                      => to.push_str(get_str!(value, job_state, StringModificationError)),
            Self::Prepend(value)                     => {let mut ret=get_string!(value, job_state, StringModificationError); ret.push_str(to); *to=ret;},
            Self::Replace{find, replace}             => *to=to.replace (get_str!(find, job_state, StringModificationError), get_str!(replace, job_state, StringModificationError)),
            Self::Replacen{find, replace, count}     => *to=to.replacen(get_str!(find, job_state, StringModificationError), get_str!(replace, job_state, StringModificationError), *count),
            Self::ReplaceRange{start, end, replace}  => {
                let range=neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, get_str!(replace, job_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidSlice)?;
                }
            },
            Self::Lowercase                          => *to=to.to_lowercase(),
            Self::Uppercase                          => *to=to.to_uppercase(),
            Self::StripPrefix(prefix)                => {
                let prefix = get_str!(prefix, job_state, StringModificationError);
                if to.starts_with(prefix) {to.drain(..prefix.len());} else {Err(StringModificationError::PrefixNotFound)?;};
            },
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripSuffix(suffix)                => {
                let suffix = get_str!(suffix, job_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;};
            },
            Self::StripMaybePrefix(prefix)           => {
                let prefix = get_str!(prefix, job_state, StringModificationError);
                if to.starts_with(prefix) {to.drain(..prefix.len());};
            },
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripMaybeSuffix(suffix)           => {
                let suffix = get_str!(suffix, job_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len());};
            },
            Self::Insert{r#where, value}               => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.insert_str(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?, get_str!(value, job_state, StringModificationError));} else {Err(StringModificationError::InvalidIndex)?;},
            Self::Remove(r#where)                      => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.remove    (neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?                                                     );} else {Err(StringModificationError::InvalidIndex)?;},
            Self::KeepRange{start, end}                => *to = to.get(neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
            #[cfg(feature = "regex")]
            Self::RegexCaptures {regex, replace} => {
                let replace = get_str!(replace, job_state, StringModificationError);
                let mut temp = "".to_string();
                regex.get_regex()?.captures(to).ok_or(StringModificationError::RegexMatchNotFound)?.expand(replace, &mut temp);
                *to = temp;
            },
            #[cfg(feature = "regex")]
            Self::JoinAllRegexCaptures {regex, replace, join} => {
                let replace = get_str!(replace, job_state, StringModificationError);
                let join = get_str!(join, job_state, StringModificationError);
                let mut temp = "".to_string();
                if join.is_empty() {
                    for captures in regex.get_regex()?.captures_iter(to) {
                        captures.expand(replace, &mut temp);
                    }
                } else {
                    let mut iter = regex.get_regex()?.captures_iter(to).peekable();
                    while let Some(captures) = iter.next() {
                        captures.expand(replace, &mut temp);
                        if iter.peek().is_some() {temp.push_str(join);}
                    }
                }
                *to = temp;
            },
            #[cfg(feature = "regex")] Self::RegexFind       (regex            ) => *to = regex.get_regex()?.find       (to                                                           ).ok_or(StringModificationError::RegexMatchNotFound)?.as_str().to_string(),
            #[cfg(feature = "regex")] Self::RegexReplace    {regex,    replace} => *to = regex.get_regex()?.replace    (to,     get_str!(replace, job_state, StringModificationError)).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplaceAll {regex,    replace} => *to = regex.get_regex()?.replace_all(to,     get_str!(replace, job_state, StringModificationError)).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplacen   {regex, n, replace} => *to = regex.get_regex()?.replacen   (to, *n, get_str!(replace, job_state, StringModificationError)).into_owned(),
            Self::IfFlag {flag, then, r#else} => if job_state.params.flags.contains(get_str!(flag, job_state, StringModificationError)) {then} else {r#else}.apply(to, job_state)?,
            Self::UrlEncode(alphabet) => *to=utf8_percent_encode(to, alphabet.get()).to_string(),
            Self::UrlDecode => *to=percent_decode_str(to).decode_utf8()?.into_owned(),
            #[cfg(feature = "base64")] Self::Base64Encode(config) => *to = config.make_engine()?.encode(to.as_bytes()),
            #[cfg(feature = "base64")] Self::Base64Decode(config) => *to = String::from_utf8(config.make_engine()?.decode(to.as_bytes())?)?,
            Self::JsonPointer(pointer) => *to = serde_json::from_str::<serde_json::Value>(to)?.pointer(get_str!(pointer, job_state, StringModificationError)).ok_or(StringModificationError::JsonValueNotFound)?.as_str().ok_or(StringModificationError::JsonValueIsNotAString)?.to_string(),



            Self::KeepNthSegment   {split, n}          => *to = neg_nth(to.split(get_str!(split, job_state, StringModificationError)), *n).ok_or(StringModificationError::SegmentNotFound)?.to_string(),
            Self::KeepSegmentRange {split, start, end} => {
                let split = get_str!(split, job_state, StringModificationError);
                *to = neg_vec_keep(to.split(split), *start, *end).ok_or(StringModificationError::SegmentRangeNotFound)?.join(split);
            },
            Self::SetNthSegment{split, n, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut temp=to.split(split).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let x = get_option_string!(value, job_state);
                #[expect(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
                match x.as_deref() {
                    Some(value) => temp[fixed_n]=value,
                    None        => {temp.remove(fixed_n);}
                }
                *to=temp.join(split);
            },
            Self::SetNthMatchingSegment {split, n, matcher, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let x = get_option_string!(value, job_state);
                let mut count = 0usize;
                let mut nth_match_found = false;
                for (index, segment) in segments.iter_mut().enumerate() {
                    if matcher.satisfied_by(segment, job_state)? {
                        if count == fixed_n {
                            match x.as_deref() {
                                #[expect(clippy::indexing_slicing, reason = "`count` is guaranteed to be in bounds.")]
                                Some(value) => segments[index] = value,
                                None        => if index < segments.len() {segments.remove(index);} else {Err(StringModificationError::SegmentNotFound)?}
                            }
                            nth_match_found = true;
                            break;
                        }
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {count += 1;}
                    }
                }
                if !nth_match_found {Err(StringModificationError::SegmentNotFound)?;}
                *to=segments.join(split);
            },
            Self::SetAroundNthMatchingSegment {split, n, matcher, value, shift} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let mut matched = 0usize;
                let mut didnt_match = 0usize;
                let mut nth_match_found = false;
                for segment in segments.iter() {
                    if matcher.satisfied_by(segment, job_state)? {
                        if matched == fixed_n {
                            nth_match_found = true;
                            break;
                        }
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {matched += 1;}
                    } else {
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {didnt_match += 1;}
                    }
                }
                if !nth_match_found {Err(StringModificationError::SegmentNotFound)?;}
                #[allow(clippy::arithmetic_side_effects, reason = "The length of a vector is at most isize::MAX so `matched + didnt_match` is always a valid isize.")]
                let shifted_n = ((matched + didnt_match) as isize).checked_add(*shift).ok_or(StringModificationError::SegmentNotFound)?.try_into().map_err(|_| StringModificationError::SegmentNotFound)?;
                match get_option_cow!(value, job_state) {
                    Some(value) => *segments.get_mut(shifted_n).ok_or(StringModificationError::SegmentNotFound)? = value,
                    None        => if shifted_n >= segments.len() {Err(StringModificationError::SegmentNotFound)?} else {segments.remove(shifted_n);}
                }
                *to = segments.join(split);
            },
            Self::SetSegmentRange {split, start, end, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).collect::<Vec<_>>();
                let fixed_n = neg_index(start.unwrap_or(0), segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let _ = segments.drain(neg_range(*start, *end, segments.len()).ok_or(StringModificationError::SegmentRangeNotFound)?).collect::<Vec<_>>();
                let x = get_option_string!(value, job_state);
                if let Some(x) = &x {segments.insert(fixed_n, x);}
                *to = segments.join(split);
            }
            Self::InsertSegmentBefore{split, n, value} => {
                if let Some(new_segment) = get_option_str!(value, job_state) {
                    let split = get_str!(split, job_state, StringModificationError);
                    let mut temp=to.split(split).collect::<Vec<_>>();
                    let fixed_n=neg_range_boundary(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                    temp.insert(fixed_n, new_segment);
                    *to=temp.join(split);
                }
            },
            Self::InsertSegmentAfter{split, n, value} => {
                if let Some(new_segment) = get_option_str!(value, job_state) {
                    let split = get_str!(split, job_state, StringModificationError);
                    let mut temp=to.split(split).collect::<Vec<_>>();
                    let fixed_n=neg_shifted_range_boundary(*n, temp.len(), 1).ok_or(StringModificationError::SegmentNotFound)?;
                    temp.insert(fixed_n, new_segment);
                    *to=temp.join(split);
                }
            },
            #[expect(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
            Self::ModifyNthSegment {split, n, modification} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).collect::<Vec<_>>();
                let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let mut temp = segments[fixed_n].to_string();
                modification.apply(&mut temp, job_state)?;
                segments[fixed_n] = &*temp;
                *to = segments.join(split);
            },
            #[expect(clippy::indexing_slicing, reason = "`fixed_n` is guaranteed to be in bounds.")]
            Self::ModifySegments {split, ns, modification} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).map(Cow::Borrowed).collect::<Vec<_>>();
                for n in ns {
                    let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                    let mut temp = segments[fixed_n].to_string();
                    modification.apply(&mut temp, job_state)?;
                    segments[fixed_n] = Cow::Owned(temp);
                }
                *to = segments.join(split);
            },
            Self::ModifyNthMatchingSegment {split, n, matcher, modification} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let mut count = 0usize;
                for segment in segments.iter_mut() {
                    if matcher.satisfied_by(segment, job_state)? {
                        if count == fixed_n {
                            let mut temp = segment.to_string();
                            modification.apply(&mut temp, job_state)?;
                            *segment = Cow::Owned(temp);
                            break;
                        }
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {count += 1;}
                    }
                }
                *to = segments.join(split);
            },
            Self::ModifyMatchingSegments {split, ns, matcher, modification} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).map(Cow::Borrowed).collect::<Vec<_>>();
                let mut count = 0usize;
                let fixed_ns = ns.iter().map(|n| neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)).collect::<Result<Vec<_>, _>>()?;
                for segment in segments.iter_mut() {
                    if matcher.satisfied_by(segment, job_state)? {
                        if fixed_ns.iter().any(|x| *x==count) {
                            let mut temp = segment.to_string();
                            modification.apply(&mut temp, job_state)?;
                            *segment = Cow::Owned(temp);
                        }
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {count += 1;}
                    }
                }
                *to = segments.join(split);
            },
            Self::ModifyAroundNthMatchingSegment {split, n, matcher, modification, shift} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let mut matched = 0usize;
                let mut didnt_match = 0usize;
                let mut nth_match_found = false;
                for segment in segments.iter() {
                    if matcher.satisfied_by(segment, job_state)? {
                        if matched == fixed_n {
                            nth_match_found = true;
                            break;
                        }
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {matched += 1;}
                    } else {
                        #[allow(clippy::arithmetic_side_effects, reason = "Never exceeds `segments.len()`")]
                        {didnt_match += 1;}
                    }
                }
                if !nth_match_found {Err(StringModificationError::SegmentNotFound)?;}
                #[allow(clippy::arithmetic_side_effects, reason = "The length of a vector is at most isize::MAX so `matched + didnt_match` is always a valid isize.")]
                let shifted_n: usize = ((matched + didnt_match) as isize).checked_add(*shift).ok_or(StringModificationError::SegmentNotFound)?.try_into().map_err(|_| StringModificationError::SegmentNotFound)?;
                let segment = segments.get_mut(shifted_n).ok_or(StringModificationError::SegmentNotFound)?.to_mut();
                modification.apply(segment, job_state)?;
                *to = segments.join(split);
            },
            Self::RemoveMatchingSegments {split, matcher} => {
                let split = get_str!(split, job_state, StringModificationError);
                *to = to.split(split).filter_map(|segment| matcher.satisfied_by(segment, job_state).map(|x| (!x).then_some(segment)).transpose()).collect::<Result<Vec<_>, _>>()?.join(split);
            },
            Self::RemoveQueryParamsMatching(matcher) => *to = to.split('&').filter_map(|kev|
                matcher.satisfied_by(
                    kev.split('=').next().unwrap_or("Why can't I #[allow] an .expect() here?"),
                    job_state
                )
                .map(|x| (!x).then_some(kev))
                .transpose()
            ).collect::<Result<Vec<_>, _>>()?.join("&"),
            Self::AllowQueryParamsMatching(matcher) => *to = to.split('&').filter_map(|kev|
                matcher.satisfied_by(
                    kev.split('=').next().unwrap_or("Why can't I #[allow] an .expect() here?"),
                    job_state
                )
                .map(|x| x.then_some(kev))
                .transpose()
            ).collect::<Result<Vec<_>, _>>()?.join("&"),



            Self::Map(map) => *to = get_string!(map.get(to).ok_or(StringModificationError::StringNotInMap)?, job_state, StringModificationError),
            Self::ExtractBetween {start, end} => {
                *to = to
                    .split_once(get_str!(start, job_state, StringModificationError))
                    .ok_or(StringModificationError::ExtractBetweenStartNotFound)?
                    .1
                    .split_once(get_str!(end, job_state, StringModificationError))
                    .ok_or(StringModificationError::ExtractBetweenEndNotFound)?
                    .0
                    .to_string()
            },
            Self::MapChars {map, not_found_behavior} => {
                *to = match not_found_behavior {
                    CharNotFoundBehavior::Nothing => to.chars().filter_map(|c| *map.get(&c).unwrap_or(&Some(c))).collect::<String>(),
                    CharNotFoundBehavior::Error   => to.chars().map(|c| map.get(&c)).filter_map(|x| match x {
                        // `Option<Option<T>>` should impl a `fn transpose(self) -> Self` that does this
                        Some(None) => None,
                        None => Some(None),
                        Some(Some(c)) => Some(Some(c))
                    }).collect::<Option<String>>().ok_or(StringModificationError::CharNotInMap)?,
                    CharNotFoundBehavior::Replace(replace) => to.chars().filter_map(|c| *map.get(&c).unwrap_or(replace)).collect::<String>()
                }
            },
            Self::Common(common_call) => {
                job_state.commons.string_modifications.get(get_str!(common_call.name, job_state, StringModificationError)).ok_or(StringModificationError::CommonStringModificationNotFound)?.apply(
                    to,
                    &JobStateView {
                        url: job_state.url,
                        context: job_state.context,
                        params: job_state.params,
                        scratchpad: job_state.scratchpad,
                        #[cfg(feature = "cache")]
                        cache: job_state.cache,
                        commons: job_state.commons,
                        common_args: Some(&common_call.args.make(job_state)?),
                        jobs_context: job_state.jobs_context
                    }
                )?
            },
            Self::RunEscapeCodes(map) => {
                let mut ret = String::new();
                let mut to_munch = &**to;
                'a: while !to_munch.is_empty() {
                    for (escape, replace) in map {
                        if let Some(tail) = to_munch.strip_prefix(escape) {
                            to_munch = tail;
                            ret.push_str(replace);
                            continue 'a;
                        }
                    }
                    let mut chars = to_munch.chars();
                    if let Some(next_char) = chars.next() {ret.push(next_char);}
                    to_munch = chars.as_str();
                }
                *to=ret;
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(to, job_state)?
        };
        Ok(())
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(match self {
            Self::IfStringMatches {matcher, modification, else_modification} => matcher.is_suitable_for_release(config) && modification.is_suitable_for_release(config) && else_modification.as_ref().is_none_or(|else_modification| else_modification.is_suitable_for_release(config)),
            Self::StringMatcherChain(chain) => chain.iter().all(|link| link.matcher.is_suitable_for_release(config) && link.modification.is_suitable_for_release(config)),
            Self::IgnoreError(modification) => modification.is_suitable_for_release(config),
            Self::All(modifications) => modifications.iter().all(|modification| modification.is_suitable_for_release(config)),
            Self::AllNoRevert(modifications) => modifications.iter().all(|modification| modification.is_suitable_for_release(config)),
            Self::AllIgnoreError(modifications) => modifications.iter().all(|modification| modification.is_suitable_for_release(config)),
            Self::FirstNotError(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::TryElse {r#try, r#else} => r#try.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::Set(source) => source.is_suitable_for_release(config),
            Self::Append(source) => source.is_suitable_for_release(config),
            Self::Prepend(source) => source.is_suitable_for_release(config),
            Self::Replace {find, replace} => find.is_suitable_for_release(config) && replace.is_suitable_for_release(config),
            Self::ReplaceRange {replace, ..} => replace.is_suitable_for_release(config),
            Self::StripPrefix(source) => source.is_suitable_for_release(config),
            Self::StripSuffix(source) => source.is_suitable_for_release(config),
            Self::StripMaybePrefix(source) => source.is_suitable_for_release(config),
            Self::StripMaybeSuffix(source) => source.is_suitable_for_release(config),
            Self::Replacen {find, replace, ..} => find.is_suitable_for_release(config) && replace.is_suitable_for_release(config),
            Self::Insert {value, ..} => value.is_suitable_for_release(config),
            #[cfg(feature = "regex")] Self::RegexCaptures {replace, ..} => replace.is_suitable_for_release(config),
            #[cfg(feature = "regex")] Self::JoinAllRegexCaptures {replace, join, ..} => replace.is_suitable_for_release(config) && join.is_suitable_for_release(config),
            #[cfg(feature = "regex")] Self::RegexReplace {replace, ..} => replace.is_suitable_for_release(config),
            #[cfg(feature = "regex")] Self::RegexReplaceAll {replace, ..} => replace.is_suitable_for_release(config),
            #[cfg(feature = "regex")] Self::RegexReplacen {replace, ..} => replace.is_suitable_for_release(config),
            Self::IfFlag {flag, then, r#else} => flag.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::JsonPointer(pointer) => pointer.is_suitable_for_release(config),
            Self::KeepNthSegment {split, ..} => split.is_suitable_for_release(config),
            Self::KeepSegmentRange {split, ..} => split.is_suitable_for_release(config),
            Self::SetNthSegment {split, value, ..} => split.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::SetNthMatchingSegment {split, matcher, value, ..} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::SetAroundNthMatchingSegment {split, matcher, value, ..} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::SetSegmentRange {split, value, ..} => split.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::InsertSegmentBefore {split, value, ..} => split.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::InsertSegmentAfter {split, value, ..} => split.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::ModifyNthSegment {split, modification, ..} => split.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::ModifySegments {split, modification, ..} => split.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::ModifyNthMatchingSegment {split, matcher, modification, ..} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::ModifyAroundNthMatchingSegment {split, matcher, modification, ..} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::RemoveMatchingSegments {split, matcher} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config),
            Self::RemoveQueryParamsMatching(matcher) => matcher.is_suitable_for_release(config),
            Self::AllowQueryParamsMatching(matcher) => matcher.is_suitable_for_release(config),
            Self::ModifyMatchingSegments {split, matcher, modification, ..} => split.is_suitable_for_release(config) && matcher.is_suitable_for_release(config) && modification.is_suitable_for_release(config),
            Self::Map(map) => map.iter().all(|(_, x)| x.is_suitable_for_release(config)),
            Self::Debug(_) => false,
            Self::None | Self::Error | Self::Lowercase | Self::Uppercase | Self::Remove(_) |
                Self::KeepRange {..} | Self::UrlEncode(_) | Self::UrlDecode | Self::RunEscapeCodes(_) => true,
            #[cfg(feature = "regex")]
            Self::RegexFind(_) => true,
            #[cfg(feature = "base64")]
            Self::Base64Encode(_) | Self::Base64Decode(_) => true,
            Self::ExtractBetween {start, end} => start.is_suitable_for_release(config) && end.is_suitable_for_release(config),
            Self::MapChars{..} => true,
            Self::Common(common_call) => common_call.is_suitable_for_release(config),
            #[cfg(feature = "custom")]
            Self::Custom(_) => false
        }, "Unsuitable StringModification detected: {self:?}");
        true
    }
}

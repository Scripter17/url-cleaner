//! Provides [`StringModification`] which provides an easy API for all the ways one might want to modify a [`String`].

use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
// Used just for documentation.
#[allow(unused_imports)]
#[cfg(feature = "regex")]
use regex::Regex;
use base64::prelude::*;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Various ways to modify a string.
/// 
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
pub enum StringModification {
    /// Does nothing.
    None,
    /// Always returns the error [`StringModificationError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`StringModificationError::ExplicitError`].
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its application to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),
    /// Ignores any error the contained [`Self`] may return.
    IgnoreError(Box<Self>),
    /// If `try` returns an error, `else` is applied.
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
    /// If one of the contained [`Self`]s returns an error, the string is left unchanged and the error is returned.
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the string remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the string is left as whatever the previous contained mapper set it to and the error is returned.
    AllNoRevert(Vec<Self>),
    /// If any of the contained [`Self`]s returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    AllIgnoreError(Vec<Self>),
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] errors, returns the last error.
    FirstNotError(Vec<Self>),



    /// Replaces the entire target string to the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Set("ghi".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "ghi");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Set(StringSource),
    /// Append the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Append("ghi".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "abcdefghi");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Append(StringSource),
    /// Prepend the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Prepend("ghi".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "ghiabcdef");
    /// ```
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    Prepend(StringSource),
    /// Replace all instances of `find` with `replace`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcabc".to_string();
    /// StringModification::Replace{find: "ab".into(), replace: "xy".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
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
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::ReplaceRange{start: Some( 6), end: Some( 7), replace: "123" .into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// assert_eq!(&x, "abcdef");
    /// StringModification::ReplaceRange{start: Some( 1), end: Some( 4), replace: "ab"  .into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "aabef");
    /// StringModification::ReplaceRange{start: Some(-3), end: Some(-1), replace: "abcd".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "aaabcdf");
    /// StringModification::ReplaceRange{start: Some(-3), end: None    , replace: "efg" .into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "aaabefg");
    /// StringModification::ReplaceRange{start: Some(-8), end: None    , replace: "hij" .into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
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
    /// # use url::Url;
    /// let mut x = "ABCdef".to_string();
    /// StringModification::Lowercase.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "abcdef");
    /// ```
    Lowercase,
    /// [`str::to_uppercase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcDEF".to_string();
    /// StringModification::Uppercase.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "ABCDEF");
    /// ```
    Uppercase,
    /// Mimics [`str::strip_prefix`] using [`str::starts_with`] and [`String::drain`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't begin with the specified prefix, returns the error [`StringModificationError::PrefixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripPrefix("abc".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripPrefix("abc".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// assert_eq!(&x, "def");
    /// ```
    StripPrefix(StringSource),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't end with the specified suffix, returns the error [`StringModificationError::SuffixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripSuffix("def".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripSuffix("def".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// assert_eq!(&x, "abc");
    /// ```
    StripSuffix(StringSource),
    /// [`Self::StripPrefix`] but does nothing if the target string doesn't begin with the specified prefix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybePrefix("abc".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripMaybePrefix("abc".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "def");
    /// ```
    StripMaybePrefix(StringSource),
    /// [`Self::StripSuffix`] but does nothing if the target string doesn't end with the specified suffix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybeSuffix("def".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripMaybeSuffix("def".into()).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "abc");
    /// ```
    StripMaybeSuffix(StringSource),
    /// [`str::replacen`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "aaaaa".to_string();
    /// StringModification::Replacen{find: "a" .into(), replace: "x".into(), count: 2}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "xxaaa");
    /// StringModification::Replacen{find: "xa".into(), replace: "x".into(), count: 2}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
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
    /// # use url::Url;
    /// let mut x = "abc".to_string();
    /// StringModification::Insert{r#where:  0, value: "def".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "defabc");
    /// StringModification::Insert{r#where:  2, value: "ghi".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "deghifabc");
    /// StringModification::Insert{r#where: -1, value: "jhk".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
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
    /// # use url::Url;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Remove( 1).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "acdef");
    /// StringModification::Remove(-1).apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "acde");
    /// ```
    Remove(isize),
    /// Discards everything outside the specified range.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abcdefghi".into();
    /// StringModification::KeepRange{start: Some( 1), end: Some( 8)}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "bcdefgh");
    /// StringModification::KeepRange{start: None    , end: Some( 6)}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "bcdefg");
    /// StringModification::KeepRange{start: Some(-3), end: None    }.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "efg");
    /// StringModification::KeepRange{start: Some(-3), end: Some(-1)}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "ef");
    /// ```
    KeepRange {
        /// The start of the range to keep.
        start: Option<isize>,
        /// The end of the range to keep.
        end: Option<isize>
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
    /// If the segemnt range is not found, returns the error [`StringModificationError::SegmentRangeNotFound`].
    KeepSegmentRange {
        /// The value to split the string by.
        split: StringSource,
        /// The start of the range of segments to keep.
        start: Option<isize>,
        /// The end of the range of segments to keep.
        end: Option<isize>
    },
    /// Splits the provided string by `split`, replaces the `n`th segment with `value` or removes the segment if `value` is `None`, then joins the string back together.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "a.b.c.d.e.f".to_string();
    /// StringModification::SetNthSegment{split: ".".into(), n:  1, value: Some( "1".into())}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.f");
    /// StringModification::SetNthSegment{split: ".".into(), n: -1, value: Some("-1".into())}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.-1");
    /// StringModification::SetNthSegment{split: ".".into(), n: -2, value: None}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.c.d.-1");
    /// StringModification::SetNthSegment{split: ".".into(), n:  5, value: Some( "E".into())}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".into(), n: -6, value: Some( "E".into())}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".into(), n: -5, value: Some("-5".into())}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "-5.1.c.d.-1");
    /// ```
    SetNthSegment {
        /// The value to split the string by.
        split: StringSource,
        /// The index of the segment to modify.
        n: isize,
        /// The value to place at the segment index. If `None` then the segment is erased.
        value: Option<StringSource>
    },
    SetSegmentRange {
        split: StringSource,
        start: Option<isize>,
        end: Option<isize>,
        value: StringSource
    },
    /// Like [`Self::SetNthSegment`] except it inserts `value` before the `n`th segment instead of overwriting.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "a.b.c".to_string();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  1, value:  "1".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.b.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -1, value: "-1".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  4, value:  "4".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.4.c");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  6, value:  "6".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.4.c.6");
    /// StringModification::InsertSegmentBefore{split: ".".into(), n:  8, value:  "E".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -8, value:  "E".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".into(), n: -7, value: "-7".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "-7.a.1.b.-1.4.c.6");
    /// ```
    InsertSegmentBefore {
        /// The value to split the string by.
        split: StringSource,
        /// The segment index to insert before.
        n: isize,
        /// The value to insert.
        value: StringSource
    },
    /// Like [`Self::SetNthSegment`] except it inserts `value` after the `n`th segment instead of overwriting.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "a.b.c".to_string();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  1, value:  "1".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.b.1.c");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -1, value: "-1".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.b.1.c.-1");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  4, value:  "4".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.b.1.c.-1.4");
    /// StringModification::InsertSegmentAfter{split: ".".into(), n:  6, value:  "E".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -7, value:  "E".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap_err();
    /// StringModification::InsertSegmentAfter{split: ".".into(), n: -6, value: "-6".into()}.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a.-6.b.1.c.-1.4");
    /// ```
    InsertSegmentAfter {
        /// The value to split the string by.
        split: StringSource,
        /// The segment index to insert before.
        n: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`Regex::captures`] and [`regex::Captures::expand`].
    /// # Errors
    /// When the call to [`Regex::captures`] returns [`None`], returns the error [`StringModificationError::RegexMatchNotFound`]
    #[cfg(feature = "regex")]
    RegexCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`regex::Captures::expand`] with.
        replace: StringSource
    },
    /// [`Regex::captures_iter`] and [`regex::Captures::expand`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use url_cleaner::glue::*;
    /// # use std::str::FromStr;
    /// let mut x = "...a2..a3....a4".to_string();
    /// StringModification::JoinAllRegexCaptures {
    ///     regex: RegexWrapper::from_str(r"a(\d)").unwrap(),
    ///     replace: "A$1$1".into(),
    ///     join: "---".into()
    /// }.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(x, "A22---A33---A44");
    /// ```
    /// # Errors
    /// If the call to [`RegexWrapper::get_regex`] returns an error, that error is returned,
    #[cfg(feature = "regex")]
    JoinAllRegexCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`regex::Captures::expand`] with.
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
    /// # use url::Url;
    /// let mut x = "a/b/c".to_string();
    /// StringModification::URLEncode.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a%2Fb%2Fc");
    /// ```
    URLEncode,
    /// [`percent_encoding::percent_decode_str`]
    /// # Errors
    /// If the call to [`percent_encoding::percent_decode_str`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "a%2fb%2Fc".to_string();
    /// StringModification::URLDecode.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(&x, "a/b/c");
    /// ```
    URLDecode,
    /// Encode the string using [`base64::prelude::BASE64_STANDARD`].
    Base64EncodeStandard,
    /// Decode the string using [`base64::prelude::BASE64_STANDARD`].
    /// # Errors
    /// If the call to [`base64::engine::Engine::decode`] returns an error, that error is returned.
    /// 
    /// If the call to [`String::from_utf8`] returns an error, that error is returned.
    Base64DecodeStandard,
    /// Encode the string using [`base64::prelude::BASE64_URL_SAFE`].
    Base64EncodeUrlSafe,
    /// Encode the string using [`base64::prelude::BASE64_URL_SAFE`].
    /// # Errors
    /// If the call to [`base64::engine::Engine::decode`] returns an error, that error is returned.
    /// 
    /// If the call to [`String::from_utf8`] returns an error, that error is returned.
    Base64DecodeUrlSafe,
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
    /// # use url::Url;
    /// let mut x = "abc xyz".to_string();
    /// StringModification::ModifyNthSegment {
    ///     split: " ".into(),
    ///     n: 1,
    ///     modification: Box::new(StringModification::Set("abc".into()))
    /// }.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(x, "abc abc");
    /// ```
    /// # Errors
    /// If the segment is not found, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the modification returns an error, that error is returned.
    ModifyNthSegment {
        /// The value to split the sring by.
        split: StringSource,
        /// The index of the segment to modify.
        n: isize,
        /// The modification to apply.
        modification: Box<Self>
    },
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut x = "abc def xyz".to_string();
    /// StringModification::ModifySegments {
    ///     split: " ".into(),
    ///     ns: vec![1, 2],
    ///     modification: Box::new(StringModification::Set("a b c".into()))
    /// }.apply(&mut x, &JobState::new(&mut Url::parse("https://example.com").unwrap())).unwrap();
    /// assert_eq!(x, "abc a b c a b c");
    /// ```
    /// # Errors
    /// If a segment is not found, returns the error [`StringModificationError::SegmentNotFound`].
    /// 
    /// If the modification returns an error, that error is returned.
    ModifySegments {
        /// The value to split the sring by.
        split: StringSource,
        /// The indices of the segments to modify.
        ns: Vec<isize>,
        /// The modification to apply.
        modification: Box<Self>
    }
}

/// The enum of all possible errors [`StringModification::apply`] can return.
#[allow(clippy::enum_variant_names)]
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
    /// Returned when a [`regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Returned when both the `try` and `else` of a [`StringModification::TryElse`] both return errors.
    #[error("A `StringModification::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`StringModification::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`StringModification::TryElse::else`],
        else_error: Box<Self>
    },
    /// Returned when a [`base64::DecodeError`] is encountered.
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
    /// Returned when a [`std::string::FromUtf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    #[error("TODO")]
    StringSourceIsNone
}

impl From<StringSourceError> for StringModificationError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl StringModification {
    /// Apply the modification in-place using the provided [`Params`].
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, to: &mut String, job_state: &JobState) -> Result<(), StringModificationError> {
        #[cfg(feature = "debug")]
        println!("Modification: {self:?}");
        match self {
            Self::None => {},
            Self::Error => Err(StringModificationError::ExplicitError)?,
            Self::Debug(modification) => {
                let to_before_mapper=to.clone();
                let modification_result=modification.apply(to, job_state);
                eprintln!("=== StringModification::Debug ===\nModification: {modification:?}\nJob state: {job_state:?}\nString before mapper: {to_before_mapper:?}\nModification return value: {modification_result:?}\nString after mapper: {to:?}");
                modification_result?;
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
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripSuffix(suffix)                => {
                let suffix = get_str!(suffix, job_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;};
            },
            Self::StripMaybePrefix(prefix)           => {
                let prefix = get_str!(prefix, job_state, StringModificationError);
                if to.starts_with(prefix) {to.drain(..prefix.len());};
            },
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripMaybeSuffix(suffix)           => {
                let suffix = get_str!(suffix, job_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len());};
            },
            Self::Insert{r#where, value}               => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.insert_str(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?, get_str!(value, job_state, StringModificationError));} else {Err(StringModificationError::InvalidIndex)?;},
            Self::Remove(r#where)                      => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.remove    (neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?                                                     );} else {Err(StringModificationError::InvalidIndex)?;},
            Self::KeepRange{start, end}                => *to = to.get(neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
            Self::KeepNthSegment   {split, n}          => *to = neg_nth(to.split(get_str!(split, job_state, StringModificationError)), *n).ok_or(StringModificationError::SegmentNotFound)?.to_string(),
            Self::KeepSegmentRange {split, start, end} => {
                let split = get_str!(split, job_state, StringModificationError);
                *to = neg_vec_keep(to.split(split), *start, *end).ok_or(StringModificationError::SegmentRangeNotFound)?.join(split);
            },
            Self::SetNthSegment{split, n, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut temp=to.split(split).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                if fixed_n==temp.len() {Err(StringModificationError::SegmentNotFound)?;}
                let x = get_option_string!(value, job_state);
                // fixed_n is guaranteed to be in bounds.
                #[allow(clippy::indexing_slicing)]
                match x.as_deref() {
                    Some(value) => temp[fixed_n]=value,
                    None        => {temp.remove(fixed_n);}
                }
                *to=temp.join(split);
            },
            Self::SetSegmentRange {split, start, end, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).collect::<Vec<_>>();
                let fixed_n = neg_index(start.unwrap_or(0), segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let _ = segments.drain(neg_range(*start, *end, segments.len()).ok_or(StringModificationError::SegmentRangeNotFound)?).collect::<Vec<_>>();
                let x = get_string!(value, job_state, StringModificationError);
                segments.insert(fixed_n, &x);
                *to = segments.join(split);
            }
            Self::InsertSegmentBefore{split, n, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut temp=to.split(split).collect::<Vec<_>>();
                let fixed_n=neg_range_boundary(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let x = get_string!(value, job_state, StringModificationError);
                temp.insert(fixed_n, &x);
                *to=temp.join(split);
            },
            Self::InsertSegmentAfter{split, n, value} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut temp=to.split(split).collect::<Vec<_>>();
                let fixed_n=neg_shifted_range_boundary(*n, temp.len(), 1).ok_or(StringModificationError::SegmentNotFound)?;
                #[allow(clippy::arithmetic_side_effects)]
                let x = get_string!(value, job_state, StringModificationError);
                temp.insert(fixed_n, &x);
                *to=temp.join(split);
            },
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
            Self::URLEncode => *to=utf8_percent_encode(to, NON_ALPHANUMERIC).to_string(),
            Self::URLDecode => *to=percent_decode_str(to).decode_utf8()?.into_owned(),
            Self::Base64EncodeStandard => *to = BASE64_STANDARD.encode(to.as_bytes()),
            Self::Base64DecodeStandard => *to = String::from_utf8(BASE64_STANDARD.decode(to.as_bytes())?)?,
            Self::Base64EncodeUrlSafe  => *to = BASE64_URL_SAFE.encode(to.as_bytes()),
            Self::Base64DecodeUrlSafe  => *to = String::from_utf8(BASE64_URL_SAFE.decode(to.as_bytes())?)?,
            Self::JsonPointer(pointer) => *to = serde_json::from_str::<serde_json::Value>(to)?.pointer(get_str!(pointer, job_state, StringModificationError)).ok_or(StringModificationError::JsonValueNotFound)?.as_str().ok_or(StringModificationError::JsonValueIsNotAString)?.to_string(),
            // fixed_n is guaranteed to be in bounds.
            #[allow(clippy::indexing_slicing)]
            Self::ModifyNthSegment {split, n, modification} => {
                let split = get_str!(split, job_state, StringModificationError);
                let mut segments = to.split(split).collect::<Vec<_>>();
                let fixed_n = neg_index(*n, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                let mut temp = segments[fixed_n].to_string();
                modification.apply(&mut temp, job_state)?;
                segments[fixed_n] = &*temp;
                *to = segments.join(split);
            },
            // fixed_n is guaranteed to be in bounds.
            #[allow(clippy::indexing_slicing)]
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
            }
        };
        Ok(())
    }
}

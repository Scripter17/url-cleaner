//! Provides [`StringModification`] which provides an easy API for all the ways one might want to modify a [`String`].

use serde::{Serialize, Deserialize};
use thiserror::Error;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use url::Url;
// Used just for documentation.
#[allow(unused_imports)]
use regex::Regex;

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
    /// If one of the contained [`Self`]s returns an error, the URL is left unchanged and the error is returned.
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
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
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Set("ghi".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "ghi");
    /// ```
    Set(String),
    /// Append the contained string.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Append("ghi".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "abcdefghi");
    /// ```
    Append(String),
    /// Prepend the contained string.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Prepend("ghi".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "ghiabcdef");
    /// ```
    Prepend(String),
    /// Replace all instances of `find` with `replace`.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcabc".to_string();
    /// StringModification::Replace{find: "ab".to_string(), replace: "xy".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "xycxyc");
    /// ```
    Replace {
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String
    },
    /// Replace the specified range with `replace`.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::ReplaceRange{start: Some( 6), end: Some( 7), replace: "123" .to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// assert_eq!(&x, "abcdef");
    /// StringModification::ReplaceRange{start: Some( 1), end: Some( 4), replace: "ab"  .to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "aabef");
    /// StringModification::ReplaceRange{start: Some(-3), end: Some(-1), replace: "abcd".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "aaabcdf");
    /// StringModification::ReplaceRange{start: Some(-3), end: None    , replace: "efg" .to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "aaabefg");
    /// StringModification::ReplaceRange{start: Some(-8), end: None    , replace: "hij" .to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// assert_eq!(&x, "aaabefg");
    /// ```
    ReplaceRange {
        /// The start of the range to replace.
        start: Option<isize>,
        /// The end of the range to replace.
        end: Option<isize>,
        /// The value to replace the range with.
        replace: String
    },
    /// [`str::to_lowercase`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "ABCdef".to_string();
    /// StringModification::Lowercase.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "abcdef");
    /// ```
    Lowercase,
    /// [`str::to_uppercase`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcDEF".to_string();
    /// StringModification::Uppercase.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "ABCDEF");
    /// ```
    Uppercase,
    /// Mimics [`str::strip_prefix`] using [`str::starts_with`] and [`String::drain`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't begin with the specified prefix, returns the error [`StringModificationError::PrefixNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripPrefix("abc".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripPrefix("abc".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// assert_eq!(&x, "def");
    /// ```
    StripPrefix(String),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't end with the specified suffix, returns the error [`StringModificationError::SuffixNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripSuffix("def".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripSuffix("def".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// assert_eq!(&x, "abc");
    /// ```
    StripSuffix(String),
    /// [`Self::StripPrefix`] but does nothing if the target string doesn't begin with the specified prefix.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "def");
    /// StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "def");
    /// ```
    StripMaybePrefix(String),
    /// [`Self::StripSuffix`] but does nothing if the target string doesn't end with the specified suffix.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "abc");
    /// StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "abc");
    /// ```
    StripMaybeSuffix(String),
    /// [`str::replacen`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "aaaaa".to_string();
    /// StringModification::Replacen{find: "a" .to_string(), replace: "x".to_string(), count: 2}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "xxaaa");
    /// StringModification::Replacen{find: "xa".to_string(), replace: "x".to_string(), count: 2}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "xxaa");
    /// ```
    Replacen {
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String,
        /// The number of times to do the replacement.
        count: usize
    },
    /// [`String::insert_str`].
    /// # Errors
    /// If `where` is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abc".to_string();
    /// StringModification::Insert{r#where:  0, value: "def".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "defabc");
    /// StringModification::Insert{r#where:  2, value: "ghi".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "deghifabc");
    /// StringModification::Insert{r#where: -1, value: "jhk".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "deghifabjhkc");
    /// ```
    Insert {
        /// The location to insert `value`.
        r#where: isize,
        /// The string to insert.
        value: String
    },
    /// [`String::remove`].
    /// # Errors
    /// If the specified index is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdef".to_string();
    /// StringModification::Remove( 1).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "acdef");
    /// StringModification::Remove(-1).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "acde");
    /// ```
    Remove(isize),
    /// Discards everything outside the specified range.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringModificationError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "abcdefghi".to_string();
    /// StringModification::KeepRange{start: Some( 1), end: Some( 8)}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "bcdefgh");
    /// StringModification::KeepRange{start: None    , end: Some( 6)}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "bcdefg");
    /// StringModification::KeepRange{start: Some(-3), end: None    }.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "efg");
    /// StringModification::KeepRange{start: Some(-3), end: Some(-1)}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "ef");
    /// ```
    KeepRange {
        /// The start of the range to keep.
        start: Option<isize>,
        /// The end of the range to keep.
        end: Option<isize>
    },
    /// Splits the provided string by `split`, replaces the `n`th segment with `value` or removes the segment if `value` is `None`, then joins the string back together.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "a.b.c.d.e.f".to_string();
    /// StringModification::SetNthSegment{split: ".".to_string(), n:  1, value: Some( "1".to_string())}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.f");
    /// StringModification::SetNthSegment{split: ".".to_string(), n: -1, value: Some("-1".to_string())}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.e.-1");
    /// StringModification::SetNthSegment{split: ".".to_string(), n: -2, value: None}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.c.d.-1");
    /// StringModification::SetNthSegment{split: ".".to_string(), n:  5, value: Some( "E".to_string())}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".to_string(), n: -6, value: Some( "E".to_string())}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// StringModification::SetNthSegment{split: ".".to_string(), n: -5, value: Some("-5".to_string())}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "-5.1.c.d.-1");
    /// ```
    SetNthSegment {
        /// The value to split the string by.
        split: String,
        /// The segment index to modify.
        n: isize,
        /// The value to place at the segment index. If `None` then the segment is erased.
        value: Option<String>
    },
    /// Like [`Self::SetNthSegment`] except it inserts `value` before the `n`th segment instead of overwriting.
    /// # Errors
    /// If `n` is not in the range of of segments, returns the error [`StringModificationError::SegmentNotFound`].
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "a.b.c".to_string();
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n:  1, value:  "1".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.b.c");
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n: -1, value: "-1".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.c");
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n:  5, value:  "5".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a.1.b.-1.c.5");
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n:  7, value:  "E".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n: -7, value:  "E".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// StringModification::InsertSegmentBefore{split: ".".to_string(), n: -6, value: "-6".to_string()}.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "-6.a.1.b.-1.c.5");
    /// ```
    InsertSegmentBefore {
        /// The value to split the string by.
        split: String,
        /// The segment index to insert before.
        n: isize,
        /// The value to insert.
        value: String
    },
    /// [`Regex::captures`] and [`regex::Captures::expand`].
    /// # Errors
    /// When the call to [`Regex::captures`] returns [`None`], returns the error [`StringModificationError::RegexMatchNotFound`]
    #[cfg(feature = "regex")]
    RegexCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`regex::Captures::expand`] with.
        replace: String
    },
    /// [`Regex::captures_iter`] and [`regex::Captures::expand`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::*;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut x = "...a2..a3....a4".to_string();
    /// StringModification::RegexJoinAllCaptures {
    ///     regex: RegexWrapper::from_str(r"a(\d)").unwrap(),
    ///     replace: "A$1$1".to_string(),
    ///     join: "---".to_string()
    /// }.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(x, "A22---A33---A44");
    /// ```
    /// # Errors
    /// If the call to [`RegexWrapper::get_regex`] returns an error, that error is returned,
    #[cfg(feature = "regex")]
    RegexJoinAllCaptures {
        /// The regex to search with.
        regex: RegexWrapper,
        /// The replacement string to call [`regex::Captures::expand`] with.
        replace: String,
        /// The value to join the captures with. Does not default to anything.
        join: String
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
        replace: String
    },
    /// [`Regex::replace_all`]
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        /// The regex to do replacement with.
        regex: RegexWrapper,
        /// The replacement string.
        replace: String
    },
    /// [`Regex::replacen`]
    #[cfg(feature = "regex")]
    RegexReplacen {
        /// The regex to do replacement with.
        regex: RegexWrapper,
        /// The number of replacements to do.
        n: usize,
        /// The replacement string.
        replace: String
    },
    /// Choose which string modification to apply based on of a flag is set.
    IfFlag {
        /// The flag to check the setness of.
        flag: String,
        /// The string modification to apply if the flag is set.
        then: Box<Self>,
        /// The string modification to apply if the flag is not set.
        r#else: Box<Self>
    },
    /// Uses [`percent_encoding::utf8_percent_encode`] to percent encode all bytes that are not alphanumeric ASCII.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "a/b/c".to_string();
    /// StringModification::URLEncode.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a%2Fb%2Fc");
    /// ```
    URLEncode,
    /// [`percent_encoding::percent_decode_str`]
    /// # Errors
    /// If the call to [`percent_encoding::percent_decode_str`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::types::Params;
    /// let mut x = "a%2fb%2Fc".to_string();
    /// StringModification::URLDecode.apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default()).unwrap();
    /// assert_eq!(&x, "a/b/c");
    /// ```
    URLDecode,
    /// Runs the contained command with the provided string as the STDIN.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::glue::CommandConfig;
    /// # use url_cleaner::types::Params;
    /// # use std::str::FromStr;
    /// let mut x = "abc\n".to_string();
    /// StringModification::CommandOutput(CommandConfig::from_str("cat").unwrap()).apply(&mut x, &Url::parse("https://example.com").unwrap(), &Params::default());
    /// assert_eq!(&x, "abc\n");
    /// ````
    #[cfg(feature = "commands")]
    CommandOutput(CommandConfig),
    /// [`serde_json::Value::pointer`].
    /// Does not do any string conversions. I should probably add an option for that.
    /// # Errors
    /// If the pointer doesn't point to anything, returns the error [`StringModificationError::JsonValueNotFound`].
    /// If the pointer points to a non-string vailue, returns the error [`StringModificationError::JsonValueIsNotAString`].
    JsonPointer(String),
    /// [`Url::join`].
    /// # Errors
    /// TODO
    UrlJoin(#[cfg(feature = "string-source")] Box<StringSource>, #[cfg(not(feature = "string-source"))] String)
}

/// The enum of all possible errors [`StringModification::apply`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// Returned when [`StringModification::Error`] is used.
    #[error("StringModification::Error was used.")]
    ExplicitError,
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] CommandError),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    FromUtf8Error(#[from] std::str::Utf8Error),
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
    /// Returned when the provided string does not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// Returned when the provided string does not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
    /// Returned when the requested regex pattern is not found in the provided string.
    #[error("The requested regex pattern was not found in the provided string.")]
    RegexMatchNotFound,
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(Box<StringSourceError>),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[cfg(feature = "string-source")]
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`regex::Error`] is encountered.
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] regex::Error)
}

#[cfg(feature = "string-source")]
impl From<StringSourceError> for StringModificationError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl StringModification {
    /// Apply the modification in-place using the provided [`Params`].
    /// # Errors
    /// See the documentation for [`Self`]'s variants for details.
    pub fn apply(&self, to: &mut String, url: &Url, params: &Params) -> Result<(), StringModificationError> {
        #[cfg(feature = "debug")]
        println!("Modification: {self:?}");
        match self {
            Self::None => {},
            Self::Error => Err(StringModificationError::ExplicitError)?,
            Self::Debug(modification) => {
                let to_before_mapper=to.clone();
                let modification_result=modification.apply(to, url, params);
                eprintln!("=== StringModification::Debug ===\nModification: {modification:?}\nParams: {params:?}\nString before mapper: {to_before_mapper:?}\nModification return value: {modification_result:?}\nString after mapper: {to:?}");
                modification_result?;
            },
            Self::IgnoreError(modification) => {let _=modification.apply(to, url, params);},
            Self::TryElse{r#try, r#else} => r#try.apply(to, url, params).or_else(|_| r#else.apply(to, url, params))?,
            Self::All(modifications) => {
                let mut temp_to=to.clone();
                for modification in modifications {
                    modification.apply(&mut temp_to, url, params)?;
                }
                *to=temp_to;
            }
            Self::AllNoRevert(modifications) => {
                for modification in modifications {
                    modification.apply(to, url, params)?;
                }
            },
            Self::AllIgnoreError(modifications) => {
                for modification in modifications {
                    let _=modification.apply(to, url, params);
                }
            },
            Self::FirstNotError(modifications) => {
                let mut error=Ok(());
                for modification in modifications {
                    error=modification.apply(to, url, params);
                    if error.is_ok() {break}
                }
                error?
            },
            Self::Set(value)                         => *to=value.clone(),
            Self::Append(value)                      => to.push_str(value),
            Self::Prepend(value)                     => {let mut ret=value.to_string(); ret.push_str(to); *to=ret;},
            Self::Replace{find, replace}             => *to=to.replace(find, replace),
            Self::ReplaceRange{start, end, replace}  => {
                let range=neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, replace);
                } else {
                    Err(StringModificationError::InvalidSlice)?;
                }
            },
            Self::Lowercase                          => *to=to.to_lowercase(),
            Self::Uppercase                          => *to=to.to_uppercase(),
            Self::StripPrefix(prefix)                => if to.starts_with(prefix) {to.drain(..prefix.len());} else {Err(StringModificationError::PrefixNotFound)?;},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripSuffix(suffix)                => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;},
            Self::StripMaybePrefix(prefix)           => if to.starts_with(prefix) {to.drain(..prefix.len());},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripMaybeSuffix(suffix)           => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len());},
            Self::Replacen{find, replace, count}     => *to=to.replacen(find, replace, *count),
            Self::Insert{r#where, value}             => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.insert_str(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?, value);} else {Err(StringModificationError::InvalidIndex)?;},
            Self::Remove(r#where)                    => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.remove    (neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?       );} else {Err(StringModificationError::InvalidIndex)?;},
            Self::KeepRange{start, end}              => *to=to.get(neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
            Self::SetNthSegment{split, n, value} => {
                let mut temp=to.split(split.as_str()).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                if fixed_n==temp.len() {Err(StringModificationError::SegmentNotFound)?;}
                match value {
                    Some(value) => *temp.get_mut(fixed_n).ok_or(StringModificationError::SegmentNotFound)?=value.as_str(),
                    None        => {temp.remove(fixed_n);}
                }
                *to=temp.join(split);
            },
            Self::InsertSegmentBefore{split, n, value} => {
                let mut temp=to.split(split.as_str()).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringModificationError::SegmentNotFound)?;
                temp.insert(fixed_n, value.as_str());
                *to=temp.join(split);
            },
            #[cfg(feature = "regex")]
            Self::RegexCaptures {regex, replace} => {
                let mut temp = "".to_string();
                regex.get_regex()?.captures(to).ok_or(StringModificationError::RegexMatchNotFound)?.expand(replace, &mut temp);
                *to = temp;
            },
            #[cfg(feature = "regex")] Self::RegexJoinAllCaptures {regex, replace, join} => {
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
            #[cfg(feature = "regex")] Self::RegexFind       (regex            ) => *to = regex.get_regex()?.find       (to             ).ok_or(StringModificationError::RegexMatchNotFound)?.as_str().to_string(),
            #[cfg(feature = "regex")] Self::RegexReplace    {regex,    replace} => *to = regex.get_regex()?.replace    (to,     replace).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplaceAll {regex,    replace} => *to = regex.get_regex()?.replace_all(to,     replace).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplacen   {regex, n, replace} => *to = regex.get_regex()?.replacen   (to, *n, replace).into_owned(),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then} else {r#else}.apply(to, url, params)?,
            Self::URLEncode => *to=utf8_percent_encode(to, NON_ALPHANUMERIC).to_string(),
            Self::URLDecode => *to=percent_decode_str(to).decode_utf8()?.into_owned(),
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => *to=command.output(None, Some(to.as_bytes()))?,
            Self::JsonPointer(pointer) => *to=serde_json::from_str::<serde_json::Value>(to)?.pointer(pointer).ok_or(StringModificationError::JsonValueNotFound)?.as_str().ok_or(StringModificationError::JsonValueIsNotAString)?.to_string(),
            Self::UrlJoin(with) => *to=Url::parse(to)?.join(get_string!(with, url, params, StringModificationError))?.to_string(),
        };
        Ok(())
    }
}

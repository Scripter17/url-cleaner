use std::str::Utf8Error;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

use super::*;
#[cfg(feature = "regex")]
use crate::glue::RegexWrapper;
#[cfg(feature = "commands")]
use crate::glue::{CommandWrapper, CommandError};
use crate::glue::string_or_struct;
use crate::config::Params;

/// A wrapper around [`str`]'s various substring modification functions.
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq)]
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
    /// [`urlencoding::encode`].
    /// # Errors
    /// [`urlencoding::decode`].
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
    /// # Examples
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, returns that error.
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order. If an error occurs, the URL remains changed by the previous contained [`Self`]s and the error is returned.
    /// Technically the name is wrong as [`Self::All`] only actually applies the change after all the contained [`Self`] pass, but this is conceptually simpler.
    /// # Errors
    /// If one of the contained [`Self`]s returns an error, the URL is left as whatever the previous contained mapper set it to and the error is returned.
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, returns that error.
    AllNoRevert(Vec<Self>),
    /// If any of the contained [`Self`]s returns an error, the error is ignored and subsequent [`Self`]s are still applied.
    /// This is equivalent to wrapping every contained [`Self`] in a [`Self::IgnoreError`].
    AllIgnoreError(Vec<Self>),
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] errors, returns the last error.
    /// # Errors
    /// If the last [`Self`] errors, returns that error.
    FirstNotError(Vec<Self>),



    /// Replaces the entire target string to the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Set("ghi".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "ghi");
    /// ```
    Set(String),
    /// Append the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Append("ghi".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "abcdefghi");
    /// ```
    Append(String),
    /// Prepend the contained string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Prepend("ghi".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "ghiabcdef");
    /// ```
    Prepend(String),
    /// Replace all instances of `find` with `replace`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcabc".to_string();
    /// assert!(StringModification::Replace{find: "ab".to_string(), replace: "xy".to_string()}.apply(&mut x, &Params::default()).is_ok());
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
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::ReplaceRange{start: Some( 6), end: Some( 7), replace: "123" .to_string()}.apply(&mut x, &Params::default()).is_err());
    /// assert_eq!(&x, "abcdef");
    /// assert!(StringModification::ReplaceRange{start: Some( 1), end: Some( 4), replace: "ab"  .to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "aabef");
    /// assert!(StringModification::ReplaceRange{start: Some(-3), end: Some(-1), replace: "abcd".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "aaabcdf");
    /// assert!(StringModification::ReplaceRange{start: Some(-3), end: None    , replace: "efg" .to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "aaabefg");
    /// assert!(StringModification::ReplaceRange{start: Some(-8), end: None    , replace: "hij" .to_string()}.apply(&mut x, &Params::default()).is_err());
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
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "ABCdef".to_string();
    /// assert!(StringModification::Lowercase.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "abcdef");
    /// ```
    Lowercase,
    /// [`str::to_uppercase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcDEF".to_string();
    /// assert!(StringModification::Uppercase.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "ABCDEF");
    /// ```
    Uppercase,
    /// [`str::strip_prefix`].
    /// # Errors
    /// If the target string doesn't begin with the specified prefix, returns the error [`StringError::PrefixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "def");
    /// assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x, &Params::default()).is_err());
    /// assert_eq!(&x, "def");
    /// ```
    StripPrefix(String),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the target string doesn't end with the specified suffix, returns the error [`StringError::SuffixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "abc");
    /// assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x, &Params::default()).is_err());
    /// assert_eq!(&x, "abc");
    /// ```
    StripSuffix(String),
    /// [`Self::StripPrefix`] but does nothing if the target string doesn't begin with the specified prefix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "def");
    /// assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "def");
    /// ```
    StripMaybePrefix(String),
    /// [`Self::StripSuffix`] but does nothing if the target string doesn't end with the specified suffix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "abc");
    /// assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "abc");
    /// ```
    StripMaybeSuffix(String),
    /// [`str::replacen`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "aaaaa".to_string();
    /// assert!(StringModification::Replacen{find: "a" .to_string(), replace: "x".to_string(), count: 2}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "xxaaa");
    /// assert!(StringModification::Replacen{find: "xa".to_string(), replace: "x".to_string(), count: 2}.apply(&mut x, &Params::default()).is_ok());
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
    /// If `where` is out of bounds or not on a UTF-8 character boundary, returns the error [`StringError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abc".to_string();
    /// assert!(StringModification::Insert{r#where:  0, value: "def".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "defabc");
    /// assert!(StringModification::Insert{r#where:  2, value: "ghi".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "deghifabc");
    /// assert!(StringModification::Insert{r#where: -1, value: "jhk".to_string()}.apply(&mut x, &Params::default()).is_ok());
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
    /// If the specified index is out of bounds or not on a UTF-8 character boundary, returns the error [`StringError::InvalidIndex`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Remove( 1).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "acdef");
    /// assert!(StringModification::Remove(-1).apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "acde");
    /// ```
    Remove(isize),
    /// Discards everything outside the specified range.
    /// # Errors
    /// If either end of the specified range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "abcdefghi".to_string();
    /// assert!(StringModification::KeepRange{start: Some( 1), end: Some( 8)}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "bcdefgh");
    /// assert!(StringModification::KeepRange{start: None    , end: Some( 6)}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "bcdefg");
    /// assert!(StringModification::KeepRange{start: Some(-3), end: None    }.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "efg");
    /// assert!(StringModification::KeepRange{start: Some(-3), end: Some(-1)}.apply(&mut x, &Params::default()).is_ok());
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
    /// If `n` is not in the range of of segments, returns the error [`StringError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "a.b.c.d.e.f".to_string();
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n:  1, value: Some( "1".to_string())}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.c.d.e.f");
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n: -1, value: Some("-1".to_string())}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.c.d.e.-1");
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n: -2, value: None}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.c.d.-1");
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n:  5, value: Some( "E".to_string())}.apply(&mut x, &Params::default()).is_err());
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n: -6, value: Some( "E".to_string())}.apply(&mut x, &Params::default()).is_err());
    /// assert!(StringModification::SetNthSegment{split: ".".to_string(), n: -5, value: Some("-5".to_string())}.apply(&mut x, &Params::default()).is_ok());
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
    /// If `n` is not in the range of of segments, returns the error [`StringError::SegmentNotFound`].
    /// Please note that trying to append a new segment at the end still errors.
    /// # Examples
    /// ```
    /// use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "a.b.c".to_string();
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n:  1, value:  "1".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.b.c");
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n: -1, value: "-1".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.b.-1.c");
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n:  5, value:  "5".to_string()}.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a.1.b.-1.c.5");
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n:  7, value:  "E".to_string()}.apply(&mut x, &Params::default()).is_err());
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n: -7, value:  "E".to_string()}.apply(&mut x, &Params::default()).is_err());
    /// assert!(StringModification::InsertSegmentBefore{split: ".".to_string(), n: -6, value: "-6".to_string()}.apply(&mut x, &Params::default()).is_ok());
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
    /// [`RegexWrapper::replace`]
    #[cfg(feature = "regex")]
    RegexReplace {
        /// The regex to do replacement with.
        #[serde(deserialize_with = "string_or_struct")]
        regex: RegexWrapper,
        /// The replacement string.
        replace: String
    },
    /// [`RegexWrapper::replace_all`]
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        /// The regex to do replacement with.
        #[serde(deserialize_with = "string_or_struct")]
        regex: RegexWrapper,
        /// The replacement string.
        replace: String
    },
    /// [`RegexWrapper::replacen`]
    #[cfg(feature = "regex")]
    RegexReplacen {
        /// The regex to do replacement with.
        #[serde(deserialize_with = "string_or_struct")]
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
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "a/b/c".to_string();
    /// assert!(StringModification::URLEncode.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a%2Fb%2Fc");
    /// ```
    URLEncode,
    /// [`percent_encoding::percent_decode_str`]
    /// # Errors
    /// If the call to [`urlencoding::decode`] errors, returns that error.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::config::Params;
    /// let mut x = "a%2fb%2Fc".to_string();
    /// assert!(StringModification::URLDecode.apply(&mut x, &Params::default()).is_ok());
    /// assert_eq!(&x, "a/b/c");
    /// ```
    URLDecode,
    /// Runs the contained command with the provided string as the STDIN.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// # use url_cleaner::glue::CommandWrapper;
    /// # use url_cleaner::config::Params;
    /// # use std::str::FromStr;
    /// let mut x = "abc\n".to_string();
    /// StringModification::CommandOutput(CommandWrapper::from_str("cat").unwrap()).apply(&mut x, &Params::default());
    /// assert_eq!(&x, "abc\n");
    /// ````
    #[cfg(feature = "commands")]
    CommandOutput(CommandWrapper)
}

/// An enum of all possible errors a [`StringModification`] can return.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// A generic string error.
    #[error(transparent)]
    StringError(#[from] StringError),
    /// Returned by [`StringModification::CommandOutput`].
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] CommandError),
    /// Always returned by [`StringModification::Error`].
    #[error("StringModification::Error was used.")]
    ExplicitError,
    /// Returned by [`StringModification::URLDecode`].
    #[error(transparent)]
    FromUtf8Error(#[from] Utf8Error)
}

impl StringModification {
    /// Apply the modification in-place using the provided [`Params`].
    /// # Errors
    /// See the documentation for [`Self`]'s variants for details.
    pub fn apply(&self, to: &mut String, params: &Params) -> Result<(), StringModificationError> {
        #[cfg(feature = "debug")]
        println!("Modification: {self:?}");
        match self {
            Self::None                               => {},
            Self::Set(value)                         => *to=value.clone(),
            Self::Append(value)                      => to.push_str(value),
            Self::Prepend(value)                     => {let mut ret=value.to_string(); ret.push_str(to); *to=ret;},
            Self::Replace{find, replace}             => *to=to.replace(find, replace),
            Self::ReplaceRange{start, end, replace}  => {
                let range=neg_range(*start, *end, to.len()).ok_or(StringError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, replace);
                } else {
                    Err(StringError::InvalidSlice)?;
                }
            },
            Self::Lowercase                          => *to=to.to_lowercase(),
            Self::Uppercase                          => *to=to.to_uppercase(),
            Self::StripPrefix(prefix)                => if to.starts_with(prefix) {to.drain(..prefix.len());} else {Err(StringError::PrefixNotFound)?;},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripSuffix(suffix)                => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringError::SuffixNotFound)?;},
            Self::StripMaybePrefix(prefix)           => if to.starts_with(prefix) {to.drain(..prefix.len());},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripMaybeSuffix(suffix)           => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len());},
            Self::Replacen{find, replace, count}     => *to=to.replacen(find, replace, *count),
            Self::Insert{r#where, value}             => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringError::InvalidIndex)?) {to.insert_str(neg_index(*r#where, to.len()).ok_or(StringError::InvalidIndex)?, value);} else {Err(StringError::InvalidIndex)?;},
            Self::Remove(r#where)                    => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringError::InvalidIndex)?) {to.remove    (neg_index(*r#where, to.len()).ok_or(StringError::InvalidIndex)?       );} else {Err(StringError::InvalidIndex)?;},
            Self::KeepRange{start, end}              => *to=to.get(neg_range(*start, *end, to.len()).ok_or(StringError::InvalidSlice)?).ok_or(StringError::InvalidSlice)?.to_string(),
            Self::SetNthSegment{split, n, value} => {
                let mut temp=to.split(split.as_str()).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringError::SegmentNotFound)?;
                if fixed_n==temp.len() {Err(StringError::SegmentNotFound)?;}
                match value {
                    Some(value) => temp[fixed_n]=value.as_str(),
                    None        => {temp.remove(fixed_n);}
                }
                *to=temp.join(split);
            },
            Self::InsertSegmentBefore{split, n, value} => {
                let mut temp=to.split(split.as_str()).collect::<Vec<_>>();
                let fixed_n=neg_index(*n, temp.len()).ok_or(StringError::SegmentNotFound)?;
                temp.insert(fixed_n, value.as_str());
                *to=temp.join(split);
            },
            #[cfg(feature = "regex")] Self::RegexReplace    {regex,    replace} => *to=regex.replace    (to,     replace).to_string(),
            #[cfg(feature = "regex")] Self::RegexReplaceAll {regex,    replace} => *to=regex.replace_all(to,     replace).to_string(),
            #[cfg(feature = "regex")] Self::RegexReplacen   {regex, n, replace} => *to=regex.replacen   (to, *n, replace).to_string(),
            Self::IfFlag {flag, then, r#else} => if params.flags.contains(flag) {then.apply(to, params)} else {r#else.apply(to, params)}?,
            Self::URLEncode => *to=utf8_percent_encode(to, NON_ALPHANUMERIC).to_string(),
            Self::URLDecode => *to=percent_decode_str(to).decode_utf8()?.into_owned(),
            Self::All(modifications) => {
                let mut temp_to=to.clone();
                for modification in modifications {
                    modification.apply(&mut temp_to, params)?;
                }
                *to=temp_to;
            }
            Self::AllNoRevert(modifications) => {
                for modification in modifications {
                    modification.apply(to, params)?;
                }
            },
            Self::AllIgnoreError(modifications) => {
                for modification in modifications {
                    let _=modification.apply(to, params);
                }
            },
            Self::FirstNotError(modifications) => {
                let mut error=Ok(());
                for modification in modifications {
                    error=modification.apply(to, params);
                    if error.is_ok() {break}
                }
                error?
            },
            Self::IgnoreError(modification) => {let _=modification.apply(to, params);},
            Self::TryElse{r#try, r#else} => r#try.apply(to, params).or_else(|_| r#else.apply(to, params))?,
            Self::Debug(modification) => {
                let to_before_mapper=to.clone();
                let modification_result=modification.apply(to, params);
                eprintln!("=== StringModification::Debug ===\nModification: {modification:?}\nParams: {params:?}\nString before mapper: {to_before_mapper:?}\nModification return value: {modification_result:?}\nString after mapper: {to:?}");
                modification_result?;
            },
            #[cfg(feature = "commands")]
            Self::CommandOutput(command) => *to=command.output(None, Some(to.as_bytes()))?,
            Self::Error => Err(StringModificationError::ExplicitError)?
        };
        Ok(())
    }
}

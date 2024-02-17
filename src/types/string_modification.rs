use serde::{Serialize, Deserialize};
use urlencoding;

use super::{StringError, neg_index, neg_range};
#[cfg(feature = "regex")]
use crate::glue::RegexWrapper;
use crate::glue::string_or_struct;
use crate::config::Params;

/// A wrapper around [`str`]'s various substring modification functions.
/// [`isize`] is used to allow Python-style negative indexing.
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
pub enum StringModification {
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
    /// If the specified index is out of bounds or not on a UTF-8 character boundary, returns the errorr [`StringError::InvalidIndex`].
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
    /// Discards everything outside the spcified range.
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
        /// THe flag to check the setness of.
        flag: String,
        /// The string modification to apply if the flag is set.
        then: Box<Self>,
        /// The string modification to apply if the flag is not set.
        r#else: Box<Self>
    },
    URLEncode,
    URLDecode,
    All(Vec<Self>),
    AllNoRevert(Vec<Self>),
    AllIgnoreError(Vec<Self>),
    FirstNotError(Vec<Self>)
}

impl StringModification {
    /// Apply the modification in-place using the provided [`Params`].
    /// # Errors
    /// See the docs for each [`Self`] variant for details on which operations error and when.
    pub fn apply(&self, to: &mut String, params: &Params) -> Result<(), StringError> {
        match self {
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
            Self::URLEncode => *to=urlencoding::encode(to).into_owned(),
            Self::URLDecode => *to=urlencoding::decode(to)?.into_owned(),
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

        };
        Ok(())
    }
}

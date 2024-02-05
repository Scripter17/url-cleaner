use serde::{Serialize, Deserialize};

use super::{StringError, f, fo, r};

/// Where and how to modify a string. Used by [`crate::rules::mappers::Mapper::ModifyPart`].
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
pub enum StringModification {
    /// Replaces the entire target string with the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Set("ghi".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "ghi");
    /// ```
    Set(String),
    /// Append the contained string to the end of the part.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Append("ghi".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "abcdefghi");
    /// ```
    Append(String),
    /// Prepend the contained string to the beginning of the part.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Prepend("ghi".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "ghiabcdef");
    /// ```
    Prepend(String),
    /// Replace all instances of `find` with `replace`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcabc".to_string();
    /// assert!(StringModification::Replace{find: "ab".to_string(), replace: "xy".to_string()}.apply(&mut x).is_ok());
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
    /// If either end of the specified range is either not on a UTF-8 boundary or out of bounds, returns the error [`StringError::InvalidSlice`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::ReplaceRange{start: Some( 6), end: Some( 7), replace: "123" .to_string()}.apply(&mut x).is_err());
    /// assert_eq!(&x, "abcdef");
    /// assert!(StringModification::ReplaceRange{start: Some( 1), end: Some( 4), replace: "ab"  .to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "aabef");
    /// assert!(StringModification::ReplaceRange{start: Some(-3), end: Some(-1), replace: "abcd".to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "aaabcdf");
    /// assert!(StringModification::ReplaceRange{start: Some(-3), end: None    , replace: "efg" .to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "aaabefg");
    /// assert!(StringModification::ReplaceRange{start: Some(-8), end: None    , replace: "hij" .to_string()}.apply(&mut x).is_err());
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
    /// let mut x = "ABCdef".to_string();
    /// assert!(StringModification::Lowercase.apply(&mut x).is_ok());
    /// assert_eq!(&x, "abcdef");
    /// ```
    Lowercase,
    /// [`str::to_uppercase`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcDEF".to_string();
    /// assert!(StringModification::Uppercase.apply(&mut x).is_ok());
    /// assert_eq!(&x, "ABCDEF");
    /// ```
    Uppercase,
    /// [`str::strip_prefix`].
    /// # Errors
    /// If the provided string doesn't begin with the specified prefix, returns the error [`StringError::PrefixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "def");
    /// assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x).is_err());
    /// assert_eq!(&x, "def");
    /// ```
    StripPrefix(String),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the provided string doesn't end with the specified suffix, returns the error [`StringError::SuffixNotFound`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "abc");
    /// assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x).is_err());
    /// assert_eq!(&x, "abc");
    /// ```
    StripSuffix(String),
    /// [`Self::StripPrefix`] but does nothing if the provided string doesn't begin with the specified prefix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "def");
    /// assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "def");
    /// ```
    StripMaybePrefix(String),
    /// [`Self::StripSuffix`] but does nothing if the provided string doesn't end with the specified suffix.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "abc");
    /// assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x).is_ok());
    /// assert_eq!(&x, "abc");
    /// ```
    StripMaybeSuffix(String),
    /// [`str::replacen`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdefaaa".to_string();
    /// assert!(StringModification::ReplaceN{find: "a".to_string(), replace: "x".to_string(), count: 2}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "xbcdefxaa");
    /// ```
    ReplaceN {
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String,
        /// The number of times to do the replacement.
        count: usize
    },
    /// [`String::insert_str`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abc".to_string();
    /// assert!(StringModification::Insert{r#where:  0, value: "def".to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "defabc");
    /// assert!(StringModification::Insert{r#where:  2, value: "ghi".to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "deghifabc");
    /// assert!(StringModification::Insert{r#where: -1, value: "jhk".to_string()}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "deghifabjhkc");
    /// ```
    Insert {
        /// The location to insert `value`.
        r#where: isize,
        /// The string to insert.
        value: String
    },
    /// [`String::remove`]
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdef".to_string();
    /// assert!(StringModification::Remove(1).apply(&mut x).is_ok());
    /// assert_eq!(&x, "acdef");
    /// ```
    Remove(isize),
    /// Discards everything outside the spcified range.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::StringModification;
    /// let mut x = "abcdefghi".to_string();
    /// assert!(StringModification::GetRange{start: Some( 1), end: Some(8)}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "bcdefgh");
    /// assert!(StringModification::GetRange{start: None    , end: Some(6)}.apply(&mut x).is_ok());
    /// assert_eq!(&x, "bcdefg");
    /// assert!(StringModification::GetRange{start: Some(-3), end: None   }.apply(&mut x).is_ok());
    /// assert_eq!(&x, "efg");
    /// ```
    GetRange {
        /// The start of the range to keep.
        start: Option<isize>,
        /// The end of the range to keep.
        end: Option<isize>
    }
}

impl StringModification {
    /// Apply the modification in-place.
    /// # Errors
    /// If the location is [`Self::ReplaceRange`] and at least one end of the range is out of bounds or not on a UTF-8 character boundary, returns the error [`StringError::InvalidSlice`].
    /// If the location is [`Self::StripPrefix`] and the prefix is not found, returns the error [`StringError::PrefixNotFound`].
    /// If the location is [`Self::StripSuffix`] and the suffix is not found, returns the error [`StringError::SuffixNotFound`].
    pub fn apply(&self, to: &mut String) -> Result<(), StringError> {
        match self {
            Self::Set(value)                        => *to=value.clone(),
            Self::Append(value)                     => to.push_str(value),
            Self::Prepend(value)                    => {let mut ret=value.to_string(); ret.push_str(to); *to=ret},
            Self::Replace{find, replace}            => *to=to.replace(find, replace),
            Self::ReplaceRange{start, end, replace} => if fo(to, *start)?.map_or(true, |i| to.is_char_boundary(i)) && fo(to, *end)?.map_or(true, |i| to.is_char_boundary(i)) {to.replace_range(r(fo(to, *start)?, fo(to, *end)?), replace)} else {Err(StringError::InvalidSlice)?}, // Why does `String::try_replace_range` not exist???
            Self::Lowercase                         => *to=to.to_lowercase(),
            Self::Uppercase                         => *to=to.to_uppercase(),
            Self::StripPrefix(prefix)               => if to.starts_with(prefix) {to.drain(..prefix.len());} else {Err(StringError::PrefixNotFound)?},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripSuffix(suffix)               => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringError::SuffixNotFound)?},
            Self::StripMaybePrefix(prefix)          => if to.starts_with(prefix) {to.drain(..prefix.len());},
            #[allow(clippy::arithmetic_side_effects)] // `suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.
            Self::StripMaybeSuffix(suffix)          => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())},
            Self::ReplaceN{find, replace, count}    => *to=to.replacen(find, replace, *count),
            Self::Insert{r#where, value}            => if to.is_char_boundary(f(to, *r#where)?) {to.insert_str(f(to, *r#where)?, value)} else {Err(StringError::InvalidLocation)?}
            Self::Remove(r#where)                   => if to.is_char_boundary(f(to, *r#where)?) {to.remove(f(to, *r#where)?);} else {Err(StringError::InvalidLocation)?},
            Self::GetRange{start, end}              => *to=to.get(r(fo(to, *start)?, fo(to, *end)?)).ok_or(StringError::InvalidSlice)?.to_string()
        };
        Ok(())
    }
}

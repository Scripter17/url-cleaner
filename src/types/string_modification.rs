use thiserror::Error;

use serde::{Serialize, Deserialize};

/// Where and how to modify a string. Used by [`crate::rules::mappers::Mapper::ModifyPart`].
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq)]
pub enum StringModification {
    /// Replaces the entire target string with the specified string.
    Set(String),
    /// Append the contained string to the end of the part.
    Append(String),
    /// Prepend the contained string to the beginning of the part.
    Prepend(String),
    /// Replace all instances of `find` with `replace`.
    Replace{
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String
    },
    /// Replace the specified range with `replace`.
    /// # Errors
    /// If either end of the specified range is either not on a UTF-8 boundary or out of bounds, returns the error [`StringError::InvalidSlice`].
    ReplaceAt{
        /// The start of the range to replace.
        start: usize,
        /// The end of the range to replace.
        end: usize,
        /// The value to replace the range with.
        replace: String
    },
    /// [`str::to_lowercase`].
    Lowercase,
    /// [`str::to_uppercase`].
    Uppercase,
    /// [`str::strip_prefix`].
    /// # Errors
    /// If the provided string doesn't begin with the specified prefix, returns the error [`StringError::PrefixNotFound`].
    StripPrefix(String),
    /// Mimics [`str::strip_suffix`] using [`str::ends_with`] and [`String::truncate`]. Should be faster due to not needing an additional heap allocation.
    /// # Errors
    /// If the provided string doesn't end with the specified suffix, returns the error [`StringError::SuffixNotFound`].
    StripSuffix(String),
    /// [`Self::StripPrefix`] but does nothing if the provided string doesn't begin with the specified prefix.
    StripMaybePrefix(String),
    /// [`Self::StripSuffix`] but does nothing if the provided string doesn't end with the specified suffix.
    StripMaybeSuffix(String),
    /// [`str::replacen`].
    ReplaceN{
        /// The value to look for.
        find: String,
        /// The value to replace with.
        replace: String,
        /// The number of times to do the replacement.
        count: usize
    }
}

impl StringModification {
    /// Apply the modification in-place.
    pub fn apply(&self, to: &mut String) -> Result<(), StringError> {
        match self {
            Self::Set(value)                     => *to=value.clone(),
            Self::Append(value)                  => to.push_str(value),
            Self::Prepend(value)                 => {let mut ret=value.to_string(); ret.push_str(to); *to=ret},
            Self::Replace{find, replace}         => *to=to.replace(find, replace),
            Self::ReplaceAt{start, end, replace} => if to.get(*start..*end).is_some() {to.replace_range(start..end, replace)} else {Err(StringError::InvalidSlice)?}, // Why does `String::try_replace_range` not exist???
            Self::Lowercase                      => *to=to.to_lowercase(),
            Self::Uppercase                      => *to=to.to_uppercase(),
            Self::StripPrefix(prefix)            => if to.starts_with(prefix) {std::mem::drop(to.drain(..prefix.len()))} else {Err(StringError::PrefixNotFound)?},
            Self::StripSuffix(suffix)            => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringError::SuffixNotFound)?},
            Self::StripMaybePrefix(prefix)       => if to.starts_with(prefix) {std::mem::drop(to.drain(..prefix.len()))},
            Self::StripMaybeSuffix(suffix)       => if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())},
            Self::ReplaceN{find, replace, count} => *to=to.replacen(find, replace, *count)
        };
        Ok(())
    }
}

/// The enum of all possible errors that can happen when using `StringModification`.
#[derive(Debug, Clone, Error)]
pub enum StringError {
    /// The requested slice either was not on a UTF-8 boundary or was out of bounds.
    #[error("The requested slice either was not on a UTF-8 boundary or was out of bounds.")]
    InvalidSlice,
    /// The provided string did not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try StringModification::StripMaybePrefix?")]
    PrefixNotFound,
    /// The provided string did not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try StringModification::StripMaybeSuffix?")]
    SuffixNotFound
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn string_modification_append() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::Append("ghi".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "abcdefghi");
    }

    #[test]
    fn string_modification_prepend() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::Prepend("ghi".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "ghiabcdef");
    }

    #[test]
    fn string_modification_replace() {
        let mut x = "abcabc".to_string();
        assert!(StringModification::Replace{find: "ab".to_string(), replace: "xy".to_string()}.apply(&mut x).is_ok());
        assert_eq!(&x, "xycxyc");
    }

    #[test]
    fn string_modification_replace_at() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::ReplaceAt{start: 6, end: 7, replace: "g".to_string()}.apply(&mut x).is_err());
        assert_eq!(&x, "abcdef");
        assert!(StringModification::ReplaceAt{start: 1, end: 4, replace: "...".to_string()}.apply(&mut x).is_ok());
        assert_eq!(&x, "a...ef");
    }

    #[test]
    fn string_modification_lowercase() {
        let mut x = "ABCdef".to_string();
        assert!(StringModification::Lowercase.apply(&mut x).is_ok());
        assert_eq!(&x, "abcdef");
    }

    #[test]
    fn string_modification_uppercase() {
        let mut x = "abcDEF".to_string();
        assert!(StringModification::Uppercase.apply(&mut x).is_ok());
        assert_eq!(&x, "ABCDEF");
    }

    #[test]
    fn string_modification_strip_prefix() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "def");
        assert!(StringModification::StripPrefix("abc".to_string()).apply(&mut x).is_err());
        assert_eq!(&x, "def");
    }

    #[test]
    fn string_modification_strip_suffix() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "abc");
        assert!(StringModification::StripSuffix("def".to_string()).apply(&mut x).is_err());
        assert_eq!(&x, "abc");
    }

    #[test]
    fn string_modification_strip_maybe_prefix() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "def");
        assert!(StringModification::StripMaybePrefix("abc".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "def");
    }

    #[test]
    fn string_modification_strip_maybe_suffix() {
        let mut x = "abcdef".to_string();
        assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "abc");
        assert!(StringModification::StripMaybeSuffix("def".to_string()).apply(&mut x).is_ok());
        assert_eq!(&x, "abc");
    }

    #[test]
    fn string_modification_replacen() {
        let mut x = "abcdefaaa".to_string();
        assert!(StringModification::ReplaceN{find: "a".to_string(), replace: "x".to_string(), count: 2}.apply(&mut x).is_ok());
        assert_eq!(&x, "xbcdefxaa");
    }
}

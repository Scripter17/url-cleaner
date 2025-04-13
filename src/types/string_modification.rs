//! Rules for modifying strings.

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

#[derive(Debug, Clone,Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[serde(remote = "Self")]
pub enum StringModification {
    /// Doesn't do any modification.
    None,



    /// Print debug info about the contained [`Self`] and its call to [`Self::get`].
    ///
    /// The exact info printed is unspecified and subject to change at any time for any reason.
    /// # Suitability
    /// Always unsuiable to be in the default config.
    /// # Errors
    /// If the call to [`Self::get`] returns an error, that error is returned.
    #[suitable(never)]
    Debug(Box<Self>),



    /// Always returns the error [`StringModificationError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`StringModificationError::ExplicitError`].
    Error(String),
    /// If the contained [`Self`] returns an error, ignore it.
    ///
    /// Does not revert any successful calls to [`Self::apply`]. For that, also use [`Self::RevertOnError`].
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the [`String`] to its previous value then return the error.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    RevertOnError(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::apply`] returns an error, apply [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::apply`] return errors, both errors are returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },
    /// Applies the contained [`Self`]s in order.
    /// # Errors
    /// If any call to [`Self::apply`] returns an error, that error is returned.
    All(Vec<Self>),
    /// Applies the contained [`Self`]s in order, stopping as soon as a call to [`Self::apply`] doesn't return an error.
    /// # Errors
    /// If all calls to [`Self::apply`] return errors, the last error is returned. In the future this should be changed to return all errors.
    FirstNotError(Vec<Self>),



    /// If the call to [`Self::If::if`] passes, apply [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails and [`Self::If::else`] is [`Some`], apply [`Self::If::else`].
    /// # Errors
    /// If the call to [`Condition::satisifed_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    If {
        /// The [`Condition`] to decide between [`Self::If::mapper`] and [`Self::If::else_mapper`].
        condition: Box<Condition>,
        /// The [`Self`] to apply if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::If::if`] fails.
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
    /// See [`String::to_lowercase`] for details.
    Lowercase,
    /// Sets the string to uppercase.
    ///
    /// See [`String::to_uppercase`] for details.
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
    /// Replcae up to [`Self::Replacen::count`] instances of [`Self::Replacen::find`] with [`Self::Replacen::replace`].
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



    #[cfg(feature = "regex")]
    RegexCaptures {
        regex: RegexWrapper,
        replace: StringSource
    },
    #[cfg(feature = "regex")]
    JoinAllRegexCaptures {
        regex: RegexWrapper,
        replace: StringSource,
        join: StringSource
    },
    #[cfg(feature = "regex")]
    RegexFind(RegexWrapper),
    #[cfg(feature = "regex")]
    RegexReplace {
        regex: RegexWrapper,
        replace: StringSource
    },
    #[cfg(feature = "regex")]
    RegexReplaceAll {
        regex: RegexWrapper,
        replace: StringSource
    },
    #[cfg(feature = "regex")]
    RegexReplacen {
        regex: RegexWrapper,
        n: usize,
        replace: StringSource
    },



    UrlEncode(UrlEncodeAlphabet),
    UrlDecode,
    #[cfg(feature = "base64")]
    Base64Encode(#[serde(default)] Base64Config),
    #[cfg(feature = "base64")]
    Base64Decode(#[serde(default)] Base64Config),



    GetJsStringLiteralPrefix,
    UnescapeHtmlText,
    GetHtmlAttribute(StringSource),



    JsonPointer(StringSource),
    RemoveQueryParamsMatching(Box<StringMatcher>),
    AllowQueryParamsMatching(Box<StringMatcher>),
    ExtractBetween {
        start: StringSource,
        end: StringSource
    },
    KeepNthSegment {
        split: StringSource,
        index: isize
    },
    KeepSegmentRange {
        split: StringSource,
        #[serde(default, skip_serializing_if = "is_default")]
        start: isize,
        #[serde(default, skip_serializing_if = "is_default")]
        end: Option<isize>
    },
    SetSegment {
        split: StringSource,
        index: isize,
        value: StringSource
    },
    InsertSegment {
        split: StringSource,
        index: isize,
        value: StringSource
    },
    StringMap {
        value: Box<StringSource>,
        #[serde(flatten)]
        map: Map<Self>,
    },
    Common(CommonCall),
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut String, &TaskStateView) -> Result<(), StringModificationError>)
}

/// The [`AsciiSet`] that emulates [`encodeURIComponent`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent).
pub const JS_ENCODE_URI_COMPONENT_ASCII_SET: AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'-').remove(b'_').remove(b'.')
    .remove(b'!').remove(b'~').remove(b'*')
    .remove(b'\'').remove(b'(').remove(b')');

/// THe [`AsciiSet`] that emulates [`encodeURI`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI).
pub const JS_ENCODE_URI_ASCII_SET: AsciiSet = JS_ENCODE_URI_COMPONENT_ASCII_SET
    .remove(b';').remove(b'/').remove(b'?')
    .remove(b':').remove(b'@').remove(b'&')
    .remove(b'=').remove(b'+').remove(b'$')
    .remove(b',').remove(b'#');

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum UrlEncodeAlphabet {
    #[default]
    JsEncodeUriComponent,
    JsEncodeUri,
    NonAlphanumeric
}

impl UrlEncodeAlphabet {
    pub fn get(&self) -> &'static AsciiSet {
        match self {
            Self::JsEncodeUriComponent => &JS_ENCODE_URI_COMPONENT_ASCII_SET,
            Self::JsEncodeUri          => &JS_ENCODE_URI_ASCII_SET,
            Self::NonAlphanumeric      => NON_ALPHANUMERIC
        }
    }
}

string_or_struct_magic!(StringModification);

#[derive(Debug, Error)]
#[error("Tried deserializing a StringModification variant with non-defaultable fields from a string.")]
pub struct NonDefaultableVariant;

impl FromStr for StringModification {
    type Err = NonDefaultableVariant;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            #[cfg(feature = "base64")] "Base64Decode" => StringModification::Base64Decode(Default::default()),
            #[cfg(feature = "base64")] "Base64Encode" => StringModification::Base64Encode(Default::default()),
            "UrlDecode"                => StringModification::UrlDecode,
            "UrlEncode"                => StringModification::UrlEncode(Default::default()),
            "None"                     => StringModification::None,
            "Lowercase"                => StringModification::Lowercase,
            "Uppercase"                => StringModification::Uppercase,
            "GetJsStringLiteralPrefix" => StringModification::GetJsStringLiteralPrefix,
            "UnescapeHtmlText"         => StringModification::UnescapeHtmlText,
            _                          => Err(NonDefaultableVariant)?
        })
    }
}

/// The enum of errors [`StringModification::apply`] can return.
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// Returned when a [`StringModification::ExplicitError`] is used.
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
    /// Returned when a JSON value isn't a string.
    #[error("The requested JSON value was not a string.")]
    JsonValueIsNotAString,
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
    /// Returned wehn the string being modified doesn't start with the specified prefix.
    #[error("The string being modified didn't start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// Returned when the string being modified doesn't end with the specified suffix.
    #[error("The string being modified didn't end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
    /// Returned when a [`Regex`] doesn't find any matches in the string.
    #[error("The regex didn't find any matches in the string.")]
    #[cfg(feature = "regex")]
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
    /// Returned when a [`StringModification`] with the specified name ins't found in the [`Commons::string_modifications`].
    #[error("A StringModification with the specified name wasn't found in the Commons::string_modifications.")]
    CommonStringModificationNotFound,
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] Box<ConditionError>),
    /// Returned when a [`parse::js::StringLiteralPrefixErro`] is encountered.
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
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// An arbitrary [`std::error::Error`] returned by [`StringModification::Custom`].
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
    pub fn apply(&self, to: &mut String, task_state: &TaskStateView) -> Result<(), StringModificationError> {
        debug!(StringModification::apply, self);
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
            }
            Self::TryElse{r#try, r#else} => r#try.apply(to, task_state).or_else(|try_error| r#else.apply(to, task_state).map_err(|else_error| StringModificationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::All(modifications) => {
                for modification in modifications {
                    modification.apply(to, task_state)?;
                }
            }
            Self::FirstNotError(modifications) => {
                let mut error=Ok(());
                for modification in modifications {
                    error=modification.apply(to, task_state);
                    if error.is_ok() {break}
                }
                error?
            },
            Self::If {condition, then, r#else} => if condition.satisfied_by(task_state)? {
                then.apply(to, task_state)?
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?
            },
            Self::IfMatches {matcher, then, r#else} => if matcher.satisfied_by(to, task_state)? {
                then.apply(to, task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(to, task_state)?
            },
            Self::SetSegment {split, index, value} => {
                let split = get_cow!(split, task_state, StringModificationError);
                let mut segments = to.split(&*split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_index = neg_index(*index, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                match value.get(task_state)? {
                    Some(value) => *segments.get_mut(fixed_index).expect("StringModification::SetSegment to be implemented correctly") = value,
                    None => {segments.remove(fixed_index);}
                }
                *to = segments.join(&*split);
            },
            Self::InsertSegment {split, index, value} => {
                let split = get_cow!(split, task_state, StringModificationError);
                let mut segments = to.split(&*split).map(Cow::Borrowed).collect::<Vec<_>>();
                let fixed_index = neg_range_boundary(*index, segments.len()).ok_or(StringModificationError::SegmentNotFound)?;
                if let Some(value) = value.get(task_state)? {
                    segments.insert(fixed_index, value);
                }
                *to = segments.join(&*split);
            },
            Self::Set(value)                         => get_string!(value, task_state, StringModificationError).clone_into(to),
            Self::Append(value)                      => to.push_str(get_str!(value, task_state, StringModificationError)),
            Self::Prepend(value)                     => {let mut ret=get_string!(value, task_state, StringModificationError); ret.push_str(to); *to=ret;},
            Self::Replace{find, replace}             => *to=to.replace (get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError)),
            Self::Replacen{find, replace, count}     => *to=to.replacen(get_str!(find, task_state, StringModificationError), get_str!(replace, task_state, StringModificationError), *count),
            Self::ReplaceRange{start, end, replace}  => {
                let range=neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?;
                if to.get(range).is_some() {
                    to.replace_range(range, get_str!(replace, task_state, StringModificationError));
                } else {
                    Err(StringModificationError::InvalidSlice)?;
                }
            },
            Self::Lowercase => *to=to.to_lowercase(),
            Self::Uppercase => *to=to.to_uppercase(),
            Self::StripPrefix(prefix) => {
                let prefix = get_str!(prefix, task_state, StringModificationError);
                if to.starts_with(prefix) {to.drain(..prefix.len());} else {Err(StringModificationError::PrefixNotFound)?;};
            },
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripSuffix(suffix)                => {
                let suffix = get_str!(suffix, task_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len())} else {Err(StringModificationError::SuffixNotFound)?;};
            },
            Self::StripMaybePrefix(prefix)           => {
                let prefix = get_str!(prefix, task_state, StringModificationError);
                if to.starts_with(prefix) {to.drain(..prefix.len());};
            },
            #[expect(clippy::arithmetic_side_effects, reason = "`suffix.len()>=to.len()` is guaranteed by `to.ends_with(suffix)`.")]
            Self::StripMaybeSuffix(suffix)           => {
                let suffix = get_str!(suffix, task_state, StringModificationError);
                if to.ends_with  (suffix) {to.truncate(to.len()-suffix.len());};
            },
            Self::Insert{index, value} => if to.is_char_boundary(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.insert_str(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?, get_str!(value, task_state, StringModificationError));} else {Err(StringModificationError::InvalidIndex)?;},
            Self::RemoveChar(index)    => if to.is_char_boundary(neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.remove    (neg_index(*index, to.len()).ok_or(StringModificationError::InvalidIndex)?                                                      );} else {Err(StringModificationError::InvalidIndex)?;},
            Self::KeepRange{start, end}  => *to = to.get(neg_range(*start, *end, to.len()).ok_or(StringModificationError::InvalidSlice)?).ok_or(StringModificationError::InvalidSlice)?.to_string(),
            #[cfg(feature = "regex")]
            Self::RegexCaptures {regex, replace} => {
                let replace = get_str!(replace, task_state, StringModificationError);
                let mut temp = "".to_string();
                regex.get()?.captures(to).ok_or(StringModificationError::RegexMatchNotFound)?.expand(replace, &mut temp);
                *to = temp;
            },
            #[cfg(feature = "regex")]
            Self::JoinAllRegexCaptures {regex, replace, join} => {
                let replace = get_str!(replace, task_state, StringModificationError);
                let join = get_str!(join, task_state, StringModificationError);
                let mut temp = "".to_string();
                if join.is_empty() {
                    for captures in regex.get()?.captures_iter(to) {
                        captures.expand(replace, &mut temp);
                    }
                } else {
                    let mut iter = regex.get()?.captures_iter(to).peekable();
                    while let Some(captures) = iter.next() {
                        captures.expand(replace, &mut temp);
                        if iter.peek().is_some() {temp.push_str(join);}
                    }
                }
                *to = temp;
            },
            #[cfg(feature = "regex")] Self::RegexFind       (regex            ) => *to = regex.get()?.find       (to                                                           ).ok_or(StringModificationError::RegexMatchNotFound)?.as_str().to_string(),
            #[cfg(feature = "regex")] Self::RegexReplace    {regex,    replace} => *to = regex.get()?.replace    (to,     get_str!(replace, task_state, StringModificationError)).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplaceAll {regex,    replace} => *to = regex.get()?.replace_all(to,     get_str!(replace, task_state, StringModificationError)).into_owned(),
            #[cfg(feature = "regex")] Self::RegexReplacen   {regex, n, replace} => *to = regex.get()?.replacen   (to, *n, get_str!(replace, task_state, StringModificationError)).into_owned(),
            Self::UrlEncode(alphabet) => *to=utf8_percent_encode(to, alphabet.get()).to_string(),
            Self::UrlDecode => *to=percent_decode_str(to).decode_utf8()?.into_owned(),
            #[cfg(feature = "base64")] Self::Base64Encode(config) => *to = config.build().encode(to.as_bytes()),
            #[cfg(feature = "base64")] Self::Base64Decode(config) => *to = String::from_utf8(config.build().decode(to.as_bytes())?)?,
            Self::JsonPointer(pointer) => *to = serde_json::from_str::<serde_json::Value>(to)?.pointer(get_str!(pointer, task_state, StringModificationError)).ok_or(StringModificationError::JsonValueNotFound)?.as_str().ok_or(StringModificationError::JsonValueIsNotAString)?.to_string(),


            Self::RemoveQueryParamsMatching(matcher) => *to = to.split('&').filter_map(|kev|
                matcher.satisfied_by(
                    kev.split('=').next().unwrap_or("Why can't I #[allow] an .expect() here?"),
                    task_state
                )
                .map(|x| (!x).then_some(kev))
                .transpose()
            ).collect::<Result<Vec<_>, _>>()?.join("&"),
            Self::AllowQueryParamsMatching(matcher) => *to = to.split('&').filter_map(|kev|
                matcher.satisfied_by(
                    kev.split('=').next().unwrap_or("Why can't I #[allow] an .expect() here?"),
                    task_state
                )
                .map(|x| x.then_some(kev))
                .transpose()
            ).collect::<Result<Vec<_>, _>>()?.join("&"),



            Self::ExtractBetween {start, end} => {
                *to = to
                    .split_once(get_str!(start, task_state, StringModificationError))
                    .ok_or(StringModificationError::ExtractBetweenStartNotFound)?
                    .1
                    .split_once(get_str!(end, task_state, StringModificationError))
                    .ok_or(StringModificationError::ExtractBetweenEndNotFound)?
                    .0
                    .to_string();
            },
            Self::Common(common_call) => {
                task_state.commons.string_modifications.get(get_str!(common_call.name, task_state, StringModificationError)).ok_or(StringModificationError::CommonStringModificationNotFound)?.apply(
                    to,
                    &TaskStateView {
                        url: task_state.url,
                        context: task_state.context,
                        params: task_state.params,
                        scratchpad: task_state.scratchpad,
                        #[cfg(feature = "cache")]
                        cache: task_state.cache,
                        commons: task_state.commons,
                        common_args: Some(&common_call.args.build(task_state)?),
                        job_context: task_state.job_context
                    }
                )?
            },

            Self::GetJsStringLiteralPrefix => *to = parse::js::string_literal_prefix(to)?,
            Self::UnescapeHtmlText => *to = parse::html::unescape_text(to)?,
            Self::GetHtmlAttribute(name) => *to = parse::html::get_attribute_value(to, get_str!(name, task_state, StringModificationError))?.ok_or(StringModificationError::HtmlAttributeNotFound)?.ok_or(StringModificationError::HtmlAttributeHasNoValue)?,

            Self::StringMap {value, map} => if let Some(x) = map.get(value.get(task_state)?) {x.apply(to, task_state)?;},

            Self::KeepNthSegment {split, index} => *to = neg_nth(to.split(get_str!(split, task_state, StringModificationError)), *index).ok_or(StringModificationError::SegmentNotFound)?.to_string(),
            Self::KeepSegmentRange {split, start, end} => {
                let split = get_str!(split, task_state, StringModificationError);
                *to = neg_vec_keep(to.split(split), *start, *end).ok_or(StringModificationError::SegmentRangeNotFound)?.join(split);
            },

            #[cfg(feature = "custom")]
            Self::Custom(function) => function(to, task_state)?
        };
        Ok(())
    }
}

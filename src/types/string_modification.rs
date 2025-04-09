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



    Error(String),
    IgnoreError(Box<Self>),
    TryElse {
        r#try: Box<Self>,
        r#else: Box<Self>
    },
    All(Vec<Self>),
    AllNoRevert(Vec<Self>),
    AllIgnoreError(Vec<Self>),
    FirstNotError(Vec<Self>),



    If {
        condition: Box<Condition>,
        then: Box<Self>,
        r#else: Option<Box<Self>>
    },
    IfMatches {
        matcher: Box<StringMatcher>,
        then: Box<Self>,
        #[serde(default)]
        r#else: Option<Box<Self>>
    },



    Set(StringSource),
    Append(StringSource),
    Prepend(StringSource),
    Replace {
        find: StringSource,
        replace: StringSource
    },
    ReplaceRange {
        start: isize,
        end: Option<isize>,
        replace: StringSource
    },
    Lowercase,
    Uppercase,
    StripPrefix(StringSource),
    StripSuffix(StringSource),
    StripMaybePrefix(StringSource),
    StripMaybeSuffix(StringSource),
    Replacen {
        find: StringSource,
        replace: StringSource,
        count: usize
    },
    Insert {
        r#where: isize,
        value: StringSource
    },
    RemoveChar(isize),
    KeepRange {
        start: isize,
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
    Unescape(UnescapeMode),



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
    Custom(FnWrapper<fn(&mut String, &TaskStateView) -> Result<(), StringModificationError>>)
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
            _                          => Err(NonDefaultableVariant)?
        })
    }
}

#[allow(clippy::enum_variant_names, reason = "I disagree.")]
#[derive(Debug, Error)]
pub enum StringModificationError {
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("The requested JSON value was not found.")]
    JsonValueNotFound,
    #[error("The requested JSON value was not a string.")]
    JsonValueIsNotAString,
    #[error("The requested slice was either not on a UTF-8 boundary or out of bounds.")]
    InvalidSlice,
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    #[error("The requested segments were not found.")]
    SegmentRangeNotFound,
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
    #[error("The requested regex pattern was not found in the provided string.")]
    RegexMatchNotFound,
    #[cfg(feature = "regex")]
    #[error(transparent)]
    RegexError(#[from] ::regex::Error),
    #[error("A `StringModification::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    },
    #[error(transparent)]
    #[cfg(feature = "base64")]
    Base64DecodeError(#[from] ::base64::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[error("The provided string was not in the specified map.")]
    StringNotInMap,
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    #[error("The `start` of an `ExtractBetween` was not found in the provided string.")]
    ExtractBetweenStartNotFound,
    #[error("The `end` of an `ExtractBetween` was not found in the provided string.")]
    ExtractBetweenEndNotFound,
    #[error(transparent)]
    UnescapeError(#[from] UnescapeError),
    #[error("The common StringModification was not found.")]
    CommonStringModificationNotFound,
    #[error(transparent)]
    ConditionError(#[from] Box<ConditionError>),
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
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
            Self::TryElse{r#try, r#else} => r#try.apply(to, task_state).or_else(|try_error| r#else.apply(to, task_state).map_err(|else_error| StringModificationError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::All(modifications) => {
                let mut temp_to=to.clone();
                for modification in modifications {
                    modification.apply(&mut temp_to, task_state)?;
                }
                *to=temp_to;
            }
            Self::AllNoRevert(modifications) => {
                for modification in modifications {
                    modification.apply(to, task_state)?;
                }
            },
            Self::AllIgnoreError(modifications) => {
                for modification in modifications {
                    let _=modification.apply(to, task_state);
                }
            },
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
            Self::Insert{r#where, value} => if to.is_char_boundary(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.insert_str(neg_index(*r#where, to.len()).ok_or(StringModificationError::InvalidIndex)?, get_str!(value, task_state, StringModificationError));} else {Err(StringModificationError::InvalidIndex)?;},
            Self::RemoveChar(index)      => if to.is_char_boundary(neg_index(*index  , to.len()).ok_or(StringModificationError::InvalidIndex)?) {to.remove    (neg_index(*index  , to.len()).ok_or(StringModificationError::InvalidIndex)?                                                     );} else {Err(StringModificationError::InvalidIndex)?;},
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
            Self::Unescape(mode) => *to = mode.unescape(to)?,

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

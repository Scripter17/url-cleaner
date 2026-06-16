//! [`StringModification`].


#[expect(unused_imports, reason = "Used in a doc comment.")]
use regex::Regex;
use base64::prelude::*;

use crate::prelude::*;

/// Modify a string.
///
/// Defaults to [`Self::None`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum StringModification {
    /// Do nothing.
    ///
    /// The default.
    #[default]
    None,



    /// [`ExplicitError`].
    /// # Errors
    /// [`ExplicitError`].
    Error(String),
    /// Map [`Err`] to [`()`].
    IgnoreError(Box<Self>),
    /// Revert the inner [`Self`] if it returns [`Err`], then returns the error.
    RevertOnError(Box<Self>),
    /// [`Self::TryElse::try`], or [`Self::TryElse::else`] if it returns [`Err`].
    TryElse {
        /// The try.
        r#try: Box<Self>,
        /// The else.
        r#else: Box<Self>
    },
    /// All contained [`Self`]s.
    All(Vec<Self>),
    /// [`Self::All`] but stops after the first [`Ok`].
    /// # Errors
    /// If all return [`Err`], returns the error [`FirstNotErrorErrors`].
    FirstNotError(Vec<Self>),



    /// Only apply the [`Self`] if the string is [`Some`].
    IfSome(Box<Self>),
    /// If the string satisfies [`Self::IfMatches::matcher`], apply [`Self::IfMatches::then`].
    ///
    /// If the string does not satisfy [`Self::IfMatches::matcher`] and [`Self::IfMatches::else`] is [`Some`], apply [`Self::IfMatches::else`].
    IfMatches {
        /// The [`StringMatcher`] to check if the string satisfies.
        matcher: Box<StringMatcher>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] is satisfied.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::IfMatches::matcher`] isn't satisfied.
        ///
        /// Defaults to [`Self::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>
    },
    /// Indexes [`Self::Map::map`] with [`Self::Map::value`] and, if a [`Self`] is found, applies it.
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    StringMap {
        /// The value to index [`Self::Map::map`] with.
        value: Box<StringSource>,
        /// The [`Map`] to index with [`Self::Map::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>,
    },



    /// Sets the string to the specified value.
    Set(StringSource),
    /// Appends the specified value.
    Append(StringSource),
    /// Prepends the specified value.
    Prepend(StringSource),
    /// Sets the string to lowercase.
    Lowercase,
    /// Sets the string to uppercase.
    Uppercase,



    /// Removes the specified prefix.
    StripPrefix(StringSource),
    /// Removes the specified suffix.
    StripSuffix(StringSource),
    /// If the string starts with the specified value, remove it.
    TrimPrefix(StringSource),
    /// If the string ends with the specified value, remove it.
    TrimSuffix(StringSource),



    /// Finds the first instance of the specified substring and keeps only everything before it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepBefore(StringSource),
    /// Finds the first instance of the specified substring and removes everything after it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripAfter(StringSource),
    /// Finds the first instance of the specified substring and removes everything before it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripBefore(StringSource),
    /// Finds the first instance of the specified substring and keeps only everything after it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepAfter(StringSource),

    /// Finds the last instance of the specified substring and removes everything before it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripBeforeLast(StringSource),
    /// Finds the last instance of the specified substring and removes everything after it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripAfterLast(StringSource),
    /// Finds the last instance of the specified substring and keeps only everything before it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepBeforeLast(StringSource),
    /// Finds the last instance of the specified substring and keeps only everything after it.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepAfterLast(StringSource),

    /// [`Self::StripBefore`] but does nothing if the substring isn't found.
    TrimBefore(StringSource),
    /// [`Self::StripAfter`] but does nothing if the substring isn't found.
    TrimAfter(StringSource),
    /// [`Self::KeepAfter`] but does nothing if the substring isn't found.
    KeepTrimBefore(StringSource),
    /// [`Self::KeepAfter`] but does nothing if the substring isn't found.
    KeepTrimAfter(StringSource),

    /// [`Self::StripBeforeLast`] but does nothing if the substring isn't found.
    TrimBeforeLast(StringSource),
    /// [`Self::StripAfterLast`] but does nothing if the substring isn't found.
    TrimAfterLast(StringSource),
    /// [`Self::KeepAfterLast`] but does nothing if the substring isn't found.
    KeepTrimBeforeLast(StringSource),
    /// [`Self::KeepAfterLast`] but does nothing if the substring isn't found.
    KeepTrimAfterLast(StringSource),



    /// Replace up to [`Self::Replacen::count`] instances of [`Self::Replacen::find`] with [`Self::Replacen::replace`].
    ///
    /// See [`str::replacen`] for details.
    Replacen {
        /// The value to replace with [`Self::Replacen::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::Replacen::find`] with.
        replace: StringSource,
        /// The maximum amount of instances to replace.
        count: usize
    },
    /// Replace all instances of [`Self::ReplaceAll::find`] with [`Self::ReplaceAll::replace`].
    ReplaceAll {
        /// The value to replace with [`Self::ReplaceAll::replace`].
        find: StringSource,
        /// The value to replace instances of [`Self::ReplaceAll::find`] with.
        replace: StringSource
    },



    /// Parses the javascript string literal at the start of the string and returns its value.
    ///
    /// Useful in combination with [`Self::KeepAfter`].
    GetJsStringLiteralPrefix,
    /// Processes HTML character references/escape codes like `&map;` into `&` and `&41;` into `A`.
    UnescapeHtml,
    /// Parses the HTML element at the start of the string and returns the [`Self::UnescapeHtml`]ed value of the last attribute with the specified name.
    ///
    /// Useful in combination with [`Self::StripBefore`].
    GetHtmlAttribute(StringSource),



    /// Calls [`Regex::find`] and returns its value.
    RegexFind(LazyRegex),
    /// [`regex::Regex::captures`] + [`RegexExpansion::expand`].
    RegexExpansion {
        /// The [`LazyRegex`].
        regex: LazyRegex,
        /// The [`RegexExpansion`].
        expansion: Box<RegexExpansion>
    },
    /// [`Regex::replace`]s the first match of [`Self::RegexReplaceOne::regex`] with [`Self::RegexReplaceOne::replace`].
    RegexReplaceOne {
        /// The [`LazyRegex`] to search with.
        regex: LazyRegex,
        /// The format string to expand the capture with.
        replace: StringSource
    },
    /// [`Regex::replace`]s the all matches of [`Self::RegexReplaceAll::regex`] with [`Self::RegexReplaceAll::replace`].
    RegexReplaceAll {
        /// The [`LazyRegex`] to search with.
        regex: LazyRegex,
        /// The format string to expand the captures with.
        replace: StringSource
    },
    /// [`Regex::replacen`]s the first [`Self::RegexReplacen::n`] of [`Self::RegexReplacen::regex`] with [`Self::RegexReplacen::replace`].
    RegexReplacen {
        /// The [`LazyRegex`] to search with.
        regex: LazyRegex,
        /// The number of captures to find and replace.
        n: usize,
        /// The format string to expand the captures with.
        replace: StringSource
    },



    /// Parses the string as JSON and uses [`serde_json::Value::pointer_mut`] with the specified pointer.
    ///
    /// When extracting values from javascript, it's often faster to find the start of the desired string and use [`Self::GetJsStringLiteralPrefix`].
    JsonPointer(StringSource),



    /// [`better_url::util::encode_query_part`].
    EncodeQueryPart,
    /// [`better_url::util::try_decode_query_part`].
    TryDecodeQueryPart,
    /// [`better_url::util::lossy_decode_query_part`].
    LossyDecodeQueryPart,
    /// [`better_url::util::try_percent_decode`].
    TryPercentDecode,
    /// [`better_url::util::lossy_percent_decode`].
    LossyPercentDecode,



    /// Base64 encodes the string.
    Base64Encode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),
    /// Base64 decodes the string.
    Base64Decode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),



    /// Uses a [`Self`] from [`Cleaner::functions`].
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    FunctionArg(StringSource),
    /// Calls the contained function.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    #[suitable(never)]
    #[serde(skip)]
    Extern(StringModificationExtern)
}

/// The enum of errors [`StringModification::apply`] can return.
#[derive(Debug, Error)]
pub enum StringModificationError {
    /// [`ExplicitError`].
    #[error(transparent)]
    ExplicitError(#[from] ExplicitError),
    /// [`TryElseError`].
    #[error(transparent)]
    TryElseError(#[from] Box<TryElseError<Self>>),
    /// [`FirstNotErrorErrors`].
    #[error(transparent)]
    FirstNotErrorErrors(#[from] FirstNotErrorErrors<Self>),

    /// [`StringSourceError`].
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// [`StringNotFound`].
    #[error(transparent)]
    StringNotFound(#[from] StringNotFound),
    /// [`StringMatcherError`].
    #[error(transparent)]
    StringMatcherError(#[from] Box<StringMatcherError>),
    /// [`StringLocationError`].
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// [`RegexExpansionError`].
    #[error(transparent)]
    RegexExpansionError(#[from] Box<RegexExpansionError>),

    /// [`serde_json::Error`].
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Returned when a JSON value isn't found.
    #[error("The requested JSON value was not found.")]
    JsonValueNotFound,
    /// Returned when a JSON pointee isn't a string.
    #[error("The requested JSON pointee was not a string.")]
    JsonPointeeIsNotAString,

    /// Returned when a StringModification set a string to None when it had to keep/set it to Some.
    #[error("A StringModification set a string to None when it had to keep/set it to Some.")]
    StringModificationSetStringToNone,

    /// Returned when the string being modified doesn't start with the specified prefix.
    #[error("The string being modified didn't start with the provided prefix. Maybe try `StringModification::TrimPrefix`?")]
    PrefixNotFound,
    /// Returned when the string being modified doesn't end with the specified suffix.
    #[error("The string being modified didn't end with the provided suffix. Maybe try `StringModification::TrimSuffix`?")]
    SuffixNotFound,

    /// [`std::str::Utf8Error`].
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// [`std::string::FromUtf8Error`].
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// [`GetJsStringLiteralPrefixError`].
    #[error(transparent)]
    GetJsStringLiteralPrefixError(#[from] GetJsStringLiteralPrefixError),
    /// [`UnescapeHtmlError`].
    #[error(transparent)]
    HtmlUnescapeHtmlError(#[from] UnescapeHtmlError),
    /// [`GetHtmlAttributeError`].
    #[error(transparent)]
    HtmlGetAttributeValueError(#[from] GetHtmlAttributeError),
    /// Returned when the requested HTML attribute isn't found.
    #[error("The requested HTML attribute wasn't found.")]
    HtmlAttributeNotFound,
    /// Returned when the requested HTML attribute doesn't have a value.
    #[error("The requested HTML attribute doesn't have a value.")]
    HtmlAttributeHasNoValue,

    /// [`SubstringNotFound`].
    #[error(transparent)]
    SubstringNotFound(#[from] SubstringNotFound),
    /// [`SubjectIsNone`].
    #[error(transparent)]
    SubjectIsNone(#[from] SubjectIsNone),
    /// [`SetNotFound`].
    #[error(transparent)]
    SetNotFound(#[from] SetNotFound),
    /// [`ListNotFound`].
    #[error(transparent)]
    ListNotFound(#[from] ListNotFound),

    /// [`regex::Error`].
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Returned when a [`Regex`] doesn't find any matches in the string.
    #[error("The regex didn't find any matches in the string.")]
    RegexMatchNotFound,
    /// [`base64::DecodeError`].
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),

    /// [`FunctionNotFound`].
    #[error(transparent)]
    FunctionNotFound(#[from] FunctionNotFound),
    /// [`NotInFunction`].
    #[error(transparent)]
    NotInFunction(#[from] NotInFunction),
    /// [`FunctionArgFunctionNotFound`].
    #[error(transparent)]
    FunctionArgFunctionNotFound(#[from] FunctionArgFunctionNotFound),

    /// An arbitrary [`std::error::Error`] for use with [`StringModification::Extern`].
    #[error(transparent)]
    Extern(Box<dyn std::error::Error + Send + Sync>)
}

impl From<StringSourceError  > for StringModificationError {fn from(value: StringSourceError  ) -> Self {Box::new(value).into()}}
impl From<StringMatcherError > for StringModificationError {fn from(value: StringMatcherError ) -> Self {Box::new(value).into()}}
impl From<RegexExpansionError> for StringModificationError {fn from(value: RegexExpansionError) -> Self {Box::new(value).into()}}

impl StringModification {
    /// Modified a string.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, to: &mut Option<Cow<'t, str>>) -> Result<bool, StringModificationError> {
        debug!(StringModification::apply, self, args, to; self._apply(task_state, args, to))
    }

    /// [`Self::apply`].
    fn _apply<'j: 't, 't>(&'j self, task_state: &'t TaskState<'j>, args: Option<&'j FunctionArgs>, to: &mut Option<Cow<'t, str>>) -> Result<bool, StringModificationError> {
        Ok(match self {
            // Debug/constants

            Self::None       => false,
            Self::Error(msg) => Err(ExplicitError(msg.clone()))?,

            // Error handling

            Self::IgnoreError(modification) => modification.apply(task_state, args, to).unwrap_or_default(),
            Self::RevertOnError(modification) => {
                let old_to = to.clone();
                match modification.apply(task_state, args, to) {
                    Ok(x) => x,
                    Err(e) => {
                        *to = old_to;
                        Err(e)?
                    }
                }
            },
            Self::TryElse {r#try, r#else} => match r#try.apply(task_state, args, to) {
                Ok(x) => x,
                Err(try_error) => match r#else.apply(task_state, args, to) {
                    Ok(x) => x,
                    Err(else_error) => Err(TryElseError {try_error, else_error})?
                }
            },

            // Logic

            Self::All(modifications) => {
                let mut changed = false;

                for modification in modifications {
                    changed |= modification.apply(task_state, args, to)?;
                }

                changed
            },
            Self::FirstNotError(modifications) => {
                let mut errors = Vec::new();

                for modification in modifications {
                    match modification.apply(task_state, args, to) {
                        Ok (x) => return Ok(x),
                        Err(e) => errors.push(e)
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },
            Self::IfSome(modification) => if to.is_some() {modification.apply(task_state, args, to)?} else {false},
            Self::IfMatches {matcher, then, r#else} => match matcher.check(task_state, args, to.as_deref())? {
                true  => then  .apply(task_state, args, to)?,
                false => r#else.apply(task_state, args, to)?,
            },

            Self::StringMap {value, map} => if let Some(x) = map.get(get!(?&value)) {x.apply(task_state, args, to)?} else {false},



            Self::Set    (value) => {let new = get!(?value); if *to != new {*to = new; true} else {false}},
            Self::Append (value) => {to.as_mut().ok_or(SubjectIsNone)?.to_mut().push_str  (   get!(&value)); true},
            Self::Prepend(value) => {to.as_mut().ok_or(SubjectIsNone)?.to_mut().insert_str(0, get!(&value)); true},

            Self::Lowercase => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.to_lowercase())); true},
            Self::Uppercase => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.to_uppercase())); true},



            Self::StripPrefix        (p) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.strip_prefix(get!(&p)).ok_or(StringModificationError::PrefixNotFound)?); true},
            Self::StripSuffix        (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.strip_suffix(get!(&s)).ok_or(StringModificationError::SuffixNotFound)?); true},
            Self::TrimPrefix         (p) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_prefix(get!(&p)) {to.retain_substr(x); true} else {false}},
            Self::TrimSuffix         (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_suffix(get!(&s)) {to.retain_substr(x); true} else {false}},

            Self::StripBefore        (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.remove_substr(to.split_once(get!(&s)).ok_or(SubstringNotFound)?.0); true},
            Self::StripAfter         (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.remove_substr(to.split_once(get!(&s)).ok_or(SubstringNotFound)?.1); true},
            Self::KeepBefore         (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.split_once(get!(&s)).ok_or(SubstringNotFound)?.0); true},
            Self::KeepAfter          (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.split_once(get!(&s)).ok_or(SubstringNotFound)?.1); true},

            Self::StripBeforeLast    (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.remove_substr(to.rsplit_once(get!(&s)).ok_or(SubstringNotFound)?.0); true},
            Self::StripAfterLast     (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.remove_substr(to.rsplit_once(get!(&s)).ok_or(SubstringNotFound)?.1); true},
            Self::KeepBeforeLast     (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.rsplit_once(get!(&s)).ok_or(SubstringNotFound)?.0); true},
            Self::KeepAfterLast      (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; to.retain_substr(to.rsplit_once(get!(&s)).ok_or(SubstringNotFound)?.1); true},

            Self::TrimBefore         (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((x, _)) = to. split_once(get!(&s)) {to.remove_substr(x); true} else {false}},
            Self::TrimAfter          (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((_, x)) = to. split_once(get!(&s)) {to.remove_substr(x); true} else {false}},
            Self::KeepTrimBefore     (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((x, _)) = to. split_once(get!(&s)) {to.retain_substr(x); true} else {false}},
            Self::KeepTrimAfter      (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((_, x)) = to. split_once(get!(&s)) {to.retain_substr(x); true} else {false}},

            Self::TrimBeforeLast     (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((x, _)) = to.rsplit_once(get!(&s)) {to.remove_substr(x); true} else {false}},
            Self::TrimAfterLast      (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((_, x)) = to.rsplit_once(get!(&s)) {to.remove_substr(x); true} else {false}},
            Self::KeepTrimBeforeLast (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((x, _)) = to.rsplit_once(get!(&s)) {to.retain_substr(x); true} else {false}},
            Self::KeepTrimAfterLast  (s) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some((_, x)) = to.rsplit_once(get!(&s)) {to.retain_substr(x); true} else {false}},



            Self::Replacen   {find, replace, count} => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.replacen(get!(&find), get!(&replace), *count))); true},
            Self::ReplaceAll {find, replace       } => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.replace (get!(&find), get!(&replace)        ))); true},



            Self::GetJsStringLiteralPrefix => {*to = Some(Cow::Owned(get_js_string_literal_prefix(to.as_ref().ok_or(SubjectIsNone)?)?)); true},
            Self::UnescapeHtml             => {*to = Some(Cow::Owned(unescape_html               (to.as_ref().ok_or(SubjectIsNone)?)?)); true},
            Self::GetHtmlAttribute(name)   => {
                *to = Some(Cow::Owned(
                    get_html_attribute(to.as_ref().ok_or(SubjectIsNone)?, get!(&name))?
                    .ok_or(StringModificationError::HtmlAttributeNotFound)?
                    .ok_or(StringModificationError::HtmlAttributeHasNoValue)?
                ));
                true
            }



            Self::RegexFind(regex) => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                to.retain_substr(regex.get()?.find(to).ok_or(StringModificationError::RegexMatchNotFound)?.as_str());
                true
            },
            Self::RegexExpansion {regex, expansion} => {
                *to = expansion.expand(task_state, args, &regex.get()?.captures(to.as_ref().ok_or(SubjectIsNone)?).ok_or(StringModificationError::RegexMatchNotFound)?)?.map(|x| x.into_owned().into());
                true
            },
            Self::RegexReplaceOne {regex, replace} => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                let temp = regex.get()?.replace(to, get!(&replace));
                if *to != temp {
                    *to = temp.into_owned().into();
                    true
                } else {
                    false
                }
            },
            Self::RegexReplaceAll {regex, replace} => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                let temp = regex.get()?.replace_all(to, get!(&replace));
                if *to != temp {
                    *to = temp.into_owned().into();
                    true
                } else {
                    false
                }
            },
            Self::RegexReplacen {regex, n, replace} => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                let temp = regex.get()?.replacen(to, *n, get!(&replace));
                if *to != temp {
                    *to = temp.into_owned().into();
                    true
                } else {
                    false
                }
            },



            Self::JsonPointer(pointer) => {
                match serde_json::from_str::<serde_json::Value>(to.as_ref().ok_or(SubjectIsNone)?)?.pointer_mut(get!(&pointer)).ok_or(StringModificationError::JsonValueNotFound)?.take() {
                    serde_json::Value::String(s) => {*to = Some(s.into()); true},
                    _ => Err(StringModificationError::JsonPointeeIsNotAString)?
                }
            },


            Self::EncodeQueryPart      => if let Some(x) = to.take() {let (changed, x) =       encode_query_part(x) ; *to = Some(x); changed} else {false},
            Self::TryDecodeQueryPart   => if let Some(x) = to.take() {let (changed, x) =   try_decode_query_part(x)?; *to = Some(x); changed} else {false},
            Self::LossyDecodeQueryPart => if let Some(x) = to.take() {let (changed, x) = lossy_decode_query_part(x) ; *to = Some(x); changed} else {false},
            Self::TryPercentDecode     => if let Some(x) = to.take() {let (changed, x) =   try_percent_decode   (x)?; *to = Some(x); changed} else {false},
            Self::LossyPercentDecode   => if let Some(x) = to.take() {let (changed, x) = lossy_percent_decode   (x) ; *to = Some(x); changed} else {false},



            Self::Base64Encode(config) => if let Some(x) = to.take() {*to = Some(Cow::Owned(config.make().encode(x.as_bytes())             )); true} else {false},
            Self::Base64Decode(config) => if let Some(x) = to.take() {*to = Some(Cow::Owned(config.make().decode(x.as_bytes())?.try_into()?)); true} else {false},

            // Misc

            Self::Function(call) => task_state.job.cleaner.functions.string_modifications
                .get(&call.name).ok_or(FunctionNotFound)?
                .apply(task_state, Some(&call.args), to)?,

            Self::FunctionArg(name) => args.ok_or(NotInFunction)?.string_modifications
                .get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?
                .apply(task_state, args, to)?,

            Self::Extern(function) => function(task_state, args, to)?
        })
    }
}

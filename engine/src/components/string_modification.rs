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
    /// Map [`Err`] to [`true`].
    IgnoreError(Box<Self>),
    /// [`Self::IgnoreError`] + [`Self::RevertOnError`].
    IgnoreAndRevertOnError(Box<Self>),
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
    /// Index the [`Map`] with the [`StringSource`] and use that or [`Self::StringMap::else`].
    StringMap {
        /// The [`StringSource`].
        value: Box<StringSource>,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },
    /// Index the [`Map`] with the [`UrlPart`] and use that or [`Self::PartMap::else`].
    PartMap {
        /// The [`UrlPart`].
        part: Box<UrlPart>,
        /// The [`Map`].
        ///
        /// Flattened.
        #[serde(flatten)]
        map: Box<Map<Self>>,
        /// The else.
        ///
        /// Defaulted.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Box<Self>,
    },



    /// Sets the string to the specified value.
    Set(StringSource),

    /// Appends the specified value.
    Append(StringSource),
    /// Prepends the specified value.
    Prepend(StringSource),

    /** [`str::to_lowercase`]. **/ Lowercase,
    /** [`str::to_uppercase`]. **/ Uppercase,



    /// Removes the specified prefix.
    /// # Errors
    /// If the prefix isn't found, returns the error [`StringModificationError::PrefixNotFound`].
    StripPrefix(StringSource),
    /// Removes the specified suffix.
    /// # Errors
    /// If the suffix isn't found, returns the error [`StringModificationError::SuffixNotFound`].
    StripSuffix(StringSource),

    /** [`Self::StripPrefix`], ignoring missing prefixes. **/ TrimPrefix(StringSource),
    /** [`Self::StripSuffix`], ignoring missing suffixes. **/ TrimSuffix(StringSource),



    /// Keep everything before the first instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepBefore(StringSource),
    /// Remove everything after the first instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripAfter(StringSource),
    /// Remove everything before the first instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripBefore(StringSource),
    /// Keep everything before the first instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepAfter(StringSource),

    /// Keep everything before the last instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripBeforeLast(StringSource),
    /// Remove everything after the last instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    StripAfterLast(StringSource),
    /// Remove everything before the last instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepBeforeLast(StringSource),
    /// Keep everything before the last instance of the value.
    /// # Errors
    /// If the substring isn't found, returns the error [`SubstringNotFound`].
    KeepAfterLast(StringSource),

    /** [`Self::StripBefore`],     ignoring missing substrings. **/ TrimBefore        (StringSource),
    /** [`Self::StripAfter`],      ignoring missing substrings. **/ TrimAfter         (StringSource),
    /** [`Self::KeepAfter`],       ignoring missing substrings. **/ KeepTrimBefore    (StringSource),
    /** [`Self::KeepAfter`],       ignoring missing substrings. **/ KeepTrimAfter     (StringSource),

    /** [`Self::StripBeforeLast`], ignoring missing substrings. **/ TrimBeforeLast    (StringSource),
    /** [`Self::StripAfterLast`],  ignoring missing substrings. **/ TrimAfterLast     (StringSource),
    /** [`Self::KeepAfterLast`],   ignoring missing substrings. **/ KeepTrimBeforeLast(StringSource),
    /** [`Self::KeepAfterLast`],   ignoring missing substrings. **/ KeepTrimAfterLast (StringSource),



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



    /** [`get_js_string_literal_prefix`]. **/ GetJsStringLiteralPrefix,
    /** [`unescape_html`].                **/ UnescapeHtml,
    /** [`get_html_attribute`].           **/ GetHtmlAttribute(StringSource),



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



    /** [`better_url::util::encode_query_part`].       **/ EncodeQueryPart,
    /** [`better_url::util::lossy_decode_query_part`]. **/ LossyDecodeQueryPart,
    /** [`better_url::util::lossy_percent_decode`].    **/ LossyPercentDecode,



    /** Base64 encodes the string. **/ Base64Encode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),
    /** Base64 decodes the string. **/ Base64Decode(#[serde(default, skip_serializing_if = "is_default")] Base64Config),



    /// Uses a [`Self`] from [`Cleaner::functions`].
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`FunctionArgs`].
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

            Self::IgnoreError(modification) => modification.apply(task_state, args, to).unwrap_or(true),
            Self::IgnoreAndRevertOnError(modification) => {
                let old_to = to.clone();

                match modification.apply(task_state, args, to) {
                    Ok(x) => x,
                    Err(_) => {
                        *to = old_to;
                        false
                    }
                }
            },
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
                    Ok(_) => true,
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
                        Ok (x) => return Ok(x || !errors.is_empty()),
                        Err(e) => errors.push(e),
                    }
                }

                Err(FirstNotErrorErrors(errors))?
            },
            Self::IfSome(modification) => if to.is_some() {modification.apply(task_state, args, to)?} else {false},
            Self::IfMatches {matcher, then, r#else} => match matcher.check(task_state, args, to.as_deref())? {
                true  =>   then.apply(task_state, args, to)?,
                false => r#else.apply(task_state, args, to)?,
            },

            Self::StringMap {value, map, r#else} => map.get(get!(?&value)                       ).unwrap_or(r#else).apply(task_state, args, to)?,
            Self::PartMap   {part , map, r#else} => map.get(part.get(&task_state.url).as_deref()).unwrap_or(r#else).apply(task_state, args, to)?,



            Self::Set    (value) => {*to = get!(?value); true},
            Self::Append (value) => {to.as_mut().ok_or(SubjectIsNone)?.push_str  (   get!(&value)); true},
            Self::Prepend(value) => {to.as_mut().ok_or(SubjectIsNone)?.insert_str(0, get!(&value)); true},

            Self::Lowercase => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.to_lowercase())); true},
            Self::Uppercase => {*to = Some(Cow::Owned(to.as_ref().ok_or(SubjectIsNone)?.to_uppercase())); true},



            Self::StripPrefix        (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_prefix(get!(&x)) {to.retain_range(  to.len() - x.len()..); true} else {Err(StringModificationError::PrefixNotFound)?}},
            Self::StripSuffix        (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_suffix(get!(&x)) {to.retain_range(..x .len()            ); true} else {Err(StringModificationError::SuffixNotFound)?}},
            Self::TrimPrefix         (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_prefix(get!(&x)) {to.retain_range(  to.len() - x.len()..); true} else {false}},
            Self::TrimSuffix         (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(x) = to.strip_suffix(get!(&x)) {to.retain_range(..x .len()            ); true} else {false}},

            Self::StripBefore        (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_start (get!(&x)) {to.retain_range(  i..); true} else {Err(SubstringNotFound)?}},
            Self::StripAfter         (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_after (get!(&x)) {to.retain_range(..i  ); true} else {Err(SubstringNotFound)?}},
            Self::KeepBefore         (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_start (get!(&x)) {to.retain_range(..i  ); true} else {Err(SubstringNotFound)?}},
            Self::KeepAfter          (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_after (get!(&x)) {to.retain_range(  i..); true} else {Err(SubstringNotFound)?}},

            Self::StripBeforeLast    (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_start (get!(&x)) {to.retain_range(  i..); true} else {Err(SubstringNotFound)?}},
            Self::StripAfterLast     (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_after (get!(&x)) {to.retain_range(..i  ); true} else {Err(SubstringNotFound)?}},
            Self::KeepBeforeLast     (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_start (get!(&x)) {to.retain_range(..i  ); true} else {Err(SubstringNotFound)?}},
            Self::KeepAfterLast      (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_after (get!(&x)) {to.retain_range(  i..); true} else {Err(SubstringNotFound)?}},

            Self::TrimBefore         (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_start (get!(&x)) {to.retain_range(  i..); true} else {false}},
            Self::TrimAfter          (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_after (get!(&x)) {to.retain_range(..i  ); true} else {false}},
            Self::KeepTrimBefore     (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_start (get!(&x)) {to.retain_range(..i  ); true} else {false}},
            Self::KeepTrimAfter      (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to. find_after (get!(&x)) {to.retain_range(  i..); true} else {false}},

            Self::TrimBeforeLast     (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_start (get!(&x)) {to.retain_range(  i..); true} else {false}},
            Self::TrimAfterLast      (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_after (get!(&x)) {to.retain_range(..i  ); true} else {false}},
            Self::KeepTrimBeforeLast (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_start (get!(&x)) {to.retain_range(..i  ); true} else {false}},
            Self::KeepTrimAfterLast  (x) => {let to = to.as_mut().ok_or(SubjectIsNone)?; if let Some(i) = to.rfind_after (get!(&x)) {to.retain_range(  i..); true} else {false}},



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
                if !std::ptr::eq::<str>(&**to, &*temp) {
                    *to = temp.into_owned().into();
                    true
                } else {
                    false
                }
            },
            Self::RegexReplaceAll {regex, replace} => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                let temp = regex.get()?.replace_all(to, get!(&replace));
                if !std::ptr::eq::<str>(&**to, &*temp) {
                    *to = temp.into_owned().into();
                    true
                } else {
                    false
                }
            },
            Self::RegexReplacen {regex, n, replace} => {
                let to = to.as_mut().ok_or(SubjectIsNone)?;
                let temp = regex.get()?.replacen(to, *n, get!(&replace));
                if !std::ptr::eq::<str>(&**to, &*temp) {
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


            Self::EncodeQueryPart      => if let Some(x) = to.take() {let (changed, x) =       encode_query_part(x); *to = Some(x); changed} else {false},
            Self::LossyDecodeQueryPart => if let Some(x) = to.take() {let (changed, x) = lossy_decode_query_part(x); *to = Some(x); changed} else {false},
            Self::LossyPercentDecode   => if let Some(x) = to.take() {let (changed, x) = lossy_percent_decode   (x); *to = Some(x); changed} else {false},



            Self::Base64Encode(config) => if let Some(x) = to.take() {*to = Some(Cow::Owned(config.make().encode(x.as_bytes())             )); true} else {false},
            Self::Base64Decode(config) => if let Some(x) = to.take() {*to = Some(Cow::Owned(config.make().decode(x.as_bytes())?.try_into()?)); true} else {false},

            // Misc

            Self::Function   (call    ) => task_state.job.cleaner.functions.string_modifications.get(&call.name ).ok_or(FunctionNotFound           )?.apply(task_state, Some(&call.args), to)?,
            Self::FunctionArg(name    ) => args.ok_or(NotInFunction)?.string_modifications      .get(get!(&name)).ok_or(FunctionArgFunctionNotFound)?.apply(task_state, args            , to)?,
            Self::Extern     (function) => function(task_state, args, to)?,
        })
    }
}

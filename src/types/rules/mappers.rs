//! Logic for how a [`TaskState`] should be modified.

use std::str::Utf8Error;
use std::collections::HashSet;
use std::time::Duration;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SetPreventDuplicates, DurationSecondsWithFrac};
use thiserror::Error;
use url::Url;
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// Mappers are how [`TaskState`]s get manipulated to clean URLs.
///
/// Please note that, in general, when a [`Mapper`] returns an [`Err`], the [`TaskState`] may still be modified.
///
/// For example, a [`Mapper::All`] containing 3 [`Mapper`]s and the second one returns an error, the effects of the first [`Mapper`] is still applied.
///
/// In practice this should rarely be an issue, but when it is, use [`Mapper::RevertOnError`].
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Mapper {
    /// Does nothing.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::None.apply(&mut task_state).unwrap();
    ///
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    None,
    /// Always returns the error [`MapperError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`MapperError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::Error("...".into()).apply(&mut task_state).unwrap_err();
    ///
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskStateView`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    /// If the call to [`Self::If::condition`] passes, apply [`Self::If::mapper`].
    ///
    /// If the call to [`Self::If::condition`] fails and [`Self::If::else_mapper`] is [`Some`], apply [`Self::If::else_mapper`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::If {
    ///     condition  : Condition::Always,
    ///     mapper     : Box::new(Mapper::None),
    ///     else_mapper: Some(Box::new(Mapper::Error("...".into())))
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Mapper::If {
    ///     condition  : Condition::Never,
    ///     mapper     : Box::new(Mapper::None),
    ///     else_mapper: Some(Box::new(Mapper::Error("...".into())))
    /// }.apply(&mut task_state).unwrap_err();
    ///
    /// Mapper::If {
    ///     condition  : Condition::Always,
    ///     mapper     : Box::new(Mapper::None),
    ///     else_mapper: None
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Mapper::If {
    ///     condition  : Condition::Never,
    ///     mapper     : Box::new(Mapper::None),
    ///     else_mapper: None
    /// }.apply(&mut task_state).unwrap();
    /// ```
    If {
        /// The [`Condition`] to decide between [`Self::If::mapper`] and [`Self::If::else_mapper`].
        condition: Condition,
        /// The [`Self`] to apply if [`Self::If::condition`] passes.
        mapper: Box<Self>,
        /// The [`Self`] to apply if [`Self::If::condition`] fails.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        else_mapper: Option<Box<Self>>
    },
    /// Applies the contained [`Self`]s in order.
    ///
    /// Please note that if one of the contained [`Self`]s returns an error, previous calls to [`Self::apply`] aren't reverted.
    /// # Errors
    /// If any call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::All(vec![
    ///     Mapper::SetHost(Some("example2.com".to_string())),
    ///     Mapper::Error("...".into()),
    ///     Mapper::SetHost(Some("example3.com".to_string())),
    /// ]).apply(&mut task_state).unwrap_err();
    ///
    /// assert_eq!(task_state.url, "https://example2.com/");
    /// ```
    All(Vec<Self>),
    /// Gets the value specified by [`Self::PartMap::part`], indexes [`Self::PartMap::map`], and applies the returned [`Self`]
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing..
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::PartMap {
    ///     part: UrlPart::Host,
    ///     map: Map {
    ///         map: [
    ///             ("example.com".into(), Mapper::Error("...".into()))
    ///         ].into(),
    ///         if_null: None,
    ///         r#else: None
    ///     }
    /// }.apply(&mut task_state).unwrap_err();
    /// ```
    PartMap {
        /// The [`UrlPart`] to index [`Self::PartMap::map`] with.
        part: UrlPart,
        /// The [`Map`] to index with [`Self::PartMap::part`].
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Gets the string specified by [`Self::StringMap::value`], indexes [`Self::StringMap::map`], and applies the returned [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::StringMap {
    ///     value: StringSource::String("a".into()),
    ///     map: Map {
    ///         map: [
    ///             ("a".into(), Mapper::Error("...".into()))
    ///         ].into(),
    ///         if_null: None,
    ///         r#else: None
    ///     }
    /// }.apply(&mut task_state).unwrap_err();
    /// ```
    StringMap {
        /// The [`StringSource`] to index [`Self::StringMap::map`] with.
        value: StringSource,
        /// The [`Map`] to index with [`Self::StringMap::value`].
        #[serde(flatten)]
        map: Map<Self>
    },

    /// If the contained [`Self`] returns an error that matches [`Self::IgnoreError::filter`], ignore it.
    ///
    /// Does not revert any successful calls to [`Self::apply`]. For that, also use [`Self::RevertOnError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::IgnoreError {
    ///     mapper: Box::new(Mapper::Error("...".into())),
    ///     filter: Default::default()
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Mapper::IgnoreError {
    ///     mapper: Box::new(Mapper::Error("...".into())),
    ///     filter: MapperErrorFilter(Some([MapperErrorName::ExplicitError].into()))
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Mapper::IgnoreError {
    ///     mapper: Box::new(Mapper::Error("...".into())),
    ///     filter: MapperErrorFilter(Some([MapperErrorName::StringSourceIsNone].into()))
    /// }.apply(&mut task_state).unwrap_err();
    /// ```
    IgnoreError {
        /// The [`Self`] to try to apply.
        #[serde(flatten)]
        mapper: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: MapperErrorFilter
    },
    /// If the contained [`Self`] returns an error that matches [`Self::RevertOnError::filter`], revert the [`TaskState`] to its previous state.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    RevertOnError {
        /// The [`Self`] to try to apply.
        #[serde(flatten)]
        mapper: Box<Self>,
        /// The filters of which error to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: MapperErrorFilter
    },
    /// If [`Self::TryElse::try`]'s call to [`Self::apply`] returns an error that matches [`Self::TryElse::filter`], apply [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::apply`] return errors, both errors are returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: MapperErrorFilter
    },
    /// Applies the contained [`Self`]s in order, stopping as soon as a call to [`Self::apply`] doesn't return an error.
    /// # Errors
    /// If all calls to [`Self::apply`] return errors, the last error is returned. In the future this should be changed to return all errors.
    FirstNotError(Vec<Self>),

    /// Remove the entire [`UrlPart::Query`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2");
    ///
    /// Mapper::RemoveQuery.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    RemoveQuery,
    /// Removes all query parameters with the specified name.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Mapper::RemoveQueryParam("a".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/?b=3&c=5");
    /// ```
    RemoveQueryParam(StringSource),
    /// Removes all query params with names in the specified [`HashSet`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Mapper::RemoveQueryParams(["a".to_string(), "c".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/?b=3");
    /// ```
    RemoveQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Keeps only query params with names in the specified [`HashSet`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Mapper::AllowQueryParams(["a".to_string(), "c".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/?a=2&a=4&c=5");
    /// ```
    AllowQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Removes all query params with names matching the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Mapper::RemoveQueryParamsMatching(StringMatcher::Is("a".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/?b=3&c=5");
    /// ```
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only query params with names matching the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Mapper::AllowQueryParamsMatching(StringMatcher::Is("a".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/?a=2&a=4");
    /// ```
    AllowQueryParamsMatching(StringMatcher),
    /// Sets [`UrlPart::Whole`] to the value of the first query parameter with a name determined by the [`TaskState`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MapperError::StringSourceIsNone`].
    ///
    /// If no matching query parameter is found, returns the error [`MapperError::CannotFindQueryParam`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?redirect=https://example.com/2");
    ///
    /// Mapper::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/2");
    ///
    /// Mapper::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap_err();
    /// ```
    GetUrlFromQueryParam(StringSource),



    /// Sets the [`UrlPart::Host`] to the specified value.
    /// # Errors
    /// If the call to [`BetterUrl::set_host`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::SetHost(Some("example2.com".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example2.com/")
    /// ```
    SetHost(Option<String>),
    /// "Join"s a URL like how relative links on websites work.
    ///
    /// See [`Url::join`] for details.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Url::join`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com/a/b/c");
    ///
    /// Mapper::Join("..".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/a/");
    ///
    /// 
    /// url_cleaner::task_state!(task_state, url = "https://example.com/a/b/c/");
    ///
    /// Mapper::Join("..".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/a/b/");
    /// ```
    Join(StringSource),



    /// Sets the specified [`UrlPart`] to the specified value.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::SetPart {part: UrlPart::Path, value: "abc".into()}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc");
    /// ```
    SetPart {
        /// The part to set the value of.
        part: UrlPart,
        /// The value to set the part to.
        value: StringSource
    },
    /// If the specified [`UrlPart`] is [`Some`], applies [`Self::ModifyPart::modification`].
    ///
    /// If the part is [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    ///
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::ModifyPart {part: UrlPart::Path, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc");
    ///
    /// Mapper::ModifyPart {part: UrlPart::Query, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc");
    /// ```
    ModifyPart {
        /// The part to modify.
        part: UrlPart,
        /// The modification to apply to the part.
        modification: StringModification
    },
    /// Sets [`Self::CopyPart::to`] to the value of [`Self::CopyPart::from`], leaving [`Self::CopyPart::from`] unchanged.
    /// # Errors
    /// If the call to [`UrlPart::set`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, url = "https://example.com/abc#def");
    ///
    /// Mapper::CopyPart {from: UrlPart::Fragment, to: UrlPart::Path}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/def#def");
    /// ```
    CopyPart {
        /// The part whose value to copy.
        from: UrlPart,
        /// The part whose value to set.
        to: UrlPart
    },
    /// Sets [`Self::CopyPart::to`] to the value of [`Self::CopyPart::from`], then sets [`Self::CopyPart::from`] to [`None`].
    /// # Errors
    /// If either call to [`UrlPart::set`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, url = "https://example.com/abc#def");
    ///
    /// Mapper::MovePart {from: UrlPart::Fragment, to: UrlPart::Path}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/def");
    /// ```
    MovePart {
        /// The part whose value to move.
        from: UrlPart,
        /// The part whose value to set.
        to: UrlPart
    },

    /// Sends an HTTP GET request to the current [`TaskState::url`], and sets it either to the value of the response's `Location` header (if the response is a redirect) or the final URL after redirects.
    ///
    /// If the `cache` feature flag is enabled, caches the operation with the category `redirect`, the key set to the input URL, and the value set to the returned URL.
    /// # Errors
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::read`] returns an error, that error is returned.")]
    #[cfg_attr(feature = "cache", doc = "")]
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::read`] returns [`None`], returns the error [`MapperError::CachedUrlIsNone`].")]
    #[cfg_attr(feature = "cache", doc = "")]
    #[cfg_attr(feature = "cache", doc = "If the call to [`BetterUrl::parse`] returns an error, that error is returned.")]
    #[cfg_attr(feature = "cache", doc = "")]
    /// If the call to [`TaskStateView::http_client`] returns an error, that error is returned.
    ///
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    ///
    /// If the response is a redirect and doesn't contain a `Location` header, returns the error [`MapperError::LocationHeaderNotFound`].
    ///
    /// If the `Location` header's call to [`std::str::from_utf8`] returns an error, that error is returned.
    ///
    /// If the `Location` header's call to [`BetterUrl::parse`] returns an error, that error is returned.
    #[cfg_attr(feature = "cache", doc = "")]
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::write`] returns an error, that error is returned.")]
    #[cfg(feature = "http")]
    ExpandRedirect {
        /// The extra headers to send.
        ///
        /// Defaults to an empty [`HeaderMap`].
        #[serde(default, skip_serializing_if = "is_default", with = "serde_headermap")]
        headers: HeaderMap,
        /// The [`HttpClientConfigDiff`] to apply.
        ///
        /// Defaults to [`None`].
        ///
        /// Boxed because it's massive.
        #[serde(default, skip_serializing_if = "is_default")]
        http_client_config_diff: Option<Box<HttpClientConfigDiff>>
    },
    /// Sets the specified [`TaskScratchpad::flags`] to [`Self::SetScratchpadFlag::value`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MapperError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state);
    ///
    /// assert_eq!(task_state.scratchpad.flags.contains("abc"), false);
    /// Mapper::SetScratchpadFlag {name: "abc".into(), value: true}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.flags.contains("abc"), true);
    /// ```
    SetScratchpadFlag {
        /// The name of the flag to set.
        name: StringSource,
        /// The value to set the flag to.
        value: bool
    },
    /// Sets the specified [`TaskScratchpad::vars`] to [`Self::SetScratchpadVar::value`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::SetScratchpadVar {name: "abc".into(), value: "def".into()}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("def"));
    /// Mapper::SetScratchpadVar {name: "abc".into(), value: StringSource::None}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// ```
    SetScratchpadVar {
        /// The name of the var to set.
        name: StringSource,
        /// The value to set the var to.
        value: StringSource
    },
    /// If the specified [`TaskScratchpad::vars`] is [`Some`], applies [`Self::ModifyScratchpadVar::modification`].
    ///
    /// If the part is [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MapperError::StringSourceIsNone`].
    ///
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state);
    ///
    /// Mapper::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set("123".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// Mapper::SetScratchpadVar {name: "abc".into(), value: "def".into()}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("def"));
    /// Mapper::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set("123".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("123"));
    /// Mapper::SetScratchpadVar {name: "abc".into(), value: StringSource::None}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// ```
    ModifyScratchpadVar {
        /// The name of the var to modify.
        name: StringSource,
        /// The modification to apply.
        modification: StringModification
    },
    /// Applies the contained [`Rule`].
    /// # Errors
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::Rule(Box::new(Rule::Mapper(Mapper::SetHost(Some("example2.com".into()))))).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example2.com/");
    /// ```
    Rule(Box<Rule>),
    /// Applies the contained [`Rules`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Mapper::Rules(Rules(vec![Rule::Mapper(Mapper::SetHost(Some("example2.com".into())))])).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example2.com/");
    /// ```
    Rules(Rules),
    /// If an entry with the specified category and a key of the current [`TaskState::url`] exists in the cache, sets the [`TaskState::url`] to the entry's value.
    ///
    /// If no matching entry exists, applies [`Self::CacheUrl::mapper`] and makes a new entry with the specified category, the previous [`TaskState::url`] as the key, and the new [`TaskState::url`] as the value.
    ///
    /// If an error is returned, no new cache entry is written.
    /// # Errors
    /// If the call to [`Cache::read`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::read`] returns [`None`], returns the error [`MapperError::CachedUrlIsNone`].
    ///
    /// If the call to [`BetterUrl::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::write`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    CacheUrl {
        /// The category for the cache entry.
        category: StringSource,
        /// The mapper to apply and cache.
        mapper: Box<Self>
    },
    /// Applies [`Self::Retry::mapper`] and, if it returns an error, waits [`Self::Retry::wait`] and applies it again.
    ///
    /// Attempts to apply it at most [`Self::Retry::limit`] times.
    /// # Errors
    /// If call calls to [`Self::apply`] return an error, the final error is returned.
    Retry {
        /// The [`Self`] to apply.
        mapper: Box<Self>,
        /// The time to wait between retries.
        #[serde_as(as = "DurationSecondsWithFrac<f64>")]
        wait: Duration,
        /// The max amount of times to try.
        ///
        /// Defaults to `10`.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// Applies a [`Self`] from [`TaskState::commons`]'s [`Commons::mappers`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`MapperError::StringSourceIsNone`].
    ///
    /// If the [`Commons::mappers`] doesn't contain a [`Self`] with the specified name, returns the error [`MapperError::CommonMapperNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, commons = Commons {
    ///     mappers: [("abc".into(), Mapper::None)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// Mapper::Common(CommonCall {name: "abc".into(), args: Default::default()}).apply(&mut task_state).unwrap();
    /// ```
    Common(CommonCall),
    /// Calls the specified function and returns its value.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut TaskState) -> Result<(), MapperError>)
}

/// Serde helper function.
const fn get_10_u8() -> u8 {10}

/// The enum of errors [`Mapper::apply`] can return.
#[derive(Debug, Error, ErrorFilter)]
pub enum MapperError {
    /// Returned when a [`Mapper::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`Mapper`]s in a [`Mapper::TryElse`] return errors.
    #[error("Both Mappers in a Mapper::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`Mapper::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`Mapper::TryElse::else`]. 
        else_error: Box<Self>
    },

    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// Returned when attempting to get a URL from a query parameter that doesn't exist.
    #[error("Attempted to get a URL from a query parameter that didn't exist.")]
    CannotFindQueryParam,
    /// Returned when a [`Mapper`] with the specified name isn't found in the [`Commons::mappers`].
    #[error("A Mapper with the specified name wasn't found in the Commons::mappers.")]
    CommonMapperNotFound,
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// Returned when a [`UrlPartSetError`] is encountered.
    #[error(transparent)]
    UrlPartSetError(#[from] UrlPartSetError),
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)]
    GetConfigError(#[from] GetConfigError),
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)]
    RuleError(Box<RuleError>),

    /// Returned when a [`reqwest::Error`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a redirect's `Location` header isn't found.
    #[cfg(feature = "http")]
    #[error("The redirect's Location header wasn't found")]
    LocationHeaderNotFound,

    /// Returned when a [`reqwest::header::ToStrError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),

    /// Returned when attempting to get a URL from the cache but its value is [`None`].
    #[cfg(feature = "cache")]
    #[error("Attempted to get a URL from the cache but its value was None.")]
    CachedUrlIsNone,
    /// Returned when a [`ReadFromCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    ReadFromCacheError(#[from] ReadFromCacheError),
    /// Returned when a [`WriteToCacheError`] is encountered.
    #[cfg(feature = "cache")]
    #[error(transparent)]
    WriteToCacheError(#[from] WriteToCacheError),

    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// An arbitrary [`std::error::Error`] returned by [`Mapper::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl From<RuleError> for MapperError {
    fn from(value: RuleError) -> Self {
        Self::RuleError(Box::new(value))
    }
}

impl Mapper {
    /// Applies the specified variant of [`Self`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), MapperError> {
        debug!(self, Mapper::apply, self, task_state);
        match self {
            // Testing.

            Self::None => {},
            Self::Error(msg) => Err(MapperError::ExplicitError(msg.clone()))?,
            Self::Debug(mapper) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                let mapper_result=mapper.apply(task_state);
                eprintln!("=== Mapper::Debug ===\nMapper: {mapper:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nMapper return value: {mapper_result:?}\nNew task_state: {task_state:?}");
                mapper_result?;
            },

            // Logic.

            Self::If {condition, mapper, else_mapper} => if condition.satisfied_by(&task_state.to_view())? {
                mapper.apply(task_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(task_state)?;
            },
            Self::All(mappers) => {
                for mapper in mappers {
                    mapper.apply(task_state)?;
                }
            },
            Self::PartMap  {part , map} => if let Some(mapper) = map.get(part .get( task_state.url      ) ) {mapper.apply(task_state)?},
            Self::StringMap{value, map} => if let Some(mapper) = map.get(value.get(&task_state.to_view())?) {mapper.apply(task_state)?},

            // Error handling.

            Self::IgnoreError {mapper, filter} => if let Err(e) = mapper.apply(task_state) {if !filter.matches(&e) {Err(e)?}},
            Self::TryElse{ r#try, filter, r#else } => match r#try.apply(task_state) {
                Ok(x) => x,
                Err(try_error) => if filter.matches(&try_error) {
                    match r#else.apply(task_state) {
                        Ok(x) => x,
                        Err(else_error) => Err(MapperError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                    }
                } else {
                    Err(try_error)?
                }
            },
            Self::FirstNotError(mappers) => {
                let mut result = Ok(());
                for mapper in mappers {
                    result = mapper.apply(task_state);
                    if result.is_ok() {break}
                }
                result?
            },
            Self::RevertOnError {mapper, filter} => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                if let Err(e) = mapper.apply(task_state) {
                    if filter.matches(&e) {
                        *task_state.url = old_url;
                        *task_state.scratchpad = old_scratchpad;
                    }
                    Err(e)?;
                }
            },

            // Query.

            Self::RemoveQuery => task_state.url.set_query(None),
            Self::RemoveQueryParam(name) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let task_state_view = task_state.to_view();
                let name = get_cow!(name, task_state_view, MapperError);
                let new_query = form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(x, _)| *x != name)).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParams(names) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(name, _)| !names.contains(name.as_ref()))).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::AllowQueryParams(names) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len)).extend_pairs(task_state.url.query_pairs().filter(|(name, _)|  names.contains(name.as_ref()))).finish();
                task_state.url.set_query((!new_query.is_empty()).then_some(&new_query));
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in task_state.url.query_pairs() {
                    if !matcher.satisfied_by(&name, &task_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                task_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query_len) = task_state.url.query().map(|x| x.len()) {
                let mut new_query=form_urlencoded::Serializer::new(String::with_capacity(query_len));
                for (name, value) in task_state.url.query_pairs() {
                    if matcher.satisfied_by(&name, &task_state.to_view())? {
                        new_query.append_pair(&name, &value);
                    }
                }
                let x = new_query.finish();
                task_state.url.set_query((!x.is_empty()).then_some(&x));
            },
            Self::GetUrlFromQueryParam(name) => {
                let task_state_view = task_state.to_view();
                let name = name.get(&task_state_view)?.ok_or(MapperError::StringSourceIsNone)?;

                match task_state.url.query_pairs().find(|(param_name, _)| *param_name==name) {
                    Some((_, new_url)) => {*task_state.url=Url::parse(&new_url)?.into()},
                    None => Err(MapperError::CannotFindQueryParam)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => task_state.url.set_host(new_host.as_deref())?,
            Self::Join(with) => *task_state.url=task_state.url.join(get_str!(with, task_state, MapperError))?.into(),

            // Generic part handling.

            Self::SetPart{part, value} => part.set(task_state.url, value.get(&task_state.to_view())?.map(Cow::into_owned).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart{part, modification} => if let Some(mut temp) = part.get(task_state.url).map(|x| x.into_owned()) {
                modification.apply(&mut temp, &task_state.to_view())?;
                part.set(task_state.url, Some(&temp))?;
            }
            Self::CopyPart{from, to} => to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart{from, to} => {
                let mut temp_url = task_state.url.clone();
                let temp_url_ref = &mut temp_url;
                to.set(temp_url_ref, from.get(temp_url_ref).map(|x| x.into_owned()).as_deref())?;
                from.set(&mut temp_url, None)?;
                *task_state.url = temp_url;
            },

            // Miscellaneous.

            #[cfg(feature = "http")]
            Self::ExpandRedirect {headers, http_client_config_diff} => {
                #[cfg(feature = "cache")]
                if task_state.params.read_cache {
                    if let Some(new_url) = task_state.cache.read("redirect", task_state.url.as_str())? {
                        *task_state.url = BetterUrl::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                let response = task_state.to_view().http_client(http_client_config_diff.as_deref())?.get(task_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    BetterUrl::parse(std::str::from_utf8(response.headers().get("location").ok_or(MapperError::LocationHeaderNotFound)?.as_bytes())?)?
                } else {
                    response.url().clone().into()
                };
                #[cfg(feature = "cache")]
                if task_state.params.write_cache {
                    task_state.cache.write("redirect", task_state.url.as_str(), Some(new_url.as_str()))?;
                }
                *task_state.url=new_url;
            },

            Self::SetScratchpadFlag {name, value} => {
                let name = get_string!(name, task_state, MapperError);
                match value {
                    true  => task_state.scratchpad.flags.insert( name),
                    false => task_state.scratchpad.flags.remove(&name)
                };
            },
            Self::SetScratchpadVar {name, value} => match value.get(&task_state.to_view())?.map(Cow::into_owned) {
                Some(value) => {let _ = task_state.scratchpad.vars.insert( get_string!(name, task_state, MapperError), value);}
                None        => {let _ = task_state.scratchpad.vars.remove(&get_string!(name, task_state, MapperError));}
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, task_state, MapperError).to_owned();
                if let Some(mut value) = task_state.scratchpad.vars.get(&name).map(ToOwned::to_owned) {
                    modification.apply(&mut value, &task_state.to_view())?;
                    let _ = task_state.scratchpad.vars.insert(name, value);
                }
            },
            Self::Rule(rule) => {rule.apply(task_state)?;},
            Self::Rules(rules) => {rules.apply(task_state)?;},
            #[cfg(feature = "cache")]
            Self::CacheUrl {category, mapper} => {
                let category = get_string!(category, task_state, MapperError);
                if task_state.params.read_cache {
                    if let Some(new_url) = task_state.cache.read(&category, task_state.url.as_str())? {
                        *task_state.url = BetterUrl::parse(&new_url.ok_or(MapperError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                let old_url = task_state.url.to_string();
                mapper.apply(task_state)?;
                if task_state.params.write_cache {
                    task_state.cache.write(&category, &old_url, Some(task_state.url.as_str()))?;
                }
            },
            Self::Retry {mapper, wait, limit} => {
                for i in 0..*limit {
                    match mapper.apply(task_state) {
                        Ok(()) => return Ok(()),
                        #[allow(clippy::arithmetic_side_effects, reason = "`i` is never 255 and therefore never overflows.")]
                        e @ Err(_) if i+1==*limit => e?,
                        Err(_) => {std::thread::sleep(*wait);}
                    }
                }
            },
            Self::Common(common_call) => {
                task_state.commons.mappers.get(get_str!(common_call.name, task_state, MapperError)).ok_or(MapperError::CommonMapperNotFound)?.apply(&mut TaskState {
                    common_args: Some(&common_call.args.build(&task_state.to_view())?),
                    url: task_state.url,
                    context: task_state.context,
                    params: task_state.params,
                    scratchpad: task_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: task_state.cache,
                    commons: task_state.commons,
                    job_context: task_state.job_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        };
        Ok(())
    }
}

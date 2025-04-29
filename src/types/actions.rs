//! Logic for how a [`TaskState`] should be modified.

use std::str::Utf8Error;
use std::collections::HashSet;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SetPreventDuplicates};
use thiserror::Error;
use url::Url;
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// Actions are how [`TaskState`]s get manipulated to clean URLs.
///
/// Please note that, in general, when a [`Action`] returns an [`Err`], the [`TaskState`] may still be modified.
///
/// For example, a [`Action::All`] containing 3 [`Action`]s and the second one returns an error, the effects of the first [`Action`] is still applied.
///
/// In practice this should rarely be an issue, but when it is, use [`Action::RevertOnError`].
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Action {
    /// Does nothing.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::None.apply(&mut task_state).unwrap();
    ///
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    None,
    /// Always returns the error [`ActionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ActionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::Error("...".into()).apply(&mut task_state).unwrap_err();
    ///
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskStateView`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    /// If the call to [`Self::If::if`] passes, apply [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails and [`Self::If::else`] is [`Some`], apply [`Self::If::else`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::If {
    ///     r#if  : Condition::Always,
    ///     then  : Box::new(Action::None),
    ///     r#else: Some(Box::new(Action::Error("...".into())))
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Action::If {
    ///     r#if  : Condition::Never,
    ///     then  : Box::new(Action::None),
    ///     r#else: Some(Box::new(Action::Error("...".into())))
    /// }.apply(&mut task_state).unwrap_err();
    ///
    /// Action::If {
    ///     r#if  : Condition::Always,
    ///     then  : Box::new(Action::None),
    ///     r#else: None
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Action::If {
    ///     r#if  : Condition::Never,
    ///     then  : Box::new(Action::None),
    ///     r#else: None
    /// }.apply(&mut task_state).unwrap();
    /// ```
    If {
        /// The [`Condition`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Condition,
        /// The [`Self`] to apply if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to apply if [`Self::If::if`] fails.
        ///
        /// Defaults to [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
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
    /// Action::All(vec![
    ///     Action::SetHost(Some("example2.com".to_string())),
    ///     Action::Error("...".into()),
    ///     Action::SetHost(Some("example3.com".to_string())),
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
    /// Action::PartMap {
    ///     part: UrlPart::Host,
    ///     map: Map {
    ///         map: [
    ///             ("example.com".into(), Action::Error("...".into()))
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
    /// Action::StringMap {
    ///     value: StringSource::String("a".into()),
    ///     map: Map {
    ///         map: [
    ///             ("a".into(), Action::Error("...".into()))
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

    /// Repeat [`Self::Repeat::actions`] until either no changes happen or the rules were executed [`Self::Repeat::limit`] times.
    /// # Errors
    /// If any call to [`Action::apply`] returns an error, that error is returned.
    Repeat {
        /// The [`Self`]s to repeat.
        actions: Vec<Action>,
        /// The maximum amount of times to repeat.
        ///
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64
    },

    /// If the contained [`Self`] returns an error that matches [`Self::IgnoreError::filter`], ignore it.
    ///
    /// Does not revert any successful calls to [`Self::apply`]. For that, also use [`Self::RevertOnError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state);
    ///
    /// Action::IgnoreError {
    ///     action: Box::new(Action::Error("...".into())),
    ///     filter: Default::default()
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Action::IgnoreError {
    ///     action: Box::new(Action::Error("...".into())),
    ///     filter: ActionErrorFilter(Some([ActionErrorName::ExplicitError].into()))
    /// }.apply(&mut task_state).unwrap();
    ///
    /// Action::IgnoreError {
    ///     action: Box::new(Action::Error("...".into())),
    ///     filter: ActionErrorFilter(Some([ActionErrorName::StringSourceIsNone].into()))
    /// }.apply(&mut task_state).unwrap_err();
    /// ```
    IgnoreError {
        /// The [`Self`] to try to apply.
        #[serde(flatten)]
        action: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ActionErrorFilter
    },
    /// If the contained [`Self`] returns an error that matches [`Self::RevertOnError::filter`], revert the [`TaskState`] to its previous state.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    RevertOnError {
        /// The [`Self`] to try to apply.
        #[serde(flatten)]
        action: Box<Self>,
        /// The filters of which error to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ActionErrorFilter
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
        filter: ActionErrorFilter
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
    /// Action::RemoveQuery.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    RemoveQuery,
    /// Removes all query parameters with the specified name.
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Action::RemoveQueryParam("a".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("b=3&c=5"));
    /// Action::RemoveQueryParam("b".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("c=5"));
    /// Action::RemoveQueryParam("c".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    RemoveQueryParam(StringSource),
    /// Removes all query params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Action::RemoveQueryParams(["a".to_string(), "b".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("c=5"));
    /// Action::RemoveQueryParams(["c".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    RemoveQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Keeps only query params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Action::AllowQueryParams(["a".to_string(), "b".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("a=2&b=3&a=4"));
    /// Action::AllowQueryParams(["c".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    AllowQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Removes all query params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Action::RemoveQueryParamsMatching(StringMatcher::Is("a".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("b=3&c=5"));
    /// Action::RemoveQueryParamsMatching(StringMatcher::Is("b".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("c=5"));
    /// Action::RemoveQueryParamsMatching(StringMatcher::Is("c".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only query params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// Action::AllowQueryParamsMatching(StringMatcher::Is("a".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("a=2&a=4"));
    /// Action::AllowQueryParamsMatching(StringMatcher::Is("b".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    AllowQueryParamsMatching(StringMatcher),
    /// Sets [`UrlPart::Whole`] to the value of the first query parameter with a name determined by the [`TaskState`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ActionError::StringSourceIsNone`].
    ///
    /// If no matching query parameter is found, returns the error [`ActionError::QueryParamNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state!(task_state, url = "https://example.com?redirect=https://example.com/2");
    ///
    /// Action::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/2");
    ///
    /// Action::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap_err();
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
    /// Action::SetHost(Some("example2.com".into())).apply(&mut task_state).unwrap();
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
    /// Action::Join("..".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/a/");
    ///
    /// 
    /// url_cleaner::task_state!(task_state, url = "https://example.com/a/b/c/");
    ///
    /// Action::Join("..".into()).apply(&mut task_state).unwrap();
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
    /// Action::SetPart {part: UrlPart::Path, value: "abc".into()}.apply(&mut task_state).unwrap();
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
    /// Action::ModifyPart {part: UrlPart::Path, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc");
    ///
    /// Action::ModifyPart {part: UrlPart::Query, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
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
    /// Action::CopyPart {from: UrlPart::Fragment, to: UrlPart::Path}.apply(&mut task_state).unwrap();
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
    /// Action::MovePart {from: UrlPart::Fragment, to: UrlPart::Path}.apply(&mut task_state).unwrap();
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
    #[cfg_attr(feature = "cache", doc = "If the call to [`Cache::read`] returns [`None`], returns the error [`ActionError::CachedUrlIsNone`].")]
    #[cfg_attr(feature = "cache", doc = "")]
    #[cfg_attr(feature = "cache", doc = "If the call to [`BetterUrl::parse`] returns an error, that error is returned.")]
    #[cfg_attr(feature = "cache", doc = "")]
    /// If the call to [`TaskStateView::http_client`] returns an error, that error is returned.
    ///
    /// If the call to [`reqwest::blocking::RequestBuilder::send`] returns an error, that error is returned.
    ///
    /// If the response is a redirect and doesn't contain a `Location` header, returns the error [`ActionError::LocationHeaderNotFound`].
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
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ActionError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state);
    ///
    /// assert_eq!(task_state.scratchpad.flags.contains("abc"), false);
    /// Action::SetScratchpadFlag {name: "abc".into(), value: true}.apply(&mut task_state).unwrap();
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
    /// Action::SetScratchpadVar {name: "abc".into(), value: "def".into()}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("def"));
    /// Action::SetScratchpadVar {name: "abc".into(), value: StringSource::None}.apply(&mut task_state).unwrap();
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
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ActionError::StringSourceIsNone`].
    ///
    /// If the call to [`StringModification::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state);
    ///
    /// Action::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set("123".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// Action::SetScratchpadVar {name: "abc".into(), value: "def".into()}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("def"));
    /// Action::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set("123".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("123"));
    /// Action::SetScratchpadVar {name: "abc".into(), value: StringSource::None}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// ```
    ModifyScratchpadVar {
        /// The name of the var to modify.
        name: StringSource,
        /// The modification to apply.
        modification: StringModification
    },
    /// If an entry with the specified category and a key of the current [`TaskState::url`] exists in the cache, sets the [`TaskState::url`] to the entry's value.
    ///
    /// If no matching entry exists, applies [`Self::CacheUrl::action`] and makes a new entry with the specified category, the previous [`TaskState::url`] as the key, and the new [`TaskState::url`] as the value.
    ///
    /// If an error is returned, no new cache entry is written.
    /// # Errors
    /// If the call to [`Cache::read`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::read`] returns [`None`], returns the error [`ActionError::CachedUrlIsNone`].
    ///
    /// If the call to [`BetterUrl::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`Action::apply`] returns an error, that error is returned.
    ///
    /// If the call to [`Cache::write`] returns an error, that error is returned.
    #[cfg(feature = "cache")]
    CacheUrl {
        /// The category for the cache entry.
        category: StringSource,
        /// The action to apply and cache.
        action: Box<Self>
    },
    /// Applies a [`Self`] from [`TaskState::commons`]'s [`Commons::actions`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ActionError::StringSourceIsNone`].
    ///
    /// If the [`Commons::actions`] doesn't contain a [`Self`] with the specified name, returns the error [`ActionError::CommonActionNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// url_cleaner::task_state!(task_state, commons = Commons {
    ///     actions: [("abc".into(), Action::None)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// Action::Common(CommonCall {name: "abc".into(), args: Default::default()}).apply(&mut task_state).unwrap();
    /// ```
    Common(CommonCall),
    /// Calls the specified function and returns its value.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut TaskState) -> Result<(), ActionError>)
}

/// Helper function to get the default [`Rule::Repeat::limit`].
const fn get_10_u64() -> u64 {10}

/// The enum of errors [`Action::apply`] can return.
#[derive(Debug, Error, ErrorFilter)]
pub enum ActionError {
    /// Returned when a [`Action::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`Action`]s in a [`Action::TryElse`] return errors.
    #[error("Both Actions in a Action::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`Action::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`Action::TryElse::else`]. 
        else_error: Box<Self>
    },

    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// Returned when attempting to get the value of a query param from a URL with no query.
    #[error("Attempted to get the value of a query param from a URL with no query.")]
    NoQuery,
    /// Returned when attempting to get the value of a query param that wasn't found.
    #[error("Attempted to get the value of a query param that wasn't found.")]
    QueryParamNotFound,
    /// Returned when attempting to get the value of a query param that didn't have a value.
    #[error("Attempted to get the value of a query param that didn't have a value.")]
    QueryParamNoValue,
    /// Returned when a [`Action`] with the specified name isn't found in the [`Commons::actions`].
    #[error("A Action with the specified name wasn't found in the Commons::actions.")]
    CommonActionNotFound,
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
    /// An arbitrary [`std::error::Error`] returned by [`Action::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl Action {
    /// Applies the specified variant of [`Self`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), ActionError> {
        debug!(self, Action::apply, self, task_state);
        match self {
            // Testing.

            Self::None => {},
            Self::Error(msg) => Err(ActionError::ExplicitError(msg.clone()))?,
            Self::Debug(action) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                let action_result=action.apply(task_state);
                eprintln!("=== Action::Debug ===\nAction: {action:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nAction return value: {action_result:?}\nNew task_state: {task_state:?}");
                action_result?;
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(&task_state.to_view())? {
                then.apply(task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(task_state)?;
            },
            Self::All(actions) => {
                for action in actions {
                    action.apply(task_state)?;
                }
            },
            Self::PartMap  {part , map} => if let Some(action) = map.get(part .get( task_state.url      ) ) {action.apply(task_state)?},
            Self::StringMap{value, map} => if let Some(action) = map.get(value.get(&task_state.to_view())?) {action.apply(task_state)?},
            Self::Repeat{actions, limit} => {
                let mut previous_url;
                let mut previous_scratchpad;
                for _ in 0..*limit {
                    previous_url = task_state.url.clone();
                    previous_scratchpad = task_state.scratchpad.clone();
                    for action in actions {
                        action.apply(task_state)?;
                    }
                    if task_state.url == &previous_url && task_state.scratchpad == &previous_scratchpad {break;}
                }
            },
            // Error handling.

            Self::IgnoreError {action, filter} => if let Err(e) = action.apply(task_state) {if !filter.matches(&e) {Err(e)?}},
            Self::TryElse{ r#try, filter, r#else } => match r#try.apply(task_state) {
                Ok(x) => x,
                Err(try_error) => if filter.matches(&try_error) {
                    match r#else.apply(task_state) {
                        Ok(x) => x,
                        Err(else_error) => Err(ActionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                    }
                } else {
                    Err(try_error)?
                }
            },
            Self::FirstNotError(actions) => {
                let mut result = Ok(());
                for action in actions {
                    result = action.apply(task_state);
                    if result.is_ok() {break}
                }
                result?
            },
            Self::RevertOnError {action, filter} => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                if let Err(e) = action.apply(task_state) {
                    if filter.matches(&e) {
                        *task_state.url = old_url;
                        *task_state.scratchpad = old_scratchpad;
                    }
                    Err(e)?;
                }
            },

            // Query.

            Self::RemoveQuery => task_state.url.set_query(None),
            Self::RemoveQueryParam(name) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                let name = get_string!(name, task_state, ActionError);
                for param in query.split('&') {
                    if peh(param.split('=').next().expect("The first segment to always exist.")) != name {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::RemoveQueryParams(names) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                for param in query.split('&') {
                    if !names.contains(&*peh(param.split('=').next().expect("The first segment to always exist."))) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::AllowQueryParams(names) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                for param in query.split('&') {
                    if names.contains(&*peh(param.split('=').next().expect("The first segment to always exist."))) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                for param in query.split('&') {
                    if !matcher.satisfied_by(&peh(param.split('=').next().expect("The first segment to always exist.")), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                for param in query.split('&') {
                    if matcher.satisfied_by(&peh(param.split('=').next().expect("The first segment to always exist.")), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::GetUrlFromQueryParam(name) => {
                let task_state_view = task_state.to_view();
                let name = name.get(&task_state_view)?.ok_or(ActionError::StringSourceIsNone)?;

                match task_state.url.get_query_param(&name, 0) {
                    Some(Some(Some(new_url))) => {*task_state.url = Url::parse(&new_url)?.into();},
                    Some(Some(None))          => Err(ActionError::NoQuery)?,
                    Some(None)                => Err(ActionError::QueryParamNoValue)?,
                    None                      => Err(ActionError::QueryParamNotFound)?
                }
            },

            // Other parts.

            Self::SetHost(new_host) => task_state.url.set_host(new_host.as_deref())?,
            Self::Join(with) => *task_state.url=task_state.url.join(get_str!(with, task_state, ActionError))?.into(),

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
                        *task_state.url = BetterUrl::parse(&new_url.ok_or(ActionError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                let response = task_state.to_view().http_client(http_client_config_diff.as_deref())?.get(task_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    BetterUrl::parse(std::str::from_utf8(response.headers().get("location").ok_or(ActionError::LocationHeaderNotFound)?.as_bytes())?)?
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
                let name = get_string!(name, task_state, ActionError);
                match value {
                    true  => task_state.scratchpad.flags.insert( name),
                    false => task_state.scratchpad.flags.remove(&name)
                };
            },
            Self::SetScratchpadVar {name, value} => match value.get(&task_state.to_view())?.map(Cow::into_owned) {
                Some(value) => {let _ = task_state.scratchpad.vars.insert( get_string!(name, task_state, ActionError), value);}
                None        => {let _ = task_state.scratchpad.vars.remove(&get_string!(name, task_state, ActionError));}
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, task_state, ActionError).to_owned();
                if let Some(mut value) = task_state.scratchpad.vars.get(&name).map(ToOwned::to_owned) {
                    modification.apply(&mut value, &task_state.to_view())?;
                    let _ = task_state.scratchpad.vars.insert(name, value);
                }
            },
            #[cfg(feature = "cache")]
            Self::CacheUrl {category, action} => {
                let category = get_string!(category, task_state, ActionError);
                if task_state.params.read_cache {
                    if let Some(new_url) = task_state.cache.read(&category, task_state.url.as_str())? {
                        *task_state.url = BetterUrl::parse(&new_url.ok_or(ActionError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                let old_url = task_state.url.to_string();
                action.apply(task_state)?;
                if task_state.params.write_cache {
                    task_state.cache.write(&category, &old_url, Some(task_state.url.as_str()))?;
                }
            },
            Self::Common(common_call) => {
                task_state.commons.actions.get(get_str!(common_call.name, task_state, ActionError)).ok_or(ActionError::CommonActionNotFound)?.apply(&mut TaskState {
                    common_args: Some(&common_call.args.build(&task_state.to_view())?),
                    url        : task_state.url,
                    scratchpad : task_state.scratchpad,
                    context    : task_state.context,
                    job_context: task_state.job_context,
                    params     : task_state.params,
                    commons    : task_state.commons,
                    #[cfg(feature = "cache")]
                    cache      : task_state.cache
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        };
        Ok(())
    }
}

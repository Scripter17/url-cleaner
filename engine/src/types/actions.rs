//! Logic for how a [`TaskState`] should be modified.

use std::str::Utf8Error;
use std::collections::HashSet;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SetPreventDuplicates};
use thiserror::Error;
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;
#[expect(unused_imports, reason = "Used in doc comment.")]
use url::Url;

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
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
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
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
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
    #[doc = edoc!(satisfyerr(Condition), applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
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
    #[doc = edoc!(applyerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state);
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
    #[doc = edoc!(applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state);
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
    #[doc = edoc!(geterr(StringSource), applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state);
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
    /// Gets the name of the partition [`Self::PartNamedPartitioningMap::part`] is in in the specified [`NamedPartitioning`], indexes [`Self::PartNamedPartitioningMap::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Action, 2), notfound(NamedPartitioning, Action), applyerr(Self))]
    PartNamedPartitioningMap {
        /// The [`NamedPartitioning`] to search in.
        named_partitioning: StringSource,
        /// The [`UrlPart`] whose value to find in the [`NamedPartitioning`].
        part: UrlPart,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Gets the name of the partition [`Self::StringNamedPartitioningMap::value`] is in in the specified [`NamedPartitioning`], indexes [`Self::StringNamedPartitioningMap::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), notfound(NamedPartitioning, Action), applyerr(Self))]
    StringNamedPartitioningMap {
        /// The [`NamedPartitioning`] to search in.
        named_partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`NamedPartitioning`].
        value: StringSource,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Map<Self>
    },

    /// Repeat [`Self::Repeat::actions`] until either no changes happen or the rules were executed [`Self::Repeat::limit`] times.
    /// # Errors
    #[doc = edoc!(applyerr(Self, 3))]
    Repeat {
        /// The [`Self`]s to repeat.
        actions: Vec<Action>,
        /// The maximum amount of times to repeat.
        ///
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64
    },

    /// If the contained [`Self`] returns an error, ignore it.
    ///
    /// Does not revert any successful calls to [`Self::apply`]. For that, also use [`Self::RevertOnError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state);
    ///
    /// Action::IgnoreError(Box::new(Action::Error("...".into()))).apply(&mut task_state).unwrap();
    /// ```
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the [`TaskState`] to its previous state.
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    RevertOnError(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::apply`] returns an error, apply [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(applyerrte(Self, Action))]
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },
    /// Applies the contained [`Self`]s in order, stopping as soon as a call to [`Self::apply`] doesn't return an error.
    /// # Errors
    #[doc = edoc!(applyerrfne(Self, Action))]
    FirstNotError(Vec<Self>),

    /// Remove the entire [`UrlPart::Query`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2");
    ///
    /// Action::RemoveQuery.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/");
    /// ```
    RemoveQuery,
    /// Removes all query parameters with the specified name.
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2&b=3&a=4&c=5");
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
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2&b=3&%61=4&c=5");
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
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// Action::AllowQueryParams(["a".to_string(), "b".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("a=2&b=3&%61=4"));
    /// Action::AllowQueryParams(["c".to_string()].into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    AllowQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Removes all query params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(satisfyerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2&b=3&%61=4&c=5");
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
    #[doc = edoc!(satisfyerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// Action::AllowQueryParamsMatching(StringMatcher::Is("a".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), Some("a=2&%61=4"));
    /// Action::AllowQueryParamsMatching(StringMatcher::Is("b".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url.query(), None);
    /// ```
    AllowQueryParamsMatching(StringMatcher),
    /// Extreme shorthand for handling universal query parameters.
    /// # Errors
    #[doc = edoc!(notfound(Set, Action))]
    ///
    /// If the list isn't found, returns the error [`ActionError::ListNotFound`].
    RemoveQueryParamsInSetOrStartingWithAnyInList {
        /// The name of the [`Set`] in [`Params::sets`] to use.
        set: String,
        /// The name of the list in [`Params::lists`] to use.
        list: String
    },
    /// Sets [`UrlPart::Whole`] to the value of the first query parameter with a name determined by the [`TaskState`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    ///
    /// If no matching query parameter is found, returns the error [`ActionError::QueryParamNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com?redirect=https://example.com/2");
    ///
    /// Action::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/2");
    ///
    /// Action::GetUrlFromQueryParam("redirect".into()).apply(&mut task_state).unwrap_err();
    /// ```
    GetUrlFromQueryParam(StringSource),



    /// Replace an entire [`TaskState::url`] to a constant value without reparsing it for each [`Task`].
    SetWhole(BetterUrl),
    /// Sets the [`UrlPart::Host`] to the specified value.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_host))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::SetHost(Some("example2.com".into())).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example2.com/")
    /// ```
    SetHost(Option<String>),
    /// "Join"s a URL like how relative links on websites work.
    ///
    /// See [`Url::join`] for details.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(Url::join))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com/a/b/c");
    ///
    /// Action::Join("..".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/a/");
    ///
    ///
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com/a/b/c/");
    ///
    /// Action::Join("..".into()).apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/a/b/");
    /// ```
    Join(StringSource),



    /// Sets the specified [`UrlPart`] to the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), seterr(UrlPart))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
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
    #[doc = edoc!(applyerr(StringModification), seterr(UrlPart))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::ModifyPart {part: UrlPart::Path, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc");
    ///
    /// Action::ModifyPart {part: UrlPart::Query, modification: StringModification::Set("abc".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.url, "https://example.com/abc?abc");
    /// ```
    ModifyPart {
        /// The part to modify.
        part: UrlPart,
        /// The modification to apply to the part.
        modification: StringModification
    },
    /// If the specified [`UrlPart`] is [`Some`], apply [`Self::ModifyPartIfSome::modification`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), seterr(UrlPart))]
    ModifyPartIfSome {
        /// The [`UrlPart`] to modify.
        part: UrlPart,
        /// The [`StringModification`] to apply.
        modification: StringModification
    },
    /// Sets [`Self::CopyPart::to`] to the value of [`Self::CopyPart::from`], leaving [`Self::CopyPart::from`] unchanged.
    /// # Errors
    #[doc = edoc!(seterr(UrlPart))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com/abc#def");
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
    #[doc = edoc!(seterr(UrlPart, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com/abc#def");
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
    #[cfg_attr(feature = "cache", doc = edoc!(callerr(Cache::read), callnone(Cache::read, ActionError::CachedUrlIsNone), callerr(BetterUrl::parse)))]
    #[cfg_attr(feature = "cache", doc = "")]
    #[doc = edoc!(callerr(TaskStateView::http_client), callerr(reqwest::blocking::RequestBuilder::send))]
    ///
    /// If the response is a redirect:
    ///
    /// - If the `Location` header is missing, returns the error [`ActionError::LocationHeaderNotFound`].
    ///
    #[doc = edoc!(listitem, callerr(std::str::from_utf8), callerr(BetterUrl::parse))]
    #[cfg_attr(feature = "cache", doc = "")]
    #[cfg_attr(feature = "cache", doc = edoc!(callerr(Cache::write)))]
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
    /// Sets the specified [`Scratchpad::flags`] to [`Self::SetScratchpadFlag::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state);
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
    /// Sets the specified [`Scratchpad::vars`] to [`Self::SetScratchpadVar::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state);
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
    /// If the specified [`Scratchpad::vars`] is [`Some`], applies [`Self::ModifyScratchpadVar::modification`].
    ///
    /// If the part is [`None`], does nothing.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), applyerr(StringModification))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state);
    ///
    /// Action::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set("123".into())}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), Some("123"));
    /// Action::ModifyScratchpadVar {name: "abc".into(), modification: StringModification::Set(StringSource::None)}.apply(&mut task_state).unwrap();
    /// assert_eq!(task_state.scratchpad.vars.get("abc").map(|x| &**x), None);
    /// ```
    ModifyScratchpadVar {
        /// The name of the var to modify.
        name: StringSource,
        /// The modification to apply.
        modification: StringModification
    },
    /// If an entry with a category of [`Self::CacheUrl::category`] and a key of [`TaskState::url`] exists in the [`TaskState::cache`], sets the URL to the entry's value.
    ///
    /// If no such entry exists, applies [`Self::CacheUrl::action`] and inserts a new entry equivalent to applying it.
    ///
    /// Does not cache the [`TaskState::scratchpad`].
    /// # Errors
    #[doc = edoc!(callerr(Cache::read), callnone(Cache::read, ActionError::CachedUrlIsNone), callerr(BetterUrl::parse), applyerr(Self), callerr(Cache::write))]
    #[cfg(feature = "cache")]
    CacheUrl {
        /// The category for the cache entry.
        category: StringSource,
        /// The action to apply and cache.
        action: Box<Self>
    },
    /// Applies a [`Self`] from [`TaskState::commons`]'s [`Commons::actions`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), commonnotfound(Self, Action), callerr(CommonCallArgsSource::build), applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state, commons = Commons {
    ///     actions: [("abc".into(), Action::None)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// Action::Common(CommonCall {name: Box::new("abc".into()), args: Default::default()}).apply(&mut task_state).unwrap();
    /// ```
    Common(CommonCall),
    /// Calls the specified function and returns its value.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state!(task_state);
    ///
    /// fn some_complex_operation(task_state: &mut TaskState) -> Result<(), ActionError> {
    ///     Ok(())
    /// }
    ///
    /// Action::Custom(some_complex_operation).apply(&mut task_state).unwrap();
    /// ```
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut TaskState) -> Result<(), ActionError>)
}

/// Helper function to get the default [`Action::Repeat::limit`].
const fn get_10_u64() -> u64 {10}

/// The enum of errors [`Action::apply`] can return.
#[derive(Debug, Error)]
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
    /// Returned when all [`Action`]s in a [`Action::FirstNotError`] error.
    #[error("All Actions in a Action::FirstNotError errored.")]
    FirstNotErrorErrors(Vec<Self>),

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
    #[error("An Action with the specified name wasn't found in the Commons::actions.")]
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
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`NamedPartitioning`] with the specified name isn't found.
    #[error("A NamedPartitioning with the specified name wasn't found.")]
    NamedPartitioningNotFound,
    /// Returned when a [`Set`] with the specified name isn't found.
    #[error("A Set with the specified name wasn't found.")]
    SetNotFound,
    /// Returned when a list with the specified name isn't found.
    #[error("A list with the specified name wasn't found.")]
    ListNotFound,

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
        debug!(self, Action::apply, task_state);
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
            Self::PartMap  {part , map} => if let Some(action) = map.get(part .get( task_state.url      ) ) {action.apply(task_state)?;},
            Self::StringMap{value, map} => if let Some(action) = map.get(value.get(&task_state.to_view())?) {action.apply(task_state)?;},

            Self::PartNamedPartitioningMap   {named_partitioning: StringSource::String(named_partitioning), part , map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(named_partitioning).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(part .get( task_state.url      ) .as_deref())) {action.apply(task_state)?;}
            Self::StringNamedPartitioningMap {named_partitioning: StringSource::String(named_partitioning), value, map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(named_partitioning).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(value.get(&task_state.to_view())?.as_deref())) {action.apply(task_state)?;}

            Self::PartNamedPartitioningMap   {named_partitioning, part , map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(&*named_partitioning.get(&task_state.to_view())?.ok_or(ActionError::StringSourceIsNone)?).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(part .get( task_state.url      ) .as_deref())) {action.apply(task_state)?;}
            Self::StringNamedPartitioningMap {named_partitioning, value, map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(&*named_partitioning.get(&task_state.to_view())?.ok_or(ActionError::StringSourceIsNone)?).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(value.get(&task_state.to_view())?.as_deref())) {action.apply(task_state)?;}

            Self::Repeat{actions, limit} => {
                let mut previous_url;
                let mut previous_scratchpad;
                for _ in 0..*limit {
                    previous_url = task_state.url.to_string();
                    previous_scratchpad = task_state.scratchpad.clone();
                    for action in actions {
                        action.apply(task_state)?;
                    }
                    if task_state.url == &previous_url && task_state.scratchpad == &previous_scratchpad {break;}
                }
            },
            // Error handling.

            Self::IgnoreError(action) => {let _ = action.apply(task_state);},
            Self::TryElse{ r#try, r#else } => match r#try.apply(task_state) {
                Ok(x) => x,
                Err(try_error) => match r#else.apply(task_state) {
                    Ok(x) => x,
                    Err(else_error) => Err(ActionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                }
            },
            Self::FirstNotError(actions) => {
                let mut errors = Vec::new();
                for action in actions {
                    match action.apply(task_state) {
                        Ok(()) => return Ok(()),
                        Err(e) => errors.push(e)
                    }
                }
                Err(ActionError::FirstNotErrorErrors(errors))?
            },
            Self::RevertOnError(action) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                if let Err(e) = action.apply(task_state) {
                    *task_state.url = old_url;
                    *task_state.scratchpad = old_scratchpad;
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
                    if !matcher.satisfied_by(Some(&*peh(param.split('=').next().expect("The first segment to always exist."))), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                for param in query.split('&') {
                    if matcher.satisfied_by(Some(&*peh(param.split('=').next().expect("The first segment to always exist."))), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
            },
            Self::RemoveQueryParamsInSetOrStartingWithAnyInList {set, list} => if let Some(query) = task_state.url.query() {
                let mut new = String::new();
                let set = task_state.params.sets.get(set).ok_or(ActionError::SetNotFound)?;
                let list = task_state.params.lists.get(list).ok_or(ActionError::ListNotFound)?;
                for param in query.split('&') {
                    let name = peh(param.split('=').next().expect("The first segment to always exist."));
                    if !set.contains(Some(&*name)) && !list.iter().any(|x| name.starts_with(x)) {
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
                    Some(Some(Some(new_url))) => {*task_state.url = BetterUrl::parse(&new_url)?;},
                    Some(Some(None))          => Err(ActionError::QueryParamNoValue)?,
                    Some(None)                => Err(ActionError::QueryParamNotFound)?,
                    None                      => Err(ActionError::NoQuery)?
                }
            },

            // Other parts.

            Self::SetWhole(new) => *task_state.url = new.clone(),
            Self::SetHost(new_host) => task_state.url.set_host(new_host.as_deref())?,
            Self::Join(with) => *task_state.url=task_state.url.join(get_str!(with, task_state, ActionError))?.into(),

            // Generic part handling.

            Self::SetPart {part, value} => part.set(task_state.url, value.get(&task_state.to_view())?.map(Cow::into_owned).as_deref())?, // The deref is needed for borrow checking reasons.
            Self::ModifyPart {part, modification} => {
                let mut temp = part.get(task_state.url);
                modification.apply(&mut temp, &task_state.to_view())?;
                part.set(task_state.url, temp.map(Cow::into_owned).as_deref())?;
            },
            Self::ModifyPartIfSome {part, modification} => {
                if let mut temp @ Some(_) = part.get(task_state.url) {
                    modification.apply(&mut temp, &task_state.to_view())?;
                    part.set(task_state.url, temp.map(Cow::into_owned).as_deref())?;
                }
            }
            Self::CopyPart {from, to} => to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart {from, to} => {
                to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?;
                from.set(task_state.url, None)?;
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
                let mut value = task_state.scratchpad.vars.get(&name).map(|x| Cow::Borrowed(&**x));
                modification.apply(&mut value, &task_state.to_view())?;
                match value {
                    Some(value) => {let _ = task_state.scratchpad.vars.insert(name, value.into_owned());},
                    None => {let _ = task_state.scratchpad.vars.remove(&name);}
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

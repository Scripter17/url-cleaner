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
use url::{Url, PathSegmentsMut};
use ::percent_encoding::percent_decode_str as pds;

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// Actions are how [`TaskState`]s get manipulated to clean URLs.
///
/// Please note that, in general, when a [`Action`] returns an [`Err`], the [`TaskState`] may still be modified. For example:
/// ```
/// use url_cleaner_engine::types::*;
/// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
///
/// Action::All(vec![
///     Action::SetPath("/change".into()),
///     Action::Error("This won't revert the above".into()),
///     Action::SetPath("/wont-happen".into())
/// ]).apply(&mut task_state).unwrap_err();
///
/// assert_eq!(task_state.url, "https://example.com/change");
/// ```
///
/// This is because reverting on an error requires keeping a copy of the input state, which is very expensive compared to the benefit.
///
/// If you need to revert the task state when an error is returned, use [`Self::RevertOnError`] to revert the effects but still return the error, and optionally [`Self::IgnoreError`] to ignore the error.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
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
    #[doc = edoc!(checkerr(Condition), applyerr(Self))]
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
    ///     Action::SetHost("example2.com".into()),
    ///     Action::Error("...".into()),
    ///     Action::SetHost("example3.com".into()),
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
    /// Gets the name of the partition [`Self::PartNamedPartitioning::part`] is in in the specified [`NamedPartitioning`], indexes [`Self::PartNamedPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Action, 2), notfound(NamedPartitioning, Action), applyerr(Self))]
    PartNamedPartitioning {
        /// The [`NamedPartitioning`] to search in.
        named_partitioning: StringSource,
        /// The [`UrlPart`] whose value to find in the [`NamedPartitioning`].
        part: UrlPart,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Gets the name of the partition [`Self::StringNamedPartitioning::value`] is in in the specified [`NamedPartitioning`], indexes [`Self::StringNamedPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), notfound(NamedPartitioning, Action), applyerr(Self))]
    StringNamedPartitioning {
        /// The [`NamedPartitioning`] to search in.
        named_partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`NamedPartitioning`].
        value: StringSource,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Map<Self>
    },



    /// Repeat [`Self::Repeat::actions`] until either the [`TaskState::url`] and [`TaskState::scratchpad`] end up in the same state or the rules were executed [`Self::Repeat::limit`] times.
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
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::IgnoreError(Box::new(
    ///     Action::RevertOnError(Box::new(
    ///         Action::All(vec![
    ///             Action::SetPath("/change".into()),
    ///             Action::Error("This won't revert the above".into()),
    ///             Action::SetPath("/wont-happen".into())
    ///         ])
    ///     ))
    /// )).apply(&mut task_state).unwrap(); // Error is ignored.
    ///
    /// assert_eq!(task_state.url, "https://example.com/"); // The first `Action::SetPath` is reverted.
    /// ```
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the [`TaskState`] to its previous state then return the error.
    ///
    /// To ignore errors, put this in a [`Self::IgnoreError`].
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state!(task_state, url = "https://example.com");
    ///
    /// Action::RevertOnError(Box::new(
    ///     Action::All(vec![
    ///         Action::SetPath("/change".into()),
    ///         Action::Error("This won't revert the above".into()),
    ///         Action::SetPath("/wont-happen".into())
    ///     ])
    /// )).apply(&mut task_state).unwrap_err(); // Still returns an error.
    ///
    /// assert_eq!(task_state.url, "https://example.com/"); // The first `Action::SetPath` is reverted.
    /// ```
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

    // Whole

    /// Sets [`UrlPart::Whole`].
    SetWhole(StringSource),
    /// [`Url::join`].
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

    // Scheme

    /// [`BetterUrl::set_scheme`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_scheme))]
    SetScheme(StringSource),

    // Host

    /// [`BetterUrl::set_host`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_host))]
    SetHost(StringSource),
    /// [`BetterUrl::set_subdomain`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_subdomain))]
    SetSubdomain(StringSource),
    /// [`BetterUrl::set_reg_domain`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_reg_domain))]
    SetRegDomain(StringSource),
    /// [`BetterUrl::set_domain`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain))]
    SetDomain(StringSource),
    /// [`BetterUrl::set_domain_middle`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_middle))]
    SetDomainMiddle(StringSource),
    /// [`BetterUrl::set_not_domain_suffix`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_not_domain_suffix))]
    SetNotDomainSuffix(StringSource),
    /// [`BetterUrl::set_domain_suffix`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_suffix))]
    SetDomainSuffix(StringSource),
    /// [`BetterUrl::domain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::domain_segment))]
    SetDomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::subdomain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::subdomain_segment))]
    SetSubdomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::domain_suffix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::domain_suffix_segment))]
    SetDomainSuffixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_segment_after`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_segment_after))]
    InsertDomainSegmentAfter {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_subdomain_segment_after`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_subdomain_segment_after))]
    InsertSubdomainSegmentAfter {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_suffix_segment_after`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_suffix_segment_after))]
    InsertDomainSuffixSegmentAfter {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_segment_at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_segment_at))]
    InsertDomainSegmentAt {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_subdomain_segment_at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_subdomain_segment_at))]
    InsertSubdomainSegmentAt {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_suffix_segment_at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_suffix_segment_at))]
    InsertDomainSuffixSegmentAt {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },

    /// [`BetterUrl::set_fqdn`] to [`true`]
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_fqdn))]
    EnsureFqdnPeriod,
    /// [`BetterUrl::set_fqdn`] to [`false`]
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_fqdn))]
    RemoveFqdnPeriod,

    /// [`BetterUrl::set_path`].
    SetPath(StringSource),
    /// Removes the specified [`UrlPart::PathSegment`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_path_segment))]
    RemovePathSegment(isize),
    /// [`BetterUrl::set_path_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_path_segment))]
    SetPathSegment {
        /// The [`UrlPart::PathSegment`] to set.
        index: isize,
        /// The value to set it to.
        value: StringSource
    },
    /// [`BetterUrl::insert_path_segment_at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment_at))]
    InsertPathSegmentAt {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_path_segment_after`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment_after))]
    InsertPathSegmentAfter {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::set_path_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_path_segment))]
    SetRawPathSegment {
        /// The [`UrlPart::PathSegment`] to set.
        index: isize,
        /// The value to set it to.
        value: StringSource
    },
    /// [`BetterUrl::insert_path_segment_at`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment_at))]
    InsertRawPathSegmentAt {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_path_segment_after`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment_after))]
    InsertRawPathSegmentAfter {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`PathSegmentsMut::pop_if_empty`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segments_mut))]
    RemoveEmptyLastPathSegment,
    /// [`PathSegmentsMut::pop_if_empty`] and [`PathSegmentsMut::push`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::path_segments_mut))]
    RemoveEmptyLastPathSegmentAndInsertNew(StringSource),
    /// Remove the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, this means trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::remove_first_n_path_segments))]
    RemoveFirstNPathSegments(usize),
    /// Keep the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, this means trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::keep_first_n_path_segments))]
    KeepFirstNPathSegments(usize),
    /// Remove the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, this means trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::remove_last_n_path_segments))]
    RemoveLastNPathSegments(usize),
    /// Keep the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, this means trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::keep_last_n_path_segments))]
    KeepLastNPathSegments(usize),



    /// [`BetterUrl::set_query`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_query))]
    SetQuery(StringSource),
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
    /// If the [`Url::query`] is `Some("")`, set it to [`None`].
    RemoveEmptyQuery,
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
    /// Keeps all query parameters with the specified name.
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    AllowQueryParam(StringSource),
    /// Removes all query params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
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
    #[doc = edoc!(checkerr(StringMatcher))]
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
    #[doc = edoc!(checkerr(StringMatcher))]
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
    /// Rename the specified query parameter to the specified name.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::rename_query_param))]
    RenameQueryParam {
        /// The query parameter to rename.
        from: QueryParamSelector,
        /// The name to rename it to.
        to: StringSource
    },

    /// Sets [`UrlPart::Whole`] to the value of the first query parameter with a name determined by the [`TaskState`].
    /// # Errors
    /// If the URL doesn't have a query, returns the error [`ActionError::NoQuery`].
    ///
    /// If the specified query param isn't found, returns the error [`ActionError::QueryParamNotFound`].
    ///
    /// If the specified query param doesn't have a value, returns the error [`ActionError::QueryParamNoValue`].
    /// 
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

    // Fragment

    /// Removes the [`UrlPart::Fragment`].
    RemoveFragment,
    /// If the [`Url::fragment`] is `Some("")`, set it to [`None`].
    RemoveEmptyFragment,

    // General parts

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

    // Misc.

    /// Sends an HTTP GET request to the current [`TaskState::url`], and sets it either to the value of the response's `Location` header (if the response is a redirect) or the final URL after redirects.
    ///
    /// If the `cache` feature flag is enabled, caches the operation with the subject `redirect`, the key set to the input URL, and the value set to the returned URL.
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
    /// If an entry with a subject of [`Self::CacheUrl::subject`] and a key of [`TaskState::url`] exists in the [`TaskState::cache`], sets the URL to the entry's value.
    ///
    /// If no such entry exists, applies [`Self::CacheUrl::action`] and inserts a new entry equivalent to applying it.
    ///
    /// Does not cache the [`TaskState::scratchpad`].
    /// # Errors
    #[doc = edoc!(callerr(Cache::read), callnone(Cache::read, ActionError::CachedUrlIsNone), callerr(BetterUrl::parse), applyerr(Self), callerr(Cache::write))]
    #[cfg(feature = "cache")]
    CacheUrl {
        /// The subject for the cache entry.
        subject: StringSource,
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
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::actions`] and applies it.
    /// # Errors
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`ActionError::NotInCommonContext`].
    ///
    #[doc = edoc!(commoncallargnotfound(Self, Action), applyerr(Self))]
    CommonCallArg(StringSource),
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

    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringModificationError`] is encountered.
    #[error(transparent)]
    StringModificationError(#[from] StringModificationError),
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),

    /// Returned when a [`SetSchemeError`] is encountered.
    #[error(transparent)]
    SetSchemeError(#[from] SetSchemeError),
    /// Returned when attempting to set a URL's scheme to [`None`].
    #[error("Attempted to set the URL's scheme to None.")]
    SchemeCannotBeNone,

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// Returned when a [`SetSubdomainError`] is encountered.
    #[error(transparent)]
    SetSubdomainError(#[from] SetSubdomainError),
    /// Returned when a [`SetRegDomainError`] is encountered.
    #[error(transparent)]
    SetRegDomainError(#[from] SetRegDomainError),
    /// Returned when a [`SetDomainError`] is encountered.
    #[error(transparent)]
    SetDomainError(#[from] SetDomainError),
    /// Returned when a [`SetDomainMiddleError`] is encountered.
    #[error(transparent)]
    SetDomainMiddleError(#[from] SetDomainMiddleError),
    /// Returned when a [`SetNotDomainSuffixError`] is encountered.
    #[error(transparent)]
    SetNotDomainSuffixError(#[from] SetNotDomainSuffixError),
    /// Returned when a [`SetDomainSuffixError`] is encountered.
    #[error(transparent)]
    SetDomainSuffixError(#[from] SetDomainSuffixError),
    /// Returned when a [`SetFqdnError`] is encountered.
    #[error(transparent)]
    SetFqdnError(#[from] SetFqdnError),

    /// Returned when a [`InsertDomainSegmentError`] is encountered.
    #[error(transparent)]
    InsertDomainSegmentError(#[from] InsertDomainSegmentError),
    /// Returned when a [`InsertSubdomainSegmentError`] is encountered.
    #[error(transparent)]
    InsertSubdomainSegmentError(#[from] InsertSubdomainSegmentError),
    /// Returned when a [`InsertDomainSuffixSegmentError`] is encountered.
    #[error(transparent)]
    InsertDomainSuffixSegmentError(#[from] InsertDomainSuffixSegmentError),
    /// Returned when a [`SetDomainSegmentError`] is encountered.
    #[error(transparent)]
    SetDomainSegmentError(#[from] SetDomainSegmentError),
    /// Returned when a [`SetSubdomainSegmentError`] is encountered.
    #[error(transparent)]
    SetSubdomainSegmentError(#[from] SetSubdomainSegmentError),
    /// Returned when a [`SetDomainSuffixSegmentError`] is encountered.
    #[error(transparent)]
    SetDomainSuffixSegmentError(#[from] SetDomainSuffixSegmentError),

    /// Returned when attempting to set a URL's path to [`None`].
    #[error("Attempted to set the URL's path to None.")]
    PathCannotBeNone,
    /// Returned when attempting to get the path segments of a URL with no path segments.
    #[error("Attempted to get the path segments of a URL with no path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when a [`SetPathSegmentError`] is encountered.
    #[error(transparent)]
    SetPathSegmentError(#[from] SetPathSegmentError),
    /// Returned when a [`InsertPathSegmentError`] is encountered.
    #[error(transparent)]
    InsertPathSegmentError(#[from] InsertPathSegmentError),
    /// Returned when attempting to keep/remove more path segments than are available.
    #[error("Attempted to keep/remove more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when a [`RemoveFirstNPathSegmentsError`] is encountered.
    #[error(transparent)]
    RemoveFirstNPathSegmentsError(#[from] RemoveFirstNPathSegmentsError),
    /// Returned when a [`KeepFirstNPathSegmentsError`] is encountered.
    #[error(transparent)]
    KeepFirstNPathSegmentsError(#[from] KeepFirstNPathSegmentsError),
    /// Returned when a [`RemoveLastNPathSegmentsError`] is encountered.
    #[error(transparent)]
    RemoveLastNPathSegmentsError(#[from] RemoveLastNPathSegmentsError),
    /// Returned when a [`KeepLastNPathSegmentsError`] is encountered.
    #[error(transparent)]
    KeepLastNPathSegmentsError(#[from] KeepLastNPathSegmentsError),

    /// Returned when attempting to get the value of a query param from a URL with no query.
    #[error("Attempted to get the value of a query param from a URL with no query.")]
    NoQuery,
    /// Returned when attempting to get the value of a query param that wasn't found.
    #[error("Attempted to get the value of a query param that wasn't found.")]
    QueryParamNotFound,
    /// Returned when attempting to get the value of a query param that didn't have a value.
    #[error("Attempted to get the value of a query param that didn't have a value.")]
    QueryParamNoValue,
    /// Returned when a [`RenameQueryParamError`] is encountered.
    #[error(transparent)]
    RenameQueryParamError(#[from] RenameQueryParamError),

    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// Returned when a [`SetUrlPartError`] is encountered.
    #[error(transparent)]
    SetUrlPartError(#[from] SetUrlPartError),

    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),

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
    /// Returned when a [`Action`] with the specified name isn't found in the [`Commons::actions`].
    #[error("An Action with the specified name wasn't found in the Commons::actions.")]
    CommonActionNotFound,
    /// Returned when trying to use [`Action::CommonCallArg`] outside of a common context.
    #[error("Tried to use Action::CommonCallArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the [`Action`] requested from an [`Action::CommonCallArg`] isn't found.
    #[error("The Action requested from an Action::CommonCallArg wasn't found.")]
    CommonCallArgActionNotFound,
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
        debug!(Action::apply, self, task_state.debug_helper());

        match self {
            // Debug/constants

            Self::None => {},
            Self::Error(msg) => Err(ActionError::ExplicitError(msg.clone()))?,
            Self::Debug(action) => {
                let old_url = task_state.url.clone();
                let old_scratchpad = task_state.scratchpad.clone();
                let action_result=action.apply(task_state);
                eprintln!("=== Action::Debug ===\nAction: {action:?}\nOld URL: {old_url:?}\nOld scratchpad: {old_scratchpad:?}\nAction return value: {action_result:?}\nNew task_state: {task_state:?}");
                action_result?;
            },

            // Error handling

            Self::IgnoreError(action) => {let _ = action.apply(task_state);},
            Self::TryElse {r#try, r#else} => match r#try.apply(task_state) {
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

            // Logic

            Self::If {r#if, then, r#else} => if r#if.check(&task_state.to_view())? {
                then.apply(task_state)?;
            } else if let Some(r#else) = r#else {
                r#else.apply(task_state)?;
            },
            Self::All(actions) => {
                for action in actions {
                    action.apply(task_state)?;
                }
            },
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

            // Maps

            Self::PartMap   {part , map} => if let Some(action) = map.get(part .get( task_state.url      ) ) {action.apply(task_state)?;},
            Self::StringMap {value, map} => if let Some(action) = map.get(value.get(&task_state.to_view())?) {action.apply(task_state)?;},

            Self::PartNamedPartitioning   {named_partitioning, part , map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(get_str!(named_partitioning, task_state, ActionError)).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(part.get(task_state.url).as_deref())) {action.apply(task_state)?;}
            Self::StringNamedPartitioning {named_partitioning, value, map} => if let Some(action) = map.get(task_state.params.named_partitionings.get(get_str!(named_partitioning, task_state, ActionError)).ok_or(ActionError::NamedPartitioningNotFound)?.get_partition_of(get_option_str!(value, task_state) )) {action.apply(task_state)?;}

            // Whole

            Self::SetWhole(new) => *task_state.url = BetterUrl::parse(get_new_str!(new, task_state, ActionError))?,
            Self::Join(with) => *task_state.url=task_state.url.join(get_str!(with, task_state, ActionError))?.into(),

            // Scheme

            Self::SetScheme(to) => task_state.url.set_scheme(get_new_str!(to, task_state, ActionError))?,

            // Domain

            Self::SetHost           (to) => task_state.url.set_host             (get_new_option_str!(to, task_state))?,
            Self::SetSubdomain      (to) => task_state.url.set_subdomain        (get_new_option_str!(to, task_state))?,
            Self::SetRegDomain      (to) => task_state.url.set_reg_domain       (get_new_option_str!(to, task_state))?,
            Self::SetDomain         (to) => task_state.url.set_domain           (get_new_option_str!(to, task_state))?,
            Self::SetDomainMiddle   (to) => task_state.url.set_domain_middle    (get_new_option_str!(to, task_state))?,
            Self::SetNotDomainSuffix(to) => task_state.url.set_not_domain_suffix(get_new_option_str!(to, task_state))?,
            Self::SetDomainSuffix   (to) => task_state.url.set_domain_suffix    (get_new_option_str!(to, task_state))?,

            Self::SetDomainSegment               {index, value} => task_state.url.set_domain_segment                (*index, get_new_option_str!(value, task_state))?,
            Self::SetSubdomainSegment            {index, value} => task_state.url.set_subdomain_segment             (*index, get_new_option_str!(value, task_state))?,
            Self::SetDomainSuffixSegment         {index, value} => task_state.url.set_domain_suffix_segment         (*index, get_new_option_str!(value, task_state))?,
            Self::InsertDomainSegmentAt          {index, value} => task_state.url.insert_domain_segment_at          (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertSubdomainSegmentAt       {index, value} => task_state.url.insert_subdomain_segment_at       (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertDomainSuffixSegmentAt    {index, value} => task_state.url.insert_domain_suffix_segment_at   (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertDomainSegmentAfter       {index, value} => task_state.url.insert_domain_segment_after       (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertSubdomainSegmentAfter    {index, value} => task_state.url.insert_subdomain_segment_after    (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertDomainSuffixSegmentAfter {index, value} => task_state.url.insert_domain_suffix_segment_after(*index, get_new_str!(value, task_state, ActionError))?,

            Self::EnsureFqdnPeriod => task_state.url.set_fqdn(true)?,
            Self::RemoveFqdnPeriod => task_state.url.set_fqdn(false)?,

            // Path

            Self::SetPath(to) => task_state.url.set_path(get_new_str!(to, task_state, ActionError)),

            Self::RemovePathSegment         (index) => task_state.url.set_path_segment(*index, None)?,
            Self::SetPathSegment            {index, value} => task_state.url.set_path_segment             (*index, get_new_option_str!(value, task_state))?,
            Self::InsertPathSegmentAt       {index, value} => task_state.url.insert_path_segment_at       (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertPathSegmentAfter    {index, value} => task_state.url.insert_path_segment_after    (*index, get_new_str!(value, task_state, ActionError))?,
            Self::SetRawPathSegment         {index, value} => task_state.url.set_raw_path_segment         (*index, get_new_option_str!(value, task_state))?,
            Self::InsertRawPathSegmentAt    {index, value} => task_state.url.insert_raw_path_segment_at   (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertRawPathSegmentAfter {index, value} => task_state.url.insert_raw_path_segment_after(*index, get_new_str!(value, task_state, ActionError))?,
            Self::RemoveEmptyLastPathSegment => {task_state.url.path_segments_mut().ok_or(ActionError::UrlDoesNotHavePathSegments)?.pop_if_empty();},
            Self::RemoveEmptyLastPathSegmentAndInsertNew(value) => {
                let value = get_new_str!(value, task_state, ActionError);
                let mut segments_mut = task_state.url.path_segments_mut().ok_or(ActionError::UrlDoesNotHavePathSegments)?;
                segments_mut.pop_if_empty();
                segments_mut.push(value);
            },
            Self::RemoveFirstNPathSegments(n) => task_state.url.remove_first_n_path_segments(*n)?,
            Self::KeepFirstNPathSegments  (n) => task_state.url.keep_first_n_path_segments  (*n)?,
            Self::RemoveLastNPathSegments (n) => task_state.url.remove_last_n_path_segments (*n)?,
            Self::KeepLastNPathSegments   (n) => task_state.url.keep_last_n_path_segments   (*n)?,

            // Query

            Self::SetQuery(to) => task_state.url.set_query(get_new_option_str!(to, task_state)),
            Self::RemoveQuery => task_state.url.set_query(None),
            Self::RemoveEmptyQuery => if task_state.url.query() == Some("") {task_state.url.set_query(None)},
            Self::RemoveQueryParam(name) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                let name = get_str!(name, task_state, ActionError);
                for param in query.split('&') {
                    if pds(param.split('=').next().expect("The first segment to always exist.")).ne(name.bytes()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowQueryParam(name) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                let name = get_str!(name, task_state, ActionError);
                for param in query.split('&') {
                    if pds(param.split('=').next().expect("The first segment to always exist.")).eq(name.bytes()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RemoveQueryParams(names) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                for param in query.split('&') {
                    if !names.contains(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowQueryParams(names) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                for param in query.split('&') {
                    if names.contains(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RemoveQueryParamsMatching(matcher) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                for param in query.split('&') {
                    if !matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowQueryParamsMatching(matcher) => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                for param in query.split('&') {
                    if matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), &task_state.to_view())? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RemoveQueryParamsInSetOrStartingWithAnyInList {set, list} => if let Some(query) = task_state.url.query() {
                let mut new = String::with_capacity(query.len());
                let set = task_state.params.sets.get(set).ok_or(ActionError::SetNotFound)?;
                let list = task_state.params.lists.get(list).ok_or(ActionError::ListNotFound)?;
                for param in query.split('&') {
                    let name = pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy();
                    if !(set.contains(Some(&*name)) || list.iter().any(|x| name.starts_with(x))) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != query.len() {
                    task_state.url.set_query(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RenameQueryParam {from, to} => task_state.url.rename_query_param(&from.name, from.index, get_new_str!(to, task_state, ActionError))?,

            Self::GetUrlFromQueryParam(name) => match task_state.url.query_param(get_str!(name, task_state, ActionError), 0) {
                Some(Some(Some(new_url))) => {*task_state.url = BetterUrl::parse(&new_url)?;},
                Some(Some(None))          => Err(ActionError::QueryParamNoValue)?,
                Some(None)                => Err(ActionError::QueryParamNotFound)?,
                None                      => Err(ActionError::NoQuery)?
            },

            // Fragment

            Self::RemoveFragment => task_state.url.set_fragment(None),
            Self::RemoveEmptyFragment => if task_state.url.fragment() == Some("") {task_state.url.set_fragment(None)},

            // General parts

            Self::SetPart {part, value} => part.set(task_state.url, get_new_option_str!(value, task_state))?,

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
            },

            Self::CopyPart {from, to} => to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?,
            Self::MovePart {from, to} => {
                to.set(task_state.url, from.get(task_state.url).map(|x| x.into_owned()).as_deref())?;
                from.set(task_state.url, None)?;
            },

            // Misc.

            #[cfg(feature = "http")]
            Self::ExpandRedirect {headers, http_client_config_diff} => {
                let _unthread_handle = task_state.unthreader.unthread();
                #[cfg(feature = "cache")]
                if task_state.params.read_cache {
                    if let Some(entry) = task_state.cache.read(CacheEntryKeys {subject: "redirect", key: task_state.url.as_str()})? {
                        *task_state.url = BetterUrl::parse(&entry.value.ok_or(ActionError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                #[cfg(feature = "cache")]
                let start = std::time::Instant::now();
                let response = task_state.to_view().http_client(http_client_config_diff.as_deref())?.get(task_state.url.as_str()).headers(headers.clone()).send()?;
                let new_url = if response.status().is_redirection() {
                    BetterUrl::parse(std::str::from_utf8(response.headers().get("location").ok_or(ActionError::LocationHeaderNotFound)?.as_bytes())?)?
                } else {
                    response.url().clone().into()
                };
                #[cfg(feature = "cache")]
                let duration = start.elapsed();
                #[cfg(feature = "cache")]
                if task_state.params.write_cache {
                    task_state.cache.write(NewCacheEntry {
                        subject: "redirect",
                        key: task_state.url.as_str(),
                        value: Some(new_url.as_str()),
                        duration
                    })?;
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
            Self::SetScratchpadVar {name, value} => match get_option_string!(value, task_state) {
                Some(value) => {let _ = task_state.scratchpad.vars.insert( get_string!(name, task_state, ActionError), value);}
                None        => {let _ = task_state.scratchpad.vars.remove(&get_string!(name, task_state, ActionError));}
            },
            Self::ModifyScratchpadVar {name, modification} => {
                let name = get_string!(name, task_state, ActionError);
                let mut value = task_state.scratchpad.vars.get(&name).map(|x| Cow::Borrowed(&**x));
                modification.apply(&mut value, &task_state.to_view())?;
                match value {
                    Some(value) => {let _ = task_state.scratchpad.vars.insert( name, value.into_owned());},
                    None        => {let _ = task_state.scratchpad.vars.remove(&name);}
                }
            },
            #[cfg(feature = "cache")]
            Self::CacheUrl {subject, action} => {
                let _unthread_handle = task_state.unthreader.unthread();
                let subject = get_string!(subject, task_state, ActionError);
                if task_state.params.read_cache {
                    if let Some(entry) = task_state.cache.read(CacheEntryKeys {subject: &subject, key: task_state.url.as_str()})? {
                        *task_state.url = BetterUrl::parse(&entry.value.ok_or(ActionError::CachedUrlIsNone)?)?;
                        return Ok(());
                    }
                }
                let old_url = task_state.url.to_string();
                let start = std::time::Instant::now();
                action.apply(task_state)?;
                let duration = start.elapsed();
                if task_state.params.write_cache {
                    task_state.cache.write(NewCacheEntry {
                        subject: &subject,
                        key: &old_url,
                        value: Some(task_state.url.as_str()),
                        duration
                    })?;
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
                    cache      : task_state.cache,
                    unthreader : task_state.unthreader
                })?
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(ActionError::NotInCommonContext)?.actions.get(get_str!(name, task_state, ActionError)).ok_or(ActionError::CommonCallArgActionNotFound)?.apply(task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        };
        Ok(())
    }
}

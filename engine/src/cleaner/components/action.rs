//! [`Action`].

#![allow(unused_assignments, reason = "False positive.")]

use std::str::{FromStr, Utf8Error};
use std::collections::HashSet;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in doc comment.")]
use url::{Url, PathSegmentsMut};

use crate::prelude::*;

/// Actions are how [`TaskState`]s get manipulated to clean URLs.
///
/// Please note that, in general, when a [`Action`] returns an [`Err`], the [`TaskState`] may still be modified. For example:
/// ```
/// use url_cleaner_engine::docs::*;
///
/// doc_test!(task_state, ts, task = "https://example.com");
///
/// doc_test!(apply, Err, Action::All(vec![
///     Action::SetPath("/change".into()),
///     Action::Error("This won't revert the above".into()),
///     Action::SetPath("/this-wont-happen".into())
/// ]), &mut ts);
///
/// assert_eq!(ts.url, "https://example.com/change");
/// ```
///
/// This is because reverting on an error requires keeping a copy of the input state, which is very expensive and, if the error is just going to be returned as the result of the [`Task`], not useful.
///
/// If you need to revert the [`TaskState`] when an error is returned, use [`Self::RevertOnError`] to revert the effects but still return the error, and optionally [`Self::IgnoreError`] to ignore the error.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub enum Action {
    /// Does nothing.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(apply, Ok, Action::None, &mut ts);
    ///
    /// assert_eq!(ts.url, "https://example.com/");
    /// ```
    #[default]
    None,
    /// Always returns the error [`ActionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ActionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(apply, Err, Action::Error("...".into()), &mut ts);
    ///
    /// assert_eq!(ts.url, "https://example.com/");
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(apply, Ok , Action::If {r#if: Condition::Always, then: Box::new(Action::None),r#else: Some(Box::new(Action::Error("...".into())))}, &mut ts);
    /// doc_test!(apply, Err, Action::If {r#if: Condition::Never , then: Box::new(Action::None),r#else: Some(Box::new(Action::Error("...".into())))}, &mut ts);
    /// doc_test!(apply, Ok , Action::If {r#if: Condition::Always, then: Box::new(Action::None),r#else: None                                       }, &mut ts);
    /// doc_test!(apply, Ok , Action::If {r#if: Condition::Never , then: Box::new(Action::None),r#else: None                                       }, &mut ts);
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(apply, Err, Action::All(vec![
    ///     Action::SetHost("example2.com".into()),
    ///     Action::Error("...".into()),
    ///     Action::SetHost("example3.com".into()),
    /// ]), &mut ts);
    ///
    /// assert_eq!(ts.url, "https://example2.com/");
    /// ```
    All(Vec<Self>),
    /// Gets the value specified by [`Self::PartMap::part`], indexes [`Self::PartMap::map`], and applies the returned [`Self`]
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing..
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(apply, Err, Action::PartMap {
    ///     part: UrlPart::Host,
    ///     map: Box::new(Map {
    ///         map: [
    ///             ("example.com".into(), Action::Error("...".into()))
    ///         ].into(),
    ///         if_none: None,
    ///         r#else: None
    ///     })
    /// }, &mut ts);
    /// ```
    PartMap {
        /// The [`UrlPart`] to index [`Self::PartMap::map`] with.
        part: UrlPart,
        /// The [`Map`] to index with [`Self::PartMap::part`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the string specified by [`Self::StringMap::value`], indexes [`Self::StringMap::map`], and applies the returned [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(apply, Err, Action::StringMap {
    ///     value: StringSource::String("a".into()),
    ///     map: Box::new(Map {
    ///         map: [
    ///             ("a".into(), Action::Error("...".into()))
    ///         ].into(),
    ///         if_none: None,
    ///         r#else: None
    ///     })
    /// }, &mut ts);
    /// ```
    StringMap {
        /// The [`StringSource`] to index [`Self::StringMap::map`] with.
        value: StringSource,
        /// The [`Map`] to index with [`Self::StringMap::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the name of the partition [`Self::PartPartitioning::part`] is in in the specified [`Partitioning`], indexes [`Self::PartPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Action, 2), notfound(Partitioning, Action), applyerr(Self))]
    PartPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`UrlPart`] whose value to find in the [`Partitioning`].
        part: UrlPart,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// [`Self::PartPartitioning`] but uses each [`UrlPart`] in [`Self::FirstMatchingPartPartitioning`] until a match is found.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Action, 2), notfound(Partitioning, Action), applyerr(Self))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://abc.example.com", params = Params {
    ///     partitionings: Cow::Owned([
    ///         (
    ///             "a".into(),
    ///             Partitioning::try_from_iter([
    ///                 ("b".into(), vec![Some("example.com".into())])
    ///             ]).unwrap(),
    ///         )
    ///     ].into()),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(apply, Ok, Action::FirstMatchingPartPartitioning {
    ///     partitioning: "a".into(),
    ///     parts: vec![UrlPart::Host, UrlPart::DomainOrigin],
    ///     map: Box::new([
    ///         ("b".to_string(), Action::SetPath("/123".into()))
    ///     ].into()),
    /// }, &mut ts);
    ///
    /// assert_eq!(ts.url.path(), "/123");
    /// ```
    FirstMatchingPartPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`UrlPart`]s whose value to find in the [`Partitioning`].
        parts: Vec<UrlPart>,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the name of the partition [`Self::StringPartitioning::value`] is in in the specified [`Partitioning`], indexes [`Self::StringPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), notfound(Partitioning, Action), applyerr(Self))]
    StringPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`Partitioning`].
        value: StringSource,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// [`Self::StringPartitioning`] but uses each [`StringSource`] in [`Self::FirstMatchingStringPartitioning`] until a match is found.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action), notfound(Partitioning, Action), applyerr(Self))]
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://abc.example.com", params = Params {
    ///     partitionings: Cow::Owned([
    ///         (
    ///             "a".into(),
    ///             Partitioning::try_from_iter([
    ///                 ("b".into(), vec![Some("example.com".into())])
    ///             ]).unwrap(),
    ///         )
    ///     ].into()),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(apply, Ok, Action::FirstMatchingStringPartitioning {
    ///     partitioning: "a".into(),
    ///     values: vec![StringSource::Part(UrlPart::Host), StringSource::Part(UrlPart::DomainOrigin)],
    ///     map: Box::new([
    ///         ("b".to_string(), Action::SetPath("/123".into()))
    ///     ].into()),
    /// }, &mut ts);
    ///
    /// assert_eq!(ts.url.path(), "/123");
    /// ```
    FirstMatchingStringPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`Partitioning`].
        values: Vec<StringSource>,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },



    /// Repeat [`Self::Repeat::actions`] until [`TaskState::url`] ends up in the same state or the rules were executed [`Self::Repeat::limit`] times.
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// // Error is ignored.
    /// doc_test!(apply, Ok, Action::IgnoreError(Box::new(
    ///     Action::RevertOnError(Box::new(
    ///         Action::All(vec![
    ///             Action::SetPath("/change".into()),
    ///             Action::Error("This won't revert the above".into()),
    ///             Action::SetPath("/wont-happen".into())
    ///         ])
    ///     ))
    /// )), &mut ts);
    ///
    /// // The first `Action::SetPath` is reverted.
    /// assert_eq!(ts.url, "https://example.com/");
    /// ```
    IgnoreError(Box<Self>),
    /// If the contained [`Self`] returns an error, revert the [`TaskState`] to its previous state then return the error.
    ///
    /// To ignore errors, put this in a [`Self::IgnoreError`].
    /// # Errors
    #[doc = edoc!(applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// // Still returns an error.
    /// doc_test!(apply, Err, Action::RevertOnError(Box::new(
    ///     Action::All(vec![
    ///         Action::SetPath("/change".into()),
    ///         Action::Error("This won't revert the above".into()),
    ///         Action::SetPath("/wont-happen".into())
    ///     ])
    /// )), &mut ts);
    ///
    /// // The first `Action::SetPath` is reverted.
    /// assert_eq!(ts.url, "https://example.com/");
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/a/b/c");
    /// doc_test!(apply, Ok, Action::Join("..".into()), &mut ts);
    /// assert_eq!(ts.url, "https://example.com/a/");
    ///
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/a/b/c/");
    /// doc_test!(apply, Ok, Action::Join("..".into()), &mut ts);
    /// assert_eq!(ts.url, "https://example.com/a/b/");
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
    /// [`BetterUrl::set_domain_prefix`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_prefix))]
    SetDomainPrefix(StringSource),
    /// [`BetterUrl::set_domain_origin`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_origin))]
    SetDomainOrigin(StringSource),
    /// [`BetterUrl::set_domain_middle`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_middle))]
    SetDomainMiddle(StringSource),
    /// [`BetterUrl::set_domain_suffix`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::set_domain_suffix))]
    SetDomainSuffix(StringSource),

    /// [`BetterUrl::set_domain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_segment))]
    SetDomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_origin_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_origin_segment))]
    SetDomainOriginSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_prefix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_prefix_segment))]
    SetDomainPrefixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::set_domain_suffix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_suffix_segment))]
    SetDomainSuffixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },



    /// [`StringModification::apply`] and [`BetterUrl::set_host`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_host))]
    ModifyHost(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_prefix`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_prefix))]
    ModifyDomainPrefix(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_origin`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_origin))]
    ModifyDomainOrigin(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_middle`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_middle))]
    ModifyDomainMiddle(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_suffix`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_suffix))]
    ModifyDomainSuffix(StringModification),

    /// [`StringModification::apply`] and [`BetterUrl::set_domain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_segment))]
    ModifyDomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_origin_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_origin_segment))]
    ModifyDomainOriginSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_prefix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_prefix_segment))]
    ModifyDomainPrefixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_suffix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_domain_suffix_segment))]
    ModifyDomainSuffixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },



    /// [`BetterUrl::insert_domain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_segment))]
    InsertDomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_origin_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_origin_segment))]
    InsertDomainOriginSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_prefix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_prefix_segment))]
    InsertDomainPrefixSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::insert_domain_suffix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_domain_suffix_segment))]
    InsertDomainSuffixSegment {
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
    /// Apply [`Self::ModifyPath::0`] to the path.
    /// # Errors
    #[doc = edoc!(applyerr(StringModification))]
    ///
    /// If the resulting path would be [`None`], returns the error [`ActionError::PathCantBeNone`].
    ModifyPath(StringModification),
    /// Removes the specified path segment.
    /// # Errors
    #[doc = edoc!(callerr(SegmentedPath::set))]
    RemovePathSegment(isize),
    /// Set the specified path segment.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_path_segment))]
    SetPathSegment {
        /// The [`UrlPart::PathSegment`] to set.
        index: isize,
        /// The value to set it to.
        value: StringSource
    },
    /// Apply [`Self::ModifyPathSegment::modification`] to the specified path segment.
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::segmented_path, PathIsOpaque), applyerr(StringModification), callerr(BetterUrl::set_path_segment))]
    ModifyPathSegment {
        /// The path segment to modify.
        index: isize,
        /// The [`StringModification`] to apply.
        modification: StringModification
    },    /// Insert the specified path segment.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment))]
    InsertPathSegment {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`BetterUrl::pop_path_if_empty`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::pop_path_if_empty))]
    PopPathIfEmpty,



    /// [`BetterUrl::set_query`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_query))]
    SetQuery(StringSource),
    /// Set the specified query parameter.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(MaybeQuery::set))]
    SetQueryParam {
        /// The query param to set.
        param: QueryParamSelector,
        /// The value to set it to.
        value: StringSource
    },
    /// Remove the query.
    RemoveQuery,
    /// If the query is empty, remove it.
    RemoveEmptyQuery,
    /// [`BetterUrl::filter_query`], removing segments whose name is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    RemoveQueryParam(StringSource),
    /// [`BetterUrl::filter_query`], keeping segments whose name is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    AllowQueryParam(StringSource),
    /// [`BetterUrl::filter_query`], removing segments whose name is in the specified set.
    RemoveQueryParams(HashSet<String>),
    /// [`BetterUrl::filter_query`], keeping segments whose name is in the specified set.
    AllowQueryParams(HashSet<String>),
    /// [`MaybeQuery::filter`], removing segments whose name satisfies the [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    RemoveQueryParamsMatching(StringMatcher),
    /// [`MaybeQuery::filter`], keeping segments whose name satisfies the [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    AllowQueryParamsMatching(StringMatcher),

    /// Sets [`UrlPart::Whole`] to the value of the first query parameter with a name determined by the [`TaskState`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    ///
    /// If the specified query param isn't found, returns the error [`ActionError::QueryParamNotFound`].
    ///
    /// If the specified query param doesn't have a value, returns the error [`ActionError::QueryParamNoValue`].
    ///
    /// If no matching query parameter is found, returns the error [`ActionError::QueryParamNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?redirect=https://example.com/2");
    ///
    /// doc_test!(apply, Ok, Action::GetUrlFromQueryParam("redirect".into()), &mut ts);
    /// assert_eq!(ts.url, "https://example.com/2");
    ///
    /// doc_test!(apply, Err, Action::GetUrlFromQueryParam("redirect".into()), &mut ts);
    /// ```
    GetUrlFromQueryParam(StringSource),

    // Fragment

    /// [`BetterUrl::set_fragment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_fragment))]
    SetFragment(StringSource),
    /// Set the specified fragment parameter.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(Query::set))]
    SetFragmentParam {
        /// The fragment param to set.
        param: QueryParamSelector,
        /// The value to set it to.
        value: StringSource
    },
    /// Removes the [`UrlPart::Fragment`].
    RemoveFragment,
    /// If the [`Url::fragment`] is `Some("")`, set it to [`None`].
    RemoveEmptyFragment,
    /// [`BetterUrl::filter_fragment_query`], removing segments whose name is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    RemoveFragmentParam(StringSource),
    /// [`BetterUrl::filter_fragment_query`], keeping segments whose name is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    AllowFragmentParam(StringSource),
    /// [`BetterUrl::filter_fragment_query`], removing segments whose name is in the specified set.
    RemoveFragmentParams(HashSet<String>),
    /// [`BetterUrl::filter_fragment_query`], keeping segments whose name is in the specified set.
    AllowFragmentParams(HashSet<String>),
    /// [`MaybeQuery::filter`], removing segments whose name satisfies the [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    RemoveFragmentParamsMatching(StringMatcher),
    /// [`MaybeQuery::filter`], keeping segments whose name satisfies the [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    AllowFragmentParamsMatching(StringMatcher),

    // General parts

    /// Sets the specified [`UrlPart`] to the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), seterr(UrlPart))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(apply, Ok, Action::SetPart {part: UrlPart::Path, value: "abc".into()}, &mut ts);
    /// assert_eq!(ts.url, "https://example.com/abc");
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(apply, Ok, Action::ModifyPart {part: UrlPart::Path, modification: StringModification::Set("abc".into())}, &mut ts);
    /// assert_eq!(ts.url, "https://example.com/abc");
    ///
    /// doc_test!(apply, Ok, Action::ModifyPart {part: UrlPart::Query, modification: StringModification::Set("abc".into())}, &mut ts);
    /// assert_eq!(ts.url, "https://example.com/abc?abc");
    /// ```
    ModifyPart {
        /// The part to modify.
        part: UrlPart,
        /// The modification to apply to the part.
        modification: StringModification
    },
    /// Sets [`Self::CopyPart::to`] to the value of [`Self::CopyPart::from`], leaving [`Self::CopyPart::from`] unchanged.
    /// # Errors
    #[doc = edoc!(seterr(UrlPart))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc#def");
    ///
    /// Action::CopyPart {from: UrlPart::Fragment, to: UrlPart::Path}.apply(&mut ts).unwrap();
    /// assert_eq!(ts.url, "https://example.com/def#def");
    /// ```
    CopyPart {
        /// The part whose value to copy.
        from: UrlPart,
        /// The part whose value to set.
        to: UrlPart
    },

    // Misc.

    /// Select query and/or fragment parameters to remove/keep by name and prefix.
    /// # Errors
    #[doc = edoc!(geterr(SetSource, 2), geterr(ListSource, 2))]
    HandleParams {
        /// The mode to use.
        ///
        /// Defaults to [`HandleParamsMode::Remove`].
        #[serde(default, skip_serializing_if = "is_default")]
        mode: HandleParamsMode,
        /// If [`true`], handle query parameters.
        ///
        /// Defaults to [`true`].
        #[serde(default = "get_true", skip_serializing_if = "is_true")]
        query: bool,
        /// If [`true`], handle fragment parameters.
        ///
        /// Defaults to [`false`].
        #[serde(default, skip_serializing_if = "is_default")]
        fragment: bool,
        /// The names of segments to match.
        ///
        /// Defaults to [`SetSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        names: SetSource,
        /// The prefixes of segments to match.
        ///
        /// Defaults to [`ListSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        prefixes: ListSource,
        /// The names of segments to not match.
        ///
        /// Defaults to [`SetSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        except_names: SetSource,
        /// The prefixes of segments to not match.
        ///
        /// Defaults to [`ListSource::None`].
        #[serde(default, skip_serializing_if = "is_default")]
        except_prefixes: ListSource
    },

    /// If an entry with a subject of [`Self::Cache::subject`] and a key of [`TaskState::url`] exists in the [`Job::cache`], sets the URL to the entry's value.
    ///
    /// If no such entry exists, applies [`Self::Cache::action`] and inserts a new entry equivalent to applying it.
    /// # Errors
    #[doc = edoc!(callerr(Cache::read), callnone(Cache::read, ActionError::CachedUrlIsNone), callerr(BetterUrl::parse), applyerr(Self), callerr(Cache::write))]
    #[cfg(feature = "cache")]
    Cache {
        /// The subject for the cache entry.
        subject: StringSource,
        /// The action to apply and cache.
        action: Box<Self>
    },



    /// Uses a [`Self`] from [`Cleaner::functions`].
    /// # Errors
    #[doc = edoc!(functionnotfound(Self, Action), applyerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, functions = Functions {
    ///     actions: [("abc".into(), Action::None)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(apply, Ok, Action::Function(Box::new(FunctionCall {name: "abc".into(), args: Default::default()})), &mut ts);
    /// ```
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    /// # Errors
    #[doc = edoc!(notinfunction(Action), callargfunctionnotfound(Self, Action), applyerr(Self))]
    CallArg(StringSource),
    /// Calls the specified function and returns its value.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    ///
    /// Additionally, using a function pointer means this variant cannot be [`Serialize`]d or [`Deserialize`]d.
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// fn some_complex_operation(task_state: &mut TaskState) -> Result<(), ActionError> {
    ///     Ok(())
    /// }
    ///
    /// doc_test!(apply, Ok, Action::Custom(some_complex_operation), &mut ts);
    /// ```
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut TaskState) -> Result<(), ActionError>)
}

string_or_struct_magic!(Action);

/// Decides if [`HandleParamsMode`] should remove matching parameters or keep only matching parameters.
///
/// Defaults to [`Self::Remove`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum HandleParamsMode {
    /// Remove matching parameters.
    ///
    /// The default.
    #[default]
    Remove,
    /// Keep only matching parameters.
    Keep
}

/// The error returned when trying to deserialize a [`StringModification`] variant with fields that aren't all defaultable.
#[derive(Debug, Error)]
#[error("Tried deserializing undefaultable or Action unknown variant {0}.")]
pub struct NonDefaultableActionVariant(String);

impl From<&str> for NonDefaultableActionVariant {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for NonDefaultableActionVariant {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromStr for Action {
    type Err = NonDefaultableActionVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "None"                => Action::None,
            "EnsureFqdnPeriod"    => Action::EnsureFqdnPeriod,
            "RemoveFqdnPeriod"    => Action::RemoveFqdnPeriod,
            "PopPathIfEmpty"      => Action::PopPathIfEmpty,
            "RemoveQuery"         => Action::RemoveQuery,
            "RemoveEmptyQuery"    => Action::RemoveEmptyQuery,
            "RemoveFragment"      => Action::RemoveFragment,
            "RemoveEmptyFragment" => Action::RemoveEmptyFragment,
            _                     => return Err(s.into())
        })
    }
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
    /// [`StringSourceIsNone`].
    #[error(transparent)]
    StringSourceIsNone(#[from] StringSourceIsNone),
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
    SchemeCantBeNone,

    /// Returned when a [`SetHostError`] is encountered.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// Returned when a [`SetDomainError`] is encountered.
    #[error(transparent)]
    SetDomainError(#[from] SetDomainError),

    /// Returned when a [`SetPathError`] is encountered.
    #[error(transparent)]
    SetPathError(#[from] SetPathError),
    /// Returned when attempting to set a URL's path to [`None`].
    #[error("Attempted to set the URL's path to None.")]
    PathCantBeNone,

    /// Returned when attempting to get the value of a query param that wasn't found.
    #[error("Attempted to get the value of a query param that wasn't found.")]
    QueryParamNotFound,
    /// Returned when attempting to get the value of a query param that didn't have a value.
    #[error("Attempted to get the value of a query param that didn't have a value.")]
    QueryParamNoValue,
    /// Returned when a [`SetQueryError`] is encountered.
    #[error(transparent)]
    SetQueryError(#[from] SetQueryError),

    /// Returned when a [`SetFragmentError`] is encountered.
    #[error(transparent)]
    SetFragmentError(#[from] SetFragmentError),

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

    /// Returned when a [`Partitioning`] with the specified name isn't found.
    #[error("A Partitioning with the specified name wasn't found.")]
    PartitioningNotFound,
    /// Returned when a [`ListSourceError`] is encountered.
    #[error(transparent)]
    ListSourceError(#[from] ListSourceError),
    /// Returned when a list with the specified name isn't found.
    #[error("A list with the specified name wasn't found.")]
    ListNotFound,
    /// Returned when a [`SetSourceError`] is encountered.
    #[error(transparent)]
    SetSourceError(#[from] SetSourceError),
    /// Returned when a [`Set`] with the specified name isn't found.
    #[error("A Set with the specified name wasn't found.")]
    SetNotFound,

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

    /// Returned when a [`Action`] with the specified name isn't found in the [`Functions::actions`].
    #[error("An Action with the specified name wasn't found in the Functions::actions.")]
    FunctionNotFound,
    /// Returned when attempting to use [`CallArgs`] outside a function.
    #[error("Attempted to use CallArgs outside a function.")]
    NotInFunction,
    /// Returned when a [`CallArgs`] function ins't found.
    #[error("A CallArgs function wasn't found.")]
    CallArgFunctionNotFound,
    /// An arbitrary [`std::error::Error`] returned by [`Action::Custom`].
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

/// Generate the "modify {part}" [`Action`]s.
macro_rules! modify_part {
    ($ts:expr, $mod:expr, $get:ident, $set:ident$(, $arg:expr)*) => {{
        let mut x = $ts.url.$get($($arg),*).map(Cow::Borrowed);
        $mod.apply(&mut x, $ts)?;
        $ts.url.$set($($arg,)* x.map(Cow::into_owned).as_deref())?;
    }};
}

impl Action {
    /// Applies the specified variant of [`Self`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn apply<'j>(&'j self, task_state: &mut TaskState<'j>) -> Result<(), ActionError> {
        debug!(Action::apply, self, task_state.url; Ok(match self {
            // Debug/constants

            Self::None => {},
            Self::Error(msg) => Err(ActionError::ExplicitError(msg.clone()))?,
            Self::Debug(action) => {
                let old_url = format!("{:?}", task_state.url);
                let return_value=action.apply(task_state);
                eprintln!("=== Action::Debug ===\nOld url: {old_url}\nReturn value: {return_value:?}\nNew url: {:?}", task_state.url);
                return_value?
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
                if let Err(e) = action.apply(task_state) {
                    task_state.url = old_url;
                    Err(e)?;
                }
            },

            // Logic

            Self::If {r#if, then, r#else} => if r#if.check(task_state)? {
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
                for _ in 0..*limit {
                    let previous_url = task_state.url.clone();
                    for action in actions {
                        action.apply(task_state)?;
                    }
                    if task_state.url == previous_url {break;}
                }
            },

            // Maps

            Self::PartMap   {part , map} => if let Some(action) = map.get(part .get(&task_state.url) ) {action.apply(task_state)?;},
            Self::StringMap {value, map} => if let Some(action) = map.get(value.get( task_state    )?) {action.apply(task_state)?;},

            Self::PartPartitioning   {partitioning, part , map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ActionError::PartitioningNotFound)?;
                if let Some(action) = map.get(partitioning.get(part.get(&task_state.url).as_deref())) {
                    action.apply(task_state)?;
                }
            },
            Self::StringPartitioning {partitioning, value, map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ActionError::PartitioningNotFound)?;
                if let Some(action) = map.get(partitioning.get(get_option_str!(value))) {
                    action.apply(task_state)?;
                }
            },

            Self::FirstMatchingPartPartitioning {partitioning, parts, map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ActionError::PartitioningNotFound)?;
                for part in parts.iter() {
                    if let Some(action) = map.get(partitioning.get(part.get(&task_state.url).as_deref())) {
                        return action.apply(task_state);
                    }
                }
            },
            Self::FirstMatchingStringPartitioning {partitioning, values, map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ActionError::PartitioningNotFound)?;
                for value in values.iter() {
                    if let Some(action) = map.get(partitioning.get(get_option_str!(value))) {
                        return action.apply(task_state);
                    }
                }
            },

            // Whole

            Self::SetWhole(new) => task_state.url = BetterUrl::parse(get_str!(new))?,
            Self::Join(with) => task_state.url=task_state.url.join(get_str!(with))?.into(),

            // Scheme

            Self::SetScheme(to) => task_state.url.set_scheme(get_new_str!(to))?,

            // Domain

            Self::SetHost        (value) => task_state.url.set_host         (get_new_option_str!(value))?,
            Self::SetDomainPrefix(value) => task_state.url.set_domain_prefix(get_new_option_str!(value))?,
            Self::SetDomainMiddle(value) => task_state.url.set_domain_middle(get_new_option_str!(value))?,
            Self::SetDomainSuffix(value) => task_state.url.set_domain_suffix(get_new_option_str!(value))?,
            Self::SetDomainOrigin(value) => task_state.url.set_domain_origin(get_new_option_str!(value))?,

            Self::SetDomainSegment      {index, value} => task_state.url.set_domain_segment       (*index, get_new_option_str!(value))?,
            Self::SetDomainOriginSegment{index, value} => task_state.url.set_domain_origin_segment(*index, get_new_option_str!(value))?,
            Self::SetDomainPrefixSegment{index, value} => task_state.url.set_domain_prefix_segment(*index, get_new_option_str!(value))?,
            Self::SetDomainSuffixSegment{index, value} => task_state.url.set_domain_suffix_segment(*index, get_new_option_str!(value))?,

            Self::ModifyHost        (modification) => modify_part!(task_state, modification, host_str     , set_host         ),
            Self::ModifyDomainPrefix(modification) => modify_part!(task_state, modification, domain_prefix, set_domain_prefix),
            Self::ModifyDomainMiddle(modification) => modify_part!(task_state, modification, domain_middle, set_domain_middle),
            Self::ModifyDomainSuffix(modification) => modify_part!(task_state, modification, domain_suffix, set_domain_suffix),
            Self::ModifyDomainOrigin(modification) => modify_part!(task_state, modification, domain_origin, set_domain_origin),

            Self::ModifyDomainSegment      {index, modification} => modify_part!(task_state, modification, domain_segment       , set_domain_segment       , *index),
            Self::ModifyDomainOriginSegment{index, modification} => modify_part!(task_state, modification, domain_origin_segment, set_domain_origin_segment, *index),
            Self::ModifyDomainPrefixSegment{index, modification} => modify_part!(task_state, modification, domain_prefix_segment, set_domain_prefix_segment, *index),
            Self::ModifyDomainSuffixSegment{index, modification} => modify_part!(task_state, modification, domain_suffix_segment, set_domain_suffix_segment, *index),

            Self::InsertDomainSegment       {index, value} => task_state.url.insert_domain_segment       (*index, get_new_str!(value))?,
            Self::InsertDomainOriginSegment {index, value} => task_state.url.insert_domain_origin_segment(*index, get_new_str!(value))?,
            Self::InsertDomainPrefixSegment {index, value} => task_state.url.insert_domain_prefix_segment(*index, get_new_str!(value))?,
            Self::InsertDomainSuffixSegment {index, value} => task_state.url.insert_domain_suffix_segment(*index, get_new_str!(value))?,

            Self::EnsureFqdnPeriod => task_state.url.set_fqdn(true)?,
            Self::RemoveFqdnPeriod => task_state.url.set_fqdn(false)?,

            // Path

            Self::SetPath(to) => task_state.url.set_path(get_new_str!(to))?,
            Self::ModifyPath(modification) => {
                let mut path = Some(task_state.url.path().into_inner());
                modification.apply(&mut path, task_state)?;
                task_state.url.set_path(path.ok_or(ActionError::PathCantBeNone)?.into_owned())?;
            },

            Self::RemovePathSegment(index) => task_state.url.set_path_segment(*index, None::<&str>)?,
            Self::PopPathIfEmpty           => task_state.url.pop_path_if_empty()?,

            Self::SetPathSegment {index, value} => task_state.url.set_path_segment(*index, get_new_option_str!(value))?,
            Self::ModifyPathSegment {index, modification} => {
                let mut value = task_state.url.path_segment(*index).map(PathSegment::into_inner);
                modification.apply(&mut value, task_state)?;
                task_state.url.set_path_segment(*index, value.map(Cow::into_owned).as_deref())?;
            },
            Self::InsertPathSegment {index, value} => task_state.url.insert_path_segment(*index, get_new_str!(value))?,

            // Query

            Self::SetQuery                 (to)           => {task_state.url.set_query(get_new_option_str!(to))?;},
            Self::SetQueryParam            {param, value} => {task_state.url.set_query_param(&param.name, param.index, get_new_option_str!(value).map(Some))?;},
            Self::RemoveQuery                             => {task_state.url.remove_query();},
            Self::RemoveEmptyQuery                        => {task_state.url.remove_empty_query();},
            Self::RemoveQueryParam         (name   )      => {let name = get_new_str!(name); task_state.url.filter_query(|s| s.name() != name);},
            Self::AllowQueryParam          (name   )      => {let name = get_new_str!(name); task_state.url.filter_query(|s| s.name() == name);},
            Self::RemoveQueryParams        (names  )      => {task_state.url.filter_query(|s| !names.contains(&*s.name()));},
            Self::AllowQueryParams         (names  )      => {task_state.url.filter_query(|s|  names.contains(&*s.name()));},
            Self::AllowQueryParamsMatching (matcher)      => {
                if let (true, query) = task_state.url.query().try_filtered(|s| matcher.check(Some(&s.name()), task_state))? {
                    task_state.url.set_query(query.into_owned())?;
                }
            },
            Self::RemoveQueryParamsMatching (matcher) => {
                if let (true, query) = task_state.url.query().try_filtered(|s| matcher.check(Some(&s.name()), task_state).map(|x| !x))? {
                    task_state.url.set_query(query.into_owned())?;
                }
            },

            Self::GetUrlFromQueryParam(name) => {
                task_state.url = BetterUrl::parse(&task_state.url.query_param(get_str!(name), 0).ok_or(ActionError::QueryParamNotFound)?.into_value().ok_or(ActionError::QueryParamNoValue)?)?;
            },

            // Fragment

            Self::SetFragment                 (to)           => {task_state.url.set_fragment(get_new_option_str!(to))?;},
            Self::SetFragmentParam            {param, value} => {task_state.url.set_fragment_query_param(&param.name, param.index, get_new_option_str!(value).map(Some))?;},
            Self::RemoveFragment                             => {task_state.url.remove_fragment();},
            Self::RemoveEmptyFragment                        => {task_state.url.remove_empty_fragment();},
            Self::RemoveFragmentParam         (name   )      => {let name = get_new_str!(name); task_state.url.filter_fragment_query(|s| s.name() != name);},
            Self::AllowFragmentParam          (name   )      => {let name = get_new_str!(name); task_state.url.filter_fragment_query(|s| s.name() == name);},
            Self::RemoveFragmentParams        (names  )      => {task_state.url.filter_fragment_query(|s| !names.contains(&*s.name()));},
            Self::AllowFragmentParams         (names  )      => {task_state.url.filter_fragment_query(|s|  names.contains(&*s.name()));},
            Self::AllowFragmentParamsMatching (matcher)      => {
                if let (true, fragment) = task_state.url.fragment_query().try_filtered(|s| matcher.check(Some(&s.name()), task_state))? {
                    task_state.url.set_fragment(fragment.into_owned())?;
                }
            },
            Self::RemoveFragmentParamsMatching (matcher) => {
                if let (true, fragment) = task_state.url.fragment_query().try_filtered(|s| matcher.check(Some(&s.name()), task_state).map(|x| !x))? {
                    task_state.url.set_fragment(fragment.into_owned())?;
                }
            },

            // General parts

            Self::SetPart {part, value} => {
                let temp = get_new_option_cow!(value);
                part.set(&mut task_state.url, temp.as_deref())?;
            },

            Self::ModifyPart {part, modification} => {
                let mut temp = part.get(&task_state.url);
                modification.apply(&mut temp, task_state)?;
                let temp = temp.map(Cow::into_owned);
                part.set(&mut task_state.url, temp.as_deref())?;
            },

            Self::CopyPart {from, to} => {
                let temp = from.get(&task_state.url).map(Cow::into_owned);
                to.set(&mut task_state.url, temp.as_deref())?;
            },

            // Misc.

            Self::HandleParams {mode, query, fragment, names, prefixes, except_names, except_prefixes} => {
                let ds = Default::default();
                let dl = Default::default();

                let names           = names          .get(task_state)?.unwrap_or(&ds);
                let prefixes        = prefixes       .get(task_state)?.unwrap_or(&dl);
                let except_names    = except_names   .get(task_state)?.unwrap_or(&ds);
                let except_prefixes = except_prefixes.get(task_state)?.unwrap_or(&dl);

                let excepts = !except_names.is_empty() || !except_prefixes.is_empty();

                let filter = |segment: &QuerySegment<'_>| -> bool {
                    let name = segment.name();

                    let matches = (names.contains_some(&*name) || prefixes.iter().any(|prefix| name.starts_with(prefix)))
                        && !(excepts && (except_names.contains_some(&*name) || except_prefixes.iter().any(|prefix| name.starts_with(prefix))));

                    matches!((mode, matches), (HandleParamsMode::Keep, true) | (HandleParamsMode::Remove, false))
                };

                if *fragment {
                    task_state.url.filter_fragment_query(filter);
                }

                if *query {
                    task_state.url.filter_query(filter);
                }
            },



            #[cfg(feature = "cache")]
            Self::Cache {subject, action} => {
                let _unthread_handle = task_state.job.unthreader.unthread();
                let subject = get_new_str!(subject);
                if let Some(entry) = task_state.job.cache.read(CacheEntryKeys {subject, key: task_state.url.as_str()})? {
                    task_state.url = BetterUrl::parse(&entry.value.ok_or(ActionError::CachedUrlIsNone)?)?;
                    return Ok(());
                }
                let old_url = task_state.url.to_string();
                let start = std::time::Instant::now();
                action.apply(task_state)?;
                let duration = start.elapsed();
                task_state.job.cache.write(NewCacheEntry {
                    subject,
                    key: &old_url,
                    value: Some(task_state.url.as_str()),
                    duration
                })?;
            },

            Self::Function(call) => {
                let func = task_state.job.cleaner.functions.actions.get(&call.name).ok_or(ActionError::FunctionNotFound)?;
                let old_args = task_state.call_args.replace(Some(&call.args));
                let ret = func.apply(task_state);
                task_state.call_args.replace(old_args);
                ret?
            },
            Self::CallArg(name) => task_state.call_args.get().ok_or(ActionError::NotInFunction)?
                .actions.get(get_str!(name)).ok_or(ActionError::CallArgFunctionNotFound)?
                .apply(task_state)?,
            Self::Custom(function) => function(task_state)?
        }))
    }
}

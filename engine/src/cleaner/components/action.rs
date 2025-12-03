//! [`Action`].

use std::str::{FromStr, Utf8Error};
use std::collections::HashSet;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SetPreventDuplicates};
use thiserror::Error;
#[expect(unused_imports, reason = "Used in doc comment.")]
use url::{Url, PathSegmentsMut};
use percent_encoding::percent_decode_str as pds;

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
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
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
    ///     parts: vec![UrlPart::NormalizedHost, UrlPart::RegDomain],
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
    ///     values: vec![StringSource::Part(UrlPart::NormalizedHost), StringSource::Part(UrlPart::RegDomain)],
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

    /// [`StringModification::apply`] and [`BetterUrl::set_host`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_host))]
    ModifyHost(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_subdomain`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_subdomain))]
    ModifySubdomain(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_reg_domain`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_reg_domain))]
    ModifyRegDomain(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain))]
    ModifyDomain(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_middle`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_middle))]
    ModifyDomainMiddle(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_not_domain_suffix`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_not_domain_suffix))]
    ModifyNotDomainSuffix(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::set_domain_suffix`].
    /// # Errors
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_domain_suffix))]
    ModifyDomainSuffix(StringModification),
    /// [`StringModification::apply`] and [`BetterUrl::domain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::domain_segment))]
    ModifyDomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },
    /// [`StringModification::apply`] and [`BetterUrl::subdomain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::subdomain_segment))]
    ModifySubdomainSegment {
        /// The index to insert the segment at.
        index: isize,
        /// The [`StringModification`] to apply..
        modification: StringModification
    },
    /// [`StringModification::apply`] and [`BetterUrl::domain_suffix_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::domain_suffix_segment))]
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
    /// [`BetterUrl::insert_subdomain_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::insert_subdomain_segment))]
    InsertSubdomainSegment {
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
    /// If the resulting path would be [`None`], returns the error [`ActionError::PathCannotBeNone`].
    ModifyPath(StringModification),
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
    /// Apply [`Self::ModifyPathSegment::modification`] to the specified path segment.
    /// # Errors
    /// If the call to [`BetterUrl::path_segment`] returns [`None`], returns the error [`ActionError::UrlDoesNotHavePathSegments`].
    ///
    /// If the call to [`BetterUrl::path_segment`] returns [`Some`] of [`None`], returns the error [`ActionError::PathSegmentNotFound`].
    ///
    #[doc = edoc!(applyerr(StringModification), callerr(BetterUrl::set_path_segment))]
    ModifyPathSegment {
        /// The path segment to modify.
        index: isize,
        /// The [`StringModification`] to apply.
        modification: StringModification
    },
    /// [`BetterUrl::insert_path_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment))]
    InsertPathSegment {
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
    /// [`BetterUrl::insert_path_segment`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ActionError), callerr(BetterUrl::insert_path_segment))]
    InsertRawPathSegment {
        /// The index to insert it at.
        index: isize,
        /// The value to insert.
        value: StringSource
    },
    /// [`PathSegmentsMut::pop_if_empty`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segments_mut))]
    RemoveEmptyLastPathSegment,
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
    /// [`BetterUrl::set_query_param`]
    /// # Errors
    #[doc = edoc!(geterr(StringSource), callerr(BetterUrl::set_query_param))]
    SetQueryParam {
        /// The query param to set.
        param: QueryParamSelector,
        /// The value to set it to.
        value: StringSource
    },
    /// Remove the entire [`UrlPart::Query`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2");
    ///
    /// doc_test!(apply, Ok, Action::RemoveQuery, &mut ts);
    /// assert_eq!(ts.url, "https://example.com/");
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
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3&a=4&c=5");
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParam("a".into()), &mut ts);
    /// assert_eq!(ts.url.query(), Some("b=3&c=5"));
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParam("b".into()), &mut ts);
    /// assert_eq!(ts.url.query(), Some("c=5"));
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParam("c".into()), &mut ts);
    /// assert_eq!(ts.url.query(), None);
    /// ```
    RemoveQueryParam(StringSource),
    /// Keeps all query parameters with the specified name.
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    AllowQueryParam(StringSource),
    /// Removes all query params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParams(["a".to_string(), "b".to_string()].into()), &mut ts);
    /// assert_eq!(ts.url.query(), Some("c=5"));
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParams(["c".to_string()].into()), &mut ts);
    /// assert_eq!(ts.url.query(), None);
    /// ```
    RemoveQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Keeps only query params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// doc_test!(apply, Ok, Action::AllowQueryParams(["a".to_string(), "b".to_string()].into()), &mut ts);
    /// assert_eq!(ts.url.query(), Some("a=2&b=3&%61=4"));
    ///
    /// doc_test!(apply, Ok, Action::AllowQueryParams(["c".to_string()].into()), &mut ts);
    /// assert_eq!(ts.url.query(), None);
    /// ```
    AllowQueryParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Removes all query params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParamsMatching(StringMatcher::Is("a".into())), &mut ts);
    /// assert_eq!(ts.url.query(), Some("b=3&c=5"));
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParamsMatching(StringMatcher::Is("b".into())), &mut ts);
    /// assert_eq!(ts.url.query(), Some("c=5"));
    ///
    /// doc_test!(apply, Ok, Action::RemoveQueryParamsMatching(StringMatcher::Is("c".into())), &mut ts);
    /// assert_eq!(ts.url.query(), None);
    /// ```
    RemoveQueryParamsMatching(StringMatcher),
    /// Keeps only query params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting query is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3&%61=4&c=5");
    ///
    /// doc_test!(apply, Ok, Action::AllowQueryParamsMatching(StringMatcher::Is("a".into())), &mut ts);
    /// assert_eq!(ts.url.query(), Some("a=2&%61=4"));
    ///
    /// doc_test!(apply, Ok, Action::AllowQueryParamsMatching(StringMatcher::Is("b".into())), &mut ts);
    /// assert_eq!(ts.url.query(), None);
    /// ```
    AllowQueryParamsMatching(StringMatcher),
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

    /// Removes the [`UrlPart::Fragment`].
    RemoveFragment,
    /// If the [`Url::fragment`] is `Some("")`, set it to [`None`].
    RemoveEmptyFragment,
    /// Removes all fragment parameters with the specified name.
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    RemoveFragmentParam(StringSource),
    /// Removes all fragment params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Action))]
    AllowFragmentParam(StringSource),
    /// Removes all fragment params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
    RemoveFragmentParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Keeps only fragment params with names in the specified [`HashSet`].
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
    AllowFragmentParams(#[serde_as(as = "SetPreventDuplicates<_>")] HashSet<String>),
    /// Removes all fragment params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    RemoveFragmentParamsMatching(StringMatcher),
    /// Keeps only fragment params with names matching the specified [`StringMatcher`].
    ///
    /// For performance reasons, if the resulting fragment is empty, this instead sets it to [`None`].
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
    /// Sets [`Self::CopyPart::to`] to the value of [`Self::CopyPart::from`], then sets [`Self::CopyPart::from`] to [`None`].
    /// # Errors
    #[doc = edoc!(seterr(UrlPart, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc#def");
    ///
    /// doc_test!(apply, Ok, Action::MovePart {from: UrlPart::Fragment, to: UrlPart::Path}, &mut ts);
    /// assert_eq!(ts.url, "https://example.com/def");
    /// ```
    MovePart {
        /// The part whose value to move.
        from: UrlPart,
        /// The part whose value to set.
        to: UrlPart
    },

    // Misc.

    /// Remove parameters from both the query and fragment.
    ///
    /// For each parameter, if its name is in [`Self::RemoveUTPs::names`] or starts with a string in [`Self::RemoveUTPs::prefixes`] and neither is in [`Self::RemoveUTPs::except_names`] or starts with a value in [`Self::RemoveUTPs::except_prefixes`], remove the segment.
    ///
    /// The exception stuff is because certain websites use parameters that are normall for tracking for actual features.
    ///
    /// Treats [`SetSource::get`]/[`ListSource::get`] returning [`None`] as if they returned an empty set/list.
    /// # Errors
    #[doc = edoc!(geterr(SetSource, 2), geterr(ListSource, 2))]
    RemoveUTPs {
        /// The names of segments to remove.
        #[serde(default, skip_serializing_if = "is_default")]
        names: SetSource,
        /// The prefixes of segments to remove.
        #[serde(default, skip_serializing_if = "is_default")]
        prefixes: ListSource,
        /// The names of segments to not remove.
        #[serde(default, skip_serializing_if = "is_default")]
        except_names: SetSource,
        /// The prefixes of segments to not remove.
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
    /// If [`TaskState::call_args`] is [`None`], returns the error [`ActionError::NotInFunction`].
    ///
    #[doc = edoc!(callargfunctionnotfound(Self, Action), applyerr(Self))]
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
            "None"                       => Action::None,
            "EnsureFqdnPeriod"           => Action::EnsureFqdnPeriod,
            "RemoveFqdnPeriod"           => Action::RemoveFqdnPeriod,
            "RemoveEmptyLastPathSegment" => Action::RemoveEmptyLastPathSegment,
            "RemoveQuery"                => Action::RemoveQuery,
            "RemoveEmptyQuery"           => Action::RemoveEmptyQuery,
            "RemoveFragment"             => Action::RemoveFragment,
            "RemoveEmptyFragment"        => Action::RemoveEmptyFragment,
            _                            => return Err(s.into())
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
    /// Returned whem attempting to get a path segment that doesn't exist.
    #[error("Attempted to get a path segment that didn't exist.")]
    PathSegmentNotFound,
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
    /// Returned when a [`SetQueryParamError`] is encountered.
    #[error(transparent)]
    SetQueryParamError(#[from] SetQueryParamError),

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
    #[error("TODO")]
    NotInFunction,
    #[error("TODO")]
    CallArgFunctionNotFound,
    /// An arbitrary [`std::error::Error`] returned by [`Action::Custom`].
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl Action {
    /// Applies the specified variant of [`Self`].
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    #[allow(clippy::missing_panics_doc, reason = "Can't happen.")]
    pub fn apply<'j>(&'j self, task_state: &mut TaskState<'j>) -> Result<(), ActionError> {
        match self {
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
                for action in actions {
                    action.apply(task_state)?;
                }
                let mut previous_url;
                for _ in 1..*limit {
                    previous_url = task_state.url.clone();
                    for action in actions {
                        action.apply(task_state)?;
                    }
                    if task_state.url == previous_url {break;}
                }
            },

            // Maps

            Self::PartMap   {part , map} => if let Some(action) = map.get(part .get(&task_state.url      ) ) {action.apply(task_state)?;},
            Self::StringMap {value, map} => if let Some(action) = map.get(value.get(task_state)?) {action.apply(task_state)?;},

            Self::PartPartitioning   {partitioning, part , map} => if let Some(action) = map.get(task_state.job.cleaner.params.partitionings.get(get_str!(partitioning, task_state, ActionError)).ok_or(ActionError::PartitioningNotFound)?.get(part.get(&task_state.url).as_deref())) {action.apply(task_state)?;}
            Self::FirstMatchingPartPartitioning {partitioning, parts, map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning, task_state, ActionError)).ok_or(ActionError::PartitioningNotFound)?;
                for part in parts.iter() {
                    if let Some(action) = map.get(partitioning.get(part.get(&task_state.url).as_deref())) {
                        return action.apply(task_state);
                    }
                }
            }
            Self::StringPartitioning {partitioning, value, map} => if let Some(action) = map.get(task_state.job.cleaner.params.partitionings.get(get_str!(partitioning, task_state, ActionError)).ok_or(ActionError::PartitioningNotFound)?.get(get_option_str!(value, task_state) )) {action.apply(task_state)?;}
            Self::FirstMatchingStringPartitioning {partitioning, values, map} => {
                let partitioning = task_state.job.cleaner.params.partitionings.get(get_str!(partitioning, task_state, ActionError)).ok_or(ActionError::PartitioningNotFound)?;
                for value in values.iter() {
                    if let Some(action) = map.get(partitioning.get(get_option_str!(value, task_state))) {
                        return action.apply(task_state);
                    }
                }
            }

            // Whole

            Self::SetWhole(new) => task_state.url = BetterUrl::parse(get_str!(new, task_state, ActionError))?,
            Self::Join(with) => task_state.url=task_state.url.join(get_str!(with, task_state, ActionError))?.into(),

            // Scheme

            Self::SetScheme(to) => task_state.url.set_scheme(get_new_str!(to, task_state, ActionError))?,

            // Domain

            Self::SetHost               (       value) => task_state.url.set_host                 (        get_new_option_str!(value, task_state))?,
            Self::SetSubdomain          (       value) => task_state.url.set_subdomain            (        get_new_option_str!(value, task_state))?,
            Self::SetRegDomain          (       value) => task_state.url.set_reg_domain           (        get_new_option_str!(value, task_state))?,
            Self::SetDomain             (       value) => task_state.url.set_domain               (        get_new_option_str!(value, task_state))?,
            Self::SetDomainMiddle       (       value) => task_state.url.set_domain_middle        (        get_new_option_str!(value, task_state))?,
            Self::SetNotDomainSuffix    (       value) => task_state.url.set_not_domain_suffix    (        get_new_option_str!(value, task_state))?,
            Self::SetDomainSuffix       (       value) => task_state.url.set_domain_suffix        (        get_new_option_str!(value, task_state))?,
            Self::SetDomainSegment      {index, value} => task_state.url.set_domain_segment       (*index, get_new_option_str!(value, task_state))?,
            Self::SetSubdomainSegment   {index, value} => task_state.url.set_subdomain_segment    (*index, get_new_option_str!(value, task_state))?,
            Self::SetDomainSuffixSegment{index, value} => task_state.url.set_domain_suffix_segment(*index, get_new_option_str!(value, task_state))?,

            Self::ModifyHost(modification) => {
                let mut x = task_state.url.host_str().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_host(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifySubdomain(modification) => {
                let mut x = task_state.url.subdomain().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_subdomain(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyRegDomain(modification) => {
                let mut x = task_state.url.reg_domain().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_reg_domain(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyDomain(modification) => {
                let mut x = task_state.url.domain().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_domain(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyDomainMiddle(modification) => {
                let mut x = task_state.url.domain_middle().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_domain_middle(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyNotDomainSuffix(modification) => {
                let mut x = task_state.url.not_domain_suffix().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_not_domain_suffix(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyDomainSuffix(modification) => {
                let mut x = task_state.url.domain_suffix().map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_domain_suffix(x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyDomainSegment{index, modification} => {
                let mut x = task_state.url.domain_segment(*index).map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_domain_segment(*index, x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifySubdomainSegment{index, modification} => {
                let mut x = task_state.url.subdomain_segment(*index).map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_subdomain_segment(*index, x.map(Cow::into_owned).as_deref())?;
            },

            Self::ModifyDomainSuffixSegment{index, modification} => {
                let mut x = task_state.url.domain_suffix_segment(*index).map(Cow::Borrowed);
                modification.apply(&mut x, task_state)?;
                task_state.url.set_domain_suffix_segment(*index, x.map(Cow::into_owned).as_deref())?;
            },


            Self::InsertDomainSegment       {index, value} => task_state.url.insert_domain_segment       (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertSubdomainSegment    {index, value} => task_state.url.insert_subdomain_segment    (*index, get_new_str!(value, task_state, ActionError))?,
            Self::InsertDomainSuffixSegment {index, value} => task_state.url.insert_domain_suffix_segment(*index, get_new_str!(value, task_state, ActionError))?,

            Self::EnsureFqdnPeriod => task_state.url.set_fqdn(true)?,
            Self::RemoveFqdnPeriod => task_state.url.set_fqdn(false)?,

            // Path

            Self::SetPath(to) => task_state.url.set_path(get_new_str!(to, task_state, ActionError)),
            Self::ModifyPath(modification) => {
                let mut path = Some(Cow::Borrowed(task_state.url.path()));
                modification.apply(&mut path, task_state)?;
                #[expect(clippy::unnecessary_to_owned, reason = "Borrow checker.")]
                task_state.url.set_path(&path.ok_or(ActionError::PathCannotBeNone)?.into_owned());
            },

            Self::RemovePathSegment(index       ) => task_state.url.set_path_segment(*index, None)?,
            Self::SetPathSegment   {index, value} => task_state.url.set_path_segment(*index, get_new_option_str!(value, task_state))?,
            Self::ModifyPathSegment {index, modification} => {
                let mut path_segment = Some(Cow::Borrowed(task_state.url.path_segment(*index).ok_or(ActionError::UrlDoesNotHavePathSegments)?.ok_or(ActionError::PathSegmentNotFound)?));
                modification.apply(&mut path_segment, task_state)?;
                task_state.url.set_path_segment(*index, path_segment.map(Cow::into_owned).as_deref())?;
            },
            Self::InsertPathSegment         {index, value} => task_state.url.insert_path_segment    (*index, get_new_str!(value, task_state, ActionError))?,
            Self::SetRawPathSegment         {index, value} => task_state.url.set_raw_path_segment   (*index, get_new_option_str!(value, task_state))?,
            Self::InsertRawPathSegment      {index, value} => task_state.url.insert_raw_path_segment(*index, get_new_str!(value, task_state, ActionError))?,
            Self::RemoveEmptyLastPathSegment => {task_state.url.path_segments_mut().ok_or(ActionError::UrlDoesNotHavePathSegments)?.pop_if_empty();},
            Self::RemoveFirstNPathSegments(n) => task_state.url.remove_first_n_path_segments(*n)?,
            Self::KeepFirstNPathSegments  (n) => task_state.url.keep_first_n_path_segments  (*n)?,
            Self::RemoveLastNPathSegments (n) => task_state.url.remove_last_n_path_segments (*n)?,
            Self::KeepLastNPathSegments   (n) => task_state.url.keep_last_n_path_segments   (*n)?,

            // Query

            Self::SetQuery(to) => task_state.url.set_query(get_new_option_str!(to, task_state)),
            Self::SetQueryParam {param: QueryParamSelector {name, index}, value} => task_state.url.set_query_param(name, *index, get_new_option_str!(value, task_state).map(Some))?,
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
                    if !matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), task_state)? {
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
                    if matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), task_state)? {
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
                Some(Some(Some(new_url))) => task_state.url = BetterUrl::parse(&new_url)?,
                Some(Some(None))          => Err(ActionError::QueryParamNoValue)?,
                Some(None)                => Err(ActionError::QueryParamNotFound)?,
                None                      => Err(ActionError::NoQuery)?
            },

            // Fragment

            Self::RemoveFragment => task_state.url.set_fragment(None),
            Self::RemoveEmptyFragment => if task_state.url.fragment() == Some("") {task_state.url.set_fragment(None)},
            Self::RemoveFragmentParam(name) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                let name = get_str!(name, task_state, ActionError);
                for param in fragment.split('&') {
                    if pds(param.split('=').next().expect("The first segment to always exist.")).ne(name.bytes()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowFragmentParam(name) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                let name = get_str!(name, task_state, ActionError);
                for param in fragment.split('&') {
                    if pds(param.split('=').next().expect("The first segment to always exist.")).eq(name.bytes()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RemoveFragmentParams(names) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                for param in fragment.split('&') {
                    if !names.contains(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowFragmentParams(names) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                for param in fragment.split('&') {
                    if names.contains(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()) {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::RemoveFragmentParamsMatching(matcher) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                for param in fragment.split('&') {
                    if !matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), task_state)? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },
            Self::AllowFragmentParamsMatching(matcher) => if let Some(fragment) = task_state.url.fragment() {
                let mut new = String::with_capacity(fragment.len());
                for param in fragment.split('&') {
                    if matcher.check(Some(&*pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy()), task_state)? {
                        if !new.is_empty() {new.push('&');}
                        new.push_str(param);
                    }
                }
                if new.len() != fragment.len() {
                    task_state.url.set_fragment(Some(&*new).filter(|x| !x.is_empty()));
                }
            },

            // General parts

            Self::SetPart {part, value} => {
                let temp = get_option_string!(value, task_state);
                part.set(&mut task_state.url, temp.as_deref())?;
            },

            Self::ModifyPart {part, modification} => {
                let mut temp = part.get(&task_state.url);
                modification.apply(&mut temp, task_state)?;
                let temp = temp.map(Cow::into_owned);
                part.set(&mut task_state.url, temp.as_deref())?;
            },
            Self::ModifyPartIfSome {part, modification} => {
                if let mut temp @ Some(_) = part.get(&task_state.url) {
                    modification.apply(&mut temp, task_state)?;
                    let temp = temp.map(Cow::into_owned);
                    part.set(&mut task_state.url, temp.as_deref())?;
                }
            },

            Self::CopyPart {from, to} => {
                let temp = from.get(&task_state.url).map(Cow::into_owned);
                to.set(&mut task_state.url, temp.as_deref())?;
            },
            Self::MovePart {from, to} => {
                let temp = from.get(&task_state.url).map(Cow::into_owned);
                to.set(&mut task_state.url, temp.as_deref())?;
                from.set(&mut task_state.url, None)?;
            },

            // Misc.

            Self::RemoveUTPs {names, prefixes, except_names, except_prefixes} => if task_state.url.query().is_some() || task_state.url.fragment().is_some() {
                let default_list = Default::default();
                let default_set = Default::default();

                let names = match names {
                    SetSource::Params(StringSource::String(x)) => task_state.job.cleaner.params.sets.get(x),
                    _ => names.get(task_state)?
                }.unwrap_or(&default_set);

                let prefixes = match prefixes {
                    ListSource::Params(StringSource::String(x)) => task_state.job.cleaner.params.lists.get(x),
                    _ => prefixes.get(task_state)?
                }.unwrap_or(&default_list);

                let except_names = except_names.get(task_state)?.unwrap_or(&default_set);
                let except_prefixes = except_prefixes.get(task_state)?.unwrap_or(&default_list);
                let excepts = !except_names.is_empty() || !except_prefixes.is_empty();

                let new_query = task_state.url.query().map(|query| {
                    let mut new = String::with_capacity(query.len());
                    for param in query.split('&') {
                        let name = pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy();
                        if !(names.contains_some(&*name) || prefixes.iter().any(|prefix| name.starts_with(prefix))) || (excepts && (except_names.contains_some(&*name) || except_prefixes.iter().any(|prefix| name.starts_with(prefix)))) {
                            if !new.is_empty() {new.push('&');}
                            new.push_str(param);
                        }
                    }
                    if new.is_empty() {None} else {Some(new)}
                });
                let new_fragment = task_state.url.fragment().map(|fragment| {
                    let mut new = String::with_capacity(fragment.len());
                    for param in fragment.split('&') {
                        let name = pds(param.split('=').next().expect("The first segment to always exist.")).decode_utf8_lossy();
                        if !(names.contains_some(&*name) || prefixes.iter().any(|prefix| name.starts_with(prefix))) || (excepts && (except_names.contains_some(&*name) || except_prefixes.iter().any(|prefix| name.starts_with(prefix)))) {
                            if !new.is_empty() {new.push('&');}
                            new.push_str(param);
                        }
                    }
                    if new.is_empty() {None} else {Some(new)}
                });
                if let Some(new_query   ) = new_query    {task_state.url.set_query   (new_query   .as_deref());}
                if let Some(new_fragment) = new_fragment {task_state.url.set_fragment(new_fragment.as_deref());}
            },

            #[cfg(feature = "cache")]
            Self::Cache {subject, action} => {
                let _unthread_handle = task_state.job.unthreader.unthread();
                let subject = get_string!(subject, task_state, ActionError);
                if let Some(entry) = task_state.job.cache.read(CacheEntryKeys {subject: &subject, key: task_state.url.as_str()})? {
                    task_state.url = BetterUrl::parse(&entry.value.ok_or(ActionError::CachedUrlIsNone)?)?;
                    return Ok(());
                }
                let old_url = task_state.url.to_string();
                let start = std::time::Instant::now();
                action.apply(task_state)?;
                let duration = start.elapsed();
                task_state.job.cache.write(NewCacheEntry {
                    subject: &subject,
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
                .actions.get(get_str!(name, task_state, ActionError)).ok_or(ActionError::CallArgFunctionNotFound)?
                .apply(task_state)?,
            Self::Custom(function) => function(task_state)?
        };
        Ok(())
    }
}

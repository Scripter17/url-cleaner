//! [`Condition`].

#![allow(unused_assignments, reason = "False positive.")]

use thiserror::Error;
use serde::{Serialize, Deserialize};
#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::prelude::*;

/// Conditions that decide if and when to apply an [`Action`].
///
/// - "*IsOneOf" variants should always be equivalent to a [`Self::Any`] with a respective "*Is" variant for each value in the [`Set`].
///
/// - "*IsInSet" variants should always be equivalent to moving the [`Set`] from [`Params::sets`] to the respective "*IsOneOf" variant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub enum Condition {
    // Debug/constants

    /// Always satisfied.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true, Condition::Always, &ts);
    /// ```
    Always,
    /// Never satisfied.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, false, Condition::Never, &ts);
    /// ```
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, Err, Condition::Error("...".into()), &ts);
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    // Logic

    /// If [`Self::If::if`] is satisfied, return the value of [`Self::If::then`].
    ///
    /// If [`Self::If::if`] is unsatisfied, return the value of [`Self::If::else`].
    /// # Errors
    #[doc = edoc!(checkerr(Self, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true , Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Always))}, &ts);
    /// doc_test!(check, true , Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Never ))}, &ts);
    /// doc_test!(check, false, Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Always))}, &ts);
    /// doc_test!(check, false, Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Never ))}, &ts);
    /// doc_test!(check, true , Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Always))}, &ts);
    /// doc_test!(check, false, Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Never ))}, &ts);
    /// doc_test!(check, true , Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Always))}, &ts);
    /// doc_test!(check, false, Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Never ))}, &ts);
    /// ```
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] is satisfied.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] is unsatisfied.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
    },
    /// Inverts the satisfaction of the contained [`Self`].
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, false, Condition::Not(Box::new(Condition::Always)), &ts);
    /// doc_test!(check, true , Condition::Not(Box::new(Condition::Never )), &ts);
    /// ```
    Not(Box<Self>),
    /// Satisfied if all contained [`Self`]s are satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, false, Condition::All(vec![Condition::Never , Condition::Never ]), &ts);
    /// doc_test!(check, false, Condition::All(vec![Condition::Never , Condition::Always]), &ts);
    /// doc_test!(check, false, Condition::All(vec![Condition::Always, Condition::Never ]), &ts);
    /// doc_test!(check, true , Condition::All(vec![Condition::Always, Condition::Always]), &ts);
    /// ```
    All(Vec<Self>),
    /// Satisfied if any contained [`Self`] is satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, false, Condition::Any(vec![Condition::Never , Condition::Never ]), &ts);
    /// doc_test!(check, true , Condition::Any(vec![Condition::Never , Condition::Always]), &ts);
    /// doc_test!(check, true , Condition::Any(vec![Condition::Always, Condition::Never ]), &ts);
    /// doc_test!(check, true , Condition::Any(vec![Condition::Always, Condition::Always]), &ts);
    /// ```
    Any(Vec<Self>),

    // Error handling

    /// Satisfied if the contained [`Self`] is satisfied or errors.
    ErrorToSatisfied(Box<Self>),
    /// Satisfied if the contained [`Self`] is satisfied.
    ///
    /// Unsatisfied if the contained [`Self`] errors.
    ErrorToUnsatisfied(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::check`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(checkerrte(Self, Condition))]
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },

    // Maps

    /// Gets the value specified by [`Self::PartMap::part`], indexes [`Self::PartMap::map`], and returns the value of the returned [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], fails.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    PartMap {
        /// The [`UrlPart`] to index [`Self::PartMap::map`] with.
        part: UrlPart,
        /// The [`Map`] to index with [`Self::PartMap::part`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the string specified by [`Self::StringMap::value`], indexes [`Self::StringMap::map`], and returns the value of the returned [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], fails.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), checkerr(Self))]
    StringMap {
        /// The [`StringSource`] to index [`Self::StringMap::map`] with.
        value: StringSource,
        /// The [`Map`] to index with [`Self::StringMap::value`].
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the name of the partition [`Self::PartPartitioning::part`] is in in the specified [`Partitioning`], indexes [`Self::PartPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Condition, 2), notfound(Partitioning, Condition), checkerr(Self))]
    PartPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`UrlPart`] whose value to find in the [`Partitioning`].
        part: UrlPart,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },
    /// Gets the name of the partition [`Self::StringPartitioning::value`] is in in the specified [`Partitioning`], indexes [`Self::StringPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), notfound(Partitioning, Condition), checkerr(Self))]
    StringPartitioning {
        /// The [`Partitioning`] to search in.
        partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`Partitioning`].
        value: StringSource,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Box<Map<Self>>
    },

    // Params

    /// Satisfied if the specified flag is set.
    /// # Errors
    #[doc = edoc!(geterr(FlagSource))]
    FlagIsSet(FlagSource),
    /// Satisfied if the specified flag is not set.
    /// # Errors
    #[doc = edoc!(geterr(FlagSource))]
    FlagIsNotSet(FlagSource),
    /// Satisfied if [`Self::VarIs::var`] is [`Self::VarIs::value`].
    /// # Errors
    #[doc = edoc!(geterr(VarSource), geterr(StringSource))]
    VarIs {
        /// The var to check the value of.
        var: VarSource,
        /// The value to check if [`Self::VarIs::var`] is.
        value: StringSource
    },

    // String source

    /// Satisfied if [`Self::StringIs::left`] is [`Self::StringIs::right`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true , Condition::StringIs {left: "a".into(), right: "a".into()}, &ts);
    /// doc_test!(check, false, Condition::StringIs {left: "a".into(), right: "b".into()}, &ts);
    /// ```
    StringIs {
        /// The left hand side of the equality check.
        left: StringSource,
        /// The right hand side of the equality check.
        right: StringSource
    },
    /// Satisfied if the specified [`StringSource`] is [`Some`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true , Condition::StringIsSome("abc"       .into()), &ts);
    /// doc_test!(check, false, Condition::StringIsSome(None::<&str>.into()), &ts);
    /// ```
    StringIsSome(StringSource),
    /// Satisfied if [`Self::StringContains::value`] contains [`Self::StringContains::substring`] at [`Self::StringContains::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), checkerr(StringLocation))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true, Condition::StringContains {value: "abc".into(), substring: "b".into(), at: StringLocation::Anywhere}, &ts);
    /// ```
    StringContains {
        /// The value to search for [`Self::StringContains::substring`].
        value: StringSource,
        /// The value to search for inside [`Self::StringContains::value`].
        substring: StringSource,
        /// Where in [`Self::StringContains::value`] to search for [`Self::StringContains::substring`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Satisfied if [`Self::StringMatches::value`] satisfies [`Self::StringMatches::matcher`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts);
    ///
    /// doc_test!(check, true, Condition::StringMatches {value: "abc".into(), matcher: StringMatcher::Always}, &ts);
    /// ```
    StringMatches {
        /// The value to check the value of.
        value: StringSource,
        /// The matcher to check if [`Self::StringMatches::value`] satisfies.
        matcher: StringMatcher
    },

    // Whole

    /// Satisfied if the URL is the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com");
    ///
    /// doc_test!(check, true, Condition::UrlIs("https://example.com/".into()), &ts);
    /// ```
    UrlIs(#[suitable(assert = "string_source_string_literal_is_url_literal")] StringSource),

    // Scheme

    /// Satisfied if the value of [`Url::scheme`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SchemeIs(StringSource),
    /// Satisfied if the [`Url::scheme`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::SchemeIsOneOf(
    ///     [
    ///         "http".to_string(),
    ///         "https".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "http://example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com"); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "other://example.com"); doc_test!(check, false, condition, &ts);
    /// ```
    SchemeIsOneOf(Set<String>),
    /// Satisfied if the [`Url::scheme`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    SchemeIsInSet(#[suitable(assert = "set_is_documented")] StringSource),

    // Host is

    /// Satisfied if the [`BetterUrl::host_str`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::HostIs("example.com".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    HostIs(StringSource),
    /// Satisfied if the [`BetterUrl::domain_normal`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainNormalIs("example.com".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainNormalIs(StringSource),
    /// Satisfied if the [`BetterUrl::domain_origin`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainOriginIs("example.com".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainOriginIs(StringSource),
    /// Satisfied if the [`BetterUrl::domain_prefix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainPrefixIs("www".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainPrefixIs(StringSource),
    /// Satisfied if the [`BetterUrl::domain_middle`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainMiddleIs("example".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainMiddleIs(StringSource),
    /// Satisfied if the [`BetterUrl::domain_suffix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainSuffixIs("com".into());
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainSuffixIs(StringSource),



    /// Satisfied if the [`BetterUrl::domain_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_prefix_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainPrefixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_suffix_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_origin_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainOriginSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },

    // Host starts with

    /// Satisfied if the [`BetterUrl::host_str`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    HostStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_normal`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainNormalStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_origin`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainOriginStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_prefix`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainPrefixStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_middle`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainMiddleStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_suffix`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixStartsWith(StringSource),



    /// Satisfied if the [`BetterUrl::domain_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_prefix_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainPrefixSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_suffix_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_origin_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainOriginSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },

    // Host ends with

    /// Satisfied if the [`Url::host`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    HostEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_normal`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainNormalEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_prefix`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainPrefixEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_origin`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainOriginEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_middle`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainMiddleEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::domain_suffix`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixEndsWith(StringSource),



    /// Satisfied if the [`BetterUrl::domain_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_origin_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainOriginSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_prefix_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainPrefixSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_suffix_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },

    // Host is one of

    /// Satisfied if the [`Url::host`] is contained in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::HostIsOneOf(
    ///     [
    ///         "example.com".to_string(),
    ///         "www.example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    HostIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_normal`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainNormalIsOneOf(
    ///     [
    ///         "example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainNormalIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_prefix`] is contained in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainPrefixIsOneOf(
    ///     [
    ///         "www".to_string(),
    ///         "abc".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainPrefixIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_origin`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainOriginIsOneOf(
    ///     [
    ///         "example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainOriginIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_middle`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainMiddleIsOneOf(
    ///     [
    ///         "example".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainMiddleIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_suffix`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// let condition = Condition::DomainSuffixIsOneOf(
    ///     [
    ///         "com".to_string()
    ///     ].into()
    /// );
    ///
    /// doc_test!(task_state, ts, task = "https://example.com"     ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://example.com."    ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://www.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com" ); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://abc.example.com."); doc_test!(check, true , condition, &ts);
    /// doc_test!(task_state, ts, task = "https://127.0.0.1"       ); doc_test!(check, false, condition, &ts);
    /// doc_test!(task_state, ts, task = "https://[::1]"           ); doc_test!(check, false, condition, &ts);
    /// ```
    DomainSuffixIsOneOf(Set<String>),



    /// Satisfied if the [`BetterUrl::domain_segment`] is in the specified [`Set`].
    DomainSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::domain_origin_segment`] is in the specified [`Set`].
    DomainOriginSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::domain_prefix_segment`] is in the specified [`Set`].
    DomainPrefixSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::domain_suffix_segment`] is in the specified [`Set`].
    DomainSuffixSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },

    // Host is in set

    /// Satisfied if the [`Url::host_str`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    HostIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_normal`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainNormalIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_prefix`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainPrefixIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_origin`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainOriginIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_middle`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainMiddleIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_suffix`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainSuffixIsInSet(#[suitable(assert = "set_is_documented")] StringSource),



    /// Satisfied if the [`BetterUrl::domain_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_origin_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainOriginSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_prefix_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainPrefixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_suffix_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainSuffixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },

    // Misc. host

    /// [`BetterUrl::has_host`].
    UrlHasHost,
    /// [`HostDetails::is_domain`].
    HostIsDomain,
    /// [`HostDetails::is_ip`].
    HostIsIp,
    /// [`HostDetails::is_ipv4`].
    HostIsIpv4,
    /// [`HostDetails::is_ipv6`].
    HostIsIpv6,
    /// [`HostDetails::is_opaque`].
    HostIsOpaque,
    /// [`HostDetails::is_empty`].
    HostIsEmpty,

    /// [`IpDetails::is_loopback`].
    HostIsLoopbackIp,
    /// [`IpDetails::is_multicast`].
    HostIsMulticastIp,
    /// [`IpDetails::is_unspecified`].
    HostIsUnspecifiedIp,

    /// [`Ipv4Details::is_broadcast`].
    HostIsBroadcastIpv4,
    /// [`Ipv4Details::is_documentation`].
    HostIsDocumentationIpv4,
    /// [`Ipv4Details::is_link_local`].
    HostIsLinkLocalIpv4,
    /// [`Ipv4Details::is_loopback`].
    HostIsLoopbackIpv4,
    /// [`Ipv4Details::is_multicast`].
    HostIsMulticastIpv4,
    /// [`Ipv4Details::is_private`].
    HostIsPrivateIpv4,
    /// [`Ipv4Details::is_unspecified`].
    HostIsUnspecifiedIpv4,

    /// [`Ipv6Details::is_loopback`].
    HostIsLoopbackIpv6,
    /// [`Ipv6Details::is_unicast_link_local`].
    HostIsUnicastLinkLocalIpv6,
    /// [`Ipv6Details::is_multicast`].
    HostIsMulticastIpv6,
    /// [`Ipv6Details::is_unique_local`].
    HostIsUniqueLocalIpv6,
    /// [`Ipv6Details::is_unspecified`].
    HostIsUnspecifiedIpv6,

    // Path

    /// Satisfied if [`Url::path`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    PathIs(StringSource),
    /// Satisfied if [`Url::path`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    PathStartsWith(StringSource),
    /// Satisfied if [`Url::path`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    PathEndsWith(StringSource),
    /// Satisfied if [`Url::path`] is in the specified [`Set`].
    PathIsOneOf(Set<String>),
    /// Satisfied if [`Url::path`] is in the named [`Set`].
    /// # Errors
    #[doc = edoc!(notfound(Set, Condition))]
    PathIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if [`Url::path`] contains the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), checkerr(StringLocation))]
    PathContains {
        /// The value to check for.
        value: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Satisfied if [`Url::path`] satisfies the specified [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(checkerr(StringMatcher))]
    PathMatches(StringMatcher),

    // Path segment

    /// Satisfied if the [`Self::PathSegmentIs::index`]th path segment is the specified value.
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque), geterr(StringSource))]
    PathSegmentIs {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// Satisfied if the specified path segment starts with the specified value.
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque), callsomenone(BetterUrl::path_segments, ConditionError::PathSegmentNotFound), geterr(StringSource))]
    PathSegmentStartsWith {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// Satisfied if the specified path segment ends with the specified value.
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque), callsomenone(BetterUrl::path_segments, ConditionError::PathSegmentNotFound), geterr(StringSource))]
    PathSegmentEndsWith {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// Satisfied if the specified path segment is in the specified [`Set`].
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque))]
    PathSegmentIsOneOf {
        /// The path segment to get.
        index: isize,
        /// The set to check in.
        values: Set<String>
    },
    /// Satisfied if the specified path segment is in the named [`Set`].
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque), notfound(Set, Condition))]
    PathSegmentIsInSet {
        /// The path segment to get.
        index: isize,
        /// The set to check in.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
    /// Satisfied if the specified path segment contains the specified value.
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segment, ConditionError::PathSegmentNotFound), geterr(StringSource), checkerr(StringLocation))]
    PathSegmentContains {
        /// The path segment to get.
        index: isize,
        /// The value to check for.
        value: StringSource,
        /// The location to cehck at.
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Satisfied if the specified path segment satisfies the specified [`StringMatcher`].
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segments, PathIsOpaque), checkerr(StringMatcher))]
    PathSegmentMatches {
        /// The path segment to get.
        index: isize,
        /// The matcher to check with.
        matcher: StringMatcher
    },

    /// Satisfied if the [`Url::path`] has segments.
    PathIsSegmented,
    /// Satisfied if the URL has a path segment of the specified index.
    HasPathSegment(isize),

    // Query

    /// Satisfied if the [`Url::query`] is the specified value.
    QueryIs(StringSource),
    /// Satisfied if the URL' has a query query and has a matching query parameter.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com?a=2&b=3");
    ///
    /// doc_test!(check, true , Condition::HasQueryParam("a".into()), &ts);
    /// doc_test!(check, true , Condition::HasQueryParam("b".into()), &ts);
    /// doc_test!(check, false, Condition::HasQueryParam("c".into()), &ts);
    /// ```
    HasQueryParam(QueryParamSelector),
    /// Satisfied if the specified query parameter is the specified value.
    QueryParamIs {
        /// The query param to check.
        param: QueryParamSelector,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the specified query parameter is in the specified [`Set`].
    QueryParamIsOneOf {
        /// The query param to check.
        param: QueryParamSelector,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the specified query parameter is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    QueryParamIsInSet {
        /// The query param to check.
        param: QueryParamSelector,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },

    // Fragment

    /// Satisfied if the [`Url::fragment`] is the specified value.
    FragmentIs(StringSource),
    /// Satisfied if the [`Url::fragment`] is in the specified [`Set`].
    FragmentIsOneOf(Set<String>),
    /// Satisfied if the [`Url::fragment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    FragmentIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`Url::fragment`] is [`Some`] and starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    FragmentIsSomeAndStartsWith(StringSource),

    // General parts

    /// Satisfied if the value of [`Self::PartIs::part`] is the same as the value of [`Self::PartIs::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc?a=2");
    ///
    /// doc_test!(check, true, Condition::PartIs {part: UrlPart::Host                  , value: "example.com".into()}, &ts);
    /// doc_test!(check, true, Condition::PartIs {part: UrlPart::Path                  , value: "/abc"       .into()}, &ts);
    /// doc_test!(check, true, Condition::PartIs {part: UrlPart::Query                 , value: "a=2"        .into()}, &ts);
    /// doc_test!(check, true, Condition::PartIs {part: UrlPart::QueryParam("a".into()), value: "2"          .into()}, &ts);
    /// ```
    PartIs {
        /// The [`UrlPart`] to get.
        part: UrlPart,
        /// The [`StringSource`] to compare [`Self::PartIs::value`] with.
        value: StringSource
    },
    /// Satisfied if [`Self::PartContains::part`] contains [`Self::PartContains::value`] at [`Self::PartContains::at`].
    /// # Errors
    #[doc = edoc!(getnone(UrlPart, Condition), getnone(StringSource, Condition), checkerr(StringLocation))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc");
    ///
    /// doc_test!(check, true, Condition::PartContains {part: UrlPart::Path    , value: "/ab".into(), at: StringLocation::Start}, &ts);
    /// doc_test!(check, Err , Condition::PartContains {part: UrlPart::Fragment, value: ""   .into(), at: StringLocation::Start}, &ts);
    /// ```
    PartContains {
        /// The part to look in.
        part: UrlPart,
        /// The value to look for.
        value: StringSource,
        /// Where to look in [`Self::PartContains::part`] for [`Self::PartContains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Satisfied if [`Self::PartContains::part`] is [`Some`] and contains [`Self::PartContains::value`] at [`Self::PartContains::at`].
    /// # Errors
    #[doc = edoc!(getnone(StringSource, Condition), checkerr(StringLocation))]
    PartIsSomeAndContains {
        /// The part to look in.
        part: UrlPart,
        /// The value to look for.
        value: StringSource,
        /// Where to look in [`Self::PartContains::part`] for [`Self::PartContains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation
    },
    /// Satisfied if [`Self::PartMatches::part`] satisfies [`Self::PartMatches::matcher`].
    /// # Errors
    #[doc = edoc!(getnone(UrlPart, Condition), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc");
    ///
    /// doc_test!(check, true, Condition::PartMatches {part: UrlPart::Path    , matcher: StringMatcher::Always}, &ts);
    /// doc_test!(check, true, Condition::PartMatches {part: UrlPart::Fragment, matcher: StringMatcher::Always}, &ts);
    /// ```
    PartMatches {
        /// The part to match the value of.
        part: UrlPart,
        /// The matcher to test [`Self::PartMatches::part`] with.
        matcher: StringMatcher
    },
    /// Satisfied if [`Self::PartIsOneOf::part`] is in [`Self::PartIsOneOf::values`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, task = "https://example.com/abc");
    ///
    /// doc_test!(check, true, Condition::PartIsOneOf {part: UrlPart::Path    , values: [Some("/abc".into()), None].into()}, &ts);
    /// doc_test!(check, true, Condition::PartIsOneOf {part: UrlPart::Fragment, values: [Some("/abc".into()), None].into()}, &ts);
    /// ```
    PartIsOneOf {
        /// The part to check the value of.
        part: UrlPart,
        /// The set of values to check if [`Self::PartIsOneOf::part`] is one of.
        values: Set<String>
    },
    /// Satisfied if [`Self::PartIsInSet`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    PartIsInSet {
        /// The part to check the value of.
        part: UrlPart,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },

    // Misc.

    /// Uses a [`Self`] from [`Cleaner::functions`].
    /// # Errors
    #[doc = edoc!(functionnotfound(Self, Condition), checkerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::docs::*;
    ///
    /// doc_test!(task_state, ts, functions = Functions {
    ///     conditions: [("abc".into(), Condition::Always)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// doc_test!(check, true, Condition::Function(Box::new(FunctionCall {name: "abc".into(), args: Default::default()})), &ts);
    /// ```
    Function(Box<FunctionCall>),
    /// Uses a [`Self`] from [`TaskState::call_args`].
    /// # Errors
    #[doc = edoc!(notinfunction(Condition), callargfunctionnotfound(Self, Condition), checkerr(Self))]
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
    /// fn some_complex_operation(task_state: &TaskState) -> Result<bool, ConditionError> {
    ///     Ok(true)
    /// }
    ///
    /// doc_test!(check, true, Condition::Custom(some_complex_operation), &ts);
    /// ```
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&TaskState) -> Result<bool, ConditionError>)
}

/// The enum of errors [`Condition::check`] can return.
#[derive(Debug, Error)]
pub enum ConditionError {
    /// Returned when a [`Condition::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when both [`Condition`]s in a [`Condition::TryElse`] return errors.
    #[error("Both Conditions in a Condition::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`Condition::TryElse::try`].
        try_error: Box<Self>,
        /// The error returned by [`Condition::TryElse::else`].
        else_error: Box<Self>
    },

    /// [`StringSourceIsNone`].
    #[error(transparent)]
    StringSourceIsNone(#[from] StringSourceIsNone),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),

    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A part of the URL is None where it had to be Some.")]
    UrlPartIsNone,
    /// Returned when the Host is [`None`] where it has to be [`Some`].
    #[error("The Host was None where it had to be Some.")]
    HostIsNone,
    /// Returned when the DomainNormal is [`None`] where it has to be [`Some`].
    #[error("The DomainNormal was None where it had to be Some.")]
    DomainNormalIsNone,
    /// Returned when the DomainOrigin is [`None`] where it has to be [`Some`].
    #[error("The DomainOrigin was None where it had to be Some.")]
    DomainOriginIsNone,
    /// Returned when the DomainPrefix is [`None`] where it has to be [`Some`].
    #[error("The DomainPrefix was None where it had to be Some.")]
    DomainPrefixIsNone,
    /// Returned when the DomainMiddle is [`None`] where it has to be [`Some`].
    #[error("The DomainMiddle was None where it had to be Some.")]
    DomainMiddleIsNone,
    /// Returned when the DomainSuffix is [`None`] where it has to be [`Some`].
    #[error("The DomainSuffix was None where it had to be Some.")]
    DomainSuffixIsNone,
    /// Returned when the DomainSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainSegment was None where it had to be Some.")]
    DomainSegmentIsNone,
    /// Returned when the DomainOriginSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainOriginSegment was None where it had to be Some.")]
    DomainOriginSegmentIsNone,
    /// Returned when the DomainPrefixSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainPrefixSegment was None where it had to be Some.")]
    DomainPrefixSegmentIsNone,
    /// Returned when the DomainSuffixSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainSuffixSegment was None where it had to be Some.")]
    DomainSuffixSegmentIsNone,
    /// Returned when attempting to get more path segments than are available.
    #[error("Attempted to get more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to get a path segment not in a URL.
    #[error("Attempted to get a path segment not in the URL.")]
    PathSegmentNotFound,

    /// Returned when a [`PathIsOpaque`] is encountered.
    #[error(transparent)]
    PathIsOpaque(#[from] PathIsOpaque),

    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),

    /// Returned when a [`FlagSourceError`] is encountered.
    #[error(transparent)]
    FlagSourceError(#[from] FlagSourceError),
    /// Returned when a [`VarSourceError`] is encountered.
    #[error(transparent)]
    VarSourceError(#[from] VarSourceError),

    /// Returned when a [`Set`] wasn't found.
    #[error("The requested set wasn't found.")]
    SetNotFound,
    /// Returned when a [`Partitioning`] with the specified name isn't found.
    #[error("A Partitioning with the specified name wasn't found.")]
    PartitioningNotFound,

    /// Returned when a [`Condition`] with the specified name isn't found in the [`Functions::conditions`].
    #[error("A Condition with the specified name wasn't found in the Functions::conditions.")]
    FunctionNotFound,
    /// Returned when attempting to use [`CallArgs`] outside a function.
    #[error("Attempted to use CallArgs outside a function.")]
    NotInFunction,
    /// Returned when a [`CallArgs`] function ins't found.
    #[error("A CallArgs function wasn't found.")]
    CallArgFunctionNotFound,
    /// An arbitrary [`std::error::Error`] returned by [`Condition::Custom`].
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>)
}

impl Condition {
    /// If the specified variant of [`Self`] is satisfied, return [`true`].
    ///
    /// If the specified variant of [`Self`] is unsatisfied, return [`false`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check<'j>(&'j self, task_state: &TaskState<'j>) -> Result<bool, ConditionError> {
        debug!(Condition::check, self; Ok(match self {
            // Debug/constants

            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ConditionError::ExplicitError(msg.clone()))?,
            Self::Debug(condition) => {
                let is_satisfied = condition.check(task_state);
                eprintln!("=== Condition::Debug ===\nCondition: {condition:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic

            Self::If {r#if, then, r#else} => if r#if.check(task_state)? {
                then.check(task_state)?
            } else if let Some(r#else) = r#else {
                r#else.check(task_state)?
            } else {
                false
            },
            Self::Not(condition) => !condition.check(task_state)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.check(task_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.check(task_state)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Error handling

            Self::ErrorToSatisfied  (condition) => condition.check(task_state).unwrap_or(true),
            Self::ErrorToUnsatisfied(condition) => condition.check(task_state).unwrap_or(false),
            Self::TryElse{ r#try, r#else } => match r#try.check(task_state) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(task_state) {
                    Ok(x) => x,
                    Err(else_error) => Err(ConditionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                }
            },

            // Maps

            Self::PartMap  {part , map} => if let Some(condition) = map.get(part .get(&task_state.url) ) {condition.check(task_state)?} else {false},
            Self::StringMap{value, map} => if let Some(condition) = map.get(value.get(task_state    )?) {condition.check(task_state)?} else {false},

            Self::PartPartitioning   {partitioning, part , map} => if let Some(condition) = map.get(task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ConditionError::PartitioningNotFound)?.get(part.get(&task_state.url).as_deref())) {condition.check(task_state)?} else {false},
            Self::StringPartitioning {partitioning, value, map} => if let Some(condition) = map.get(task_state.job.cleaner.params.partitionings.get(get_str!(partitioning)).ok_or(ConditionError::PartitioningNotFound)?.get(get_option_str!(value)) ) {condition.check(task_state)?} else {false},

            // Params

            Self::FlagIsSet   (FlagSource::Params(StringSource::String(name))) =>  task_state.job.cleaner.params.flags.contains(name),
            Self::FlagIsNotSet(FlagSource::Params(StringSource::String(name))) => !task_state.job.cleaner.params.flags.contains(name),

            Self::FlagIsSet(flag)    =>  flag.get(task_state)?,
            Self::FlagIsNotSet(flag) => !flag.get(task_state)?,

            Self::VarIs {var, value} => var.get(task_state)?.as_deref() == value.get(task_state)?.as_deref(),

            // String source

            Self::StringIs {left, right} => get_option_cow!(left) == get_option_cow!(right),
            Self::StringIsSome(value) => value.get(task_state)?.is_some(),
            Self::StringContains {value, substring, at} => at.check(get_str!(value), get_str!(substring))?,
            Self::StringMatches {value, matcher} => matcher.check(get_option_str!(value), task_state)?,

            // Whole

            Self::UrlIs(value) => task_state.url == *get_cow!(value),

            // Scheme

            Self::SchemeIs(value) => task_state.url.scheme_str() == get_str!(value),
            Self::SchemeIsOneOf(values) => values.contains_some(task_state.url.scheme_str()),
            Self::SchemeIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains_some(task_state.url.scheme_str()),

            // Host is

            Self::HostIs        (x) => task_state.url.host_str     () == get_option_str!(x),
            Self::DomainPrefixIs(x) => task_state.url.domain_prefix() == get_option_str!(x),
            Self::DomainMiddleIs(x) => task_state.url.domain_middle() == get_option_str!(x),
            Self::DomainSuffixIs(x) => task_state.url.domain_suffix() == get_option_str!(x),
            Self::DomainOriginIs(x) => task_state.url.domain_origin() == get_option_str!(x),
            Self::DomainNormalIs(x) => task_state.url.domain_normal() == get_option_str!(x),

            Self::DomainSegmentIs       {index, value} => task_state.url.domain_segment       (*index) == get_option_str!(value),
            Self::DomainPrefixSegmentIs {index, value} => task_state.url.domain_prefix_segment(*index) == get_option_str!(value),
            Self::DomainSuffixSegmentIs {index, value} => task_state.url.domain_suffix_segment(*index) == get_option_str!(value),
            Self::DomainOriginSegmentIs {index, value} => task_state.url.domain_origin_segment(*index) == get_option_str!(value),

            // Host starts with

            Self::HostStartsWith        (x) => task_state.url.host_str     ().ok_or(ConditionError::HostIsNone        )?.starts_with(get_str!(x)),
            Self::DomainPrefixStartsWith(x) => task_state.url.domain_prefix().ok_or(ConditionError::DomainPrefixIsNone)?.starts_with(get_str!(x)),
            Self::DomainMiddleStartsWith(x) => task_state.url.domain_middle().ok_or(ConditionError::DomainMiddleIsNone)?.starts_with(get_str!(x)),
            Self::DomainSuffixStartsWith(x) => task_state.url.domain_suffix().ok_or(ConditionError::DomainSuffixIsNone)?.starts_with(get_str!(x)),
            Self::DomainOriginStartsWith(x) => task_state.url.domain_origin().ok_or(ConditionError::DomainOriginIsNone)?.starts_with(get_str!(x)),
            Self::DomainNormalStartsWith(x) => task_state.url.domain_normal().ok_or(ConditionError::DomainNormalIsNone)?.starts_with(get_str!(x)),

            Self::DomainSegmentStartsWith       {index, value} => task_state.url.domain_segment       (*index).ok_or(ConditionError::DomainSegmentIsNone      )?.starts_with(get_str!(value)),
            Self::DomainPrefixSegmentStartsWith {index, value} => task_state.url.domain_prefix_segment(*index).ok_or(ConditionError::DomainPrefixSegmentIsNone)?.starts_with(get_str!(value)),
            Self::DomainSuffixSegmentStartsWith {index, value} => task_state.url.domain_suffix_segment(*index).ok_or(ConditionError::DomainSuffixSegmentIsNone)?.starts_with(get_str!(value)),
            Self::DomainOriginSegmentStartsWith {index, value} => task_state.url.domain_origin_segment(*index).ok_or(ConditionError::DomainOriginSegmentIsNone)?.starts_with(get_str!(value)),

            // Host ends with

            Self::HostEndsWith        (x) => task_state.url.host_str     ().ok_or(ConditionError::HostIsNone        )?.ends_with(get_str!(x)),
            Self::DomainPrefixEndsWith(x) => task_state.url.domain_prefix().ok_or(ConditionError::DomainPrefixIsNone)?.ends_with(get_str!(x)),
            Self::DomainMiddleEndsWith(x) => task_state.url.domain_middle().ok_or(ConditionError::DomainMiddleIsNone)?.ends_with(get_str!(x)),
            Self::DomainSuffixEndsWith(x) => task_state.url.domain_suffix().ok_or(ConditionError::DomainSuffixIsNone)?.ends_with(get_str!(x)),
            Self::DomainOriginEndsWith(x) => task_state.url.domain_origin().ok_or(ConditionError::DomainOriginIsNone)?.ends_with(get_str!(x)),
            Self::DomainNormalEndsWith(x) => task_state.url.domain_normal().ok_or(ConditionError::DomainNormalIsNone)?.ends_with(get_str!(x)),

            Self::DomainSegmentEndsWith       {index, value} => task_state.url.domain_segment       (*index).ok_or(ConditionError::DomainSegmentIsNone      )?.ends_with(get_str!(value)),
            Self::DomainPrefixSegmentEndsWith {index, value} => task_state.url.domain_prefix_segment(*index).ok_or(ConditionError::DomainPrefixSegmentIsNone)?.ends_with(get_str!(value)),
            Self::DomainSuffixSegmentEndsWith {index, value} => task_state.url.domain_suffix_segment(*index).ok_or(ConditionError::DomainSuffixSegmentIsNone)?.ends_with(get_str!(value)),
            Self::DomainOriginSegmentEndsWith {index, value} => task_state.url.domain_origin_segment(*index).ok_or(ConditionError::DomainOriginSegmentIsNone)?.ends_with(get_str!(value)),

            // Host is one of

            Self::HostIsOneOf        (x) => x.contains(task_state.url.host_str     ()),
            Self::DomainPrefixIsOneOf(x) => x.contains(task_state.url.domain_prefix()),
            Self::DomainMiddleIsOneOf(x) => x.contains(task_state.url.domain_middle()),
            Self::DomainSuffixIsOneOf(x) => x.contains(task_state.url.domain_suffix()),
            Self::DomainOriginIsOneOf(x) => x.contains(task_state.url.domain_origin()),
            Self::DomainNormalIsOneOf(x) => x.contains(task_state.url.domain_normal()),

            Self::DomainSegmentIsOneOf       {index, values} => values.contains(task_state.url.domain_segment       (*index)),
            Self::DomainPrefixSegmentIsOneOf {index, values} => values.contains(task_state.url.domain_prefix_segment(*index)),
            Self::DomainSuffixSegmentIsOneOf {index, values} => values.contains(task_state.url.domain_suffix_segment(*index)),
            Self::DomainOriginSegmentIsOneOf {index, values} => values.contains(task_state.url.domain_origin_segment(*index)),

            // Host is in set

            Self::HostIsInSet        (set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.host_str     ()),
            Self::DomainPrefixIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_prefix()),
            Self::DomainMiddleIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_middle()),
            Self::DomainSuffixIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix()),
            Self::DomainOriginIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_origin()),
            Self::DomainNormalIsInSet(set) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_normal()),

            Self::DomainSegmentIsInSet       {index, set} => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_segment       (*index)),
            Self::DomainPrefixSegmentIsInSet {index, set} => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_prefix_segment(*index)),
            Self::DomainSuffixSegmentIsInSet {index, set} => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix_segment(*index)),
            Self::DomainOriginSegmentIsInSet {index, set} => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_origin_segment(*index)),

            // Misc. host

            Self::UrlHasHost   => task_state.url.has_host(),
            Self::HostIsDomain => task_state.url.host_details().is_some_and(HostDetails::is_domain),
            Self::HostIsIp     => task_state.url.host_details().is_some_and(HostDetails::is_ip    ),
            Self::HostIsIpv4   => task_state.url.host_details().is_some_and(HostDetails::is_ipv4  ),
            Self::HostIsIpv6   => task_state.url.host_details().is_some_and(HostDetails::is_ipv6  ),
            Self::HostIsOpaque => task_state.url.host_details().is_some_and(HostDetails::is_opaque),
            Self::HostIsEmpty  => task_state.url.host_details().is_some_and(HostDetails::is_empty ),

            Self::HostIsLoopbackIp    => task_state.url.ip_details().is_some_and(IpDetails::is_loopback   ),
            Self::HostIsMulticastIp   => task_state.url.ip_details().is_some_and(IpDetails::is_multicast  ),
            Self::HostIsUnspecifiedIp => task_state.url.ip_details().is_some_and(IpDetails::is_unspecified),

            Self::HostIsBroadcastIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_broadcast    ),
            Self::HostIsDocumentationIpv4 => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_documentation),
            Self::HostIsLinkLocalIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_link_local   ),
            Self::HostIsLoopbackIpv4      => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_loopback     ),
            Self::HostIsMulticastIpv4     => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_multicast    ),
            Self::HostIsPrivateIpv4       => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_private      ),
            Self::HostIsUnspecifiedIpv4   => task_state.url.ipv4_details().is_some_and(Ipv4Details::is_unspecified  ),

            Self::HostIsLoopbackIpv6         => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_loopback          ),
            Self::HostIsUnicastLinkLocalIpv6 => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unicast_link_local),
            Self::HostIsMulticastIpv6        => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_multicast         ),
            Self::HostIsUniqueLocalIpv6      => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unique_local      ),
            Self::HostIsUnspecifiedIpv6      => task_state.url.ipv6_details().is_some_and(Ipv6Details::is_unspecified       ),

            // Path

            Self::PathIs        (value    ) => Some(task_state.url.path_str()) == get_option_str!(value),
            Self::PathStartsWith(value    ) => task_state.url.path_str().starts_with(get_str!(value)),
            Self::PathEndsWith  (value    ) => task_state.url.path_str().ends_with  (get_str!(value)),
            Self::PathIsOneOf   (values   ) => values.contains_some(task_state.url.path_str()),
            Self::PathIsInSet   (set      ) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains_some(task_state.url.path_str()),
            Self::PathContains  {value, at} => at.check(task_state.url.path_str(), get_str!(value))?,
            Self::PathMatches   (matcher  ) => matcher.check(Some(task_state.url.path_str()), task_state)?,

            Self::PathSegmentIs        {index, value    } => task_state.url.path_segment(*index).map(PathSegment::decode) == get_option_cow!(value),
            Self::PathSegmentStartsWith{index, value    } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode().starts_with(get_str!(value)),
            Self::PathSegmentEndsWith  {index, value    } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode().ends_with  (get_str!(value)),
            Self::PathSegmentIsOneOf   {index, values   } => values.contains(task_state.url.path_segment_str(*index)),
            Self::PathSegmentIsInSet   {index, set      } => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.path_segment(*index).map(PathSegment::decode).as_deref()),
            Self::PathSegmentContains  {index, value, at} => at.check(&task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?.decode(), get_str!(value))?,
            Self::PathSegmentMatches   {index, matcher  } => matcher.check(task_state.url.path_segment(*index).map(PathSegment::decode).as_deref(), task_state)?,

            Self::PathIsSegmented       => task_state.url.path_is_segmented(),
            Self::HasPathSegment(index) => task_state.url.path_segment(*index).is_some(),

            // Query

            Self::QueryIs(value) => task_state.url.query_str() == get_option_str!(value),

            Self::HasQueryParam(QueryParamSelector {name, index}) => task_state.url.query_param(name, *index).is_some(),

            Self::QueryParamIs {param: QueryParamSelector {name, index}, value} => {
                let value = get_option_cow!(value);
                task_state.url.query_param(name, *index).is_some_and(|p| p.value() == value)
            },

            Self::QueryParamIsOneOf {param: QueryParamSelector {name, index}, values} => task_state.url.query_param(name, *index).is_some_and(|p| values.contains(p.value().as_deref())),
            Self::QueryParamIsInSet {param: QueryParamSelector {name, index}, set   } => {
                let set = task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?;
                task_state.url.query_param(name, *index).is_some_and(|p| set.contains(p.value().as_deref()))
            },

            // Fragment

            Self::FragmentIs                 (value ) => task_state.url.fragment_str() == get_option_str!(value),
            Self::FragmentIsOneOf            (values) => values.contains(task_state.url.fragment_str()),
            Self::FragmentIsInSet            (set   ) => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.fragment_str()),
            Self::FragmentIsSomeAndStartsWith(value ) => match task_state.url.fragment_str() {
                Some(fragment) => fragment.starts_with(get_str!(value)),
                None => false
            },

            // General parts

            Self::PartIs {part, value} => part.get(&task_state.url) == get_option_cow!(value),

            Self::PartContains {part, value, at} => at.check(&part.get(&task_state.url).ok_or(ConditionError::UrlPartIsNone)?, get_str!(value))?,
            Self::PartIsSomeAndContains {part, value, at} => if let Some(x) = part.get(&task_state.url) {
                at.check(&x, get_str!(value))?
            } else {
                false
            },

            Self::PartMatches {part, matcher} => matcher.check   (part.get(&task_state.url).as_deref(), task_state)?,
            Self::PartIsOneOf {part, values } => values .contains(part.get(&task_state.url).as_deref()),
            Self::PartIsInSet {part, set    } => task_state.job.cleaner.params.sets.get(get_str!(set)).ok_or(ConditionError::SetNotFound)?.contains(part.get(&task_state.url).as_deref()),

            // Misc

            Self::Function(call) => {
                let func = task_state.job.cleaner.functions.conditions.get(&call.name).ok_or(ConditionError::FunctionNotFound)?;
                let old_args = task_state.call_args.replace(Some(&call.args));
                let ret = func.check(task_state);
                task_state.call_args.replace(old_args);
                ret?
            },
            Self::CallArg(name) => task_state.call_args.get().ok_or(ConditionError::NotInFunction)?
                .conditions.get(get_str!(name)).ok_or(ConditionError::CallArgFunctionNotFound)?
                .check(task_state)?,
            Self::Custom(function) => function(task_state)?
        }))
    }
}

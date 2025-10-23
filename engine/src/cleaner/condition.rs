//! [`Condition`].

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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::Always.check(&task_state).unwrap());
    /// ```
    Always,
    /// Never satisfied.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com");
    ///
    /// assert!(!Condition::Never.check(&task_state).unwrap());
    /// ```
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com");
    ///
    /// Condition::Error("...".into()).check(&task_state).unwrap_err();
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com");
    ///
    /// assert!( Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Always))}.check(&task_state).unwrap());
    /// assert!( Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Never ))}.check(&task_state).unwrap());
    /// assert!(!Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Always))}.check(&task_state).unwrap());
    /// assert!(!Condition::If {r#if: Box::new(Condition::Always), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Never ))}.check(&task_state).unwrap());
    /// assert!( Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Always))}.check(&task_state).unwrap());
    /// assert!(!Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Always), r#else: Some(Box::new(Condition::Never ))}.check(&task_state).unwrap());
    /// assert!( Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Always))}.check(&task_state).unwrap());
    /// assert!(!Condition::If {r#if: Box::new(Condition::Never ), then: Box::new(Condition::Never ), r#else: Some(Box::new(Condition::Never ))}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!(!Condition::Not(Box::new(Condition::Always)).check(&task_state).unwrap());
    /// assert!( Condition::Not(Box::new(Condition::Never )).check(&task_state).unwrap());
    /// ```
    Not(Box<Self>),
    /// Satisfied if all contained [`Self`]s are satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!(!Condition::All(vec![Condition::Never , Condition::Never ]).check(&task_state).unwrap());
    /// assert!(!Condition::All(vec![Condition::Never , Condition::Always]).check(&task_state).unwrap());
    /// assert!(!Condition::All(vec![Condition::Always, Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::All(vec![Condition::Always, Condition::Always]).check(&task_state).unwrap());
    /// ```
    All(Vec<Self>),
    /// Satisfied if any contained [`Self`] is satisfied.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!(!Condition::Any(vec![Condition::Never , Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Never , Condition::Always]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Always, Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Always, Condition::Always]).check(&task_state).unwrap());
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
        map: Map<Self>
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
        map: Map<Self>
    },
    /// Gets the name of the partition [`Self::PartNamedPartitioning::part`] is in in the specified [`NamedPartitioning`], indexes [`Self::PartNamedPartitioning::map`] with the partition name, and if the [`Map`] has a [`Self`] there, applies it.
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2), getnone(StringSource, Condition, 2), notfound(NamedPartitioning, Condition), checkerr(Self))]
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
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), notfound(NamedPartitioning, Condition), checkerr(Self))]
    StringNamedPartitioning {
        /// The [`NamedPartitioning`] to search in.
        named_partitioning: StringSource,
        /// The [`StringSource`] whose value to find in the [`NamedPartitioning`].
        value: StringSource,
        /// The [`Map`] to index.
        #[serde(flatten)]
        map: Map<Self>
    },

    // Params

    /// Satisfied if the specified flag is set.
    /// # Errors
    #[doc = edoc!(geterr(FlagRef))]
    FlagIsSet(FlagRef),
    /// Satisfied if the specified flag is not set.
    /// # Errors
    #[doc = edoc!(geterr(FlagRef))]
    FlagIsNotSet(FlagRef),
    /// Satisfied if [`Self::VarIs::var`] is [`Self::VarIs::value`].
    /// # Errors
    #[doc = edoc!(geterr(VarRef), geterr(StringSource))]
    VarIs {
        /// The var to check the value of.
        #[serde(flatten)]
        var: VarRef,
        /// The value to check if [`Self::VarIs::var`] is.
        value: StringSource
    },

    // String source

    /// Satisfied if [`Self::StringIs::left`] is [`Self::StringIs::right`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!( Condition::StringIs {left: "a".into(), right: "a".into()}.check(&task_state).unwrap());
    /// assert!(!Condition::StringIs {left: "a".into(), right: "b".into()}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!( Condition::StringIsSome("abc"       .into()).check(&task_state).unwrap());
    /// assert!(!Condition::StringIsSome(None::<&str>.into()).check(&task_state).unwrap());
    /// ```
    StringIsSome(StringSource),
    /// Satisfied if [`Self::StringContains::value`] contains [`Self::StringContains::substring`] at [`Self::StringContains::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), checkerr(StringLocation))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!(Condition::StringContains {value: "abc".into(), substring: "b".into(), at: StringLocation::Anywhere}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// assert!(Condition::StringMatches {value: "abc".into(), matcher: StringMatcher::Always}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::UrlIs("https://example.com/".into()).check(&task_state).unwrap());
    /// ```
    UrlIs(StringSource),

    // Scheme

    /// Satisfied if the value of [`Url::scheme`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SchemeIs(StringSource),
    /// Satisfied if the [`Url::scheme`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::SchemeIsOneOf(
    ///     [
    ///         "http".to_string(),
    ///         "https".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "http://example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com"); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "other://example.com"); assert!(!condition.check(&ts).unwrap());
    /// ```
    SchemeIsOneOf(Set<String>),
    /// Satisfied if the [`Url::scheme`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    SchemeIsInSet(#[suitable(assert = "set_is_documented")] StringSource),

    // Host is

    /// Satisfied if the value of [`Url::host`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::HostIs("example.com".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    HostIs(StringSource),
    /// Satisfied if the [`BetterUrl::normalized_host`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::NormalizedHostIs("example.com".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    NormalizedHostIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::subdomain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::SubdomainIs("www".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    SubdomainIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::reg_domain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::RegDomainIs("example.com".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    RegDomainIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainIs("example.com".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_middle`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainMiddleIs("example".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainMiddleIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::not_domain_suffix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::NotDomainSuffixIs("www.example".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    NotDomainSuffixIs(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_suffix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainSuffixIs("com".into());
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainSuffixIs(StringSource),



    /// Satisfied if the [`BetterUrl::subdomain_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentIs {
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

    // Host starts with

    /// Satisfied if the value of [`Url::host`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    HostStartsWith(StringSource),
    /// Satisfied if the [`BetterUrl::normalized_host`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    NormalizedHostStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::subdomain`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::reg_domain`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    RegDomainStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_middle`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainMiddleStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::not_domain_suffix`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    NotDomainSuffixStartsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_suffix`] starts with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixStartsWith(StringSource),



    /// Satisfied if the [`BetterUrl::subdomain_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_segment`] starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentStartsWith {
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

    // Host ends with

    /// Satisfied if the value of [`Url::host`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    HostEndsWith(StringSource),
    /// Satisfied if the [`BetterUrl::normalized_host`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    NormalizedHostEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::subdomain`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::reg_domain`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    RegDomainEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_middle`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainMiddleEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::not_domain_suffix`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    NotDomainSuffixEndsWith(StringSource),
    /// Satisfied if the value of [`BetterUrl::domain_suffix`] ends with the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixEndsWith(StringSource),



    /// Satisfied if the [`BetterUrl::subdomain_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainSegmentEndsWith {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::domain_segment`] ends with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentEndsWith {
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::HostIsOneOf(
    ///     [
    ///         "example.com".to_string(),
    ///         "www.example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    HostIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::normalized_host`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::NormalizedHostIsOneOf(
    ///     [
    ///         "example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    NormalizedHostIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::subdomain`] is contained in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::SubdomainIsOneOf(
    ///     [
    ///         "www".to_string(),
    ///         "abc".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    SubdomainIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::reg_domain`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::RegDomainIsOneOf(
    ///     [
    ///         "example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    RegDomainIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainIsOneOf(
    ///     [
    ///         "example.com".to_string(),
    ///         "abc.example.com".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_middle`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainMiddleIsOneOf(
    ///     [
    ///         "example".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainMiddleIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::not_domain_suffix`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::NotDomainSuffixIsOneOf(
    ///     [
    ///         "example".to_string(),
    ///         "abc.example".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    NotDomainSuffixIsOneOf(Set<String>),
    /// Satisfied if the [`BetterUrl::domain_suffix`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let condition = Condition::DomainSuffixIsOneOf(
    ///     [
    ///         "com".to_string()
    ///     ].into()
    /// );
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!condition.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!condition.check(&ts).unwrap());
    /// ```
    DomainSuffixIsOneOf(Set<String>),



    /// Satisfied if the [`BetterUrl::subdomain_segment`] is in the specified [`Set`].
    SubdomainSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::domain_segment`] is in the specified [`Set`].
    DomainSegmentIsOneOf {
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
    /// Satisfied if the [`BetterUrl::normalized_host`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    NormalizedHostIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::subdomain`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    SubdomainIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::reg_domain`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    RegDomainIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_middle`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainMiddleIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::not_domain_suffix`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    NotDomainSuffixIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`BetterUrl::domain_suffix`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    DomainSuffixIsInSet(#[suitable(assert = "set_is_documented")] StringSource),



    /// Satisfied if the [`BetterUrl::subdomain_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    SubdomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
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

    /// Satisfied if the [`Url::host`] is [`Some`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(Condition::UrlHasHost.check(&ts).unwrap());
    /// ```
    UrlHasHost,
    /// Satisfied if the URL's host is a fully qualified domain name.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!Condition::HostIsFqdn.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!Condition::HostIsFqdn.check(&ts).unwrap());
    /// ```
    HostIsFqdn,
    /// Satisfied if the URL's host is a domain.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!( Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!Condition::HostIsDomain.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!Condition::HostIsDomain.check(&ts).unwrap());
    /// ```
    HostIsDomain,
    /// Satisfied if the URL's host is an IP address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!( Condition::HostIsIp.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!( Condition::HostIsIp.check(&ts).unwrap());
    /// ```
    HostIsIp,
    /// Satisfied if the URL's host is an IPv4 address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!( Condition::HostIsIpv4.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!(!Condition::HostIsIpv4.check(&ts).unwrap());
    /// ```
    HostIsIpv4,
    /// Satisfied if the URL's host is an IPv6 address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(ts, url = "https://example.com"     ); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://example.com."    ); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com" ); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://www.example.com."); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com" ); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://abc.example.com."); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://127.0.0.1"       ); assert!(!Condition::HostIsIpv6.check(&ts).unwrap());
    /// tsv!(ts, url = "https://[::1]"           ); assert!( Condition::HostIsIpv6.check(&ts).unwrap());
    /// ```
    HostIsIpv6,

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
    /// [`Ipv6Details::is_multicast`].
    HostIsMulticastIpv6,
    /// [`Ipv6Details::is_unspecified`].
    HostIsUnspecifiedIpv6,

    // Path

    /// Satisfied if the [`Url::path`] is the specified value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/a/b/c");
    ///
    /// assert!( Condition::PathIs("/a/b/c" .into()).check(&task_state).unwrap());
    /// assert!(!Condition::PathIs("/a/b/c/".into()).check(&task_state).unwrap());
    /// ```
    PathIs(StringSource),
    /// Satisfied if the [`Url::path`] is in the specified [`Set`].
    PathIsOneOf(Set<String>),
    /// Satisfied if the [`Url::path`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    PathIsInSet(#[suitable(assert = "set_is_documented")] StringSource),
    /// Satisfied if the [`Url::path`] starts with the specified value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/a/b/c");
    ///
    /// assert!( Condition::PathStartsWith(""        .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/"       .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/a"      .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/a/"     .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/a/b"    .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/a/b/"   .into()).check(&task_state).unwrap());
    /// assert!( Condition::PathStartsWith("/a/b/c"  .into()).check(&task_state).unwrap());
    /// assert!(!Condition::PathStartsWith("/a/b/c/" .into()).check(&task_state).unwrap());
    /// assert!(!Condition::PathStartsWith("/a/b/c/d".into()).check(&task_state).unwrap());
    /// ```
    PathStartsWith(StringSource),
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::first_n_path_segments), geterr(StringSource))]
    FirstNPathSegmentsIs {
        /// The number of path segments to get.
        n: usize,
        /// The value to check if it's equal to.
        value: StringSource
    },
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::first_n_path_segments))]
    FirstNPathSegmentsIsOneOf {
        /// The number of path segments to get.
        n: usize,
        /// The [`Set`] to check if it's in.
        set: Set<String>
    },
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::first_n_path_segments), geterr(StringSource), notfound(Set, Condition))]
    FirstNPathSegmentsIsInSet {
        /// The number of path segments to get.
        n: usize,
        /// The name of the [`Params::sets`] [`Set`] to check if it's in.
        set: StringSource
    },
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::last_n_path_segments), geterr(StringSource))]
    LastNPathSegmentsIs {
        /// The number of path segments to get.
        n: usize,
        /// The value to check if it's equal to.
        value: StringSource
    },
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::last_n_path_segments))]
    LastNPathSegmentsIsOneOf {
        /// The number of path segments to get.
        n: usize,
        /// The [`Set`] to check if it's in.
        set: Set<String>
    },
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::last_n_path_segments), geterr(StringSource), notfound(Set, Condition))]
    LastNPathSegmentsIsInSet {
        /// The number of path segments to get.
        n: usize,
        /// The name of the [`Params::sets`] [`Set`] to check if it's in.
        set: StringSource
    },



    /// Satisfied if the [`Url::path`] has segments.
    PathHasSegments,
    /// Satisfied if the call to [`BetterUrl::path_segment`] returns [`Ok`] of [`Some`].
    ///
    /// Unsatisfied if [`BetterUrl::path_segment`] returns [`Err`] because not having path segments means it doesn't have the specified path segment.
    HasPathSegment(isize),
    /// Satisfied if the [`BetterUrl::path_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segment))]
    PathSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::path_segment`] is in the specified [`Set`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segment))]
    PathSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::path_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    /// If the call to [`BetterUrl::path_segment`] returns [`None`], returns the error [`ConditionError::PathDoesNotHaveSegments`].
    ///
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), notfound(Set, Condition))]
    PathSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "set_is_documented")]
        set: StringSource
    },
    /// Satisfied if the [`BetterUrl::path_segment`] starts with [`Self::PathSegmentStartsWith::value`].
    /// # Errors
    #[doc = edoc!(callnone(BetterUrl::path_segment, ConditionError), geterr(StringSource), checkerr(StringLocation))]
    PathSegmentStartsWith {
        /// The segment to check.
        index: isize,
        /// The value to check for.
        value: StringSource
    },
    /// Passes if the [`BetterUrl::path_segment`] contains [`Self::PathSegmentContains::value`] at [`Self::PathSegmentContains::at`].
    /// # Errors
    /// If the call to [`BetterUrl::path_segment`] returns [`None`], returns the error [`ConditionError::PathDoesNotHaveSegments`].
    ///
    /// If the call to [`BetterUrl::path_segment`] returns [`Some`] of [`None`], returns the error [`ConditionError::PathSegmentNotFound`].
    ///
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, ConditionError), checkerr(StringLocation))]
    PathSegmentContains {
        /// The segment to check.
        index: isize,
        /// Where to look in the path segment for [`Self::PathSegmentContains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default, skip_serializing_if = "is_default")]
        at: StringLocation,
        /// The value to search for [`Self::PathSegmentContains::value`].
        value: StringSource
    },
    /// Passes if the [`BetterUrl::path_segment`] matches [`Self::PathSegmentMatches::matcher`].
    /// # Errors
    /// If the call to [`BetterUrl::path_segment`] returns [`None`], returns the error [`ConditionError::PathDoesNotHaveSegments`].
    ///
    /// If the call to [`BetterUrl::path_segment`] returns [`Some`] of [`None`], returns the error [`ConditionError::PathSegmentNotFound`].
    ///
    #[doc = edoc!(geterr(StringSource), checkerr(StringMatcher))]
    PathSegmentMatches {
        /// The segment to check.
        index: isize,
        /// The matcher to check if the path segment satisfies.
        matcher: StringMatcher
    },

    // Query

    /// Satisfied if the [`Url::query`] is the specified value.
    QueryIs(StringSource),
    /// Satisfied if the URL' has a query query and has a matching query parameter.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com?a=2&b=3");
    ///
    /// assert!( Condition::HasQueryParam("a".into()).check(&task_state).unwrap());
    /// assert!( Condition::HasQueryParam("b".into()).check(&task_state).unwrap());
    /// assert!(!Condition::HasQueryParam("c".into()).check(&task_state).unwrap());
    /// ```
    HasQueryParam(QueryParamSelector),
    /// Satisfied if the [`BetterUrl::query_param`] is the specified value.
    QueryParamIs {
        /// The query param to check.
        param: QueryParamSelector,
        /// The value to compare it to.
        value: StringSource
    },
    /// Satisfied if the [`BetterUrl::query_param`] is in the specified [`Set`].
    QueryParamIsOneOf {
        /// The query param to check.
        param: QueryParamSelector,
        /// The set to check it with.
        values: Set<String>
    },
    /// Satisfied if the [`BetterUrl::query_param`] is in the specified [`Params::sets`] [`Set`].
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/abc?a=2");
    ///
    /// assert!(Condition::PartIs {part: UrlPart::Host                  , value: "example.com".into()}.check(&task_state).unwrap());
    /// assert!(Condition::PartIs {part: UrlPart::Path                  , value: "/abc"       .into()}.check(&task_state).unwrap());
    /// assert!(Condition::PartIs {part: UrlPart::Query                 , value: "a=2"        .into()}.check(&task_state).unwrap());
    /// assert!(Condition::PartIs {part: UrlPart::QueryParam("a".into()), value: "2"          .into()}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/abc");
    ///
    /// assert!(Condition::PartContains {part: UrlPart::Path, value: "/ab".into(), at: StringLocation::Start}.check(&task_state).unwrap());
    /// Condition::PartContains {part: UrlPart::Fragment, value: "".into(), at: StringLocation::Start}.check(&task_state).unwrap_err();
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/abc");
    ///
    /// assert!(Condition::PartMatches {part: UrlPart::Path    , matcher: StringMatcher::Always}.check(&task_state).unwrap());
    /// assert!(Condition::PartMatches {part: UrlPart::Fragment, matcher: StringMatcher::Always}.check(&task_state).unwrap());
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
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, url = "https://example.com/abc");
    ///
    /// assert!(Condition::PartIsOneOf {part: UrlPart::Path    , values: [Some("/abc".into()), None].into()}.check(&task_state).unwrap());
    /// assert!(Condition::PartIsOneOf {part: UrlPart::Fragment, values: [Some("/abc".into()), None].into()}.check(&task_state).unwrap());
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

    /// Satisfied if the specified [`Self`] from [`TaskStateView::commons`]'s [`Commons::conditions`] is.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), commonnotfound(Self, Condition), callerr(CommonCallArgsConfig::make), checkerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, commons = Commons {
    ///     conditions: [("abc".into(), Condition::Always)].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert!(Condition::Common(CommonCall {name: Box::new("abc".into()), args: Default::default()}).check(&task_state).unwrap());
    /// ```
    Common(CommonCall),
    /// Gets a [`Self`] from [`TaskStateView::common_args`]'s [`CommonCallArgs::conditions`] and applies it.
    /// # Errors
    /// If [`TaskStateView::common_args`] is [`None`], returns the error [`ConditionError::NotInCommonContext`].
    ///
    #[doc = edoc!(commoncallargnotfound(Self, Condition), checkerr(Self))]
    CommonCallArg(StringSource),
    /// Calls the specified function and returns its value.
    ///
    /// Because this uses function pointers, this plays weirdly with [`PartialEq`]/[`Eq`].
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state);
    ///
    /// fn some_complex_operation(task_state: &TaskStateView) -> Result<bool, ConditionError> {
    ///     Ok(true)
    /// }
    ///
    /// assert!(Condition::Custom(some_complex_operation).check(&task_state).unwrap());
    /// ```
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&TaskStateView) -> Result<bool, ConditionError>)
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

    /// Returned when a [`StringSource`] returned [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),

    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A part of the URL is None where it had to be Some.")]
    UrlPartIsNone,
    /// Returned when the Host is [`None`] where it has to be [`Some`].
    #[error("The Host was None where it had to be Some.")]
    HostIsNone,
    /// Returned when the NormalizedHost is [`None`] where it has to be [`Some`].
    #[error("The NormalizedHost was None where it had to be Some.")]
    NormalizedHostIsNone,
    /// Returned when the Subdomain is [`None`] where it has to be [`Some`].
    #[error("The Subdomain was None where it had to be Some.")]
    SubdomainIsNone,
    /// Returned when the RegDomain is [`None`] where it has to be [`Some`].
    #[error("The RegDomain was None where it had to be Some.")]
    RegDomainIsNone,
    /// Returned when the Domain is [`None`] where it has to be [`Some`].
    #[error("The Domain was None where it had to be Some.")]
    DomainIsNone,
    /// Returned when the DomainMiddle is [`None`] where it has to be [`Some`].
    #[error("The DomainMiddle was None where it had to be Some.")]
    DomainMiddleIsNone,
    /// Returned when the NotDomainSuffix is [`None`] where it has to be [`Some`].
    #[error("The NotDomainSuffix was None where it had to be Some.")]
    NotDomainSuffixIsNone,
    /// Returned when the DomainSuffix is [`None`] where it has to be [`Some`].
    #[error("The DomainSuffix was None where it had to be Some.")]
    DomainSuffixIsNone,
    /// Returned when the DomainSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainSegment was None where it had to be Some.")]
    DomainSegmentIsNone,
    /// Returned when the SubdomainSegment is [`None`] where it has to be [`Some`].
    #[error("The SubdomainSegment was None where it had to be Some.")]
    SubdomainSegmentIsNone,
    /// Returned when the DomainSuffixSegment is [`None`] where it has to be [`Some`].
    #[error("The DomainSuffixSegment was None where it had to be Some.")]
    DomainSuffixSegmentIsNone,
    /// Returned when attempting to get a segment/segments from a path with no segments.
    #[error("Attempted to get a segment/segments from a path with no segments.")]
    PathDoesNotHaveSegments,
    /// Returned when attempting to get more path segments than are available.
    #[error("Attempted to get more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to get a path segment not in a URL.
    #[error("Attempted to get a path segment not in the URL.")]
    PathSegmentNotFound,

    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),

    /// Returned when a [`GetFlagError`] is encountered.
    #[error(transparent)]
    GetFlagError(#[from] GetFlagError),
    /// Returned when a [`GetVarError`] is encountered.
    #[error(transparent)]
    GetVarError(#[from] GetVarError),

    /// Returned when a [`Set`] wasn't found.
    #[error("The requested set wasn't found.")]
    SetNotFound,
    /// Returned when a [`NamedPartitioning`] with the specified name isn't found.
    #[error("A NamedPartitioning with the specified name wasn't found.")]
    NamedPartitioningNotFound,

    /// Returned when a [`Condition`] with the specified name isn't found in the [`Commons::conditions`].
    #[error("A Condition with the specified name wasn't found in the Commons::conditions.")]
    CommonConditionNotFound,
    /// Returned when a [`MakeCommonCallArgsError`] is encountered.
    #[error(transparent)]
    MakeCommonCallArgsError(#[from] MakeCommonCallArgsError),

    /// Returned when trying to use [`Condition::CommonCallArg`] outside of a common context.
    #[error("Tried to use Condition::CommonCallArg outside of a common context.")]
    NotInCommonContext,
     /// Returned when the [`Condition`] requested from a [`Condition::CommonCallArg`] isn't found.
    #[error("The Condition requested from a Condition::CommonCallArg wasn't found.")]
    CommonCallArgConditionNotFound,
   /// An arbitrary [`std::error::Error`] returned by [`Condition::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl Condition {
    /// If the specified variant of [`Self`] is satisfied, return [`true`].
    ///
    /// If the specified variant of [`Self`] is unsatisfied, return [`false`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn check(&self, task_state: &TaskStateView) -> Result<bool, ConditionError> {
        debug!(Condition::check, self);
        Ok(match self {
            // Debug/constants

            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ConditionError::ExplicitError(msg.clone()))?,
            Self::Debug(condition) => {
                let is_satisfied = condition.check(task_state);
                eprintln!("=== Condition::Debug ===\nCondition: {condition:?}\ntask_state: {task_state:?}\nSatisfied?: {is_satisfied:?}");
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

            Self::PartMap  {part , map} => if let Some(condition) = map.get(part .get(task_state.url) ) {condition.check(task_state)?} else {false},
            Self::StringMap{value, map} => if let Some(condition) = map.get(value.get(task_state    )?) {condition.check(task_state)?} else {false},

            Self::PartNamedPartitioning   {named_partitioning, part , map} => if let Some(condition) = map.get(task_state.params.named_partitionings.get(get_str!(named_partitioning, task_state, ConditionError)).ok_or(ConditionError::NamedPartitioningNotFound)?.get_partition_of(part.get(task_state.url).as_deref())) {condition.check(task_state)?} else {false},
            Self::StringNamedPartitioning {named_partitioning, value, map} => if let Some(condition) = map.get(task_state.params.named_partitionings.get(get_str!(named_partitioning, task_state, ConditionError)).ok_or(ConditionError::NamedPartitioningNotFound)?.get_partition_of(get_option_str!(value, task_state)) ) {condition.check(task_state)?} else {false},

            // Params

            Self::FlagIsSet   (FlagRef {name: StringSource::String(name), r#type: FlagType::Params}) =>  task_state.params.flags.contains(name),
            Self::FlagIsNotSet(FlagRef {name: StringSource::String(name), r#type: FlagType::Params}) => !task_state.params.flags.contains(name),

            Self::FlagIsSet(flag)    =>  flag.get(task_state)?,
            Self::FlagIsNotSet(flag) => !flag.get(task_state)?,

            Self::VarIs {var, value} => var.get(task_state)?.as_deref() == value.get(task_state)?.as_deref(),

            // String source

            Self::StringIs {left, right} => get_option_cow!(left, task_state) == get_option_cow!(right, task_state),
            Self::StringIsSome(value) => value.get(task_state)?.is_some(),
            Self::StringContains {value, substring, at} => at.check(get_str!(value, task_state, ConditionError), get_str!(substring, task_state, ConditionError))?,
            Self::StringMatches {value, matcher} => matcher.check(get_option_str!(value, task_state), task_state)?,

            // Whole

            Self::UrlIs(value) => task_state.url == get_str!(value, task_state, ConditionError),

            // Scheme

            Self::SchemeIs(value) => task_state.url.scheme() == get_str!(value, task_state, ConditionError),
            Self::SchemeIsOneOf(values) => values.contains(Some(task_state.url.scheme())),
            Self::SchemeIsInSet(set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(Some(task_state.url.scheme())),

            // Host is

            Self::HostIs           (x) => task_state.url.host_str         () == get_option_str!(x, task_state),
            Self::NormalizedHostIs (x) => task_state.url.normalized_host  () == get_option_str!(x, task_state),
            Self::SubdomainIs      (x) => task_state.url.subdomain        () == get_option_str!(x, task_state),
            Self::RegDomainIs      (x) => task_state.url.reg_domain       () == get_option_str!(x, task_state),
            Self::DomainIs         (x) => task_state.url.domain           () == get_option_str!(x, task_state),
            Self::DomainMiddleIs   (x) => task_state.url.domain_middle    () == get_option_str!(x, task_state),
            Self::NotDomainSuffixIs(x) => task_state.url.not_domain_suffix() == get_option_str!(x, task_state),
            Self::DomainSuffixIs   (x) => task_state.url.domain_suffix    () == get_option_str!(x, task_state),

            Self::DomainSegmentIs       {index, value} => task_state.url.domain_segment       (*index) == get_option_str!(value, task_state),
            Self::SubdomainSegmentIs    {index, value} => task_state.url.subdomain_segment    (*index) == get_option_str!(value, task_state),
            Self::DomainSuffixSegmentIs {index, value} => task_state.url.domain_suffix_segment(*index) == get_option_str!(value, task_state),

            // Host starts with

            Self::HostStartsWith           (x) => task_state.url.host_str         ().ok_or(ConditionError::HostIsNone           )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::NormalizedHostStartsWith (x) => task_state.url.normalized_host  ().ok_or(ConditionError::NormalizedHostIsNone )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::SubdomainStartsWith      (x) => task_state.url.subdomain        ().ok_or(ConditionError::SubdomainIsNone      )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::RegDomainStartsWith      (x) => task_state.url.reg_domain       ().ok_or(ConditionError::RegDomainIsNone      )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::DomainStartsWith         (x) => task_state.url.domain           ().ok_or(ConditionError::DomainIsNone         )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::DomainMiddleStartsWith   (x) => task_state.url.domain_middle    ().ok_or(ConditionError::DomainMiddleIsNone   )?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::NotDomainSuffixStartsWith(x) => task_state.url.not_domain_suffix().ok_or(ConditionError::NotDomainSuffixIsNone)?.starts_with(get_str!(x, task_state, ConditionError)),
            Self::DomainSuffixStartsWith   (x) => task_state.url.domain_suffix    ().ok_or(ConditionError::DomainSuffixIsNone   )?.starts_with(get_str!(x, task_state, ConditionError)),

            Self::DomainSegmentStartsWith       {index, value} => task_state.url.domain_segment       (*index).ok_or(ConditionError::DomainSegmentIsNone      )?.starts_with(get_str!(value, task_state, ConditionError)),
            Self::SubdomainSegmentStartsWith    {index, value} => task_state.url.subdomain_segment    (*index).ok_or(ConditionError::SubdomainSegmentIsNone   )?.starts_with(get_str!(value, task_state, ConditionError)),
            Self::DomainSuffixSegmentStartsWith {index, value} => task_state.url.domain_suffix_segment(*index).ok_or(ConditionError::DomainSuffixSegmentIsNone)?.starts_with(get_str!(value, task_state, ConditionError)),

            // Host ends with

            Self::HostEndsWith           (x) => task_state.url.host_str         ().ok_or(ConditionError::HostIsNone             )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::NormalizedHostEndsWith (x) => task_state.url.normalized_host  ().ok_or(ConditionError::NormalizedHostIsNone   )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::SubdomainEndsWith      (x) => task_state.url.subdomain        ().ok_or(ConditionError::SubdomainIsNone        )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::RegDomainEndsWith      (x) => task_state.url.reg_domain       ().ok_or(ConditionError::RegDomainIsNone        )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::DomainEndsWith         (x) => task_state.url.domain           ().ok_or(ConditionError::DomainIsNone           )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::DomainMiddleEndsWith   (x) => task_state.url.domain_middle    ().ok_or(ConditionError::DomainMiddleIsNone     )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::NotDomainSuffixEndsWith(x) => task_state.url.not_domain_suffix().ok_or(ConditionError::NotDomainSuffixIsNone  )?.ends_with(get_str!(x, task_state, ConditionError)),
            Self::DomainSuffixEndsWith   (x) => task_state.url.domain_suffix    ().ok_or(ConditionError::DomainSuffixIsNone     )?.ends_with(get_str!(x, task_state, ConditionError)),

            Self::DomainSegmentEndsWith       {index, value} => task_state.url.domain_segment       (*index).ok_or(ConditionError::DomainSegmentIsNone      )?.ends_with(get_str!(value, task_state, ConditionError)),
            Self::SubdomainSegmentEndsWith    {index, value} => task_state.url.subdomain_segment    (*index).ok_or(ConditionError::SubdomainSegmentIsNone   )?.ends_with(get_str!(value, task_state, ConditionError)),
            Self::DomainSuffixSegmentEndsWith {index, value} => task_state.url.domain_suffix_segment(*index).ok_or(ConditionError::DomainSuffixSegmentIsNone)?.ends_with(get_str!(value, task_state, ConditionError)),

            // Host is one of

            Self::HostIsOneOf           (x) => x.contains(task_state.url.host_str         ()),
            Self::NormalizedHostIsOneOf (x) => x.contains(task_state.url.normalized_host  ()),
            Self::SubdomainIsOneOf      (x) => x.contains(task_state.url.subdomain        ()),
            Self::RegDomainIsOneOf      (x) => x.contains(task_state.url.reg_domain       ()),
            Self::DomainIsOneOf         (x) => x.contains(task_state.url.domain           ()),
            Self::DomainMiddleIsOneOf   (x) => x.contains(task_state.url.domain_middle    ()),
            Self::NotDomainSuffixIsOneOf(x) => x.contains(task_state.url.not_domain_suffix()),
            Self::DomainSuffixIsOneOf   (x) => x.contains(task_state.url.domain_suffix    ()),

            Self::DomainSegmentIsOneOf       {index, values} => values.contains(task_state.url.domain_segment       (*index)),
            Self::SubdomainSegmentIsOneOf    {index, values} => values.contains(task_state.url.subdomain_segment    (*index)),
            Self::DomainSuffixSegmentIsOneOf {index, values} => values.contains(task_state.url.domain_suffix_segment(*index)),

            // Host is in set

            Self::HostIsInSet           (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.host_str         ()),
            Self::NormalizedHostIsInSet (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.normalized_host  ()),
            Self::SubdomainIsInSet      (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.subdomain        ()),
            Self::RegDomainIsInSet      (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.reg_domain       ()),
            Self::DomainIsInSet         (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain           ()),
            Self::DomainMiddleIsInSet   (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_middle    ()),
            Self::NotDomainSuffixIsInSet(set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.not_domain_suffix()),
            Self::DomainSuffixIsInSet   (set) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix    ()),

            Self::DomainSegmentIsInSet       {index, set} => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_segment       (*index)),
            Self::SubdomainSegmentIsInSet    {index, set} => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.subdomain_segment    (*index)),
            Self::DomainSuffixSegmentIsInSet {index, set} => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix_segment(*index)),

            // Misc. host

            Self::UrlHasHost   => task_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(task_state.url.host_details(), Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..}))),
            Self::HostIsDomain => matches!(task_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            Self::HostIsLoopbackIp        => task_state.url.ip_details  ().is_some_and(|details| details.is_loopback     ()),
            Self::HostIsMulticastIp       => task_state.url.ip_details  ().is_some_and(|details| details.is_multicast    ()),
            Self::HostIsUnspecifiedIp     => task_state.url.ip_details  ().is_some_and(|details| details.is_unspecified  ()),

            Self::HostIsBroadcastIpv4     => task_state.url.ipv4_details().is_some_and(|details| details.is_broadcast    ()),
            Self::HostIsDocumentationIpv4 => task_state.url.ipv4_details().is_some_and(|details| details.is_documentation()),
            Self::HostIsLinkLocalIpv4     => task_state.url.ipv4_details().is_some_and(|details| details.is_link_local   ()),
            Self::HostIsLoopbackIpv4      => task_state.url.ipv4_details().is_some_and(|details| details.is_loopback     ()),
            Self::HostIsMulticastIpv4     => task_state.url.ipv4_details().is_some_and(|details| details.is_multicast    ()),
            Self::HostIsPrivateIpv4       => task_state.url.ipv4_details().is_some_and(|details| details.is_private      ()),
            Self::HostIsUnspecifiedIpv4   => task_state.url.ipv4_details().is_some_and(|details| details.is_unspecified  ()),

            Self::HostIsLoopbackIpv6      => task_state.url.ipv4_details().is_some_and(|details| details.is_loopback     ()),
            Self::HostIsMulticastIpv6     => task_state.url.ipv4_details().is_some_and(|details| details.is_multicast    ()),
            Self::HostIsUnspecifiedIpv6   => task_state.url.ipv4_details().is_some_and(|details| details.is_unspecified  ()),

            // Path

            Self::PathIs(value) => task_state.url.path() == get_str!(value, task_state, ConditionError),

            Self::PathIsOneOf   (values) => values.contains(Some(task_state.url.path())),
            Self::PathIsInSet   (set   ) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(Some(task_state.url.path())),
            Self::PathStartsWith(value ) => task_state.url.path().starts_with(get_str!(value, task_state, ConditionError)),
            Self::FirstNPathSegmentsIs      {n, value} => task_state.url.first_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)? == get_option_str!(value, task_state),
            Self::FirstNPathSegmentsIsOneOf {n, set  } => set.contains(task_state.url.first_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)?),
            Self::FirstNPathSegmentsIsInSet {n, set  } => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.first_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)?),
            Self::LastNPathSegmentsIs       {n, value} => task_state.url.last_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)? == get_option_str!(value, task_state),
            Self::LastNPathSegmentsIsOneOf  {n, set  } => set.contains(task_state.url.last_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)?),
            Self::LastNPathSegmentsIsInSet  {n, set  } => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.last_n_path_segments(*n).ok_or(ConditionError::NotEnoughPathSegments)?),

            Self::PathHasSegments => task_state.url.path_has_segments(),
            Self::HasPathSegment        (index           ) => task_state.url.path_segment(*index).is_some_and(|segment| segment.is_none()),
            Self::PathSegmentIs         {index, value    } => task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)? == get_option_str!(value, task_state),
            Self::PathSegmentStartsWith {index, value    } => task_state.url.path_segment(*index).ok_or(ConditionError::PathDoesNotHaveSegments)?.ok_or(ConditionError::PathSegmentNotFound)?.starts_with(get_str!(value, task_state, ConditionError)),
            Self::PathSegmentContains   {index, at, value} => at     .check(task_state.url.path_segment(*index).ok_or(ConditionError::PathDoesNotHaveSegments)?.ok_or(ConditionError::PathSegmentNotFound)?, get_str!(value, task_state, ConditionError))?,
            Self::PathSegmentMatches    {index, matcher  } => matcher.check(task_state.url.path_segment(*index).ok_or(ConditionError::PathDoesNotHaveSegments)?, task_state)?,

            Self::PathSegmentIsOneOf {index, values} => values.contains(task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?),
            Self::PathSegmentIsInSet {index, set} => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.path_segment(*index).ok_or(ConditionError::PathSegmentNotFound)?),

            // Query

            Self::QueryIs(value) => task_state.url.query() == get_option_str!(value, task_state),

            Self::HasQueryParam(QueryParamSelector {name, index}) => task_state.url.has_query_param(name, *index),

            Self::QueryParamIs {param: QueryParamSelector {name, index}, value } => task_state.url.query_param(name, *index).flatten().flatten() == get_option_cow!(value, task_state),

            Self::QueryParamIsOneOf {param: QueryParamSelector {name, index}, values} => values.contains(task_state.url.query_param(name, *index).flatten().flatten().as_deref()),
            Self::QueryParamIsInSet {param: QueryParamSelector {name, index}, set   } => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.query_param(name, *index).flatten().flatten().as_deref()),

            // Fragment

            Self::FragmentIs                 (value ) => task_state.url.fragment() == get_option_str!(value, task_state),
            Self::FragmentIsOneOf            (values) => values.contains(task_state.url.fragment()),
            Self::FragmentIsInSet            (set   ) => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.fragment()),
            Self::FragmentIsSomeAndStartsWith(value ) => match task_state.url.fragment() {
                Some(fragment) => fragment.starts_with(get_str!(value, task_state, ConditionError)),
                None => false
            },

            // General parts

            Self::PartIs {part, value} => part.get(task_state.url) == get_option_cow!(value, task_state),

            Self::PartContains {part, value, at} => at.check(&part.get(task_state.url).ok_or(ConditionError::UrlPartIsNone)?, get_str!(value, task_state, ConditionError))?,
            Self::PartIsSomeAndContains {part, value, at} => if let Some(x) = part.get(task_state.url) {
                at.check(&x, get_str!(value, task_state, ConditionError))?
            } else {
                false
            },

            Self::PartMatches {part, matcher} => matcher.check   (part.get(task_state.url).as_deref(), task_state)?,
            Self::PartIsOneOf {part, values } => values .contains(part.get(task_state.url).as_deref()),
            Self::PartIsInSet {part, set    } => task_state.params.sets.get(get_str!(set, task_state, ConditionError)).ok_or(ConditionError::SetNotFound)?.contains(part.get(task_state.url).as_deref()),

            // Misc

            Self::Common(common_call) => {
                task_state.commons.conditions.get(get_str!(common_call.name, task_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.check(&TaskStateView {
                    common_args : Some(&common_call.args.make(task_state)?),
                    url         : task_state.url,
                    scratchpad  : task_state.scratchpad,
                    context     : task_state.context,
                    job_context : task_state.job_context,
                    params      : task_state.params,
                    commons     : task_state.commons,
                    unthreader  : task_state.unthreader,
                    #[cfg(feature = "cache")]
                    cache_handle: task_state.cache_handle,
                    #[cfg(feature = "http")]
                    http_client : task_state.http_client
                })?
            },
            Self::CommonCallArg(name) => task_state.common_args.ok_or(ConditionError::NotInCommonContext)?.conditions.get(get_str!(name, task_state, ConditionError)).ok_or(ConditionError::CommonCallArgConditionNotFound)?.check(task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

//! Logic for when a [`TaskState`] should be modified.

use thiserror::Error;
use serde::{Serialize, Deserialize};
#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::types::*;
use crate::util::*;

/// Conditions that decide if and when to apply an [`Action`].
///
/// - "Pass" means [`Condition::check`] returns `Ok(true)` and "fail" means it returns `Ok(false)`.
///
/// - "*IsOneOf" variants should always be equivalent to a [`Self::Any`] with a respective "*Is" variant for each value in the [`Set`].
///
/// - "*IsInSet" variants should alwasy be equivalent to moving the [`Set`] from [`Params::sets`] to the respective "*IsOneOf" variant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Condition {
    // Debug/constants

    /// Always passes.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::Always.check(&task_state).unwrap());
    /// ```
    Always,
    /// Always fails.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(!Condition::Never.check(&task_state).unwrap());
    /// ```
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
    ///
    /// Condition::Error("...".into()).check(&task_state).unwrap_err();
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::check`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),

    // Error handling

    /// If the call to [`Self::check`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::check`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::check`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    #[doc = edoc!(checkerrte(Self, Condition))]
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try`] returns an error.
        r#else: Box<Self>
    },

    // Logic

    /// If the call to [`Self::If::if`] passes, return the value of [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails, return the value of [`Self::If::else`].
    /// # Errors
    #[doc = edoc!(checkerr(Self, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
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
        /// The [`Self`] to use if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] fails.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Self>>
    },
    /// If the call to [`Self::check`] passes or fails, invert it into failing or passing.
    /// # Errors
    #[doc = edoc!(checkerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert!(!Condition::Not(Box::new(Condition::Always)).check(&task_state).unwrap());
    /// assert!( Condition::Not(Box::new(Condition::Never )).check(&task_state).unwrap());
    /// ```
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert!(!Condition::All(vec![Condition::Never , Condition::Never ]).check(&task_state).unwrap());
    /// assert!(!Condition::All(vec![Condition::Never , Condition::Always]).check(&task_state).unwrap());
    /// assert!(!Condition::All(vec![Condition::Always, Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::All(vec![Condition::Always, Condition::Always]).check(&task_state).unwrap());
    /// ```
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    #[doc = edoc!(checkerr(Self, 3))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert!(!Condition::Any(vec![Condition::Never , Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Never , Condition::Always]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Always, Condition::Never ]).check(&task_state).unwrap());
    /// assert!( Condition::Any(vec![Condition::Always, Condition::Always]).check(&task_state).unwrap());
    /// ```
    Any(Vec<Self>),

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

    /// Passes if the specified flag is set.
    /// # Errors
    #[doc = edoc!(geterr(FlagRef))]
    FlagIsSet(FlagRef),
    /// Passes if the specified flag is not set.
    /// # Errors
    #[doc = edoc!(geterr(FlagRef))]
    FlagIsNotSet(FlagRef),
    /// Passes if [`Self::VarIs::var`] is [`Self::VarIs::value`].
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

    /// Passes if [`Self::StringIs::left`] is [`Self::StringIs::right`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource, 2))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// Passes if the specified [`StringSource`] is [`Some`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state);
    ///
    /// assert!( Condition::StringIsSome("abc"       .into()).check(&task_state).unwrap());
    /// assert!(!Condition::StringIsSome(None::<&str>.into()).check(&task_state).unwrap());
    /// ```
    StringIsSome(StringSource),
    /// Passes if [`Self::StringContains::value`] contains [`Self::StringContains::substring`] at [`Self::StringContains::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), checkerr(StringLocation))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state);
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
    /// Passes if [`Self::StringMatches::value`] satisfies [`Self::StringMatches::matcher`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state);
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

    /// Passes if the URL is the specified string.
    ///
    /// Used for testing.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::UrlIs("https://example.com/".into()).check(&task_state).unwrap());
    /// ```
    UrlIs(String),

    // Scheme

    /// Passes if the value of [`Url::scheme`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SchemeIs(StringSource),
    /// Passes if the [`Url::scheme`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`Url::scheme`] is in the specified [`Params::sets`] [`Set`].
    SchemeIsInSet(String),

    // Host is

    /// Passes if the value of [`Url::host`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::normalized_host`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::subdomain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::reg_domain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::domain`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::domain_middle`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::not_domain_suffix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the value of [`BetterUrl::domain_suffix`] is equal to the specified string.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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



    /// Passes if the [`BetterUrl::subdomain_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    SubdomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Passes if the [`BetterUrl::domain_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Passes if the [`BetterUrl::domain_suffix_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    DomainSuffixSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },

    // Host is one of

    /// Passes if the [`Url::host`] is contained in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::normalized_host`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::subdomain`] is contained in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::reg_domain`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::domain`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::domain_middle`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::not_domain_suffix`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the [`BetterUrl::domain_suffix`] is in the specified [`Set`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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



    /// Passes if the [`BetterUrl::subdomain_segment`] is in the specified [`Set`].
    SubdomainSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Passes if the [`BetterUrl::domain_segment`] is in the specified [`Set`].
    DomainSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Passes if the [`BetterUrl::domain_suffix_segment`] is in the specified [`Set`].
    DomainSuffixSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },

    // Host is in set

    /// Passes if the [`Url::host_str`] is in the specified [`Params::sets`] [`Set`].
    HostIsInSet(String),
    /// Passes if the [`BetterUrl::normalized_host`] is in the specified [`Params::sets`] [`Set`].
    NormalizedHostIsInSet(String),
    /// Passes if the [`BetterUrl::subdomain`] is in the specified [`Params::sets`] [`Set`].
    SubdomainIsInSet(String),
    /// Passes if the [`BetterUrl::reg_domain`] is in the specified [`Params::sets`] [`Set`].
    RegDomainIsInSet(String),
    /// Passes if the [`BetterUrl::domain`] is in the specified [`Params::sets`] [`Set`].
    DomainIsInSet(String),
    /// Passes if the [`BetterUrl::domain_middle`] is in the specified [`Params::sets`] [`Set`].
    DomainMiddleIsInSet(String),
    /// Passes if the [`BetterUrl::not_domain_suffix`] is in the specified [`Params::sets`] [`Set`].
    NotDomainSuffixIsInSet(String),
    /// Passes if the [`BetterUrl::domain_suffix`] is in the specified [`Params::sets`] [`Set`].
    DomainSuffixIsInSet(String),



    /// Passes if the [`BetterUrl::subdomain_segment`] is in the specified [`Params::sets`] [`Set`].
    SubdomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        set: String
    },
    /// Passes if the [`BetterUrl::domain_segment`] is in the specified [`Params::sets`] [`Set`].
    DomainSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        set: String
    },
    /// Passes if the [`BetterUrl::domain_suffix_segment`] is in the specified [`Params::sets`] [`Set`].
    DomainSuffixSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        set: String
    },

    // Misc. host

    /// Passes if the [`Url::host`] is [`Some`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the URL's host is a fully qualified domain name.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the URL's host is a domain.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the URL's host is an IP address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the URL's host is an IPv4 address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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
    /// Passes if the URL's host is an IPv6 address.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// use url_cleaner_engine::task_state_view as tsv;
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

    // Path

    /// Passes if the [`Url::path`] is the specified value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/a/b/c");
    /// assert!( Condition::PathStartsWith("/a/b/c" .into()).check(&task_state).unwrap());
    /// assert!(!Condition::PathStartsWith("/a/b/c/".into()).check(&task_state).unwrap());
    /// ```
    PathIs(StringSource),
    /// Passes if the [`Url::path`] is in the specified [`Set`].
    PathIsOneOf(Set<String>),
    /// Passes if the [`Url::path`] is in the specified [`Params::sets`] [`Set`].
    PathIsInSet(String),
    /// Passes if the [`Url::path`] starts with the specified value.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/a/b/c");
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
    PathStartsWith(String),



    /// Passes if the [`Url::path`] has segments.
    PathHasSegments,
    /// Passes if the call to [`BetterUrl::path_segment`] returns [`Ok`] of [`Some`].
    ///
    /// Fails instead of erroring when the call to [`BetterUrl::path_segment`] returns [`Err`] because not having path segments means it doesn't have the specified path segment.
    HasPathSegment(isize),
    /// Passes if the [`BetterUrl::path_segment`] is the specified value.
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segment))]
    PathSegmentIs {
        /// The segment to check.
        index: isize,
        /// The value to compare it to.
        value: StringSource
    },
    /// Passes if the [`BetterUrl::path_segment`] is in the specified [`Set`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segment))]
    PathSegmentIsOneOf {
        /// The segment to check.
        index: isize,
        /// The set to check it with.
        values: Set<String>
    },
    /// Passes if the [`BetterUrl::path_segment`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(callerr(BetterUrl::path_segment))]
    PathSegmentIsInSet {
        /// The segment to check.
        index: isize,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        set: String
    },

    // Query

    /// Passes if the [`Url::query`] is the specified value.
    QueryIs(StringSource),
    /// Passes if the URL' has a query query and has a matching query parameter.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com?a=2&b=3");
    /// assert!( Condition::HasQueryParam("a".into()).check(&task_state).unwrap());
    /// assert!( Condition::HasQueryParam("b".into()).check(&task_state).unwrap());
    /// assert!(!Condition::HasQueryParam("c".into()).check(&task_state).unwrap());
    /// ```
    HasQueryParam(QueryParamSelector),
    /// Passes if the [`BetterUrl::query_param`] is the specified value.
    QueryParamIs {
        /// The query param to check.
        param: QueryParamSelector,
        /// The value to compare it to.
        value: StringSource
    },
    /// Passes if the [`BetterUrl::query_param`] is in the specified [`Set`].
    QueryParamIsOneOf {
        /// The query param to check.
        param: QueryParamSelector,
        /// The set to check it with.
        values: Set<String>
    },
    /// Passes if the [`BetterUrl::query_param`] is in the specified [`Params::sets`] [`Set`].
    QueryParamIsInSet {
        /// The query param to check.
        param: QueryParamSelector,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        set: String
    },

    // Fragment

    /// Passes if the [`Url::fragment`] is the specified value.
    FragmentIs(StringSource),
    /// Passes if the [`Url::fragment`] is in the specified [`Set`].
    FragmentIsOneOf(Set<String>),
    /// Passes if the [`Url::fragment`] is in the specified [`Params::sets`] [`Set`].
    FragmentIsInSet(String),
    /// Passes if the [`Url::fragment`] is [`Some`] and starts with the specified value.
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    FragmentIsSomeAndStartsWith(StringSource),

    // General parts

    /// Passes if the value of [`Self::PartIs::part`] is the same as the value of [`Self::PartIs::value`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/abc?a=2");
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
    /// Passes if [`Self::PartContains::part`] contains [`Self::PartContains::value`] at [`Self::PartContains::at`].
    /// # Errors
    #[doc = edoc!(getnone(UrlPart, Condition), getnone(StringSource, Condition), checkerr(StringLocation))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/abc");
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
    /// Passes if [`Self::PartMatches::part`] satisfies [`Self::PartMatches::matcher`].
    /// # Errors
    #[doc = edoc!(getnone(UrlPart, Condition), checkerr(StringMatcher))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/abc");
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
    /// Passes if [`Self::PartIsOneOf::part`] is in [`Self::PartIsOneOf::values`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, url = "https://example.com/abc");
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
    /// Passes if [`Self::PartIsInSet`] is in the specified [`Params::sets`] [`Set`].
    /// # Errors
    #[doc = edoc!(notfound(Set, Condition))]
    PartIsInSet {
        /// The part to check the value of.
        part: UrlPart,
        /// The name of the [`Params::sets`] [`Set`] to check it with.
        #[suitable(assert = "lit_set_is_documented")]
        set: String
    },

    // Misc.

    /// Get a [`Self`] from [`TaskStateView::commons`]'s [`Commons::conditions`] and pass if it's satisfied.
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, Condition), commonnotfound(Self, Condition), callerr(CommonCallArgsSource::build), checkerr(Self))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, commons = Commons {
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
    /// # Errors
    #[doc = edoc!(callerr(Self::Custom::0))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state);
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

    /// Returned when a [`UrlDoesNotHavePathSegments`] is returned.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),

    /// Returned when a [`Condition`] with the specified name isn't found in the [`Commons::conditions`].
    #[error("A Condition with the specified name wasn't found in the Commons::conditions.")]
    CommonConditionNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered/
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),

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
    /// If the specified variant of [`Self`] passes, return [`true`], otherwise return [`false`].
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

            // Error handling

            Self::TreatErrorAsPass(condition) => condition.check(task_state).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.check(task_state).unwrap_or(false),
            Self::TryElse{ r#try, r#else } => match r#try.check(task_state) {
                Ok(x) => x,
                Err(try_error) => match r#else.check(task_state) {
                    Ok(x) => x,
                    Err(else_error) => Err(ConditionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                }
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

            Self::UrlIs(value) => task_state.url == value,

            // Scheme

            Self::SchemeIs(value) => task_state.url.scheme() == get_str!(value, task_state, ConditionError),
            Self::SchemeIsOneOf(values) => values.contains(Some(task_state.url.scheme())),
            Self::SchemeIsInSet(set) => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(Some(task_state.url.scheme())),

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

            Self::HostIsInSet           (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.host_str         ()),
            Self::NormalizedHostIsInSet (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.normalized_host  ()),
            Self::SubdomainIsInSet      (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.subdomain        ()),
            Self::RegDomainIsInSet      (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.reg_domain       ()),
            Self::DomainIsInSet         (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain           ()),
            Self::DomainMiddleIsInSet   (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_middle    ()),
            Self::NotDomainSuffixIsInSet(x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.not_domain_suffix()),
            Self::DomainSuffixIsInSet   (x) => task_state.params.sets.get(x).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix    ()),

            Self::DomainSegmentIsInSet       {index, set} => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_segment       (*index)),
            Self::SubdomainSegmentIsInSet    {index, set} => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.subdomain_segment    (*index)),
            Self::DomainSuffixSegmentIsInSet {index, set} => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.domain_suffix_segment(*index)),

            // Misc. host

            Self::UrlHasHost   => task_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(task_state.url.host_details(), Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..}))),
            Self::HostIsDomain => matches!(task_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            // Path

            Self::PathIs(value) => task_state.url.path() == get_str!(value, task_state, ConditionError),

            Self::PathIsOneOf   (values) => values.contains(Some(task_state.url.path())),
            Self::PathIsInSet   (set   ) => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(Some(task_state.url.path())),
            Self::PathStartsWith(value ) => task_state.url.path().starts_with(value),

            Self::PathHasSegments => task_state.url.path_has_segments(),
            Self::HasPathSegment(index) => task_state.url.path_segment(*index).is_ok_and(|segment| segment.is_none()),
            Self::PathSegmentIs {index, value                             } => task_state.url.path_segment(*index)? == get_option_str!(value, task_state),

            Self::PathSegmentIsOneOf {index, values} => values.contains(task_state.url.path_segment(*index)?),
            Self::PathSegmentIsInSet {index, set} => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.path_segment(*index)?),

            // Query

            Self::QueryIs(value) => task_state.url.query() == get_option_str!(value, task_state),

            Self::HasQueryParam(QueryParamSelector {name, index}) => task_state.url.has_query_param(name, *index),

            Self::QueryParamIs {param: QueryParamSelector {name, index}, value } => task_state.url.query_param(name, *index).flatten().flatten() == get_option_cow!(value, task_state),

            Self::QueryParamIsOneOf {param: QueryParamSelector {name, index}, values} => values.contains(task_state.url.query_param(name, *index).flatten().flatten().as_deref()),
            Self::QueryParamIsInSet {param: QueryParamSelector {name, index}, set   } => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.query_param(name, *index).flatten().flatten().as_deref()),

            // Fragment

            Self::FragmentIs                 (value ) => task_state.url.fragment() == get_option_str!(value, task_state),
            Self::FragmentIsOneOf            (values) => values.contains(task_state.url.fragment()),
            Self::FragmentIsInSet            (set   ) => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(task_state.url.fragment()),
            Self::FragmentIsSomeAndStartsWith(value ) => match task_state.url.fragment() {
                Some(fragment) => fragment.starts_with(get_str!(value, task_state, ConditionError)),
                None => false
            },

            // General parts

            Self::PartIs {part, value} => part.get(task_state.url) == get_option_cow!(value, task_state),

            Self::PartContains {part, value, at} => at.check(&part.get(task_state.url).ok_or(ConditionError::UrlPartIsNone)?, get_str!(value, task_state, ConditionError))?,

            Self::PartMatches {part, matcher} => matcher.check   (part.get(task_state.url).as_deref(), task_state)?,
            Self::PartIsOneOf {part, values } => values .contains(part.get(task_state.url).as_deref()),
            Self::PartIsInSet {part, set    } => task_state.params.sets.get(set).ok_or(ConditionError::SetNotFound)?.contains(part.get(task_state.url).as_deref()),

            // Misc

            Self::Common(common_call) => {
                task_state.commons.conditions.get(get_str!(common_call.name, task_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.check(&TaskStateView {
                    common_args: Some(&common_call.args.build(task_state)?),
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
            Self::CommonCallArg(name) => task_state.common_args.ok_or(ConditionError::NotInCommonContext)?.conditions.get(get_str!(name, task_state, ConditionError)).ok_or(ConditionError::CommonCallArgConditionNotFound)?.check(task_state)?,
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

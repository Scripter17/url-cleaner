//! The logic for when to modify a URL.

use std::collections::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// The part of a [`Rule`] that specifies when the rule's mapper will be applied.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Condition {
    // Debug/constants.

    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Condition::Error.satisfied_by(&job_state.to_view()).unwrap_err();
    /// ```
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    Debug(Box<Self>),

    // Logic.

    /// If `r#if` passes, return the result of `then`, otherwise return the value of `r#else`.
    /// # Errors
    /// If `r#if` returns an error, that error is returned.
    /// 
    /// If `r#if` passes and `then` returns an error, that error is returned.
    /// 
    /// If `r#if` fails and `r#else` returns an error, that error is returned.
    If {
        /// The [`Self`] that decides if `then` or `r#else` is used.
        r#if: Box<Self>,
        /// The [`Self`] to use if `r#if` passes.
        then: Box<Self>,
        /// The [`Self`] to use if `r#if` fails.
        r#else: Box<Self>
    },
    /// Passes if the included [`Self`] doesn't and vice-versa.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::Not(Box::new(Condition::Always)).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::Not(Box::new(Condition::Never )).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// Condition::Not(Box::new(Condition::Error )).satisfied_by(&job_state.to_view()).unwrap_err();
    /// ```
    Not(Box<Self>),
    /// Passes if all of the included [`Self`]s pass.
    /// Like [`Iterator::all`], an empty list passes.
    /// # Errors
    /// If any of the calls to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Always]).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Always]).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// Condition::All(vec![Condition::Always, Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Always]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// ```
    All(Vec<Self>),
    /// Passes if any of the included [`Self`]s pass.
    /// Like [`Iterator::any`], an empty list fails.
    /// # Errors
    /// If any of the calls to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Always]).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Always]).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// Condition::Any(vec![Condition::Never , Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Always]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Never ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Error ]).satisfied_by(&job_state.to_view()).unwrap_err();
    /// ```
    Any(Vec<Self>),
    /// Passes if the condition in `map` whose key is the value returned by `part`'s [`UrlPart::get`] passes.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    PartMap {
        /// The part to get.
        part: UrlPart,
        /// The map specifying which values should run which conditions.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Passes if the condition in `map` whose key is the value returned by `value`'s [`StringSource::get`] passes.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    StringMap {
        /// The string to index the map with.
        value: StringSource,
        /// The map specifying which values should run which conditions.
        #[serde(flatten)]
        map: Map<Self>
    },

    // Error handling.

    /// If the call to [`Self::satisfied_by`] returns an error, treat it as a pass.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Always)).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Never )).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Error )).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the call to [`Self::satisfied_by`] returns an error, treat it as a fail.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Always)).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Never )).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Error )).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    TreatErrorAsFail(Box<Self>),
    /// If `try` returns an error, `else` is executed.
    /// If `try` does not return an error, `else` is not executed.
    /// # Errors
    /// If `else` returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state.to_view()).unwrap_err();
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every call to [`Self::satisfied_by`] returns an error, returns the last error.
    FirstNotError(Vec<Self>),

    // Domain conditions.

    /// Passes if the URL's [`UrlPart::Host`] is the specified value.
    HostIs(String),
    /// Passes if the URL's [`UrlPart::Subdomain`] is the specified value.
    SubdomainIs(String),
    /// Passes if the URL's [`UrlPart::RegDomain`] is the specified value.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::RegDomainIs("example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::RegDomainIs("example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    RegDomainIs(String),
    /// Passes if the URL's [`UrlPart::MaybeWWWRegDomain`] is the specified value.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::MaybeWWWRegDomainIs("example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::MaybeWWWRegDomainIs("example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://not.example.com").unwrap();
    /// assert_eq!(Condition::MaybeWWWRegDomainIs("example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    MaybeWWWRegDomainIs(String),
    /// Passes if the URL's [`UrlPart::Domain`] is the specified value.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::DomainIs(    "example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::DomainIs("www.example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::DomainIs(    "example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::DomainIs("www.example.com".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    DomainIs(String),
    /// Passes if the URL's [`UrlPart::DomainMiddle`] is the specified value.
    /// # Footguns
    /// Please see [`UrlPart::DomainMiddle`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.example.co.uk" ).unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.aexample.example.co.uk").unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.aexample.co.uk"        ).unwrap();
    /// assert_eq!(Condition::DomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    DomainMiddleIs(String),
    /// Passes if the URL's [`UrlPart::MaybeWWWDomainMiddle`] is the specified value.
    /// # Footguns
    /// Please see [`UrlPart::MaybeWWWDomainMiddle`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// *job_state.url = BetterUrl::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// *job_state.url = BetterUrl::parse("https://not.example.com"  ).unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// *job_state.url = BetterUrl::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// *job_state.url = BetterUrl::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// *job_state.url = BetterUrl::parse("https://not.example.co.uk").unwrap();
    /// assert_eq!(Condition::MaybeWWWDomainMiddleIs("example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    MaybeWWWDomainMiddleIs(String),
    /// Passes if the URL's [`UrlPart::NotDomainSuffix`] is the specified value.
    /// # Footguns
    /// Please see [`UrlPart::NotDomainSuffix`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs(    "example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs("www.example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs(    "example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs("www.example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs(    "example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs("www.example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs(    "example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::NotDomainSuffixIs("www.example".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    NotDomainSuffixIs(String),
    /// Passes if the URL's [`UrlPart::DomainSuffix`] is the specified value.
    DomainSuffixIs(String),
    /// Passes if the URL's host is in the specified set of hosts.
    /// 
    /// Strips `www.` from the start of the host if it exists. This makes it work similar to [`UrlPart::HostWithoutWWWDotPrefix`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::HostIsOneOf([    "example.com".to_string(), "example2.com".to_string()].into()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(["www.example.com".to_string(), "example2.com".to_string()].into()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example2.com").unwrap();
    /// assert_eq!(Condition::HostIsOneOf([    "example.com".to_string(), "example2.com".to_string()].into()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(["www.example.com".to_string(), "example2.com".to_string()].into()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    HostIsOneOf(HashSet<String>),

    /// Passes if the URL has a host.
    UrlHasHost,
    /// Passes if the URL has a host that is a fully qualified domain name.
    HostIsFqdn,
    /// Passes if the URL has a host that is a domain.
    HostIsDomain,
    /// Passes if the URL has a host that is an IP address.
    HostIsIp,
    /// Passes if the URL has a host that is an IPv4 address.
    HostIsIpv4,
    /// Passes if the URL has a host that is an IPv6 address.
    HostIsIpv6,

    // Specific parts.

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("a".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("b".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("c".to_string()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    QueryHasParam(String),
    /// Passes if the URL's path is the specified string.
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com").unwrap();
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&job_state.to_view()).unwrap(), true);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com/").unwrap();
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&job_state.to_view()).unwrap(), true);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com/a").unwrap();
    /// assert_eq!(Condition::PathIs("/a" .to_string()).satisfied_by(&job_state.to_view()).unwrap(), true);
    /// 
    /// *job_state.url = BetterUrl::parse("https://example.com/a/").unwrap();
    /// assert_eq!(Condition::PathIs("/a/".to_string()).satisfied_by(&job_state.to_view()).unwrap(), true);
    /// ```
    PathIs(String),

    // General parts.

    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::PartIs{part: UrlPart::Username      , value: None}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Password      , value: None}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(0), value: None}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(1), value: None}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::Path          , value: None}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Fragment      , value: None}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    PartIs {
        /// The name of the part to check.
        part: UrlPart,
        /// The expected value of the part.
        value: Option<StringSource>
    },
    /// Passes if the specified part contains the specified value in a range specified by `where`.
    /// # Errors
    /// If the specified part is `None`, returns the error [`ConditionError::UrlPartNotFound`].
    /// 
    /// If `value.get` returns `None`, returns the error [`ConditionError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::Anywhere}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::End     }.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    PartContains {
        /// The name of the part to check.
        part: UrlPart,
        /// The value to look for.
        value: StringSource,
        /// Where to look for the value. Defaults to [`StringLocation::Anywhere`].
        #[serde(default)]
        r#where: StringLocation
    },

    /// Passes if the specified part's value matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    PartMatches {
        /// The part to check.
        part: UrlPart,
        /// The [`StringMatcher`] used to check the part's value.
        matcher: StringMatcher
    },
    /// Passes if the specified part's value is in the specified set.
    PartIsOneOf {
        /// The part to check.
        part: UrlPart,
        /// The set of values to pass for.
        values: HashSet<Option<String>>
    },

    // Miscellaneous.

    /// Passes if the specified variable is set to the specified value.
    /// # Examples
    /// ```
    /// # use std::collections::HashMap;
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// // Putting this in the `job_state!` call doesn't work???`
    /// let params = Params { vars: vec![("a".to_string(), "2".to_string())].into_iter().collect(), ..Default::default() };
    /// job_state.params = &params;
    /// 
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("2".into())}.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state.to_view()).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "b".into(), value: None            }.satisfied_by(&job_state.to_view()).unwrap(), true );
    /// ```
    VarIs {
        /// The name of the variable to check.
        name: StringSource,
        /// The expected value of the variable.
        value: Option<StringSource>
    },

    /// Passes if the specified rule flag is set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// // Putting this in the `job_state!` call doesn't work???`
    /// let params = Params { flags: ["abc".to_string()].into(), ..Default::default() };
    /// job_state.params = &params;
    /// 
    /// assert_eq!(Condition::FlagIsSet("abc".into()).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::FlagIsSet("xyz".into()).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    FlagIsSet(StringSource),

    // String source.

    /// Passes if `source` and `value`'s calls to [`StringSource::get`] return the same value.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    StringIs {
        /// The left hand side of the `==` operation.
        left: Option<StringSource>,
        /// The right hand side of the `==` operation.`
        right: Option<StringSource>
    },
    /// Passes if [`Self::StringContains::value`] contains [`Self::StringContains::substring`] at [`Self::StringContains::where`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    StringContains {
        /// The haystack to search in.
        value: StringSource,
        /// The needle to look for.
        substring: StringSource,
        /// Where to look (defaults to [`StringLocation::Anywhere`]).
        #[serde(default)]
        r#where: StringLocation
    },
    /// Passes if [`Self::StringMatches::value`] contains [`Self::StringMatches::matcher`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    StringMatches {
        /// The string to match.
        value: StringSource,
        /// The matcher.
        matcher: StringMatcher
    },

    // Commands.

    /// Checks the contained command's [`CommandConfig::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::CommandConfig;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert_eq!(Condition::CommandExists (CommandConfig::from("/usr/bin/true" )).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from("/usr/bin/false")).satisfied_by(&job_state.to_view()).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from("/usr/bin/fake" )).satisfied_by(&job_state.to_view()).unwrap(), false);
    /// ```
    #[cfg(feature = "commands")]
    CommandExists(CommandConfig),
    /// Runs the specified [`CommandConfig`] and passes if its exit code equals `expected` (which defaults to `0`).
    /// # Errors
    /// If the command is does not have an exit code (which I'm told only happens when a command is killed by a signal), returns the error [`ConditionError::CommandError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::CommandConfig;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from("/usr/bin/true" ), expected: 0}.satisfied_by(&job_state.to_view()).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from("/usr/bin/false"), expected: 0}.satisfied_by(&job_state.to_view()).is_ok_and(|x| x==false));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from("/usr/bin/fake" ), expected: 0}.satisfied_by(&job_state.to_view()).is_err());
    /// ```
    #[cfg(feature = "commands")]
    CommandExitStatus {
        /// The [`CommandConfig`] to execute.
        command: CommandConfig,
        /// The expected [`std::process::ExitStatus`]. Defaults to `0`.
        #[serde(default)]
        expected: i32
    },
    /// Passes if the provided [`JobState`]'s [`JobState::params`]'s [`Params::flags`] is non-empty.
    /// 
    /// A rarely useful optimization but an optimization none the less.
    AnyFlagIsSet,
    /// Checks that each path segment after `start` matches each [`StringMatcher`] in `matchers`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    ///
    /// url_cleaner::job_state!(job_state; url="https://example.com/a/b/c";);
    ///
    /// assert!( Condition::PathSegmentsMatch {start:  0, matchers: vec![StringMatcher::Equals("a".into())], strict: false}.satisfied_by(&job_state.to_view()).unwrap());
    /// assert!(!Condition::PathSegmentsMatch {start:  0, matchers: vec![StringMatcher::Equals("b".into())], strict: false}.satisfied_by(&job_state.to_view()).unwrap());
    /// 
    /// assert!( Condition::PathSegmentsMatch {start:  1, matchers: vec![StringMatcher::Equals("b".into())], strict: false}.satisfied_by(&job_state.to_view()).unwrap());
    /// assert!( Condition::PathSegmentsMatch {start: -1, matchers: vec![StringMatcher::Equals("c".into())], strict: false}.satisfied_by(&job_state.to_view()).unwrap());
    /// 
    /// assert!( Condition::PathSegmentsMatch {start: 0, matchers: vec![
    ///     StringMatcher::Equals("a".into()),
    ///     StringMatcher::Equals("b".into()),
    ///     StringMatcher::Equals("c".into()),
    ///     StringMatcher::Equals("d".into())
    /// ], strict: false}.satisfied_by(&job_state.to_view()).unwrap());
    /// assert!(!Condition::PathSegmentsMatch {start: 0, matchers: vec![
    ///     StringMatcher::Equals("a".into()),
    ///     StringMatcher::Equals("b".into()),
    ///     StringMatcher::Equals("c".into()),
    ///     StringMatcher::Equals("d".into())
    /// ], strict: true}.satisfied_by(&job_state.to_view()).unwrap());
    /// ```
    PathSegmentsMatch {
        /// The start of the segments to match.
        #[serde(default, skip_serializing_if = "is_default")]
        start: isize,
        /// The matchers to test with.
        matchers: Vec<StringMatcher>,
        /// If [`true`], having more matchers than segments returns [`false`].
        ///
        /// If [`false`], having more matchers than segments just pretends all the extra matchers passed.
        ///
        /// Defaults to [`true`].
        #[serde(default = "get_true", skip_serializing_if = "is_true")]
        strict: bool
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::conditions`].`
    Common(CommonCall),
    /// Uses a function pointer.
    /// 
    /// Cannot be serialized or deserialized.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    Custom(FnWrapper<fn(&JobStateView) -> Result<bool, ConditionError>>)
}

/// An enum of all possible errors a [`Condition`] can return.
#[derive(Debug, Error)]
pub enum ConditionError {
    /// Returned when [`Condition::Error`] is used.
    #[error("Condition::Error was used.")]
    ExplicitError,
    /// Returned when a call to [`UrlPart::get`] returns `None` where it has to return `Some`.
    #[error("The provided URL does not have the requested part.")]
    UrlPartNotFound,
    /// Returned when a [`CommandError`] is encountered.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] CommandError),
    /// Returned when a [`UrlPartGetError`] is encountered.
    #[error(transparent)]
    UrlPartGetError(#[from] UrlPartGetError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when both the `try` and `else` of a [`Condition::TryElse`] both return errors.
    #[error("A `Condition::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`Condition::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`Condition::TryElse::else`],
        else_error: Box<Self>
    },
    /// Returned when the common [`Condition`] is not found.
    #[error("The common Condition was not found.")]
    CommonConditionNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Custom error.
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, job_state: &JobStateView) -> Result<bool, ConditionError> {
        debug!(Condition::satisfied_by, self, job_state);
        Ok(match self {
            // Debug/constants.

            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(job_state);
                eprintln!("=== Condition::Debug ===\nCondition: {condition:?}\nJob state: {job_state:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(job_state)? {then} else {r#else}.satisfied_by(job_state)?,
            Self::Not(condition) => !condition.satisfied_by(job_state)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.satisfied_by(job_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.satisfied_by(job_state)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::PartMap  {part , map} => map.get(part .get(job_state.url) ).map(|x| x.satisfied_by(job_state)).unwrap_or(Ok(false))?,
            Self::StringMap{value, map} => map.get(value.get(job_state    )?).map(|x| x.satisfied_by(job_state)).unwrap_or(Ok(false))?,

            // Error handling.

            Self::TreatErrorAsPass(condition) => condition.satisfied_by(job_state).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by(job_state).unwrap_or(false),
            Self::TryElse{ r#try, r#else } => r#try.satisfied_by(job_state).or_else(|try_error| r#else.satisfied_by(job_state).map_err(|else_error| ConditionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)}))?,
            Self::FirstNotError(conditions) => {
                let mut result = Ok(false); // Initial value doesn't mean anything.
                for condition in conditions {
                    result = condition.satisfied_by(job_state);
                    if result.is_ok() {return result}
                }
                result?
            },

            // Domain conditions.

            Self::HostIs                (x) => UrlPart::Host                .get(job_state.url).as_deref() == Some(&**x),
            Self::SubdomainIs           (x) => UrlPart::Subdomain           .get(job_state.url).as_deref() == Some(&**x),
            Self::RegDomainIs           (x) => UrlPart::RegDomain           .get(job_state.url).as_deref() == Some(&**x),
            Self::MaybeWWWRegDomainIs   (x) => UrlPart::MaybeWWWRegDomain   .get(job_state.url).as_deref() == Some(&**x),
            Self::DomainIs              (x) => UrlPart::Domain              .get(job_state.url).as_deref() == Some(&**x),
            Self::DomainMiddleIs        (x) => UrlPart::DomainMiddle        .get(job_state.url).as_deref() == Some(&**x),
            Self::MaybeWWWDomainMiddleIs(x) => UrlPart::MaybeWWWDomainMiddle.get(job_state.url).as_deref() == Some(&**x),
            Self::NotDomainSuffixIs     (x) => UrlPart::NotDomainSuffix     .get(job_state.url).as_deref() == Some(&**x),
            Self::DomainSuffixIs        (x) => UrlPart::DomainSuffix        .get(job_state.url).as_deref() == Some(&**x),

            Self::HostIsOneOf(hosts) => job_state.url.host_str().is_some_and(|url_host| hosts.contains(url_host)),

            Self::UrlHasHost   => job_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(job_state.url.host_details(), Some(HostDetails::Domain(d @ DomainDetails {..})) if d.is_fqdn()),
            Self::HostIsDomain => matches!(job_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(job_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(job_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(job_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            // Specific parts.

            Self::QueryHasParam(name) => job_state.url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathIs(value) => if job_state.url.cannot_be_a_base() {
                Err(UrlPartGetError::UrlDoesNotHaveAPath)?
            } else {
                job_state.url.path()==value
            },

            Self::PathSegmentsMatch {start, matchers, strict} => {
                let segments_count = job_state.url.path_segments().ok_or(UrlPartGetError::UrlDoesNotHaveAPath)?.count();
                let segments_skip = neg_index(*start, segments_count).ok_or(UrlPartGetError::SegmentRangeNotFound)?;
                let segments = job_state.url.path_segments().ok_or(UrlPartGetError::UrlDoesNotHaveAPath)?.skip(segments_skip);

                if *strict && segments_count.checked_sub(segments_skip).is_none_or(|x| x <= matchers.len()) {return Ok(false);}

                for (segment, matcher) in segments.zip(matchers.iter()) {
                    if !matcher.satisfied_by(segment, job_state)? {
                        return Ok(false);
                    }
                };
                true
            },

            // General parts.

            Self::PartIs{part, value} => part.get(job_state.url).as_deref()==get_option_str!(value, job_state),
            Self::PartContains{part, value, r#where} => r#where.satisfied_by(&part.get(job_state.url).ok_or(ConditionError::UrlPartNotFound)?, get_str!(value, job_state, ConditionError))?,
            Self::PartMatches {part, matcher} => matcher.satisfied_by(&part.get(job_state.url).ok_or(ConditionError::UrlPartNotFound)?, job_state)?,
            Self::PartIsOneOf {part, values} => values.contains(&part.get(job_state.url).map(|x| x.into_owned())),

            // Miscellaneous.

            Self::FlagIsSet(name) => job_state.params.flags.contains(get_str!(name, job_state, ConditionError)),
            Self::AnyFlagIsSet => !job_state.params.flags.is_empty(),
            Self::VarIs {name, value} => job_state.params.vars.get(get_str!(name, job_state, ConditionError)).map(|x| &**x)==get_option_str!(value, job_state),

            // String source.

            Self::StringIs {left, right} => get_option_str!(left, job_state)==get_option_str!(right, job_state),
            Self::StringContains {value, substring, r#where} => r#where.satisfied_by(get_str!(value, job_state, ConditionError), get_str!(substring, job_state, ConditionError))?,
            Self::StringMatches {value, matcher} => matcher.satisfied_by(get_str!(value, job_state, ConditionError), job_state)?,

            // Commands.

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(job_state)?==expected},

            Self::Common(common_call) => {
                job_state.commons.conditions.get(get_str!(common_call.name, job_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.satisfied_by(&JobStateView {
                    url: job_state.url,
                    context: job_state.context,
                    params: job_state.params,
                    scratchpad: job_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: job_state.cache,
                    commons: job_state.commons,
                    common_args: Some(&common_call.args.make(job_state)?),
                    jobs_context: job_state.jobs_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        })
    }
}

impl Suitable for Condition {
    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(match self {
            Self::Debug(_) => false,
            Self::If {r#if, then, r#else} => r#if.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::Not(condition) => condition.is_suitable_for_release(config),
            Self::All(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::Any(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::PartMap   {part , map} => part .is_suitable_for_release(config) && map.is_suitable_for_release(config),
            Self::StringMap {value, map} => value.is_suitable_for_release(config) && map.is_suitable_for_release(config),
            Self::TreatErrorAsPass(condition) => condition.is_suitable_for_release(config),
            Self::TreatErrorAsFail(condition) => condition.is_suitable_for_release(config),
            Self::TryElse {r#try, r#else} => r#try.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::FirstNotError(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::PartIs {part, value} => part.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::PartContains {part, value, r#where} => part.is_suitable_for_release(config) && value.is_suitable_for_release(config) && r#where.is_suitable_for_release(config),
            Self::PartMatches {part, matcher} => part.is_suitable_for_release(config) && matcher.is_suitable_for_release(config),
            Self::PathSegmentsMatch {start: _, matchers, strict: _} => matchers.iter().all(|matcher| matcher.is_suitable_for_release(config)),
            Self::PartIsOneOf {part, ..} => part.is_suitable_for_release(config),
            Self::VarIs {name, value} => name.is_suitable_for_release(config) && value.as_ref().is_none_or(|value| value.is_suitable_for_release(config)),
            Self::FlagIsSet(name) => name.is_suitable_for_release(config) && check_docs!(config, flags, name),
            Self::StringIs {left, right} => (left.is_some() || right.is_some()) && left.as_ref().is_none_or(|left| left.is_suitable_for_release(config)) && right.as_ref().is_none_or(|right| right.is_suitable_for_release(config)),
            Self::StringContains {value, substring, r#where} => value.is_suitable_for_release(config) && substring.is_suitable_for_release(config) && r#where.is_suitable_for_release(config),
            Self::StringMatches {value, matcher} => value.is_suitable_for_release(config) && matcher.is_suitable_for_release(config),
            #[cfg(feature = "commands")] Self::CommandExists (_) => false,
            #[cfg(feature = "commands")] Self::CommandExitStatus {..} => false,
            Self::Always | Self::Never | Self::Error | Self::HostIs(_) | Self::SubdomainIs(_) |
                Self::RegDomainIs(_) | Self::MaybeWWWRegDomainIs(_) | Self::HostIsOneOf(_) |
                Self::DomainIs(_) | Self::DomainMiddleIs(_) | Self::MaybeWWWDomainMiddleIs(_) |
                Self::NotDomainSuffixIs(_) | Self::DomainSuffixIs(_) | Self::QueryHasParam(_) | Self::PathIs(_) | Self::AnyFlagIsSet |
                Self::UrlHasHost | Self::HostIsFqdn | Self::HostIsDomain | Self::HostIsIp | Self::HostIsIpv4 | Self::HostIsIpv6 => true,
            Self::Common(common_call) => common_call.is_suitable_for_release(config),
            #[cfg(feature = "custom")]
            Self::Custom(_) => false
        }, "Unsuitable Condition detected: {self:?}");
        true
    }
}

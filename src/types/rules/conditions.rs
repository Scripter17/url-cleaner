//! The logic for when to modify a URL.

use std::collections::hash_set::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;

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
    /// # use url::Url;
    /// Condition::Error.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as in both bash and batch `x | y` only pipes `x`'s STDOUT, but you probably shouldn't.
    /// # Errors
    /// If the contained [`Self`] returns an error, that error is returned after the debug info is printed.
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
    /// If the contained [`Self`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::Not(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::Not(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// 
    /// Condition::Not(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    Not(Box<Self>),
    /// Passes if all of the included [`Self`]s pass.
    /// Like [`Iterator::all`], an empty list passes.
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// 
    /// Condition::All(vec![Condition::Always, Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    All(Vec<Self>),
    /// Passes if any of the included [`Self`]s pass.
    /// Like [`Iterator::any`], an empty list fails.
    /// # Errors
    /// If any of the contained [`Self`]s returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// 
    /// Condition::Any(vec![Condition::Never , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    Any(Vec<Self>),

    // Error handling.

    /// If the contained [`Self`] returns an error, treat it as a pass.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the contained [`Self`] returns an error, treat it as a fail.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
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
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap_err();
    /// ```
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// If `try` fails, instead return the result of this one.
        r#else: Box<Self>
    },
    /// Effectively a [`Self::TryElse`] chain but less ugly.
    /// # Errors
    /// If every contained [`Self`] returns an error, returns the last error.
    FirstNotError(Vec<Self>),

    // Domain conditions.

    /// Passes if the URL's domain is or is a subdomain of the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    UnqualifiedDomain(String),
    /// Similar to [`Condition::UnqualifiedDomain`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWDomain("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedDomain("example.com".to_string()), Condition::QualifiedDomain("www.example.com".to_string())])`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://not.example.com").unwrap(), &Params::default()).unwrap(), false);
    /// ```
    MaybeWWWDomain(String),
    /// Passes if the URL's domain is the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    QualifiedDomain(String),
    /// Passes if the URL's host is in the specified set of hosts.
    /// Compared to having `n` rules of [`Self::MaybeWWWDomain`], this is `O(1)`.
    /// Strips `www.` from the start of the host if it exists. This makes it work similar to [`Self::UnqualifiedDomain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::HashSet;
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example.com" ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example.com" ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example2.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example2.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    HostIsOneOf(HashSet<String>),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is or is a subdomain of the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).unwrap(), true );
    /// // Weird edge cases.
    /// assert_eq!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.example.co.uk" ).unwrap(), &Params::default()).unwrap(), true);
    /// assert_eq!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.aexample.example.co.uk").unwrap(), &Params::default()).unwrap(), true);
    /// assert_eq!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.aexample.co.uk"        ).unwrap(), &Params::default()).unwrap(), false);
    /// ```
    UnqualifiedAnyTld(String),
    /// Similar to [`Condition::UnqualifiedAnyTld`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWAnyTld("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedAnyTld("example.com".to_string()), Condition::QualifiedAnyTld("www.example.com".to_string())])`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://not.example.com"  ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://not.example.co.uk").unwrap(), &Params::default()).unwrap(), false);
    /// ```
    MaybeWWWAnyTld(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    QualifiedAnyTld(String),

    // Specific parts.

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::QueryHasParam("a".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QueryHasParam("b".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::QueryHasParam("c".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).unwrap(), false);
    /// ```
    QueryHasParam(String),
    /// Passes if the URL's path is the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&Url::parse("https://example.com"   ).unwrap(), &Params::default()).unwrap(), true);
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&Url::parse("https://example.com/"  ).unwrap(), &Params::default()).unwrap(), true);
    /// assert_eq!(Condition::PathIs("/a" .to_string()).satisfied_by(&Url::parse("https://example.com/a" ).unwrap(), &Params::default()).unwrap(), true);
    /// assert_eq!(Condition::PathIs("/a/".to_string()).satisfied_by(&Url::parse("https://example.com/a/").unwrap(), &Params::default()).unwrap(), true);
    /// ```
    PathIs(String),

    // General parts.

    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::PartIs{part: UrlPart::Username      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Password      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(0), value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(1), value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::Path          , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Fragment      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    #[cfg(feature = "string-source")]
    PartIs {
        /// The name of the part to check.
        part: UrlPart,
        /// The expected value of the part.
        value: Option<StringSource>
    },
    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::PartIs{part: UrlPart::Username      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Password      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(0), value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(1), value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::Path          , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Fragment      , value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// ```
    #[cfg(not(feature = "string-source"))]
    PartIs {
        /// The name of the part to check.
        part: UrlPart,
        /// The expected value of the part.
        value: Option<String>
    },
    /// Passes if the specified part contains the specified value in a range specified by `where`.
    /// # Errors
    /// If the specified part is `None`, returns the error [`ConditionError::UrlPartNotFound`].
    /// 
    /// If `value.get` returns `None`, returns the error [`ConditionError::StringSourceIsNone`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::Anywhere}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::End     }.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).unwrap(), false);
    /// ```
    #[cfg(feature = "string-location")]
    PartContains {
        /// The name of the part to check.
        part: UrlPart,
        /// The value to look for.
        #[cfg(feature = "string-source")]
        value: StringSource,
        /// The value to look for.
        #[cfg(not(feature = "string-source"))]
        value: String,
        /// Where to look for the value.
        #[serde(default)]
        r#where: StringLocation
    },

    /// Passes if the specified part's value matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    #[cfg(feature = "string-matcher")]
    PartMatches {
        /// The part to check.
        part: UrlPart,
        /// The [`StringMatcher`] used to check the part's value.
        matcher: StringMatcher
    },

    // Miscellaneous.

    /// Passes if the specified variable is set to the specified value.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::HashMap;
    /// let url=Url::parse("https://example.com").unwrap();
    /// let params=Params {vars: HashMap::from([("a".to_string(), "2".to_string())]), ..Params::default()};
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("2".into())}.satisfied_by(&url, &params           ).unwrap(), true );
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&url, &params           ).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&url, &params           ).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&url, &Params::default()).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: None            }.satisfied_by(&url, &Params::default()).unwrap(), true );
    /// ```
    VarIs {
        /// The name of the variable to check.
        #[cfg(feature = "string-source")]
        name: StringSource,
        /// The name of the variable to check.
        #[cfg(not(feature = "string-source"))]
        name: String,
        /// The expected value of the variable.
        #[cfg(feature = "string-source")]
        value: Option<StringSource>,
        /// The expected value of the variable.
        #[cfg(not(feature = "string-source"))]
        value: Option<String>
    },

    /// Passes if the specified rule flag is set.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use std::collections::HashSet;
    /// # use url::Url;
    /// assert_eq!(Condition::FlagIsSet("abc".to_string()).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params {flags: HashSet::from_iter(["abc".to_string()]), ..Params::default()}).unwrap(), true );
    /// assert_eq!(Condition::FlagIsSet("abc".to_string()).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()                                                           ).unwrap(), false);
    /// ```
    FlagIsSet(String),

    // Commands.

    /// Checks the contained command's [`CommandConfig::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::CommandConfig;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/true" ).unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/false").unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/fake" ).unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).unwrap(), false);
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
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/true" ).unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/false").unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/fake" ).unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_err());
    /// ```
    #[cfg(feature = "commands")]
    CommandExitStatus {
        /// The [`CommandConfig`] to execute.
        command: CommandConfig,
        /// The expected [`std::process::ExitStatus`]. Defaults to `0`.
        #[serde(default)]
        expected: i32
    }
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
    #[cfg(feature = "string-source")]
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    /// Returned when a [`StringMatcherError`] is encountered.
    #[cfg(feature = "string-matcher")]
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    /// Returned when a [`StringLocationError`] is encountered.
    #[cfg(feature = "string-location")]
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[cfg(feature = "string-source")]
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when both the `try` and `else` of a [`Condition::TryElse`] both return errors.
    #[error("A `Condition::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        /// The error returned by [`Condition::TryElse::try`],
        try_error: Box<Self>,
        /// The error returned by [`Condition::TryElse::else`],
        else_error: Box<Self>
    }
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, url: &Url, params: &Params) -> Result<bool, ConditionError> {
        #[cfg(feature = "debug")]
        println!("Condition: {self:?}");
        Ok(match self {
            // Debug/constants.

            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(url, params);
                eprintln!("=== Condition::Debug ===\nCondition: {condition:?}\nURL: {url:?}\nParams: {params:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(url, params)? {then} else {r#else}.satisfied_by(url, params)?,
            Self::Not(condition) => !condition.satisfied_by(url, params)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.satisfied_by(url, params)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.satisfied_by(url, params)? {
                        return Ok(true);
                    }
                }
                false
            },

            // Error handling.

            Self::TreatErrorAsPass(condition) => condition.satisfied_by(url, params).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by(url, params).unwrap_or(false),
            Self::TryElse{r#try, r#else} => r#try.satisfied_by(url, params).or_else(|try_error| r#else.satisfied_by(url, params).map_err(|else_error2| ConditionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error2)}))?,
            Self::FirstNotError(conditions) => {
                let mut result = Ok(false); // Initial value doesn't mean anything.
                for condition in conditions {
                    result = condition.satisfied_by(url, params);
                    if result.is_ok() {return result}
                }
                result?
            }

            // Domain conditions.

            Self::UnqualifiedDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_suffix(domain_suffix).is_some_and(|unqualified_part| unqualified_part.is_empty() || unqualified_part.ends_with('.'))),
            Self::MaybeWWWDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_prefix("www.").unwrap_or(url_domain)==domain_suffix),
            Self::QualifiedDomain(domain) => url.domain()==Some(domain),
            Self::HostIsOneOf(hosts) => url.host_str().is_some_and(|url_host| hosts.contains(url_host.strip_prefix("www.").unwrap_or(url_host))),
            Self::UnqualifiedAnyTld(middle) => url.domain()
                .is_some_and(|url_domain| url_domain.rsplit_once(middle)
                    .is_some_and(|(prefix_dot, dot_suffix)| (prefix_dot.is_empty() || prefix_dot.ends_with('.')) && dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| psl::suffix_str(suffix)
                            .is_some_and(|psl_suffix| psl_suffix==suffix)
                        )
                    )
                ),
            Self::MaybeWWWAnyTld(middle) => url.domain().map(|domain| domain.strip_prefix("www.").unwrap_or(domain))
                .is_some_and(|domain| domain.strip_prefix(middle)
                    .is_some_and(|dot_suffix| dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix))
                    )
                ),
            Self::QualifiedAnyTld(parts) => url.domain()
                .is_some_and(|domain| domain.strip_prefix(parts)
                    .is_some_and(|dot_suffix| dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix))
                    )
                ),

            // Specific parts.

            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathIs(value) => if url.cannot_be_a_base() {
                Err(UrlPartGetError::UrlDoesNotHaveAPath)?
            } else {
                url.path()==value
            },

            // General parts.

            Self::PartIs{part, value} => part.get(url).as_deref()==get_option_string!(value, url, params),
            #[cfg(feature = "string-location")] Self::PartContains{part, value, r#where} => r#where.satisfied_by(&part.get(url).ok_or(ConditionError::UrlPartNotFound)?, get_string!(value, url, params, ConditionError))?,
            #[cfg(feature = "string-matcher" )] Self::PartMatches {part, matcher} => matcher.satisfied_by(&part.get(url).ok_or(ConditionError::UrlPartNotFound)?, url, params)?,

            // Miscellaneous.

            Self::VarIs {name, value} => params.vars.get(get_string!(name, url, params, ConditionError)).map(|x| &**x)==get_option_string!(value, url, params),
            Self::FlagIsSet(name) => params.flags.contains(name),

            // Commands.

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected},
        })
    }
}

//! The logic for when to modify a URL.

use std::collections::hash_set::HashSet;
use std::ops::Deref;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;
use psl;

use crate::glue::{self, string_or_struct, optional_string_or_struct};
use crate::types::{
    UrlPart, PartError,
    StringLocation, StringLocationError,
    StringSource, StringSourceError,
    StringMatcher, StringMatcherError
};
use crate::config::Params;

/// The part of a [`crate::rules::Rule`] that specifies when the rule's mapper will be applied.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Condition {
    /// Always passes.
    Always,

    // Testing conditions.

    /// Never passes.
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::Error.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    Error,
    /// Prints debugging information about the contained condition to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as bash and batch only have `x | y` pipe STDOUT by default, but it'll look ugly.
    Debug(Box<Self>),

    // Error handling

    /// If the contained condition returns an error, treat it as a pass.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::TreatErrorAsPass(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TreatErrorAsPass(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TreatErrorAsPass(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the contained condition returns an error, treat it as a fail.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::TreatErrorAsFail(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TreatErrorAsFail(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TreatErrorAsFail(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    TreatErrorAsFail(Box<Self>),
    /// If the `try` condition returns an error, return the result of the `else` condition instead. If the `try` condition does not error, the `else` condition is not executed.
    /// # Errors
    /// If the `else` condition returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Always)}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Never )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Error )}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    TryElse {
        /// The condition to try first.
        r#try: Box<Self>,
        /// If the try condition fails, instead return the result of this one.
        r#else: Box<Self>
    },

    // Boolean.

    /// Passes if all of the included conditions pass.
    /// Like [`Iterator::all`], an empty list of conditions returns `true`.
    /// # Errors
    /// If any contained condition returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::All(vec![Condition::Always, Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::All(vec![Condition::Always, Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::All(vec![Condition::Always, Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::All(vec![Condition::Never , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::All(vec![Condition::Never , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::All(vec![Condition::Never , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::All(vec![Condition::Error , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::All(vec![Condition::Error , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::All(vec![Condition::Error , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    All(Vec<Self>),
    /// Passes if any of the included conditions pass.
    /// Like [`Iterator::any`], an empty list of conditions returns `false`.
    /// # Errors
    /// If any contained condition returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::Any(vec![Condition::Always, Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::Any(vec![Condition::Always, Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::Any(vec![Condition::Always, Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::Any(vec![Condition::Never , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::Any(vec![Condition::Never , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::Any(vec![Condition::Never , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::Any(vec![Condition::Error , Condition::Always]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::Any(vec![Condition::Error , Condition::Never ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// assert!(Condition::Any(vec![Condition::Error , Condition::Error ]).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    Any(Vec<Self>),
    /// Passes if the included condition doesn't and vice-versa.
    /// # Errors
    /// If the contained condition returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::Not(Box::new(Condition::Always)).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::Not(Box::new(Condition::Never )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::Not(Box::new(Condition::Error )).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_err());
    /// ```
    Not(Box<Self>),

    // Domain conditions.

    /// Passes if the URL's domain is or is a subdomain of the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    UnqualifiedDomain(String),
    /// Similar to [`Condition::UnqualifiedDomain`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWDomain("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedDomain("example.com".to_string()), Condition::QualifiedDomain("www.example.com".to_string())])`.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://not.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    MaybeWWWDomain(String),
    /// Passes if the URL's domain is the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    QualifiedDomain(String),
    /// Passes if the URL's host is in the specified set of hosts.
    /// Compared to having `n` rules of [`Self::MaybeWWWDomain`], this is `O(1)`.
    /// Strips `www.` from the start of the host if it exists. This makes it work similar to [`Self::UnqualifiedDomain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// # use std::collections::HashSet;
    /// assert!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example.com" ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example.com" ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example2.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&Url::parse("https://example2.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    HostIsOneOf(HashSet<String>),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is or is a subdomain of the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// // Weird edge cases.
    /// assert!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.example.co.uk" ).unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// assert!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.aexample.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// assert!(Condition::UnqualifiedAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.aexample.co.uk"        ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    UnqualifiedAnyTld(String),
    /// Similar to [`Condition::UnqualifiedAnyTld`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWAnyTld("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedAnyTld("example.com".to_string()), Condition::QualifiedAnyTld("www.example.com".to_string())])`.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://not.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWAnyTld("example".to_string()).satisfied_by(&Url::parse("https://not.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    MaybeWWWAnyTld(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    QualifiedAnyTld(String),

    // Query.

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// assert!(Condition::QueryHasParam("a".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("b".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("c".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    QueryHasParam(String),

    // Path.

    /// Passes if the URL's path is the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// assert!(Condition::PathIs("/"  .to_string()).satisfied_by(&Url::parse("https://example.com"   ).unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// assert!(Condition::PathIs("/"  .to_string()).satisfied_by(&Url::parse("https://example.com/"  ).unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// assert!(Condition::PathIs("/a" .to_string()).satisfied_by(&Url::parse("https://example.com/a" ).unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// assert!(Condition::PathIs("/a/".to_string()).satisfied_by(&Url::parse("https://example.com/a/").unwrap(), &Params::default()).is_ok_and(|x| x==true));
    /// ```
    PathIs(String),

    // General parts.

    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::types::UrlPart;
    /// # use url::Url;
    /// assert!(Condition::PartIs{part: UrlPart::Username      , none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::PartIs{part: UrlPart::Password      , none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::PartIs{part: UrlPart::PathSegment(0), none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::PartIs{part: UrlPart::PathSegment(1), none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::PartIs{part: UrlPart::Path          , none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::PartIs{part: UrlPart::Fragment      , none_to_empty_string: false, value: None}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    PartIs {
        /// The name of the part to check.
        part: UrlPart,
        /// If the chosen URL part's getter returns `None`, this determines if that should be interpreted as an empty string.
        /// Defaults to `true` for the sake of simplicity.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The expected value of the part.
        value: Option<String>
    },
    /// Passes if the specified part contains the specified value in a range specified by `where`.
    /// # Errors
    /// If the specified part is `None` and `none_to_empty_string` is set to `false`, returns the error [`ConditionError::UrlPartNotFound`].
    /// # Examples
    /// ```
    /// # use url::Url;
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::types::StringSource;
    /// # use url_cleaner::types::UrlPart;
    /// # use url_cleaner::types::StringLocation;
    /// assert!(Condition::PartContains {part: UrlPart::Domain, none_to_empty_string: true, value: "ple".to_string(), r#where: StringLocation::Anywhere}.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::PartContains {part: UrlPart::Domain, none_to_empty_string: true, value: "ple".to_string(), r#where: StringLocation::End     }.satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    PartContains {
        /// The name of the part to check.
        part: UrlPart,
        /// If the chosen URL part's getter returns `None`, this determines if that should be interpreted as an empty string.
        /// Defaults to `true` for the sake of simplicity.
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The value to look for.
        value: String,
        /// Where to look for the value.
        #[serde(default)]
        r#where: StringLocation
    },

    /// Passes if the specified part's value matches the specified [`StringMatcher`].
    /// # Errors
    /// If the call to [`StringMatcher::matches`] returns an error, that error is returned.
    PartMatches {
        /// The part to check.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`StringMatcher`] used to check the part's value.
        matcher: StringMatcher
    },

    // Commands.

    /// Checks the contained command's [`glue::CommandWrapper::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::glue::CommandWrapper;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// assert!(Condition::CommandExists (CommandWrapper::from_str("/usr/bin/true" ).unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExists (CommandWrapper::from_str("/usr/bin/false").unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExists (CommandWrapper::from_str("/usr/bin/fake" ).unwrap()).satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// ```
    #[cfg(feature = "commands")]
    CommandExists(glue::CommandWrapper),
    /// Runs the specified [`glue::CommandWrapper`] and passes if its exit code equals `expected` (which defaults to `0`).
    /// # Errors
    /// If the command is does not have an exit code (which I'm told only happens when a command is killed by a signal), returns the error [`ConditionError::CommandError`].
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::glue::CommandWrapper;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// assert!(Condition::CommandExitStatus {command: CommandWrapper::from_str("/usr/bin/true" ).unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExitStatus {command: CommandWrapper::from_str("/usr/bin/false").unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::CommandExitStatus {command: CommandWrapper::from_str("/usr/bin/fake" ).unwrap(), expected: 0}.satisfied_by(&Url::parse("https://url.does/not#matter").unwrap(), &Params::default()).is_err());
    /// ```
    #[cfg(feature = "commands")]
    CommandExitStatus {
        /// The [`glue::CommandWrapper`] to execute.
        command: glue::CommandWrapper,
        /// The expected [`std::process::ExitStatus`]. Defaults to `0`.
        #[serde(default)]
        expected: i32
    },

    // Miscelanious.

    /// Passes if the specified rule variable is set to the specified value.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// # use url_cleaner::types::StringSource;
    /// # use url::Url;
    /// # use std::collections::HashMap;
    /// let url=Url::parse("https://example.com").unwrap();
    /// let params=Params {vars: HashMap::from([("a".to_string(), "2".to_string())]), ..Params::default()};
    /// assert!(Condition::VarIs{name: StringSource::String("a".to_string()), value: Some(StringSource::String("2".to_string())), none_to_empty_string: false}.satisfied_by(&url, &params           ).is_ok_and(|x| x==true ));
    /// assert!(Condition::VarIs{name: StringSource::String("a".to_string()), value: Some(StringSource::String("3".to_string())), none_to_empty_string: false}.satisfied_by(&url, &params           ).is_ok_and(|x| x==false));
    /// assert!(Condition::VarIs{name: StringSource::String("a".to_string()), value: Some(StringSource::String("3".to_string())), none_to_empty_string: false}.satisfied_by(&url, &params           ).is_ok_and(|x| x==false));
    /// assert!(Condition::VarIs{name: StringSource::String("a".to_string()), value: Some(StringSource::String("3".to_string())), none_to_empty_string: false}.satisfied_by(&url, &Params::default()).is_ok_and(|x| x==false));
    /// assert!(Condition::VarIs{name: StringSource::String("a".to_string()), value: None                                       , none_to_empty_string: false}.satisfied_by(&url, &Params::default()).is_ok_and(|x| x==true ));
    /// ```
    VarIs {
        /// The name of the variable to check.
        #[serde(deserialize_with = "string_or_struct")]
        name: StringSource,
        /// The expected value of the variable.
        #[serde(deserialize_with = "optional_string_or_struct")]
        value: Option<StringSource>,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to just pretend it's an empty string.
        /// Defaults to [`false`].
        #[serde(default)]
        none_to_empty_string: bool
    },

    /// Passes if the specified rule flag is set.
    /// # Examples
    /// ```
    /// # use std::collections::HashSet;
    /// # use url::Url;
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::config::Params;
    /// assert!(Condition::FlagIsSet("abc".to_string()).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params {flags: HashSet::from_iter(["abc".to_string()]), ..Params::default()}).is_ok_and(|x| x==true ));
    /// assert!(Condition::FlagIsSet("abc".to_string()).satisfied_by(&Url::parse("https://example.com").unwrap(), &Params::default()                                                           ).is_ok_and(|x| x==false));
    /// ```
    FlagIsSet(String)
}

const fn get_true() -> bool {true}

/// An enum of all possible errors a [`Condition`] can return.
#[derive(Error, Debug)]
pub enum ConditionError {
    /// The [`Condition::Error`] condition always returns this error.
    #[error("The \"Error\" condition always returns this error.")]
    ExplicitError,
    /// The provided URL does not contain the requested part.
    /// See [`crate::types::UrlPart`] for details.
    #[error("The provided URL does not contain the requested part.")]
    UrlPartNotFound,
    /// Returned when the specified command failed to run.
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] glue::CommandError),
    /// Returned when a string condition fails.
    #[error(transparent)]
    StringError(#[from] crate::types::StringError),
    /// Returned when a [`UrlPart`] method returns an error.
    #[error(transparent)]
    PartError(#[from] PartError),
    /// The specified [`StringSource`] returned `None`.
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    /// The call to [`StringMatcher::matches`] returned an error.
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError)
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// If the condition has an error, that error is returned.
    /// See [`Condition`]'s documentation for details.
    pub fn satisfied_by(&self, url: &Url, params: &Params) -> Result<bool, ConditionError> {
        Ok(match self {
            // Domain conditions

            Self::UnqualifiedDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_suffix(domain_suffix).is_some_and(|unqualified_part| unqualified_part.is_empty() || unqualified_part.ends_with('.'))),
            Self::MaybeWWWDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_prefix("www.").unwrap_or(url_domain)==domain_suffix),
            Self::QualifiedDomain(domain) => url.domain()==Some(domain),
            Self::HostIsOneOf(hosts) => url.host_str().map(|host| host.strip_prefix("www.").unwrap_or(host)).is_some_and(|url_host| hosts.contains(url_host)),
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

            // Meta conditions

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
            Self::Not(condition) => !condition.satisfied_by(url, params)?,

            // Query

            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),

            // Path

            Self::PathIs(value) => if url.cannot_be_a_base() {
                Err(PartError::UrlDoesNotHaveAPath)?
            } else {
                url.path()==value
            },

            // General parts

            Self::PartIs{part, none_to_empty_string, value} => value.as_deref()==part.get(url, *none_to_empty_string).as_deref(),
            Self::PartContains{part, none_to_empty_string, value, r#where} => r#where.satisfied_by(&part.get(url, *none_to_empty_string).ok_or(ConditionError::UrlPartNotFound)?, value)?,
            Self::PartMatches {part, none_to_empty_string, matcher} => matcher.satisfied_by(&part.get(url, *none_to_empty_string).ok_or(ConditionError::UrlPartNotFound)?)?,

            // Miscelanious

            Self::VarIs{name, value, none_to_empty_string} => match value.as_ref() {
                Some(source) => params.vars.get(&name.get_string(url, params, false)?.ok_or(ConditionError::StringSourceIsNone)?.to_string()).map(|x| x.deref())==source.get_string(url, params, *none_to_empty_string)?.as_deref(),
                None => params.vars.get(&name.get_string(url, params, false)?.ok_or(ConditionError::StringSourceIsNone)?.to_string()).is_none()
            },
            Self::FlagIsSet(name) => params.flags.contains(name),

            // Should only ever be used once

            Self::Always => true,

            // Commands

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected},

            // Error handling

            Self::TreatErrorAsPass(condition) => condition.satisfied_by(url, params).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by(url, params).unwrap_or(false),
            Self::TryElse{r#try, r#else}  => r#try.satisfied_by(url, params).or_else(|_| r#else.satisfied_by(url, params))?,

            // Debug

            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(url, params);
                eprintln!("=== Debug condition ===\nCondition: {condition:?}\nURL: {url:?}\nParams: {params:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            }
        })
    }
}

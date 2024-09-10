//! The logic for when to modify a URL.

use std::collections::hash_set::HashSet;

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
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// Condition::Error.satisfied_by(&job_state).unwrap_err();
    /// ```
    Error,
    /// Prints debugging information about the contained [`Self`] and the details of its execution to STDERR.
    /// 
    /// Intended primarily for debugging logic errors.
    /// 
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::Not(Box::new(Condition::Always)).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::Not(Box::new(Condition::Never )).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// Condition::Not(Box::new(Condition::Error )).satisfied_by(&job_state).unwrap_err();
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Always]).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::All(vec![Condition::Always, Condition::Never ]).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Always]).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Never ]).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::All(vec![Condition::Never , Condition::Error ]).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// Condition::All(vec![Condition::Always, Condition::Error ]).satisfied_by(&job_state).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Always]).satisfied_by(&job_state).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Never ]).satisfied_by(&job_state).unwrap_err();
    /// Condition::All(vec![Condition::Error , Condition::Error ]).satisfied_by(&job_state).unwrap_err();
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Always]).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Never ]).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Always, Condition::Error ]).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Always]).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::Any(vec![Condition::Never , Condition::Never ]).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// Condition::Any(vec![Condition::Never , Condition::Error ]).satisfied_by(&job_state).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Always]).satisfied_by(&job_state).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Never ]).satisfied_by(&job_state).unwrap_err();
    /// Condition::Any(vec![Condition::Error , Condition::Error ]).satisfied_by(&job_state).unwrap_err();
    /// ```
    Any(Vec<Self>),
    /// Passes if the condition in `map` whose key is the value returned by `part`'s [`UrlPart::get`] passes.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    PartMap {
        /// The part to get.
        part: UrlPart,
        /// The map specifying which values should run which conditions.
        map: HashMap<Option<String>, Self>
    },
    /// Passes if the condition in `map` whose key is the value returned by `source`'s [`StringSource::get`] passes.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    StringMap {
        /// The string to index the map with.
        source: Option<StringSource>,
        /// The map specifying which values should run which conditions.
        map: HashMap<Option<String>, Self>
    },

    // Error handling.

    /// If the contained [`Self`] returns an error, treat it as a pass.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Always)).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Never )).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsPass(Box::new(Condition::Error )).satisfied_by(&job_state).unwrap(), true );
    /// ```
    TreatErrorAsPass(Box<Self>),
    /// If the contained [`Self`] returns an error, treat it as a fail.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Always)).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Never )).satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::TreatErrorAsFail(Box::new(Condition::Error )).satisfied_by(&job_state).unwrap(), false);
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Always), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Never ), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Always)}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Never )}.satisfied_by(&job_state).unwrap(), false);
    /// Condition::TryElse{r#try: Box::new(Condition::Error ), r#else: Box::new(Condition::Error )}.satisfied_by(&job_state).unwrap_err();
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
    /// 
    /// Similar to [`UrlPart::NotSubdomain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// ```
    UnqualifiedDomain(String),
    /// Similar to [`Condition::UnqualifiedDomain`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWDomain("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedDomain("example.com".to_string()), Condition::QualifiedDomain("www.example.com".to_string())])`.
    /// 
    /// Similar to [`UrlPart::MaybeWWWNotSubdomain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://not.example.com").unwrap();
    /// assert_eq!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// ```
    MaybeWWWDomain(String),
    /// Passes if the URL's domain is the specified domain.
    /// 
    /// Similar to [`UrlPart::Domain`].
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com"    ).unwrap();
    /// assert_eq!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com").unwrap();
    /// assert_eq!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&job_state).unwrap(), true );
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://example2.com").unwrap();
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter([    "example.com".to_string(), "example2.com".to_string()])).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::HostIsOneOf(HashSet::from_iter(["www.example.com".to_string(), "example2.com".to_string()])).satisfied_by(&job_state).unwrap(), true );
    /// ```
    HostIsOneOf(HashSet<String>),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is or is a subdomain of the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// 
    /// Similar to [`UrlPart::DomainMiddle`].
    /// # Footguns
    /// Please see [`UrlPart::DomainMiddle`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.example.co.uk" ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true);
    /// 
    /// *job_state.url = Url::parse("https://www.aexample.example.co.uk").unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true);
    /// 
    /// *job_state.url = Url::parse("https://www.aexample.co.uk"        ).unwrap();
    /// assert_eq!(Condition::UnqualifiedAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// ```
    UnqualifiedAnySuffix(String),
    /// Similar to [`Condition::UnqualifiedAnySuffix`] but only checks if the subdomain is empty or `www`.
    /// 
    /// `Condition::MaybeWWWAnySuffix("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedAnySuffix("example.com".to_string()), Condition::QualifiedAnySuffix("www.example.com".to_string())])`.
    /// 
    /// Similar to [`UrlPart::MaybeWWWDomainMiddle`].
    /// # Footguns
    /// Please see [`UrlPart::MaybeWWWDomainMiddle`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// *job_state.url = Url::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// *job_state.url = Url::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// *job_state.url = Url::parse("https://not.example.com"  ).unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// *job_state.url = Url::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// *job_state.url = Url::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// *job_state.url = Url::parse("https://not.example.co.uk").unwrap();
    /// assert_eq!(Condition::MaybeWWWAnySuffix("example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// ```
    MaybeWWWAnySuffix(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// 
    /// Similar to [`UrlPart::NotDomainSuffix`].
    /// # Footguns
    /// Please see [`UrlPart::NotDomainSuffix`] for details on how "suffix" semantics can be counterintuitive.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com"      ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.co.uk"    ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.com"  ).unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix(    "example".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// 
    /// *job_state.url = Url::parse("https://www.example.co.uk").unwrap();
    /// assert_eq!(Condition::QualifiedAnySuffix("www.example".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// ```
    QualifiedAnySuffix(String),

    // Specific parts.

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("a".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("b".to_string()).satisfied_by(&job_state).unwrap(), true );
    /// 
    /// *job_state.url = Url::parse("https://example.com?a=2&b=3").unwrap();
    /// assert_eq!(Condition::QueryHasParam("c".to_string()).satisfied_by(&job_state).unwrap(), false);
    /// ```
    QueryHasParam(String),
    /// Passes if the URL's path is the specified string.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// 
    /// *job_state.url = Url::parse("https://example.com").unwrap();
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&job_state).unwrap(), true);
    /// 
    /// *job_state.url = Url::parse("https://example.com/").unwrap();
    /// assert_eq!(Condition::PathIs("/"  .to_string()).satisfied_by(&job_state).unwrap(), true);
    /// 
    /// *job_state.url = Url::parse("https://example.com/a").unwrap();
    /// assert_eq!(Condition::PathIs("/a" .to_string()).satisfied_by(&job_state).unwrap(), true);
    /// 
    /// *job_state.url = Url::parse("https://example.com/a/").unwrap();
    /// assert_eq!(Condition::PathIs("/a/".to_string()).satisfied_by(&job_state).unwrap(), true);
    /// ```
    PathIs(String),

    // General parts.

    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::PartIs{part: UrlPart::Username      , value: None}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Password      , value: None}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(0), value: None}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::PathSegment(1), value: None}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::PartIs{part: UrlPart::Path          , value: None}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::PartIs{part: UrlPart::Fragment      , value: None}.satisfied_by(&job_state).unwrap(), true );
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
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::Anywhere}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::PartContains {part: UrlPart::Domain, value: "ple".into(), r#where: StringLocation::End     }.satisfied_by(&job_state).unwrap(), false);
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

    // Miscellaneous.

    /// Passes if the specified variable is set to the specified value.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::collections::HashMap;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = url_cleaner::types::Params { vars: vec![("a".to_string(), "2".to_string())].into_iter().collect(), ..Default::default() };
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// print!("{job_state:?}");
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("2".into())}.satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "a".into(), value: Some("3".into())}.satisfied_by(&job_state).unwrap(), false);
    /// assert_eq!(Condition::VarIs{name: "b".into(), value: None            }.satisfied_by(&job_state).unwrap(), true );
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
    /// # use std::collections::HashSet;
    /// # use url::Url;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = url_cleaner::types::Params { flags: HashSet::from_iter(vec!["abc".to_string()]), ..Default::default() };
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::FlagIsSet("abc".into()).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::FlagIsSet("xyz".into()).satisfied_by(&job_state).unwrap(), false);
    /// ```
    FlagIsSet(StringSource),

    // String source.

    /// Passes if `source` and `value`'s calls to [`StringSource::get`] return the same value.
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    StringIs {
        /// The left hand side of the `==` operation.
        source: Option<StringSource>,
        /// The right hand side of the `==` operation.`
        value: Option<StringSource>
    },
    /// Passes if [`Self::StringContains::source`] contains [`Self::StringContains::value`] at [`Self::StringContains::where`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    StringContains {
        /// The haystack to search in.
        source: StringSource,
        /// The needle to look for.
        value: StringSource,
        /// Where to look (defaults to [`StringLocation::Anywhere`]).
        #[serde(default)]
        r#where: StringLocation
    },
    /// Passes if [`Self::StringMatches::source`] contains [`Self::StringMatches::matcher`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    StringMatches {
        /// The string to match.
        source: StringSource,
        /// The matcher.
        matcher: StringMatcher
    },

    // Commands.

    /// Checks the contained command's [`CommandConfig::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url_cleaner::glue::CommandConfig;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/true" ).unwrap()).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/false").unwrap()).satisfied_by(&job_state).unwrap(), true );
    /// assert_eq!(Condition::CommandExists (CommandConfig::from_str("/usr/bin/fake" ).unwrap()).satisfied_by(&job_state).unwrap(), false);
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
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let commons = Default::default();
    /// let params = Default::default();
    /// let context = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = "test-cache.sqlite".into();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     context: &context,
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler,
    ///     commons: &commons,
    ///     common_vars: None
    /// };
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/true" ).unwrap(), expected: 0}.satisfied_by(&job_state).is_ok_and(|x| x==true ));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/false").unwrap(), expected: 0}.satisfied_by(&job_state).is_ok_and(|x| x==false));
    /// assert!(Condition::CommandExitStatus {command: CommandConfig::from_str("/usr/bin/fake" ).unwrap(), expected: 0}.satisfied_by(&job_state).is_err());
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
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::conditions`].`
    /// 
    /// Currently does not pass-in [`JobState::vars`] or preserve updates. This will eventually be changed.
    Common {
        /// The name of the [`Self`] to use.
        name: StringSource,
        /// The [`JobState::common_vars`] to pass.
        #[serde(default, skip_serializing_if = "is_default")]
        vars: HashMap<String, StringSource>
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
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn satisfied_by(&self, job_state: &JobState) -> Result<bool, ConditionError> {
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
            Self::PartMap{part, map} => match map.get(&part.get(job_state.url).map(|x| x.into_owned())) {
                Some(condition) => condition.satisfied_by(job_state)?,
                None => false
            },
            Self::StringMap{source, map} => match map.get(&get_option_string!(source, job_state)) {
                Some(condition) => condition.satisfied_by(job_state)?,
                None => false
            },

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

            Self::UnqualifiedDomain(domain_suffix) => job_state.url.domain().is_some_and(|url_domain| url_domain.strip_suffix(domain_suffix).is_some_and(|unqualified_part| unqualified_part.is_empty() || unqualified_part.ends_with('.'))),
            Self::MaybeWWWDomain(domain_suffix) => job_state.url.domain().is_some_and(|url_domain| url_domain.strip_prefix("www.").unwrap_or(url_domain)==domain_suffix),
            Self::QualifiedDomain(domain) => job_state.url.domain()==Some(domain),
            Self::HostIsOneOf(hosts) => job_state.url.host_str().is_some_and(|url_host| hosts.contains(url_host)),
            Self::UnqualifiedAnySuffix(middle) => job_state.url.domain()
                .is_some_and(|url_domain| url_domain.rsplit_once(middle)
                    .is_some_and(|(prefix_dot, dot_suffix)| (prefix_dot.is_empty() || prefix_dot.ends_with('.')) && dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| psl::suffix_str(suffix)
                            .is_some_and(|psl_suffix| psl_suffix==suffix)
                        )
                    )
                ),
            Self::MaybeWWWAnySuffix(middle) => job_state.url.domain().map(|domain| domain.strip_prefix("www.").unwrap_or(domain))
                .is_some_and(|domain| domain.strip_prefix(middle)
                    .is_some_and(|dot_suffix| dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix))
                    )
                ),
            Self::QualifiedAnySuffix(parts) => job_state.url.domain()
                .is_some_and(|domain| domain.strip_prefix(parts)
                    .is_some_and(|dot_suffix| dot_suffix.strip_prefix('.')
                        .is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix))
                    )
                ),

            // Specific parts.

            Self::QueryHasParam(name) => job_state.url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathIs(value) => if job_state.url.cannot_be_a_base() {
                Err(UrlPartGetError::UrlDoesNotHaveAPath)?
            } else {
                job_state.url.path()==value
            },

            // General parts.

            Self::PartIs{part, value} => part.get(job_state.url).as_deref()==get_option_str!(value, job_state),
            Self::PartContains{part, value, r#where} => r#where.satisfied_by(&part.get(job_state.url).ok_or(ConditionError::UrlPartNotFound)?, get_str!(value, job_state, ConditionError))?,
            Self::PartMatches {part, matcher} => matcher.satisfied_by(&part.get(job_state.url).ok_or(ConditionError::UrlPartNotFound)?, job_state)?,

            // Miscellaneous.

            Self::FlagIsSet(name) => job_state.params.flags.contains(&get_string!(name, job_state, ConditionError)),
            Self::AnyFlagIsSet => !job_state.params.flags.is_empty(),
            Self::VarIs {name, value} => job_state.params.vars.get(get_str!(name, job_state, ConditionError)).map(|x| &**x)==get_option_str!(value, job_state),

            // String source.

            Self::StringIs {source, value} => get_option_str!(source, job_state)==get_option_str!(value, job_state),
            Self::StringContains {source, value, r#where} => r#where.satisfied_by(get_str!(source, job_state, ConditionError), get_str!(value, job_state, ConditionError))?,
            Self::StringMatches {source, matcher} => matcher.satisfied_by(get_str!(source, job_state, ConditionError), job_state)?,

            // Commands.

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(job_state)?==expected},

            Self::Common {name, vars} => {
                let common_vars = vars.iter().map(|(k, v)| Ok::<_, ConditionError>((k.clone(), get_string!(v, job_state, ConditionError)))).collect::<Result<HashMap<_, _>, _>>()?;
                let mut temp_url = job_state.url.clone();
                job_state.commons.conditions.get(get_str!(name, job_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.satisfied_by(&JobState {
                    url: &mut temp_url,
                    context: job_state.context,
                    params: job_state.params,
                    vars: Default::default(),
                    #[cfg(feature = "cache")]
                    cache_handler: job_state.cache_handler,
                    commons: job_state.commons,
                    common_vars: Some(&common_vars)
                })?
            }
        })
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used, reason = "Private API, but they should be replaced by [`Option::is_none_or`] in 1.82.")]
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        if match self {
            Self::Debug(_) => false,
            Self::If {r#if, then, r#else} => r#if.is_suitable_for_release(config) && then.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::Not(condition) => condition.is_suitable_for_release(config),
            Self::All(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::Any(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::PartMap {part, map} => part.is_suitable_for_release(config) && map.iter().all(|(_, condition)| condition.is_suitable_for_release(config)),
            Self::StringMap {source, map} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release(config)) && map.iter().all(|(_, condition)| condition.is_suitable_for_release(config)),
            Self::TreatErrorAsPass(condition) => condition.is_suitable_for_release(config),
            Self::TreatErrorAsFail(condition) => condition.is_suitable_for_release(config),
            Self::TryElse {r#try, r#else} => r#try.is_suitable_for_release(config) && r#else.is_suitable_for_release(config),
            Self::FirstNotError(conditions) => conditions.iter().all(|condition| condition.is_suitable_for_release(config)),
            Self::PartIs {part, value} => part.is_suitable_for_release(config) && (value.is_none() || value.as_ref().unwrap().is_suitable_for_release(config)),
            Self::PartContains {part, value, r#where} => part.is_suitable_for_release(config) && value.is_suitable_for_release(config) && r#where.is_suitable_for_release(config),
            Self::PartMatches {part, matcher} => part.is_suitable_for_release(config) && matcher.is_suitable_for_release(config),
            Self::VarIs {name, value} => name.is_suitable_for_release(config) && (value.is_none() || value.as_ref().unwrap().is_suitable_for_release(config)),
            Self::FlagIsSet(name) => name.is_suitable_for_release(config) && check_docs!(config, flags, name),
            Self::StringIs {source, value} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release(config)) && (value.is_none() || value.as_ref().unwrap().is_suitable_for_release(config)),
            Self::StringContains {source, value, r#where} => source.is_suitable_for_release(config) && value.is_suitable_for_release(config) && r#where.is_suitable_for_release(config),
            Self::StringMatches {source, matcher} => source.is_suitable_for_release(config) && matcher.is_suitable_for_release(config),
            #[cfg(feature = "commands")] Self::CommandExists (_) => false,
            #[cfg(feature = "commands")] Self::CommandExitStatus {..} => false,
            Self::Always | Self::Never | Self::Error | Self::MaybeWWWDomain(_) |
                Self::QualifiedDomain(_) | Self::HostIsOneOf(_) | Self::UnqualifiedDomain(_) |
                Self::UnqualifiedAnySuffix(_) | Self::MaybeWWWAnySuffix(_) | Self::QualifiedAnySuffix(_) |
                Self::QueryHasParam(_) | Self::PathIs(_) | Self::AnyFlagIsSet => true,
            Self::Common {name, vars} => name.is_suitable_for_release(config) && vars.iter().all(|(_, v)| v.is_suitable_for_release(config))
        } {
            true
        } else {
            println!("Failed Condition: {self:?}.");
            false
        }
    }
}

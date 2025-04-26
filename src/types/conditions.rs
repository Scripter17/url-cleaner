//! Logic for when a [`TaskState`] should be modified.

use std::collections::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Conditions that decide if and when to apply an [`Action`].
///
/// "Pass" means [`Condition::satisfied_by`] returns `Ok(true)` and "fail" means it returns `Ok(false)`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Condition {
    /// Always passes.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::Always.satisfied_by(&task_state).unwrap());
    /// ```
    Always,
    /// Always fails.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(!Condition::Never.satisfied_by(&task_state).unwrap());
    /// ```
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, url = "https://example.com");
    ///
    /// Condition::Error("...".into()).satisfied_by(&task_state).unwrap_err();
    /// ```
    Error(String),
    /// Prints debug info about the contained [`Self`] and the current [`TaskState`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If the call to [`Self::If::if`] passes, return the value of [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails, return the value of [`Self::If::else`].
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    /// url_cleaner::task_state_view!(task_state, url = "https://example.com");
    ///
    /// assert!(Condition::If {
    ///     r#if  : Box::new(Condition::Always),
    ///     then  : Box::new(Condition::Always),
    ///     r#else: Some(Box::new(Condition::Error("...".into())))
    /// }.satisfied_by(&task_state).unwrap());
    ///
    /// Condition::If {
    ///     r#if  : Box::new(Condition::Never),
    ///     then  : Box::new(Condition::Always),
    ///     r#else: Some(Box::new(Condition::Error("...".into())))
    /// }.satisfied_by(&task_state).unwrap_err();
    ///
    /// assert!(Condition::If {
    ///     r#if  : Box::new(Condition::Always),
    ///     then  : Box::new(Condition::Always),
    ///     r#else: None
    /// }.satisfied_by(&task_state).unwrap());
    ///
    /// assert!(!Condition::If {
    ///     r#if  : Box::new(Condition::Never),
    ///     then  : Box::new(Condition::Always),
    ///     r#else: None
    /// }.satisfied_by(&task_state).unwrap());
    /// ```
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] fails.
        r#else: Option<Box<Self>>
    },
    /// If the call to [`Self::satisfied_by`] passes or fails, invert it into failing or passing.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    Not(Box<Self>),
    /// If all contained [`Self`]s pass, passes.
    ///
    /// If any contained [`Self`] fails, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    All(Vec<Self>),
    /// If any contained [`Self`] passes, passes.
    ///
    /// If all contained [`Self`]s fail, fails.
    /// # Errors
    /// If any call to [`Self::satisfied_by`] returns an error, that error is returned.
    Any(Vec<Self>),



    /// Gets the value specified by [`Self::PartMap::part`], indexes [`Self::PartMap::map`], and returns the value of the returned [`Self`].
    ///
    /// If the call to [`Map::get`] returns [`None`], fails.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
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
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned.
    StringMap {
        /// The [`StringSource`] to index [`Self::StringMap::map`] with.
        value: StringSource,
        #[serde(flatten)]
        /// The [`Map`] to index with [`Self::StringMap::value`].
        map: Map<Self>
    },



    /// If the call to [`Self::satisfied_by`] returns an error, passes.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsPass {
        /// The [`Self`] to try to test.
        #[serde(flatten)]
        condition: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ConditionErrorFilter
    },
    /// If the call to [`Self::satisfied_by`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail {
        /// The [`Self`] to try to test.
        #[serde(flatten)]
        condition: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ConditionErrorFilter
    },
    /// If [`Self::TryElse::try`]'s call to [`Self::satisfied_by`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If both calls to [`Self::satisfied_by`] return errors, both errors are returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try'] returns an error.
        r#else: Box<Self>,
        /// The filter of which errors to catch.
        ///
        /// Defaults to all errors.
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ConditionErrorFilter
    },



    /// Passes if the value of [`UrlPart::Host`] is equal to the specified string.
    HostIs(Option<String>),
    /// Passes if the value of [`UrlPart::Subdomain`] is equal to the specified string.
    SubdomainIs(Option<String>),
    /// Passes if the value of [`UrlPart::RegDomain`] is equal to the specified string.
    RegDomainIs(Option<String>),
    /// Passes if the value of [`UrlPart::Domain`] is equal to the specified string.
    DomainIs(Option<String>),
    /// Passes if the value of [`UrlPart::DomainMiddle`] is equal to the specified string.
    DomainMiddleIs(Option<String>),
    /// Passes if the value of [`UrlPart::NotDomainSuffix`] is equal to the specified string.
    NotDomainSuffixIs(Option<String>),
    /// Passes if the value of [`UrlPart::DomainSuffix`] is equal to the specified string.
    DomainSuffixIs(Option<String>),
    /// Passes if the value of [`UrlPart::DomainSuffix`] is contained in the specified [`Set`].
    HostIsOneOf(Set<String>),
    /// Passes if the value of [`UrlPart::Host`] is [`Some`].
    UrlHasHost,
    /// Passes if the URL is a [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name).
    HostIsFqdn,
    /// Passes if the URL's host is a domain.
    HostIsDomain,
    /// Passes if the URL's host is an IP address.
    HostIsIp,
    /// Passes if the URL's host is an IPv4 address.
    HostIsIpv4,
    /// Passes if the URL's host is an IPv6 address.
    HostIsIpv6,



    /// Passes if the URL's query has at least one query parameter with the specified name.
    QueryHasParam(String),
    /// Passes if the [`UrlPart::Path`] starts with the specified value.
    PathStartsWith(String),
    /// Passes if the value of [`UrlPart::Path`] is the specified value.
    PathIs(String),



    /// Passes if the value of [`Self::PartIs::part`] is the same as the value of [`Self::PartIs::value`].
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    PartIs {
        /// The [`UrlPart`] to get.
        part: UrlPart,
        /// The [`StringSource`] to compare [`Self::PartIs::value`] with.
        value: StringSource
    },
    /// Passes if [`Self::PartContains::part`] contains [`Self::PartContains::value`] at [`Self::PartContains::at`].
    /// # Errors
    /// If the call to [`UrlPart::get`] returns [`None`], returns the error [`ConditionError::PartIsNone`].
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ConditionError::StringSourceIsNone`].
    ///
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
    PartContains {
        /// The part to look in.
        part: UrlPart,
        /// The value to look for.
        value: StringSource,
        /// Where to look in [`Self::PartContains::part`] for [`Self::PartContains::value`].
        ///
        /// Defaults to [`StringLocation::Anywhere`].
        #[serde(default)]
        at: StringLocation
    },
    /// Passes if [`Self::PartMatches::part`] satisfies [`Self::PartMatches::matcher`].
    /// # Errors
    /// If the call to [`UrlPart::get`] returns [`None`], returns the error [`ConditionError::PartIsNone`].
    ///
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    PartMatches {
        /// The part to match the value of.
        part: UrlPart,
        /// The matcher to test [`Self::PartMatches::part`] with.
        matcher: StringMatcher
    },
    /// Passes if [`Self::PartIsOneOf::part`] is in [`Self::PartIsOneOf::values`].
    PartIsOneOf {
        /// The part to check the value of.
        part: UrlPart,
        /// The set of values to check if [`Self::PartIsOneOf::part`] is one of.
        values: Set<String>
    },



    /// Passes if the specified flag is set.
    /// # Errors
    /// If the call to [`FlagRef::get`] returns an error, that error is returned.
    FlagIsSet(FlagRef),
    /// Passes if [`Self::VarIs::var`] is [`Self::VarIs::value`].
    /// # Errors
    /// If the call to [`VarRef::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    VarIs {
        /// The var to check the value of.
        #[serde(flatten)]
        var: VarRef,
        /// The value to check if [`Self::VarIs::var`] is.
        value: StringSource
    },



    /// Passes if [`Self::StringIs::left`] is [`Self::StringIs::right`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    StringIs {
        /// The left hand side of the equality check.
        left: StringSource,
        /// The right hand side of the equality check.
        right: StringSource
    },
    /// Passes if [`Self::StringContains::value`] contains [`Self::StringContains::substring`] at [`Self::StringContains::value`].
    /// # Errors
    /// If either call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If either call to [`StringSource::get`] returns [`None`], returns the error [`ConditionError::StringSourceIsNone`].
    ///
    /// If the call to [`StringLocation::satisfied_by`] returns an error, that error is returned.
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
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`ConditionError::StringSourceIsNone`].
    ///
    /// If the call to [`StringMatcher::satisfied_by`] returns an error, that error is returned.
    StringMatches {
        /// The value to check the value of.
        value: StringSource,
        /// The matcher to check if [`Self::StringMatches::value`] satisfies.
        matcher: StringMatcher
    },



    /// Get a [`Self`] from [`TaskStateView::commons`]'s [`Commons::conditions`] and pass if it's satisfied.
    /// # Errors
    /// If [`CommonCall::name`]'s call to [`StringSource::get`] returns an error, returns the error [`StringSourceError::StringSourceIsNone`].
    ///
    /// If [`TaskStateView::commons`]'s [`Commons::conditions`] doesn't contain a [`Self`] with the specified name, returns the error [`ConditionError::CommonConditionNotFound`].
    ///
    /// If the call to [`CommonCallArgsSource::build`] returns an error, that error is returned.
    Common(CommonCall),
    /// Calls the specified function and returns its value.
    /// # Errors
    /// If the call to the contained function returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&TaskStateView) -> Result<bool, ConditionError>)
}

/// The enum of errors [`Condition::satisfied_by`] can return.
#[derive(Debug, Error, ErrorFilter)]
pub enum ConditionError {
    /// Returned when a [`Condition::Error`] is used.
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    /// Returned when a part of the URL is [`None`] where it has to be [`Some`].
    #[error("A part of the URL is None where it had to be Some.")]
    PartIsNone,
    /// Returned when a [`StringSource`] returned [`None`] where it has to return [`Some`].
    #[error("A StringSource returned None where it had to return Some.")]
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
    /// Returned when both [`Condition`]s in a [`Condition::TryElse`] return errors.
    #[error("Both Conditions in a Condition::TryElse returned errors.")]
    TryElseError {
        /// The error returned by [`Condition::TryElse::try`]. 
        try_error: Box<Self>,
        /// The error returned by [`Condition::TryElse::else`]. 
        else_error: Box<Self>
    },
    /// Returned when a [`UrlDoesNotHavePathSegments`] is returned.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when a [`Condition`] with the specified name isn't found in the [`Commons::conditions`].
    #[error("A Condition with the specified name wasn't found in the Commons::conditions.")]
    CommonConditionNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered/
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// An arbitrary [`std::error::Error`] returned by [`Condition::Custom`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>),
    /// Returned when a [`GetFlagError`] is encountered.
    #[error(transparent)]
    GetFlagError(#[from] GetFlagError),
    /// Returned when a [`GetVarError`] is encountered.
    #[error(transparent)]
    GetVarError(#[from] GetVarError)
}

impl Condition {
    /// If the specified variant of [`Self`] passes, return [`true`], otherwise return [`false`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn satisfied_by(&self, task_state: &TaskStateView) -> Result<bool, ConditionError> {
        debug!(self, Condition::satisfied_by, self, task_state);
        Ok(match self {
            // Debug/constants.

            Self::Always => true,
            Self::Never => false,
            Self::Error(msg) => Err(ConditionError::ExplicitError(msg.clone()))?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(task_state);
                eprintln!("=== Condition::Debug ===\nCondition: {condition:?}\ntask_state: {task_state:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Logic.

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(task_state)? {
                then.satisfied_by(task_state)?
            } else if let Some(r#else) = r#else {
                r#else.satisfied_by(task_state)?
            } else {
                false
            },
            Self::Not(condition) => !condition.satisfied_by(task_state)?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.satisfied_by(task_state)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.satisfied_by(task_state)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::PartMap  {part , map} => map.get(part .get(task_state.url) ).map(|x| x.satisfied_by(task_state)).unwrap_or(Ok(false))?,
            Self::StringMap{value, map} => map.get(value.get(task_state    )?).map(|x| x.satisfied_by(task_state)).unwrap_or(Ok(false))?,

            // Error handling.

            Self::TreatErrorAsPass {condition, filter} => match condition.satisfied_by(task_state) {Ok(x) => x, Err(e) => if filter.matches(&e) {true } else {Err(e)?}},
            Self::TreatErrorAsFail {condition, filter} => match condition.satisfied_by(task_state) {Ok(x) => x, Err(e) => if filter.matches(&e) {false} else {Err(e)?}},
            Self::TryElse{ r#try, filter, r#else } => match r#try.satisfied_by(task_state) {
                Ok(x) => x,
                Err(try_error) => if filter.matches(&try_error) {
                    match r#else.satisfied_by(task_state) {
                        Ok(x) => x,
                        Err(else_error) => Err(ConditionError::TryElseError {try_error: Box::new(try_error), else_error: Box::new(else_error)})?
                    }
                } else {
                    Err(try_error)?
                }
            },

            // Domain conditions.

            Self::HostIs           (x) => UrlPart::Host           .get(task_state.url).as_deref() == x.as_deref(),
            Self::SubdomainIs      (x) => UrlPart::Subdomain      .get(task_state.url).as_deref() == x.as_deref(),
            Self::RegDomainIs      (x) => UrlPart::RegDomain      .get(task_state.url).as_deref() == x.as_deref(),
            Self::DomainIs         (x) => UrlPart::Domain         .get(task_state.url).as_deref() == x.as_deref(),
            Self::DomainMiddleIs   (x) => UrlPart::DomainMiddle   .get(task_state.url).as_deref() == x.as_deref(),
            Self::NotDomainSuffixIs(x) => UrlPart::NotDomainSuffix.get(task_state.url).as_deref() == x.as_deref(),
            Self::DomainSuffixIs   (x) => UrlPart::DomainSuffix   .get(task_state.url).as_deref() == x.as_deref(),

            Self::HostIsOneOf(hosts) => hosts.contains(task_state.url.host_str()),

            Self::UrlHasHost   => task_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(task_state.url.host_details(), Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..}))),
            Self::HostIsDomain => matches!(task_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            // Specific parts.

            Self::QueryHasParam(name) => task_state.url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathStartsWith(value) => task_state.url.path().starts_with(value),
            Self::PathIs(value) => task_state.url.path() == value,

            // General parts.

            Self::PartIs       {part, value    } => part.get(task_state.url).as_deref() == value.get(task_state)?.as_deref(),
            Self::PartContains {part, value, at} => at.satisfied_by(&part.get(task_state.url).ok_or(ConditionError::PartIsNone)?, get_str!(value, task_state, ConditionError))?,
            Self::PartMatches  {part, matcher  } => matcher.satisfied_by(&part.get(task_state.url).ok_or(ConditionError::PartIsNone)?, task_state)?,
            Self::PartIsOneOf  {part, values   } => values .contains    ( part.get(task_state.url).as_deref()),

            // Miscellaneous.

            Self::FlagIsSet(flag)    => flag.get(task_state)?,
            Self::VarIs {var, value} => var .get(task_state)?.as_deref() == value.get(task_state)?.as_deref(),

            // String source.

            Self::StringIs {left, right} => left.get(task_state)? == right.get(task_state)?,
            Self::StringContains {value, substring, at} => at.satisfied_by(get_str!(value, task_state, ConditionError), get_str!(substring, task_state, ConditionError))?,
            Self::StringMatches {value, matcher} => matcher.satisfied_by(get_str!(value, task_state, ConditionError), task_state)?,

            // Misc.

            Self::Common(common_call) => {
                task_state.commons.conditions.get(get_str!(common_call.name, task_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.satisfied_by(&TaskStateView {
                    common_args: Some(&common_call.args.build(task_state)?),
                    url        : task_state.url,
                    scratchpad : task_state.scratchpad,
                    context    : task_state.context,
                    job_context: task_state.job_context,
                    params     : task_state.params,
                    commons    : task_state.commons,
                    #[cfg(feature = "cache")]
                    cache      : task_state.cache
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

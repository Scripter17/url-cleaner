//! Logic for when a [`TaskState`] should be modified.

use std::collections::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Used when the commands feature is enabled.")]
use crate::glue::*;
use crate::types::*;
use crate::util::*;

/// Conditions that decide if and when to apply a [`Mapper`].
///
/// "Pass" means [`Condition::satisfied_by`] returns `Ok(true)` and "fail" means it returns `Ok(false)`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Condition {
    /// Always passes.
    Always,
    /// Always fails.
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`] with the included message.
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    Error(String),
    /// Prints debug info about the contained [`Self`], then returns its return value.
    /// # Errors
    /// If the call to [`Self::satisfied_by`] returns an error, that error is returned after the debug info is printed.
    #[suitable(never)]
    Debug(Box<Self>),
    /// If the call to [`Self::If::if`] passes, return the value of [`Self::If::then`].
    ///
    /// If the call to [`Self::If::if`] fails, return the value of [`Self::if::else`].
    /// # Errors
    /// If any call to [`Self::satisifed_by`] returns an error, that error is returned.
    If {
        /// The [`Self`] to decide between [`Self::If::then`] and [`Self::If::else`].
        r#if: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] passes.
        then: Box<Self>,
        /// The [`Self`] to use if [`Self::If::if`] fails.
        r#else: Box<Self>
    },
    /// If the call to [`Self::satisifed_by`] passes or fails, invert it into failing or passing.
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
        #[serde(flatten)]
        condition: Box<Self>,
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ConditionErrorFilter
    },
    /// If the call to [`Self::satisfied_by`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail {
        #[serde(flatten)]
        condition: Box<Self>,
        #[serde(default, skip_serializing_if = "is_default")]
        filter: ConditionErrorFilter
    },
    /// If [`Self::TryElse::try`]'s call to [`Self::satisfied_by`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If [`Self::TryElse::else`]'s call to [`Self::satisifed_by`] returns an error, that error is returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try']'s call to [`Self::satisfied_by`] returns an error.
        r#else: Box<Self>,
        /// The set of errors [`Self::TryElse::try`] can return that will trigger [`Self::TryElse::else`].
        ///
        /// If [`None`], all errors will trigger [`Self::TryElse::else`].
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
    /// Passes if the value of [`UrlPart::DomainSuffix`] is contained in the specified [`HashSet`].
    HostIsOneOf(HashSet<String>),
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
    /// Passes if the specified part equals the specified string.
    ///
    /// If the call to [`UrlPart::get`] returns [`None`], return the value of [`Self::PartContains::if_part_null`].
    ///
    /// If the call to [`StringSource::get`] returns [`None`], return the value of [`Self::PartContains::if_value_null`].
    /// # Errors
    /// If the call to [`UrlPart::get`] returns [`None`] and [`Self::PartContains::if_part_null`] is [`IfError::Error`], returns the error [`ConditionError::PartIsNone`].
    ///
    /// If the call to [`StringSource::get`] returns [`None`] and [`Self::PartContains::if_value_null`] is [`IfError::Error`], returns the error [`ConditionError::StringSourceIsNone`].
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
        r#where: StringLocation,
        /// What to do if the call to [`UrlPart:;get`] returns [`None`].
        ///
        /// Defaults to [`IfError::Error`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_part_null: IfError,
        /// What to do if the call to [`StringSource:;get`] returns [`None`].
        ///
        /// Defaults to [`IfError::Error`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_value_null: IfError
    },
    PartMatches {
        part: UrlPart,
        matcher: StringMatcher,
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: IfError
    },
    PartIsOneOf {
        part: UrlPart,
        values: HashSet<String>,
        #[serde(default)]
        if_null: bool
    },



    FlagIsSet(FlagRef),
    VarIs {
        #[serde(flatten)]
        name: VarRef,
        value: StringSource
    },



    StringIs {
        left: StringSource,
        right: StringSource
    },
    StringContains {
        value: StringSource,
        substring: StringSource,
        #[serde(default)]
        r#where: StringLocation
    },
    StringMatches {
        value: StringSource,
        matcher: StringMatcher
    },



    #[cfg(feature = "commands")]
    CommandExists(CommandConfig),
    #[cfg(feature = "commands")]
    CommandExitStatus {
        command: CommandConfig,
        #[serde(default)]
        expected: i32
    },
    Common(CommonCall),
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    Custom(FnWrapper<fn(&TaskStateView) -> Result<bool, ConditionError>>)
}

#[derive(Debug, Error, url_cleaner_macros::ErrorFilter)]
pub enum ConditionError {
    #[error("Explicit error: {0}")]
    ExplicitError(String),
    #[error("The provided URL does not have the requested part.")]
    PartIsNone,
    #[cfg(feature = "commands")]
    #[error(transparent)]
    CommandError(#[from] CommandError),
    #[error("The specified StringSource returned None.")]
    StringSourceIsNone,
    #[error(transparent)]
    StringMatcherError(#[from] StringMatcherError),
    #[error(transparent)]
    StringLocationError(#[from] StringLocationError),
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    #[error("A `Condition::TryElse` had both `try` and `else` return an error.")]
    TryElseError {
        try_error: Box<Self>,
        else_error: Box<Self>
    },
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    #[error("Not in a common context.")]
    NotInACommonContext,
    #[error("The common Condition was not found.")]
    CommonConditionNotFound,
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    GetFlagError(#[from] GetFlagError),
    #[error(transparent)]
    GetVarError(#[from] GetVarError)
}

impl Condition {
    /// If the specified variant of [`Self`] passes, return [`true`], otherwise return [`false`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn satisfied_by(&self, task_state: &TaskStateView) -> Result<bool, ConditionError> {
        debug!(Condition::satisfied_by, self, task_state);
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

            Self::If {r#if, then, r#else} => if r#if.satisfied_by(task_state)? {then} else {r#else}.satisfied_by(task_state)?,
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

            Self::HostIsOneOf(hosts) => task_state.url.host_str().is_some_and(|url_host| hosts.contains(url_host)),

            Self::UrlHasHost   => task_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(task_state.url.host_details(), Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..}))),
            Self::HostIsDomain => matches!(task_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(task_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            // Specific parts.

            Self::QueryHasParam(name) => task_state.url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathIs(value) => task_state.url.path() == value,

            // General parts.

            Self::PartIs{part, value} => part.get(task_state.url).as_deref() == value.get(task_state)?.as_deref(),
            Self::PartContains{part, value, r#where, if_part_null, if_value_null} => match part.get(task_state.url) {
                None    => if_part_null.apply(Err(ConditionError::PartIsNone))?,
                Some(part) => match value.get(task_state)? {
                    None        => if_value_null.apply(Err(ConditionError::StringSourceIsNone))?,
                    Some(value) => r#where.satisfied_by(&part, &value)?,
                }
            },
            Self::PartMatches {part, matcher, if_null} => match part.get(task_state.url) {
                None    => if_null.apply(Err(ConditionError::PartIsNone))?,
                Some(x) => matcher.satisfied_by(&x, task_state)?,
            },
            Self::PartIsOneOf {part, values, if_null} => part.get(task_state.url).map(|x| values.contains(&*x)).unwrap_or(*if_null),

            // Miscellaneous.

            Self::FlagIsSet(flag)     => flag.get(task_state)?,
            Self::VarIs {name, value} => name.get(task_state)?.as_deref() == value.get(task_state)?.as_deref(),

            // String source.

            Self::StringIs {left, right} => left.get(task_state)? == right.get(task_state)?,
            Self::StringContains {value, substring, r#where} => r#where.satisfied_by(get_str!(value, task_state, ConditionError), get_str!(substring, task_state, ConditionError))?,
            Self::StringMatches {value, matcher} => matcher.satisfied_by(get_str!(value, task_state, ConditionError), task_state)?,

            // Commands.

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(task_state)?==expected},

            Self::Common(common_call) => {
                task_state.commons.conditions.get(get_str!(common_call.name, task_state, ConditionError)).ok_or(ConditionError::CommonConditionNotFound)?.satisfied_by(&TaskStateView {
                    url: task_state.url,
                    context: task_state.context,
                    params: task_state.params,
                    scratchpad: task_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: task_state.cache,
                    commons: task_state.commons,
                    common_args: Some(&common_call.args.build(task_state)?),
                    job_context: task_state.job_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

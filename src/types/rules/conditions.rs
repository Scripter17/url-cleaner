//! Logic for when a [`JobState`] should be modified.

use std::collections::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};

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
    /// Always returns the error [`ConditionError::ExplicitError`].
    /// # Errors
    /// Always returns the error [`ConditionError::ExplicitError`].
    Error,
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
    TreatErrorAsPass(Box<Self>),

    /// If the call to [`Self::satisfied_by`] returns an error, fails.
    ///
    /// Otherwise returns the value of the contained [`Self`].
    TreatErrorAsFail(Box<Self>),
    /// If [`Self::TryElse::try`]'s call to [`Self::satisfied_by`] returns an error, return the value of [`Self::TryElse::else`].
    /// # Errors
    /// If [`Self::TryElse::else`]'s call to [`Self::satisifed_by`] returns an error, that error is returned.
    TryElse {
        /// The [`Self`] to try first.
        r#try: Box<Self>,
        /// The [`Self`] to try if [`Self::TryElse::try']'s call to [`Self::satisfied_by`] returns an error.
        r#else: Box<Self>
    },
    /// Return the value of the first [`Self`] that doesn't return an error.
    /// # Errors
    /// All calls to [`Self::satisfied_by`] return an error, the last error is returned.
    FirstNotError(Vec<Self>),



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



    AnyFlagIsSet,
    FlagIsSet(#[suitable(assert = "flag_is_documented")] StringSource),
    CommonFlagIsSet(StringSource),
    ScratchpadFlagIsSet(StringSource),
    VarIs {
        #[suitable(assert = "var_is_documented")]
        name: StringSource,
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
    Custom(FnWrapper<fn(&JobStateView) -> Result<bool, ConditionError>>)
}

#[derive(Debug, Error)]
pub enum ConditionError {
    #[error("Condition::Error was used.")]
    ExplicitError,
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
    Custom(Box<dyn std::error::Error + Send>)
}

impl Condition {
    /// If the specified variant of [`Self`] passes, return [`true`], otherwise return [`false`].
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
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

            Self::HostIs           (x) => UrlPart::Host           .get(job_state.url).as_deref() == x.as_deref(),
            Self::SubdomainIs      (x) => UrlPart::Subdomain      .get(job_state.url).as_deref() == x.as_deref(),
            Self::RegDomainIs      (x) => UrlPart::RegDomain      .get(job_state.url).as_deref() == x.as_deref(),
            Self::DomainIs         (x) => UrlPart::Domain         .get(job_state.url).as_deref() == x.as_deref(),
            Self::DomainMiddleIs   (x) => UrlPart::DomainMiddle   .get(job_state.url).as_deref() == x.as_deref(),
            Self::NotDomainSuffixIs(x) => UrlPart::NotDomainSuffix.get(job_state.url).as_deref() == x.as_deref(),
            Self::DomainSuffixIs   (x) => UrlPart::DomainSuffix   .get(job_state.url).as_deref() == x.as_deref(),

            Self::HostIsOneOf(hosts) => job_state.url.host_str().is_some_and(|url_host| hosts.contains(url_host)),

            Self::UrlHasHost   => job_state.url.host().is_some(),
            Self::HostIsFqdn   => matches!(job_state.url.host_details(), Some(HostDetails::Domain(d @ DomainDetails {..})) if d.is_fqdn()),
            Self::HostIsDomain => matches!(job_state.url.host_details(), Some(HostDetails::Domain(_))),
            Self::HostIsIp     => matches!(job_state.url.host_details(), Some(HostDetails::Ipv4(_) | HostDetails::Ipv6(_))),
            Self::HostIsIpv4   => matches!(job_state.url.host_details(), Some(HostDetails::Ipv4(_))),
            Self::HostIsIpv6   => matches!(job_state.url.host_details(), Some(HostDetails::Ipv6(_))),

            // Specific parts.

            Self::QueryHasParam(name) => job_state.url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::PathIs(value) => job_state.url.path() == value,

            // General parts.

            Self::PartIs{part, value} => part.get(job_state.url).as_deref() == value.get(job_state)?.as_deref(),
            Self::PartContains{part, value, r#where, if_part_null, if_value_null} => match part.get(job_state.url) {
                None    => if_part_null.apply(Err(ConditionError::PartIsNone))?,
                Some(part) => match value.get(job_state)? {
                    None        => if_value_null.apply(Err(ConditionError::StringSourceIsNone))?,
                    Some(value) => r#where.satisfied_by(&part, &value)?,
                }
            },
            Self::PartMatches {part, matcher, if_null} => match part.get(job_state.url) {
                None    => if_null.apply(Err(ConditionError::PartIsNone))?,
                Some(x) => matcher.satisfied_by(&x, job_state)?,
            },
            Self::PartIsOneOf {part, values, if_null} => part.get(job_state.url).map(|x| values.contains(&*x)).unwrap_or(*if_null),

            // Miscellaneous.

            Self::CommonFlagIsSet(name) => job_state.common_args.ok_or(ConditionError::NotInACommonContext)?.flags.contains(get_str!(name, job_state, ConditionError)),
            Self::ScratchpadFlagIsSet(name) => job_state.scratchpad.flags.contains(get_str!(name, job_state, ConditionError)),
            Self::FlagIsSet(name) => job_state.params.flags.contains(get_str!(name, job_state, ConditionError)),
            Self::AnyFlagIsSet => !job_state.params.flags.is_empty(),
            Self::VarIs {name, value} => job_state.params.vars.get(get_str!(name, job_state, ConditionError)).map(|x| &**x) == value.get(job_state)?.as_deref(),

            // String source.

            Self::StringIs {left, right} => left.get(job_state)? == right.get(job_state)?,
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
                    common_args: Some(&common_call.args.build(job_state)?),
                    jobs_context: job_state.jobs_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        })
    }
}

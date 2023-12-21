use std::borrow::Cow;

use thiserror::Error;
use serde::Deserialize;
use url::Url;
use std::convert::identity;

use crate::glue;
use crate::types::UrlPartName;

#[derive(Debug, Deserialize, Clone)]
pub enum Condition {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`]
    Error,
    /// Logs the result of the contained condition then propagates any error.
    Debug(Box<Condition>),
    /// If the contained condition returns an error, treat it as a pass.
    TreatErrorAsPass(Box<Condition>),
    /// If the contained condition returns an error, treat it as a fail.
    TreatErrorAsFail(Box<Condition>),
    /// Passes if all of the included conditions pass.
    All(Vec<Condition>),
    /// Passes if any of the included conditions pass.
    Any(Vec<Condition>),
    /// Passes if the included condition doesn't and vice-versa.
    Not(Box<Condition>),
    /// Passes if the URL's domain is or is a subdomain of the specified domain.
    UnqualifiedDomain(String),
    /// Passes if the URL's domain is the specified domain.
    QualifiedDomain(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is or is a subdomain of the specified domain fragment.
    UnqualifiedAnyTld(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is the specified domain fragment.
    QualifiedAnyTld(String),
    /// Passes if the URL's path is the specified string.
    PathIs(String),
    /// Passes if the URL has a query of the specified name.
    QueryHasParam(String),
    /// Passes if the URL has a query of the specified name and its value is the specified value.
    QueryParamValueIs {
        name: String,
        value: String
    },
    /// Passes if the value of the specified part of the URL is the specified value.
    UrlPartIs {
        part_name: UrlPartName,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        value: String
    },
    // Disablable conditions
    /// Passes if the URL has a query of the specified name and its value matches the specified regular expression.
    QueryParamValueMatchesRegex {
        name: String,
        regex: glue::Regex
    },
    /// Passes if the URL has a query of the specified name and its value matches the specified glob.
    QueryParamValueMatchesGlob {
        name: String,
        glob: glue::Glob
    },
    /// Passes if the URL's path matches the specified regular expression.
    PathMatchesRegex(glue::Regex),
    /// Passes if the URL's path matches the specified glob.
    PathMatchesGlob(glue::Glob),
    /// Takes the specified part of the URL and passes if it matches the specified regular expression.
    /// if `none_to_empty_string` is `false`, then getting the host, domain, query, or fragment may result in a [`ConditionError::UrlPartNotFound`] error.
    UrlPartMatchesRegex {
        part_name: UrlPartName,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        regex: glue::Regex
    },
    /// Takes the specified part of the URL and passes if it matches the specified glob.
    /// if `none_to_empty_string` is `false`, then getting the host, domain, query, or fragment may result in a [`ConditionError::UrlPartNotFound`] error.
    UrlPartMatchesGlob {
        part_name: UrlPartName,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        glob: glue::Glob
    },
    CommandExitStatus {
        command: glue::Command,
        #[serde(default)]
        expected: i32
    }
}

fn get_true() -> bool {true}

#[derive(Error, Debug)]
pub enum ConditionError {
    #[allow(dead_code)]
    #[error("Url-cleaner was compiled without support for this condition.")]
    /// The required condition was disabled at compile time. This can apply to any condition that uses regular expressions or globs.
    ConditionDisabled,
    /// The [`Condition::Error`] condition always returns this error.
    #[error("The \"Error\" condition always returns this error.")]
    ExplicitError,
    #[error("The provided URL does not contain the requested part.")]
    /// The provided URL does not contain the requested part.
    /// See [`crate::types::UrlPartName`] for details.
    UrlPartNotFound,
    #[error("The command failed to run.")]
    /// Returned when the specified command failed to run.
    CommandError(#[from] glue::CommandError)
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// Returns an error if the condition is disabled or the URL part requested by the condition isn't found.
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(url);
                eprintln!("=== Debug Condition output ===\nCondition: {condition:?}\nURL: {url:?}\nCondition satisfied by URL: {is_satisfied:?}");
                is_satisfied?
            }
            Self::TreatErrorAsPass(condition) => condition.satisfied_by(url).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by(url).unwrap_or(false),
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Not(condition) => !condition.satisfied_by(url)?,
            Self::UnqualifiedDomain(parts) => match url.domain() {
                Some(domain) => domain.split(".").collect::<Vec<_>>().ends_with(&parts.split(".").collect::<Vec<_>>()),
                None => false
            },
            Self::QualifiedDomain(parts) => match url.domain() {
                Some(domain) => domain==parts,
                None => false
            },
            Self::UnqualifiedAnyTld(name) => {
                match url.domain() {
                    Some(domain) => {
                        match domain.split('.').collect::<Vec<_>>().as_slice() {
                            // All ASCII ccTLD identifiers are two letters long, and all two-letter top-level domains are ccTLDs. - https://en.wikipedia.org/wiki/Country_code_top-level_domain.
                            // I'm just hoping nobody using this ever registers google.whatever.uk and nobody ever tries to sanitize a URL from that domain.
                            [.., name2, _, cctld] => name==name2 && cctld.len()==2,
                            [.., name2, _       ] => name==name2,
                            _                     => false
                        }
                    }
                    None => false
                }
            },
            Self::QualifiedAnyTld(name) => {
                match url.domain() {
                    Some(domain) => {
                        match domain.strip_prefix(name) {
                            Some(partial_domain) => {
                                match partial_domain.split('.').collect::<Vec<_>>().as_slice() {
                                    [_, cctld] => cctld.len()==2,
                                    [_       ] => true,
                                    _          => false
                                }
                            },
                            None => false
                        }
                    },
                    None => false
                }
            },
            Self::PathIs(path) => path==url.path(),
            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::QueryParamValueIs{name, value} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && value2==value),
            Self::UrlPartIs{part_name, none_to_empty_string, value} => value==part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref(),
            // Disablable conditions
            Self::QueryParamValueMatchesRegex{name, regex} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && regex.is_match(value2)),
            Self::QueryParamValueMatchesGlob {name, glob } => url.query_pairs().any(|(ref name2, ref value2)| name2==name && glob .matches (value2)),
            Self::PathMatchesRegex(regex) => regex.is_match(url.path()),
            Self::PathMatchesGlob (glob ) => glob  .matches(url.path()),
            Self::UrlPartMatchesRegex {part_name, none_to_empty_string, regex} => regex.is_match(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            Self::UrlPartMatchesGlob {part_name, none_to_empty_string, glob} => glob.matches(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            Self::CommandExitStatus {command, expected} => {
                &command.exit_code(url)?==expected
            }
        })
    }
}

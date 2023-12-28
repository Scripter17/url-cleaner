use std::borrow::Cow;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;
use std::convert::identity;
use publicsuffix::Psl;

use crate::glue;
use crate::types::{UrlPartName, DomainConditionRule};

/// The part of a [`crate::rules::Rule`] that specifies when the rule's mapper will be applied.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Condition {
    /// Always passes.
    Always,
    /// Never passes.
    Never,
    /// Always returns the error [`ConditionError::ExplicitError`]
    Error,
    /// Prints debugging information about the contained condition to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as bash and batch only have `x | y` pipe STDOUT by default, but it'll look ugly.
    Debug(Box<Condition>),
    /// If the contained condition returns an error, treat it as a pass.
    TreatErrorAsPass(Box<Condition>),
    /// If the contained condition returns an error, treat it as a fail.
    TreatErrorAsFail(Box<Condition>),
    /// If the `try` condition reuterns an error, use the `else` condition instead.
    /// Noe that the `else` condition can still return an error, in which case this whole condition returns that error.
    IfErrorThen {
        r#try: Box<Condition>,
        r#else: Box<Condition>
    },
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

    DomainCondition {
        yes_domains: Vec<String>,
        yes_domain_regexes: Vec<glue::RegexWrapper>,
        unless_domains: Vec<String>,
        unless_domain_regexes: Vec<glue::RegexWrapper>
    },

    /// Passes if the URL has a query of the specified name and its value matches the specified regular expression.
    /// Requires the `regex` feature to be enabled.
    QueryParamValueMatchesRegex {
        name: String,
        regex: glue::RegexWrapper
    },
    /// Passes if the URL has a query of the specified name and its value matches the specified glob.
    /// Requires the `glob` feature to be enabled.
    QueryParamValueMatchesGlob {
        name: String,
        glob: glue::GlobWrapper
    },
    /// Passes if the URL's path matches the specified regular expression.
    /// Requires the `regex` feature to be enabled.
    PathMatchesRegex(glue::RegexWrapper),
    /// Passes if the URL's path matches the specified glob.
    /// Requires the `glob` feature to be enabled.
    PathMatchesGlob(glue::GlobWrapper),
    /// Takes the specified part of the URL and passes if it matches the specified regular expression.
    /// if `none_to_empty_string` is `false`, then getting the host, domain, query, or fragment may result in a [`ConditionError::UrlPartNotFound`] error.
    /// Requires the `regex` feature to be enabled.
    UrlPartMatchesRegex {
        part_name: UrlPartName,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        regex: glue::RegexWrapper
    },
    /// Takes the specified part of the URL and passes if it matches the specified glob.
    /// if `none_to_empty_string` is `false`, then getting the host, domain, query, or fragment may result in a [`ConditionError::UrlPartNotFound`] error.
    /// Requires the `glob` feature to be enabled.
    UrlPartMatchesGlob {
        part_name: UrlPartName,
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        glob: glue::GlobWrapper
    },
    /// Checks the contained comand's [`glue::CommandWrapper::exists`], which uses [this StackOverflow](https://stackoverflow.com/a/37499032/10720231) post to check the system's PATH
    /// Requires the `commands` feature to be enabled.
    CommandExists (glue::CommandWrapper),
    /// Runs the specified [`glue::CommandWrapper`] and passes if its exit code equals `expected` (which defaults to `0`).
    /// Requires the `commands` feature to be enabled.
    CommandExitStatus {
        command: glue::CommandWrapper,
        #[serde(default)]
        expected: i32
    }
}

fn get_true() -> bool {true}

#[derive(Error, Debug)]
pub enum ConditionError {
    /// The required condition was disabled at compile time. This can apply to any condition that uses regular expressions, globs, or commands.
    #[allow(dead_code)]
    #[error("Url-cleaner was compiled without support for this condition.")]
    ConditionDisabled,
    /// The [`Condition::Error`] condition always returns this error.
    #[error("The \"Error\" condition always returns this error.")]
    ExplicitError,
    /// The provided URL does not contain the requested part.
    /// See [`crate::types::UrlPartName`] for details.
    #[error("The provided URL does not contain the requested part.")]
    UrlPartNotFound,
    /// Returned when the specified command failed to run.
    #[error("The command failed to run.")]
    CommandError(#[from] glue::CommandError)
}

impl Condition {
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        self.satisfied_by_with_dcr(url, &DomainConditionRule::default())
    }
    
    /// Checks whether or not the provided URL passes the condition.
    /// Returns an error if the condition is disabled, the URL part requested by the condition isn't found, or if the condition is [`Condition::Error`].
    pub fn satisfied_by_with_dcr(&self, url: &Url, dcr: &DomainConditionRule) -> Result<bool, ConditionError> {
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
            Self::IfErrorThen{r#try, r#else}  => r#try.satisfied_by(url).or_else(|_| r#else.satisfied_by(url))?,
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Not(condition) => !condition.satisfied_by(url)?,
            Self::UnqualifiedDomain(parts) => match url.domain() {
                Some(domain) => domain.split('.').collect::<Vec<_>>().ends_with(&parts.split('.').collect::<Vec<_>>()),
                None => false
            },
            Self::QualifiedDomain(parts) => match url.domain() {
                Some(domain) => domain==parts,
                None => false
            },
            Self::UnqualifiedAnyTld(name) => {
                // if let Some(domain) = url.domain() {
                //     match domain.split('.').collect::<Vec<_>>().as_slice() {
                //         // All ASCII ccTLD identifiers are two letters long, and all two-letter top-level domains are ccTLDs. - https://en.wikipedia.org/wiki/Country_code_top-level_domain.
                //         // I'm just hoping nobody using this ever registers google.whatever.uk and nobody ever tries to sanitize a URL from that domain.
                //         [.., name2, _, cctld] => name==name2 && cctld.len()==2,
                //         [.., name2, _       ] => name==name2,
                //         _                     => false
                //     }
                // } else {
                //     false
                // }
                match url.domain() {
                    Some(url_domain) => match crate::suffix::TLDS.get().unwrap().domain(url_domain.as_bytes()) {
                        Some(parsed_domain) => {
                            println!("{parsed_domain:?}");
                            url.domain().unwrap().as_bytes().strip_suffix(parsed_domain.suffix().as_bytes()).unwrap().strip_suffix(b".").unwrap().split(|x| *x==b'.').collect::<Vec<_>>()
                                .ends_with(&name.as_bytes().split(|x| *x==b'.').collect::<Vec<_>>())
                        },
                        None => false
                    },
                    None => false
                }
            },
            Self::QualifiedAnyTld(name) => {
                // if let Some(partial_domain) = url.domain().and_then(|domain| domain.strip_prefix(name)) {
                //     match partial_domain.split('.').collect::<Vec<_>>().as_slice() {
                //         [_, cctld] => cctld.len()==2,
                //         [_       ] => true,
                //         _          => false
                //     }
                // } else {
                //     false
                // }
                match url.domain() {
                    Some(url_domain) => match crate::suffix::TLDS.get().unwrap().domain(url_domain.as_bytes()) {
                        Some(parsed_domain) => {
                            println!("{parsed_domain:?}");
                            url.domain().unwrap().as_bytes().strip_suffix(parsed_domain.suffix().as_bytes()).unwrap().strip_suffix(b".").unwrap()==name.as_bytes()
                        },
                        None => false
                    },
                    None => false
                }
                // match crate::suffix::TLDS.get().unwrap().domain(name.as_bytes()) {
                //     Some(domain) => url.domain().unwrap().as_bytes()==domain.as_bytes().strip_suffix(domain.suffix().as_bytes()).unwrap(),
                //     None => false
                // }
            },
            Self::PathIs(path) => path==url.path(),
            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::QueryParamValueIs{name, value} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && value2==value),
            Self::UrlPartIs{part_name, none_to_empty_string, value} => value==part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref(),
            
            // Disablable conditions

            Self::DomainCondition {yes_domains, yes_domain_regexes, unless_domains, unless_domain_regexes} => {
                match dcr {
                    DomainConditionRule::Always => true,
                    DomainConditionRule::Never => false,
                    DomainConditionRule::Url(url) => {
                        if let Some(host)=url.host_str() {
                            (yes_domains.iter().any(|domain| domain==host) || yes_domain_regexes.iter().any(|regex| regex.is_match(host))) &&
                                !(unless_domains.iter().any(|domain| domain==host) || unless_domain_regexes.iter().any(|regex| regex.is_match(host)))
                        } else {
                            false
                        }
                    },
                    DomainConditionRule::UseUrlBeingCleaned => {
                        if let Some(host)=url.host_str() {
                            (yes_domains.iter().any(|domain| domain==host) || yes_domain_regexes.iter().any(|regex| regex.is_match(host))) &&
                                !(unless_domains.iter().any(|domain| domain==host) || unless_domain_regexes.iter().any(|regex| regex.is_match(host)))
                        } else {
                            false
                        }
                    },
                }
            }

            Self::QueryParamValueMatchesRegex{name, regex} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && regex.is_match(value2)),
            Self::QueryParamValueMatchesGlob {name, glob } => url.query_pairs().any(|(ref name2, ref value2)| name2==name && glob .matches (value2)),
            Self::PathMatchesRegex(regex) => regex.is_match(url.path()),
            Self::PathMatchesGlob (glob ) => glob  .matches(url.path()),
            Self::UrlPartMatchesRegex {part_name, none_to_empty_string, regex} => regex.is_match(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            Self::UrlPartMatchesGlob {part_name, none_to_empty_string, glob} => glob.matches(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Owned("".to_string()))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            Self::CommandExists (command) => command.exists(),
            Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected}
        })
    }
}

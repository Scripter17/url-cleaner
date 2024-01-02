//! The logic for when to modify a URL.

use std::borrow::Cow;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;
use std::convert::identity;
use publicsuffix::Psl;

use crate::glue;
use crate::types::{UrlPartName, DomainConditionRule};

/// The part of a [`crate::rules::Rule`] that specifies when the rule's mapper will be applied.
/// Note that conditions are checked by the output of the previous mapper.
/// A `Mapper::SwapHost` will make `Condition::UnqualifiedDomain` match on the host that was swapped in.
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
    /// If the `try` condition reuterns an error, return the result of the `else` condition instead.
    /// Note that the `else` condition can still return an error, in which case this whole condition returns that error.
    TryCatch {
        /// The condition to try first.
        r#try: Box<Condition>,
        /// If the try condition fails, instead return the result of this one.
        catch: Box<Condition>
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
        /// The name of the query paramater.
        name: String,
        /// The expected value of the query paramater.
        value: String
    },
    /// Passes if the value of the specified part of the URL is the specified value.
    UrlPartIs {
        /// The name of the part to check.
        part_name: UrlPartName,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The expected value of the part.
        value: String
    },

    // Disablable conditions

    /// A condition meant specifically to handle AdGuard's `$domain` rule modifier.
    /// Please see [AdGuard's docs](https://adguard.com/kb/general/ad-filtering/create-own-filters/#domain-modifier) for details.
    #[allow(clippy::enum_variant_names)]
    DomainCondition {
        /// Unqualified domains where the rule is valid.
        yes_domains: Vec<String>,
        /// Regexes that match domains where the rule is valie.
        yes_domains_regexes: Vec<glue::RegexWrapper>,
        /// Unqualified domains that marks a domain invalid. Takes priority over `yes_domains` and `yes_domains_regexes`.
        unless_domains: Vec<String>,
        /// Regexes that match domains where the rule is invalid. Takes priority over `yes_domains` and `yes_domains_regexes`.
        unless_domains_regexes: Vec<glue::RegexWrapper>
    },

    /// Passes if the URL has a query of the specified name and its value matches the specified regular expression.
    /// Requires the `regex` feature to be enabled.
    QueryParamValueMatchesRegex {
        /// The name of the query paramater.
        name: String,
        /// The [`glue::RegexWrapper`] the query paramater's value is checked agains.
        regex: glue::RegexWrapper
    },
    /// Passes if the URL has a query of the specified name and its value matches the specified glob.
    /// Requires the `glob` feature to be enabled.
    QueryParamValueMatchesGlob {
        /// The name of the query paramater.
        name: String,
        /// The [`glue::GlobWrapper`] the query paramater's value is checked agains.
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
        /// The name of the part to check.
        part_name: UrlPartName,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::RegexWrapper`] the part's value is checked agains.
        regex: glue::RegexWrapper
    },
    /// Takes the specified part of the URL and passes if it matches the specified glob.
    /// if `none_to_empty_string` is `false`, then getting the host, domain, query, or fragment may result in a [`ConditionError::UrlPartNotFound`] error.
    /// Requires the `glob` feature to be enabled.
    UrlPartMatchesGlob {
        /// The name of the part to check.
        part_name: UrlPartName,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::GlobWrapper`] the part's value is checked agains.
        glob: glue::GlobWrapper
    },
    /// Checks the contained comand's [`glue::CommandWrapper::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    /// Requires the `commands` feature to be enabled.
    CommandExists (glue::CommandWrapper),
    /// Runs the specified [`glue::CommandWrapper`] and passes if its exit code equals `expected` (which defaults to `0`).
    /// Requires the `commands` feature to be enabled.
    CommandExitStatus {
        /// The [`glue::CommandWrapper`] to execute.
        command: glue::CommandWrapper,
        /// The expected [`std::process::ExitStatus`]. Defaults to `0`.
        #[serde(default)]
        expected: i32
    }
}

/// Serde doesn't have an equivalent to Clap's `default_value_t`
const fn get_true() -> bool {true}

/// An enum of all possible errors a [`Condition`] can reutrn.
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
    CommandError(#[from] glue::CommandError),
    /// Could not parse the included TLD list.
    #[error("Could not parse the included TLD list.")]
    GetTldError(#[from] crate::suffix::GetTldsError)
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        self.satisfied_by_with_dcr(url, &DomainConditionRule::default())
    }
    
    /// Checks whether or not the provided URL passes the condition.
    pub fn satisfied_by_with_dcr(&self, url: &Url, dcr: &DomainConditionRule) -> Result<bool, ConditionError> {
        println!("{self:?} - {url:?}");
        Ok(match self {
            Self::Always => true,
            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by(url);
                eprintln!("=== Debug Condition output ===\nCondition: {condition:?}\nURL: {url:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            }, // For some reason leaving this comma out (as of Rust 1.75) doesn't cause a compilation error.
            Self::TreatErrorAsPass(condition) => condition.satisfied_by(url).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by(url).unwrap_or(false),
            Self::TryCatch{r#try, catch}  => r#try.satisfied_by(url).or_else(|_| catch.satisfied_by(url))?,
            Self::All(conditions) => conditions.iter().all(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.satisfied_by(url).is_ok_and(identity)),
            Self::Not(condition) => !condition.satisfied_by(url)?,
            Self::UnqualifiedDomain(parts) => url.domain().is_some_and(|domain| domain.strip_suffix(parts).map_or(false, |x| {x.is_empty() || x.ends_with('.')})),
            Self::QualifiedDomain(parts) => url.domain()==Some(parts),
            Self::UnqualifiedAnyTld(name) => {
                match url.domain() {
                    Some(url_domain) => url_domain.contains(name) && match crate::suffix::get_tlds()?.suffix(url_domain.as_bytes()) {
                        Some(suffix) => {
                            // https://github.com/rust-lang/libs-team/issues/212
                            url_domain.as_bytes().strip_suffix(suffix.as_bytes()).is_some_and(|x| x.strip_suffix(b".").is_some_and(|y| y.ends_with(name.as_bytes()) && y.get(y.len()-name.bytes().len()-1).map_or(true, |x| *x==b'.')))
                        },
                        None => false
                    },
                    None => false
                }
            },
            Self::QualifiedAnyTld(name) => {
                match url.domain() {
                    Some(url_domain) => url_domain.contains(name) && match crate::suffix::get_tlds()?.suffix(url_domain.as_bytes()) {
                        Some(suffix) => {
                            url_domain.as_bytes().strip_suffix(suffix.as_bytes()).is_some_and(|x| x.strip_suffix(b".")==Some(name.as_bytes()))
                        },
                        None => false
                    },
                    None => false
                }
            },
            Self::PathIs(path) => path==url.path(),
            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::QueryParamValueIs{name, value} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && value2==value),
            Self::UrlPartIs{part_name, none_to_empty_string, value} => value==part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or(if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref(),

            // Disablable conditions

            #[cfg(feature = "regex")]
            Self::DomainCondition {yes_domains, yes_domains_regexes, unless_domains, unless_domains_regexes} => {
                match dcr {
                    DomainConditionRule::Always => true,
                    DomainConditionRule::Never => false,
                    // Somewhatly annoyingly `DomainConditionRule::Url(Url) | DomainConditionRule::UseUrlBeingCloned` doesn't desugar to this.
                    // I get it's a niche and weird case, but in this one specific instance it'd be nice.
                    DomainConditionRule::Url(url) => {
                        if let Some(host)=url.host_str() {
                            !(unless_domains.iter().any(|domain| domain==host) || unless_domains_regexes.iter().any(|regex| regex.is_match(host))) &&
                                (yes_domains.iter().any(|domain| domain==host) || yes_domains_regexes.iter().any(|regex| regex.is_match(host)))
                        } else {
                            false
                        }
                    },
                    DomainConditionRule::UseUrlBeingCleaned => {
                        if let Some(host)=url.host_str() {
                            !(unless_domains.iter().any(|domain| domain==host) || unless_domains_regexes.iter().any(|regex| regex.is_match(host))) &&
                                (yes_domains.iter().any(|domain| domain==host) || yes_domains_regexes.iter().any(|regex| regex.is_match(host)))
                        } else {
                            false
                        }
                    },
                }
            }

            #[cfg(not(feature = "regex"))]
            Self::DomainCondition{..} => Err(ConditionError::ConditionDisabled)?,

            #[cfg(feature = "regex")] Self::QueryParamValueMatchesRegex{name, regex} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && regex.is_match(value2)),
            #[cfg(feature = "regex")] Self::PathMatchesRegex(regex) => regex.is_match(url.path()),
            #[cfg(feature = "regex")] Self::UrlPartMatchesRegex {part_name, none_to_empty_string, regex} => regex.is_match(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),

            #[cfg(not(feature = "regex"))] Self::QueryParamValueMatchesRegex{..} => Err(ConditionError::ConditionDisabled)?,
            #[cfg(not(feature = "regex"))] Self::PathMatchesRegex(..)            => Err(ConditionError::ConditionDisabled)?,
            #[cfg(not(feature = "regex"))] Self::UrlPartMatchesRegex{..}         => Err(ConditionError::ConditionDisabled)?,

            #[cfg(feature = "glob")] Self::QueryParamValueMatchesGlob {name, glob} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && glob.matches(value2)),
            #[cfg(feature = "glob")] Self::PathMatchesGlob (glob) => glob  .matches(url.path()),
            #[cfg(feature = "glob")] Self::UrlPartMatchesGlob {part_name, none_to_empty_string, glob} => glob.matches(part_name.get_from(url)
                .ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),

            #[cfg(not(feature = "glob"))] Self::QueryParamValueMatchesGlob{..} => Err(ConditionError::ConditionDisabled)?,
            #[cfg(not(feature = "glob"))] Self::PathMatchesGlob(..)            => Err(ConditionError::ConditionDisabled)?,
            #[cfg(not(feature = "glob"))] Self::UrlPartMatchesGlob{..}         => Err(ConditionError::ConditionDisabled)?,

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected},

            #[cfg(not(feature = "commands"))] Self::CommandExists(..)     => Err(ConditionError::ConditionDisabled)?,
            #[cfg(not(feature = "commands"))] Self::CommandExitStatus{..} => Err(ConditionError::ConditionDisabled)?,
        })
    }
}

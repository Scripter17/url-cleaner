//! The logic for when to modify a URL.

use std::collections::hash_set::HashSet;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;
use psl;

use crate::glue;
use crate::types::{UrlPart, DomainConditionRule, StringLocation};
use crate::config::Params;

/// The part of a [`crate::rules::Rule`] that specifies when the rule's mapper will be applied.
/// Note that conditions are checked by the output of the previous mapper.
/// A `Mapper::SwapHost` will make `Condition::UnqualifiedDomain` match on the host that was swapped in.
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
    /// # use url::Url;
    /// assert!(Condition::Error.satisfied_by(&Url::parse("https://example.com").unwrap()).is_err());
    /// ```
    Error,
    /// Prints debugging information about the contained condition to STDERR.
    /// Intended primarily for debugging logic errors.
    /// *Can* be used in production as bash and batch only have `x | y` pipe STDOUT by default, but it'll look ugly.
    Debug(Box<Condition>),

    // Error handling
    
    /// If the contained condition returns an error, treat it as a pass.
    TreatErrorAsPass(Box<Condition>),
    /// If the contained condition returns an error, treat it as a fail.
    TreatErrorAsFail(Box<Condition>),
    /// If the `try` condition returns an error, return the result of the `else` condition instead.
    /// # Errors
    /// If the `else` condition returns an error, that error is returned.
    TryCatch {
        /// The condition to try first.
        r#try: Box<Condition>,
        /// If the try condition fails, instead return the result of this one.
        catch: Box<Condition>
    },

    // Boolean.

    /// Passes if all of the included conditions pass. Like [`Iterator::all`], an empty list of conditions returns `true`.
    /// # Errors
    /// If any contained condition returns an error, that error is returned.
    All(Vec<Condition>),
    /// Passes if any of the included conditions pass. Like [`Iterator::any`], an empty list of conditions returns `false`.
    /// # Errors
    /// If any contained condition returns an error, that error is returned.
    Any(Vec<Condition>),
    /// Passes if the included condition doesn't and vice-versa.
    /// # Errors
    /// If the contained condition returns an error, that error is returned.
    Not(Box<Condition>),

    // Domain conditions.
    
    /// Passes if the URL's domain is or is a subdomain of the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap()).is_ok_and(|x| x==true ));
    /// ```
    UnqualifiedDomain(String),
    /// Similar to [`Condition::UnqualifiedDomain`] but only checks if the subdomain is empty or `www`.
    /// `Condition::MaybeWWWDomain("example.com".to_string())` is effectively the same as `Condition::Any(vec![Condition::QualifiedDomain("example.com".to_string()), Condition::QualifiedDomain("www.example.com".to_string())])`.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::MaybeWWWDomain("example.com".to_string()).satisfied_by(&Url::parse("https://not.example.com").unwrap()).is_ok_and(|x| x==false));
    /// ```
    MaybeWWWDomain(String),
    /// Passes if the URL's domain is the specified domain.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://example.com"    ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedDomain(    "example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedDomain("www.example.com".to_string()).satisfied_by(&Url::parse("https://www.example.com").unwrap()).is_ok_and(|x| x==true ));
    /// ```
    QualifiedDomain(String),
    /// Passes if the URL's host is in the specified set of hosts.
    HostIsOneOf(HashSet<String>),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is or is a subdomain of the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::UnqualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap()).is_ok_and(|x| x==true ));
    /// ```
    UnqualifiedAnyTld(String),
    /// Passes if the URL's domain, minus the TLD/ccTLD, is the specified domain fragment.
    /// See [the psl crate](https://docs.rs/psl/latest/psl/) and [Mozilla's public suffix list](https://publicsuffix.org/) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.com"      ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://example.co.uk"    ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.com"  ).unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QualifiedAnyTld(    "example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap()).is_ok_and(|x| x==false));
    /// assert!(Condition::QualifiedAnyTld("www.example".to_string()).satisfied_by(&Url::parse("https://www.example.co.uk").unwrap()).is_ok_and(|x| x==true ));
    /// ```
    QualifiedAnyTld(String),
    /// A condition meant specifically to handle AdGuard's `$domain` rule modifier.
    /// All domains are treated as unqualified.
    /// Please see [AdGuard's docs](https://adguard.com/kb/general/ad-filtering/create-own-filters/#domain-modifier) for details.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url_cleaner::glue::RegexParts;
    /// # use url_cleaner::types::DomainConditionRule;
    /// # use url_cleaner::config::Params;
    /// # use url::Url;
    /// let dc=Condition::DomainCondition {
    ///     yes_domains: vec!["example.com".to_string()],
    ///     yes_domain_regexes: vec![RegexParts::new(r"example\d\.com").unwrap().into()],
    ///     unless_domains: vec!["wawawa.example.com".to_string()],
    ///     unless_domain_regexes: vec![RegexParts::new(r"thing\d\.example.com").unwrap().into()]
    /// };
    ///
    /// assert!(dc.satisfied_by(&Url::parse("https://example.com"       ).unwrap()).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by(&Url::parse("https://example9.com"      ).unwrap()).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by(&Url::parse("https://wawawa.example.com").unwrap()).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by(&Url::parse("https://thing2.example.com").unwrap()).is_ok_and(|x| x==false));
    ///
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example.com"       ).unwrap(), &Params{dcr: DomainConditionRule::Always, ..Params::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example9.com"      ).unwrap(), &Params{dcr: DomainConditionRule::Always, ..Params::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://wawawa.example.com").unwrap(), &Params{dcr: DomainConditionRule::Always, ..Params::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://thing2.example.com").unwrap(), &Params{dcr: DomainConditionRule::Always, ..Params::default()}).is_ok_and(|x| x==true));
    ///
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example.com"       ).unwrap(), &Params{dcr: DomainConditionRule::Never, ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example9.com"      ).unwrap(), &Params{dcr: DomainConditionRule::Never, ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://wawawa.example.com").unwrap(), &Params{dcr: DomainConditionRule::Never, ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://thing2.example.com").unwrap(), &Params{dcr: DomainConditionRule::Never, ..Params::default()}).is_ok_and(|x| x==false));
    ///
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example.com"       ).unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example9.com"      ).unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://wawawa.example.com").unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://thing2.example.com").unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    ///
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example.com"       ).unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://www.example.com"     ).unwrap()), ..Params::default()}).is_ok_and(|x| x==true ));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://example9.com"      ).unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://www.example9.com"    ).unwrap()), ..Params::default()}).is_ok_and(|x| x==true ));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://wawawa.example.com").unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://a.wawawa.example.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_params(&Url::parse("https://thing2.example.com").unwrap(), &Params{dcr: DomainConditionRule::Url(Url::parse("https://a.thing2.example.com").unwrap()), ..Params::default()}).is_ok_and(|x| x==false));
    /// ```
    #[cfg(feature = "regex")]
    #[allow(clippy::enum_variant_names)]
    DomainCondition {
        /// Unqualified domains where the rule is valid.
        yes_domains: Vec<String>,
        /// Regexes that match domains where the rule is value.
        yes_domain_regexes: Vec<glue::RegexWrapper>,
        /// Unqualified domains that marks a domain invalid. Takes priority over `yes_domains` and `yes_domains_regexes`.
        unless_domains: Vec<String>,
        /// Regexes that match domains where the rule is invalid. Takes priority over `yes_domains` and `yes_domains_regexes`.
        unless_domain_regexes: Vec<glue::RegexWrapper>
    },

    // Query.

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::QueryHasParam("a".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("b".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("c".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==false));
    /// ```
    QueryHasParam(String),

    // Path.
    
    /// Passes if the URL's path is the specified string.
    PathIs(String),

    // General parts.

    /// Passes if the part's getter is `Some`.
    PartExists(UrlPart),
    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
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
    /// Takes the specified part of the URL and passes if it matches the specified regular expression.
    /// # Errors
    /// If chosen part's getter returns `None` and `none_to_empty_string` is set to `false`, returns the error [`ConditionError::UrlPartNotFound`].
    #[cfg(feature = "regex")]
    PartMatchesRegex {
        /// The name of the part to check.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::RegexWrapper`] the part's value is checked against.
        regex: glue::RegexWrapper
    },
    /// Takes the specified part of the URL and passes if it matches the specified glob.
    /// # Errors
    /// If chosen part's getter returns `None` and `none_to_empty_string` is set to `false`, returns the error [`ConditionError::UrlPartNotFound`].
    #[cfg(feature = "glob")]
    PartMatchesGlob {
        /// The name of the part to check.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::GlobWrapper`] the part's value is checked against.
        glob: glue::GlobWrapper
    },

    // Commands.
    
    /// Checks the contained command's [`glue::CommandWrapper::exists`], which uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the system's PATH.
    #[cfg(feature = "commands")]
    CommandExists(glue::CommandWrapper),
    /// Runs the specified [`glue::CommandWrapper`] and passes if its exit code equals `expected` (which defaults to `0`).
    /// # Errors
    /// If the command is does not have an exit code (which I'm told only happens when a command is killed by a signal), returns the error [`ConditionError::CommandError`].
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
    /// # use url::Url;
    /// # use url_cleaner::config::Params;
    /// # use std::collections::HashMap;
    /// let url=Url::parse("https://example.com").unwrap();
    /// let params=Params {vars: HashMap::from([("a".to_string(), "2".to_string())]), ..Params::default()};
    /// assert!(Condition::VariableIs{name: "a".to_string(), value: "2".to_string(), default: false}.satisfied_by_with_params(&url, &params).is_ok_and(|x| x==true ));
    /// assert!(Condition::VariableIs{name: "a".to_string(), value: "3".to_string(), default: false}.satisfied_by_with_params(&url, &params).is_ok_and(|x| x==false));
    /// assert!(Condition::VariableIs{name: "a".to_string(), value: "3".to_string(), default: true }.satisfied_by_with_params(&url, &params).is_ok_and(|x| x==false));
    /// assert!(Condition::VariableIs{name: "a".to_string(), value: "3".to_string(), default: true }.satisfied_by_with_params(&url, &Params::default()).is_ok_and(|x| x==true));
    /// ```
    VariableIs {
        /// The name of the variable to check.
        name: String,
        /// The expected value of the variable.
        value: String,
        /// The default value if the variable isn't provided. Defaults to `false`
        #[serde(default = "get_false")]
        default: bool
    },

    /// Passes if the specified rule flag is set.
    FlagSet(String)
}

const fn get_true() -> bool {true}
const fn get_false() -> bool {false}

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
    StringError(#[from] crate::types::StringError)
}

impl Condition {
    /// Checks whether or not the provided URL passes the condition.
    /// Thin wrapper around [`Self::satisfied_by_with_params`] using [`Params::default`].
    /// # Errors
    /// If the condition has an error, that error is returned.
    /// See [`Condition`]'s documentation for details.
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        self.satisfied_by_with_params(url, &Params::default())
    }

    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// If the condition has an error, that error is returned.
    /// See [`Condition`]'s documentation for details.
    pub fn satisfied_by_with_params(&self, url: &Url, params: &Params) -> Result<bool, ConditionError> {
        Ok(match self {
            // Domain conditions

            Self::UnqualifiedDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_suffix(domain_suffix).is_some_and(|unqualified_part| unqualified_part.is_empty() || unqualified_part.ends_with('.'))),
            Self::MaybeWWWDomain(domain_suffix) => url.domain().is_some_and(|url_domain| url_domain.strip_prefix("www.").unwrap_or(url_domain)==domain_suffix),
            Self::QualifiedDomain(domain) => url.domain()==Some(domain),
            Self::HostIsOneOf(hosts) => url.host_str().is_some_and(|url_host| hosts.contains(url_host)),
            Self::UnqualifiedAnyTld(middle) => {
                // Sometimes you just gotta write garbage.
                // url.domain()
                //     .is_some_and(|domain| domain.contains(middle) && psl::suffix_str(domain)
                //         .is_some_and(|suffix| domain.strip_suffix(suffix)
                //             .is_some_and(|not_suffix_dot| not_suffix_dot.strip_suffix('.')
                //                 .is_some_and(|not_suffix| not_suffix.strip_suffix(middle)
                //                     .is_some_and(|prefix_dot| prefix_dot.is_empty() || prefix_dot.ends_with('.'))
                //                 )
                //             )
                //         )
                //     )
                url.domain().is_some_and(|url_domain| url_domain.split_once(middle).is_some_and(|(_, dot_suffix)| dot_suffix.strip_prefix('.').is_some_and(|suffix| psl::suffix_str(suffix).is_some_and(|psl_suffix| psl_suffix==suffix))))
            },
            Self::QualifiedAnyTld(parts) => url.domain().is_some_and(|domain| domain.strip_prefix(parts).is_some_and(|dot_suffix| dot_suffix.strip_prefix('.').is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix)))),
            #[cfg(feature = "regex")]
            Self::DomainCondition {yes_domains, yes_domain_regexes, unless_domains, unless_domain_regexes} => {
                fn unqualified_domain(domain: &str, parts: &str) -> bool {
                    domain.strip_suffix(parts).map_or(false, |x| {x.is_empty() || x.ends_with('.')})
                }
                match &params.dcr {
                    DomainConditionRule::Always => true,
                    DomainConditionRule::Never => false,
                    // Somewhat annoyingly `DomainConditionRule::Url(Url) | DomainConditionRule::UseUrlBeingCloned` doesn't desugar to this.
                    // I get it's a niche and weird case, but in this one specific instance it'd be nice.
                    DomainConditionRule::Url(url) => {
                        url.domain()
                            .map_or(false, |url_domain|
                                !(unless_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || unless_domain_regexes.iter().any(|regex| regex.is_match(url_domain))) &&
                                    (yes_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || yes_domain_regexes   .iter().any(|regex| regex.is_match(url_domain)))
                            )
                    },
                    DomainConditionRule::UseUrlBeingCleaned => {
                        url.domain()
                            .map_or(false, |url_domain|
                                !(unless_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || unless_domain_regexes.iter().any(|regex| regex.is_match(url_domain))) &&
                                    (yes_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || yes_domain_regexes   .iter().any(|regex| regex.is_match(url_domain)))
                            )
                    },
                }
            },

            // Meta conditions

            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.satisfied_by_with_params(url, params)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.satisfied_by_with_params(url, params)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(condition) => !condition.satisfied_by_with_params(url, params)?,

            // Query

            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),

            // Path

            Self::PathIs(path) => url.path()==path,

            // General parts

            Self::PartIs{part, none_to_empty_string, value} => value.as_deref()==part.get(url, *none_to_empty_string).as_deref(),
            Self::PartContains{part, none_to_empty_string, value, r#where} => r#where.satisfied_by(&part.get(url, *none_to_empty_string).ok_or(ConditionError::UrlPartNotFound)?, value)?,
            Self::PartExists(part) => part.get(url, false).is_some(),
            #[cfg(feature = "regex")] Self::PartMatchesRegex {part, none_to_empty_string, regex} => regex.is_match(part.get(url, *none_to_empty_string).ok_or(ConditionError::UrlPartNotFound)?.as_ref()),
            #[cfg(feature = "glob" )] Self::PartMatchesGlob  {part, none_to_empty_string, glob } => glob .matches (part.get(url, *none_to_empty_string).ok_or(ConditionError::UrlPartNotFound)?.as_ref()),

            // Miscelanious

            Self::VariableIs{name, value, default} => params.vars.get(name).map_or(*default, |x| x==value),
            Self::FlagSet(name) => params.flags.contains(name),

            // Should only ever be used once

            Self::Always => true,

            // Commands

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected},

            // Error handling

            Self::TreatErrorAsPass(condition) => condition.satisfied_by_with_params(url, params).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by_with_params(url, params).unwrap_or(false),
            Self::TryCatch{r#try, catch}  => r#try.satisfied_by_with_params(url, params).or_else(|_| catch.satisfied_by_with_params(url, params))?,

            // Debug

            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by_with_params(url, params);
                eprintln!("=== Debug condition ===\nCondition: {condition:?}\nURL: {url:?}\nparams: {params:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            }
        })
    }
}

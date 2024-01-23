//! The logic for when to modify a URL.

use std::borrow::Cow;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;
use psl;

use crate::glue;
use crate::types::{UrlPart, RuleConfig, DomainConditionRule, StringLocation};

/// The part of a [`crate::rules::Rule`] that specifies when the rule's mapper will be applied.
/// Note that conditions are checked by the output of the previous mapper.
/// A `Mapper::SwapHost` will make `Condition::UnqualifiedDomain` match on the host that was swapped in.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Condition {
    /// Always passes.
    Always,

    // Testing conditions
    
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

    // Meta conditions
    
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

    // Domain conditions
    
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
    /// # use url_cleaner::types::{RuleConfig, DomainConditionRule};
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
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example.com"       ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Always, ..RuleConfig::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example9.com"      ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Always, ..RuleConfig::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://wawawa.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Always, ..RuleConfig::default()}).is_ok_and(|x| x==true));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://thing2.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Always, ..RuleConfig::default()}).is_ok_and(|x| x==true));
    ///
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example.com"       ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Never, ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example9.com"      ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Never, ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://wawawa.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Never, ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://thing2.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Never, ..RuleConfig::default()}).is_ok_and(|x| x==false));
    ///
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example.com"       ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example9.com"      ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://wawawa.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://thing2.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://test.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
    ///
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example.com"       ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://www.example.com"     ).unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==true ));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://example9.com"      ).unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://www.example9.com"    ).unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==true ));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://wawawa.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://a.wawawa.example.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
    /// assert!(dc.satisfied_by_with_config(&Url::parse("https://thing2.example.com").unwrap(), &RuleConfig{dcr: DomainConditionRule::Url(Url::parse("https://a.thing2.example.com").unwrap()), ..RuleConfig::default()}).is_ok_and(|x| x==false));
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

    // Query

    /// Passes if the URL has a query of the specified name.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::QueryHasParam("a".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("b".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryHasParam("c".to_string()).satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==false));
    /// ````
    QueryHasParam(String),
    /// Passes if the URL has a query of the specified name and its value is the specified value.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// assert!(Condition::QueryParamValueIs{name: "a".to_string(), value: "2".to_string()}.satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryParamValueIs{name: "b".to_string(), value: "3".to_string()}.satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==true ));
    /// assert!(Condition::QueryParamValueIs{name: "b".to_string(), value: "4".to_string()}.satisfied_by(&Url::parse("https://example.com?a=2&b=3").unwrap()).is_ok_and(|x| x==false));
    /// ````
    QueryParamValueIs {
        /// The name of the query parameter.
        name: String,
        /// The expected value of the query parameter.
        value: String
    },
    /// Passes if the URL has a query of the specified name and its value matches the specified regular expression.
    #[cfg(feature = "regex")]
    QueryParamValueMatchesRegex {
        /// The name of the query parameter.
        name: String,
        /// The [`glue::RegexWrapper`] the query parameter's value is checked against.
        regex: glue::RegexWrapper
    },
    /// Passes if the URL has a query of the specified name and its value matches the specified glob.
    #[cfg(feature = "glob")]
    QueryParamValueMatchesGlob {
        /// The name of the query parameter.
        name: String,
        /// The [`glue::GlobWrapper`] the query parameter's value is checked against.
        glob: glue::GlobWrapper
    },

    // Path
    
    /// Passes if the URL's path is the specified string.
    PathIs(String),
    /// Passes if the URL's path matches the specified regular expression.
    #[cfg(feature = "regex")]
    PathMatchesRegex(glue::RegexWrapper),
    /// Passes if the URL's path matches the specified glob.
    #[cfg(feature = "glob")]
    PathMatchesGlob(glue::GlobWrapper),

    // General parts

    /// Passes if the part's getter is `Some`.
    UrlPartExists(UrlPart),
    /// Passes if the value of the specified part of the URL is the specified value.
    /// Does not error when the specified part is `None`.
    UrlPartIs {
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
    UrlPartContains {
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
    UrlPartMatchesRegex {
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
    UrlPartMatchesGlob {
        /// The name of the part to check.
        part: UrlPart,
        /// If the relevant [`Url`] part getter returns [`None`], this decides whether to return a [`ConditionError::UrlPartNotFound`] or pretend it's just an empty string and check that.
        /// Defaults to [`true`].
        #[serde(default = "get_true")]
        none_to_empty_string: bool,
        /// The [`glue::GlobWrapper`] the part's value is checked against.
        glob: glue::GlobWrapper
    },

    // Commands
    
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

    // Other

    /// Passes if the specified rule variable is set to the specified value.
    /// # Examples
    /// ```
    /// # use url_cleaner::rules::conditions::Condition;
    /// # use url::Url;
    /// # use url_cleaner::types::RuleConfig;
    /// # use std::collections::HashMap;
    /// let url=Url::parse("https://example.com").unwrap();
    /// let config=RuleConfig {variables: HashMap::from([("a".to_string(), "2".to_string())]), ..RuleConfig::default()};
    /// assert!(Condition::RuleVariableIs{name: "a".to_string(), value: "2".to_string(), default: false}.satisfied_by_with_config(&url, &config).is_ok_and(|x| x==true ));
    /// assert!(Condition::RuleVariableIs{name: "a".to_string(), value: "3".to_string(), default: false}.satisfied_by_with_config(&url, &config).is_ok_and(|x| x==false));
    /// assert!(Condition::RuleVariableIs{name: "a".to_string(), value: "3".to_string(), default: true }.satisfied_by_with_config(&url, &config).is_ok_and(|x| x==false));
    /// assert!(Condition::RuleVariableIs{name: "a".to_string(), value: "3".to_string(), default: true }.satisfied_by_with_config(&url, &RuleConfig::default()).is_ok_and(|x| x==true));
    /// ````
    RuleVariableIs {
        /// The name of the variable to check.
        name: String,
        /// The expected value of the variable.
        value: String,
        /// The default value if the variable isn't provided. Defaults to `false`
        #[serde(default = "get_false")]
        default: bool
    }
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
    /// # Errors
    /// If the condition has an error, that error is returned.
    pub fn satisfied_by(&self, url: &Url) -> Result<bool, ConditionError> {
        self.satisfied_by_with_config(url, &RuleConfig::default())
    }

    /// Checks whether or not the provided URL passes the condition.
    /// # Errors
    /// If the condition has an error, that error is returned.
    pub fn satisfied_by_with_config(&self, url: &Url, config: &RuleConfig) -> Result<bool, ConditionError> {
        Ok(match self {
            // Domain conditions

            Self::MaybeWWWDomain(parts) => url.domain().is_some_and(|x| x.strip_prefix("www.").unwrap_or(x)==parts),
            Self::UnqualifiedDomain(parts) => url.domain().is_some_and(|domain| domain.strip_suffix(parts).is_some_and(|unqualified_part| unqualified_part.is_empty() || unqualified_part.ends_with('.'))),
            Self::QualifiedDomain(parts) => url.domain()==Some(parts),
            Self::UnqualifiedAnyTld(parts) => {
                // Sometimes you just gotta write garbage.
                url.domain()
                    .is_some_and(|domain| domain.contains(parts) && psl::suffix_str(domain)
                        .is_some_and(|suffix| domain.strip_suffix(suffix)
                            .is_some_and(|prefix_dot| prefix_dot.strip_suffix('.')
                                .is_some_and(|prefix| prefix.strip_suffix(parts)
                                    .is_some_and(|subdomain_dot| subdomain_dot.is_empty() || subdomain_dot.ends_with('.'))
                                )
                            )
                        )
                    )
            },
            Self::QualifiedAnyTld(parts) => url.domain().is_some_and(|domain| domain.strip_prefix(parts).is_some_and(|dot_suffix| dot_suffix.strip_prefix('.').is_some_and(|suffix| Some(suffix)==psl::suffix_str(suffix)))),
            #[cfg(feature = "regex")]
            Self::DomainCondition {yes_domains, yes_domain_regexes, unless_domains, unless_domain_regexes} => {
                fn unqualified_domain(domain: &str, parts: &str) -> bool {
                    domain.strip_suffix(parts).map_or(false, |x| {x.is_empty() || x.ends_with('.')})
                }
                match &config.dcr {
                    DomainConditionRule::Always => true,
                    DomainConditionRule::Never => false,
                    // Somewhat annoyingly `DomainConditionRule::Url(Url) | DomainConditionRule::UseUrlBeingCloned` doesn't desugar to this.
                    // I get it's a niche and weird case, but in this one specific instance it'd be nice.
                    DomainConditionRule::Url(url) => {
                        url.domain()
                            .map_or(false, |url_domain|
                                !(unless_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || unless_domain_regexes.iter().any(|regex| regex.is_match(url_domain))) &&
                                    (yes_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || yes_domain_regexes.iter().any(|regex| regex.is_match(url_domain)))
                            )
                    },
                    DomainConditionRule::UseUrlBeingCleaned => {
                        url.domain()
                            .map_or(false, |url_domain|
                                !(unless_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || unless_domain_regexes.iter().any(|regex| regex.is_match(url_domain))) &&
                                    (yes_domains.iter().any(|domain| unqualified_domain(url_domain, domain)) || yes_domain_regexes.iter().any(|regex| regex.is_match(url_domain)))
                            )
                    },
                }
            }

            // Should only ever be used once

            Self::Always => true,

            // Meta conditions

            Self::TreatErrorAsPass(condition) => condition.satisfied_by_with_config(url, config).unwrap_or(true),
            Self::TreatErrorAsFail(condition) => condition.satisfied_by_with_config(url, config).unwrap_or(false),
            Self::TryCatch{r#try, catch}  => r#try.satisfied_by_with_config(url, config).or_else(|_| catch.satisfied_by_with_config(url, config))?,
            Self::All(conditions) => {
                for condition in conditions {
                    if !condition.satisfied_by_with_config(url, config)? {
                        return Ok(false);
                    }
                }
                true
            },
            Self::Any(conditions) => {
                for condition in conditions {
                    if condition.satisfied_by_with_config(url, config)? {
                        return Ok(true);
                    }
                }
                false
            },
            Self::Not(condition) => !condition.satisfied_by_with_config(url, config)?,


            // Query

            Self::QueryHasParam(name) => url.query_pairs().any(|(ref name2, _)| name2==name),
            Self::QueryParamValueIs{name, value} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && value2==value),
            #[cfg(feature = "regex")] Self::QueryParamValueMatchesRegex{name, regex} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && regex.is_match(value2)),
            #[cfg(feature = "glob" )] Self::QueryParamValueMatchesGlob {name, glob} => url.query_pairs().any(|(ref name2, ref value2)| name2==name && glob.matches(value2)),

            // Path

            Self::PathIs(path) => url.path()==path,
            #[cfg(feature = "regex")] Self::PathMatchesRegex(regex) => regex.is_match(url.path()),
            #[cfg(feature = "glob" )] Self::PathMatchesGlob (glob) => glob  .matches(url.path()),

            // General parts

            Self::UrlPartExists(part) => part.get(url).is_some(),
            Self::UrlPartIs{part, none_to_empty_string, value} => value.as_deref()==if *none_to_empty_string {
                Some(part.get(url).unwrap_or(Cow::Borrowed("")))
            } else {
                part.get(url)
            }.as_deref(),
            Self::UrlPartContains{part, none_to_empty_string, value, r#where} => {
                let part_value=part.get(url)
                    .or_else(|| none_to_empty_string.then_some(Cow::Borrowed("")))
                    .ok_or(ConditionError::UrlPartNotFound)?;
                r#where.satisfied_by(&part_value, value)?
            }
            #[cfg(feature = "regex")] Self::UrlPartMatchesRegex {part, none_to_empty_string, regex} => regex.is_match(part.get(url).ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),
            #[cfg(feature = "glob" )] Self::UrlPartMatchesGlob {part, none_to_empty_string, glob} => glob.matches(part.get(url).ok_or(ConditionError::UrlPartNotFound).or_else(|_| if *none_to_empty_string {Ok(Cow::Borrowed(""))} else {Err(ConditionError::UrlPartNotFound)})?.as_ref()),

            // Disablable conditions

            #[cfg(feature = "commands")] Self::CommandExists (command) => command.exists(),
            #[cfg(feature = "commands")] Self::CommandExitStatus {command, expected} => {&command.exit_code(url)?==expected},

            // Debug conditions

            Self::Never => false,
            Self::Error => Err(ConditionError::ExplicitError)?,
            Self::Debug(condition) => {
                let is_satisfied=condition.satisfied_by_with_config(url, config);
                eprintln!("=== Debug Condition output ===\nCondition: {condition:?}\nURL: {url:?}\nConfig: {config:?}\nSatisfied?: {is_satisfied:?}");
                is_satisfied?
            },

            // Other

            Self::RuleVariableIs{name, value, default} => config.variables.get(name).map_or(*default, |x| x==value)
        })
    }
}

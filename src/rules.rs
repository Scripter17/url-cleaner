//! The [`rules::Rule`] type is how this crate modifies URLs. A [`rules::Rule`] contains a [`rules::conditions::Condition`] and a [`rules::mappers::Mapper`].
//! If the condition passes (returns `Ok(true)`), then the mapper is applied to a mutable reference to the URL.

use url::Url;
#[cfg(feature = "default-rules")]
use std::sync::OnceLock;
use std::fs::read_to_string;
use std::path::Path;
use std::ops::{Deref, DerefMut};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// The logic for when to modify a URL.
pub mod conditions;
/// The logic for how to modify a URL.
pub mod mappers;
use crate::types;

/// The core unit describing when and how URLs are modified.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    /// The condition under which the provided URL is modified.
    pub condition: conditions::Condition,
    /// The mapper used to modify the provided URL.
    pub mapper: mappers::Mapper
}

/// Denotes that either the condition failed (returned `Ok(false)`), the condition errored, or the mapper errored.
#[derive(Error, Debug)]
pub enum RuleError {
    /// The URL does not meet the rule's conditon.
    #[error("The URL does not meet the rule's conditon.")]
    FailedCondition,
    /// The condition returned an error.
    #[error("The condition returned an error.")]
    ConditionError(#[from] conditions::ConditionError),
    /// The mapper returned an error.
    #[error("The mapper returned an error.")]
    MapperError(#[from] mappers::MapperError)
}

impl Rule {
    /// Apply the rule to the url in-place.
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.apply_with_dcr(url, &types::DomainConditionRule::default())
    }
    
    /// Apply the rule to the url in-place.
    pub fn apply_with_dcr(&self, url: &mut Url, dcr: &types::DomainConditionRule) -> Result<(), RuleError> {
        if self.condition.satisfied_by_with_dcr(url, dcr)? {
            Ok(self.mapper.apply(url)?)
        } else {
            Err(RuleError::FailedCondition)
        }
    }
}

#[cfg(all(feature = "default-rules", feature = "minify-default-rules"))]
const RULES_STR: &str=const_str::replace!(const_str::replace!(const_str::replace!(include_str!("../default-config.json"), ' ', ""), '\t', ""), '\n', "");
#[cfg(all(feature = "default-rules", not(feature = "minify-default-rules")))]
const RULES_STR: &str=include_str!("../default-config.json");
#[cfg(feature = "default-rules")]
static RULES: OnceLock<Rules>=OnceLock::new();

/// Gets the rules compiled into the URL Cleaner binary.
/// Panics if the it isn't parseable into [`Rules`].
pub fn get_default_rules() -> Result<&'static Rules, GetRulesError> {
    #[cfg(feature = "default-rules")]
    {
        if let Some(rules) = RULES.get() {
            Ok(rules)
        } else {
            let rules=serde_json::from_str(RULES_STR).map_err(GetRulesError::CantParseDefaultRules)?; // I don't know why that syntax is allowed. Literally it makes sense. Conceptually it does not.
            Ok(RULES.get_or_init(|| rules))
        }
    }
    #[cfg(not(feature = "default-rules"))]
    Err(GetRulesError::NoDefaultRules)
}

/// If `path` is `Some`, loads and parses [`Rules`] from the JSON file it points to.
/// If `None`, returns [`get_default_rules`].
pub fn get_rules(path: Option<&Path>) -> Result<Cow<Rules>, GetRulesError> {
    Ok(match path {
        Some(path) => Cow::Owned(serde_json::from_str::<Rules>(&read_to_string(path).or(Err(GetRulesError::CantLoadFile))?)?),
        None => Cow::Borrowed(get_default_rules()?)
    })
}

/// A thin wrapper around a vector of rules.
/// Exists mainly for convenience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {fn from(value: Vec<Rule>) -> Self {Self(value)}}
impl From<Rules> for Vec<Rule> {fn from(value: Rules    ) -> Self {value.0    }}

impl Deref for Rules {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for Rules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

#[allow(dead_code)]
impl Rules {
    /// A wrapper around [`Rules::deref`]
    pub fn as_slice(&self) -> &[Rule] {self.deref()}
    /// A wrapper around [`Rules::deref_mut`]
    pub fn as_mut_slice(&mut self) -> &mut [Rule] {self.deref_mut()}

    /// Applies each rule to the provided [`Url`] in order.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`].
    /// If an error is returned, `url` is left unmodified.
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.apply_with_dcr(url, &types::DomainConditionRule::default())
    }

    /// Applies each rule to the provided [`Url`] in order.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`].
    /// If an error is returned, `url` is left unmodified.
    pub fn apply_with_dcr(&self, url: &mut Url, dcr: &types::DomainConditionRule) -> Result<(), RuleError> {
        let mut temp_url=url.clone();
        for rule in self.deref() {
            match rule.apply_with_dcr(&mut temp_url, dcr) {
                Err(RuleError::FailedCondition) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        *url=temp_url;
        Ok(())
    }

    /// TODO: REMOVE THIS.
    /// A mess of a function used to simplify the rules parsed from AdGuard lists.
    /// Currently just merges consecutive [`mappers::Mapper::RemoveSomeQueryParams`] and [`mappers::Mapper::AllowSomeQueryParams`].
    /// [`Rules::apply`] should always give the same result regardless of if this function was used first.
    /// Also this function should always be idempotent.
    /// There is, however, no guarantee that this function always makes the rules as simple as possible for any definition of "simpler".
    pub fn simplify(self) -> Self {
        let mut ret=Vec::<Rule>::new();
        for mut rule in self.0.into_iter() {
            match ret.last_mut() {
                Some(last_rule) => {
                    // match rule.condition {
                    //     conditions::Condition::All(x) if x.len()==1 => {rule.condition=x[0];},
                    //     conditions::Condition::Any(x) if x.len()==1 => {rule.condition=x[0];},
                    //     _ => {}
                    // }
                    if last_rule.condition==rule.condition {
                        match (&mut last_rule.mapper, &mut rule.mapper) {
                            (&mut mappers::Mapper::RemoveSomeQueryParams(ref mut last_params), &mut mappers::Mapper::RemoveSomeQueryParams(ref mut params)) => {
                                last_params.append(params)
                            },
                            (&mut mappers::Mapper::AllowSomeQueryParams (ref mut last_params), &mut mappers::Mapper::AllowSomeQueryParams (ref mut params)) => {
                                last_params.append(params)
                            },
                            (_, _) => {ret.push(rule);}
                        }
                    } else {
                        ret.push(rule);
                    }
                },
                None => {ret.push(rule);}
            }
        }
        Rules::from(ret)
    }
}

/// An enum containing all possible errors that can happen when loading/parsing a rules into a [`Rules`]
#[derive(Error, Debug)]
pub enum GetRulesError {
    /// Could not load the specified rules file.
    #[error("Could not load the specified rules file.")]
    CantLoadFile,
    /// The loaded file did not contain valid JSON.
    #[error("The loaded file did not contain valid JSON.")]
    CantParseFile(#[from] serde_json::Error),
    /// URL Cleaner was compiled without default rules.
    #[allow(dead_code)]
    #[error("URL Cleaner was compiled without default rules.")]
    NoDefaultRules,
    /// The default rules compiled into URL Cleaner aren't valid JSON.
    #[allow(dead_code)]
    #[error("The default rules compiled into URL Cleaner aren't valid JSON.")]
    CantParseDefaultRules(serde_json::Error)
}

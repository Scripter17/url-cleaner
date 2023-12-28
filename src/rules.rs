use url::Url;
#[cfg(feature = "default-rules")]
use std::sync::OnceLock;
use std::fs::read_to_string;
use std::path::Path;
use std::ops::{Deref, DerefMut};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use thiserror::Error;

pub mod conditions;
pub mod mappers;
use crate::types;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub condition: conditions::Condition,
    pub mapper: mappers::Mapper
}

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("The URL does not meet the rule's conditon.")]
    FailedCondition,
    #[error("The condition returned an error.")]
    ConditionError(#[from] conditions::ConditionError),
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

pub fn get_default_rules() -> Option<&'static Rules> {
    #[cfg(feature = "default-rules")]
    {
        Some(RULES.get_or_init(|| {
            serde_json::from_str(RULES_STR).unwrap()
        }))
    }
    #[cfg(not(feature = "default-rules"))]
    None
}

pub fn get_rules(path: Option<&Path>) -> Result<Cow<Rules>, GetRulesError> {
    Ok(match path {
        Some(path) => Cow::Owned(serde_json::from_str::<Rules>(&read_to_string(path).or(Err(GetRulesError::CantLoadFile))?)?),
        None => Cow::Borrowed(get_default_rules().ok_or(GetRulesError::NoDefaultRules)?)
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {fn from(value: Vec<Rule>) -> Self {Self(value)}}
impl From<Rules> for Vec<Rule> {fn from(value: Rules)     -> Self {value.0}}

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
    fn as_slice(&self) -> &[Rule] {self.deref()}
    fn as_mut_slice(&mut self) -> &mut [Rule] {self.deref_mut()}

    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.apply_with_dcr(url, &types::DomainConditionRule::default())
    }

    /// Applies each rule to the provided [`Url`] one after another.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`].
    /// If an error is returned, the `url` is left unmodified.
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

#[derive(Error, Debug)]
pub enum GetRulesError {
    /// Could not load the specified.
    #[error("Could not load the specified.")]
    CantLoadFile,
    /// The loaded file did not contain valid JSON.
    #[error("The loaded file did not contain valid JSON.")]
    CantParseFile(#[from] serde_json::Error),
    /// URL Cleaner was compiled without default rules.
    #[error("URL Cleaner was compiled without default rules.")]
    NoDefaultRules
}

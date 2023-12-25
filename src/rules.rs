use url::Url;
#[cfg(feature = "default-rules")]
use std::sync::OnceLock;
use std::fs::read_to_string;
use std::path::Path;
use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};
use serde_json;
use thiserror::Error;

pub mod conditions;
pub mod mappers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    // #[serde(skip)]
    // pub credit: (),
    pub condition: conditions::Condition,
    pub mapper: mappers::Mapper
}

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("The URL does not meet the rule's conditon")]
    FailedCondition,
    #[error("The condition returned an error")]
    ConditionError(#[from] conditions::ConditionError),
    #[error("The mapper returned an error")]
    MapperError(#[from] mappers::MapperError)
}

impl Rule {
    /// Apply the rule to the url in-place.
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        if self.condition.satisfied_by(url)? {
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

pub fn get_default_rules() -> Option<Rules> {
    #[cfg(feature = "default-rules")]
    {
        Some(RULES.get_or_init(|| {
            serde_json::from_str(RULES_STR).unwrap()
        }).clone())
    }
    #[cfg(not(feature = "default-rules"))]
    None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {
    fn from(value: Vec<Rule>) -> Self {Self(value)}
}

impl Into<Vec<Rule>> for Rules {
    fn into(self) -> Vec<Rule> {self.0}
}

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
    fn as_slice<'a>(&'a self) -> &'a [Rule] {self.deref()}
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Rule] {self.deref_mut()}

    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        let mut temp_url=url.clone();
        for rule in self.deref() {
            match rule.apply(&mut temp_url) {
                Err(RuleError::FailedCondition) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        *url=temp_url;
        Ok(())
    }

    pub fn simplify(self) -> Self {
        let mut ret=Vec::<Rule>::new();
        for mut rule in self.0.into_iter() {
            match ret.last_mut() {
                Some(last_rule) => {
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
    #[error("Can't load file")]
    CantLoadFile,
    #[error("Can't parse file")]
    CantParseFile(#[from] serde_json::Error),
    #[error("No default rules")]
    NoDefaultRules
}

pub fn get_rules(path: Option<&Path>) -> Result<Rules, GetRulesError> {
    Ok(match path {
        Some(path) => serde_json::from_str::<Rules>(&read_to_string(path).or(Err(GetRulesError::CantLoadFile))?)?,
        None => get_default_rules().ok_or(GetRulesError::NoDefaultRules)?
    })
}

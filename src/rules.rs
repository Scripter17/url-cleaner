use url::Url;
use serde::Deserialize;
use thiserror::Error;
#[cfg(feature = "default-rules")]
use std::sync::OnceLock;
use std::fs::read_to_string;
use std::path::Path;

mod conditions;
mod mappings;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub condition: conditions::Condition,
    pub mapping: mappings::Mapping
}

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("The URL does not meet the rule's conditon")]
    FailedCondition,
    #[error("The condition failed")]
    ConditionError(#[from] conditions::ConditionError),
    #[error("The mapping failed")]
    MappingError(#[from] mappings::MappingError)
}

impl Rule {
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        if self.condition.satisfied_by(url)? {
            Ok(self.mapping.apply(url)?)
        } else {
            Err(RuleError::FailedCondition)
        }
    }
}

#[cfg(feature = "default-rules")]
const RULES_STR: &str=const_str::replace!(const_str::replace!(const_str::replace!(include_str!("../config.json"), ' ', ""), '\t', ""), '\n', "");
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

#[derive(Debug, Clone, Deserialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {
    fn from(value: Vec<Rule>) -> Self {Self(value)}
}

impl Into<Vec<Rule>> for Rules {
    fn into(self) -> Vec<Rule> {self.0}
}

impl Rules {
    fn into_inner(self) -> Vec<Rule> {self.0}
    fn as_slice<'a>(&'a self) -> &'a [Rule] {self.0.as_slice()}
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        let mut temp_url=url.clone();
        for rule in self.as_slice() {
            match rule.apply(&mut temp_url) {
                Err(RuleError::FailedCondition) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        *url=temp_url;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GetRulesError {
    #[error("Can't load file")]
    CantLoadFile,
    #[error("Can't parse file")]
    CantParseFile,
    #[error("No default rules")]
    NoDefaultRules
}

pub fn get_rules(path: Option<&Path>) -> Result<Rules, GetRulesError> {
    Ok(match path {
        Some(path) => Rules(serde_json::from_str::<Vec<Rule>>(&read_to_string(path).or(Err(GetRulesError::CantLoadFile))?).or(Err(GetRulesError::CantParseFile))?),
        None => get_default_rules().ok_or(GetRulesError::NoDefaultRules)?
    })
}

//! The [`rules::Rule`] type is how this crate modifies URLs.

use url::Url;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// The logic for when to modify a URL.
pub mod conditions;
/// The logic for how to modify a URL.
pub mod mappers;
use crate::config;

/// The core unit describing when and how URLs are modified.
/// # Examples
/// ```
/// # use url_cleaner::rules::{Rule, conditions, mappers};
/// # use url::Url;
/// # use std::collections::HashMap;
/// assert!(Rule::Normal{condition: conditions::Condition::Never, mapper: mappers::Mapper::None}.apply(&mut Url::parse("https://example.com").unwrap()).is_err());
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rule {
    /// A faster but slightly less versatile mode that uses a hashmap to save on iterations in [`Rules`].
    HostMap(HashMap<String, mappers::Mapper>),
    /// Runs all the contained rules until none of their conditions pass.
    /// Runs at most `limit` times. (Defaults to 10).
    RepeatUntilNonePass {
        /// The rules to repeat.
        rules: Vec<Rule>,
        /// The max amount of times to repeat them.
        /// Defaults to 10.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// The basic condition mapper rule type.
    #[serde(untagged)]
    Normal {
        /// The condition under which the provided URL is modified.
        condition: conditions::Condition,
        /// The mapper used to modify the provided URL.
        mapper: mappers::Mapper
    }
}

// Serde helpers.
const fn get_10_u8() -> u8 {10}

/// The errors that [`Rule`] can return.
#[derive(Error, Debug)]
pub enum RuleError {
    /// The URL does not meet the rule's condition.
    #[error("The URL does not meet the rule's condition.")]
    FailedCondition,
    /// The condition returned an error.
    #[error(transparent)]
    ConditionError(#[from] conditions::ConditionError),
    /// The mapper returned an error.
    #[error(transparent)]
    MapperError(#[from] mappers::MapperError),
    /// Returned when the provided URL doesn't have a host to find in a [`Rule::HostMap`].
    #[error("The provided URL doesn't have a host to find in the hashmap.")]
    UrlHasNoHost,
    /// Returned when the provided URL's host isn't in a [`Rule::HostMap`].
    #[error("The provided URL's host was not found in the `Rule::HostMap`.")]
    HostNotInMap
}

impl Rule {
    /// Apply the rule to the url in-place.
    /// # Errors
    /// If the call to [`Self::apply_with_params`] returns an error, that error is returned.
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.apply_with_params(url, &config::Params::default())
    }

    /// Apply the rule to the url in-place.
    /// # Errors
    /// If the rule is a [`Self::Normal`] and the contained condition or mapper returns an error, that error is returned.
    /// If the rule is a [`Self::HostMap`] and the provided URL doesn't have a host, returns the error [`RuleError::UrlHasNoHost`].
    /// If the rule is a [`Self::HostMap`] and the provided URL's host isn't in the rule's map, returns the error [`RuleError::HostNotInMap`].
    pub fn apply_with_params(&self, url: &mut Url, params: &config::Params) -> Result<(), RuleError> {
        match self {
            Self::Normal{condition, mapper} => if condition.satisfied_by_with_params(url, params)? {
                mapper.apply_with_params(url, params)?;
                Ok(())
            } else {
                Err(RuleError::FailedCondition)
            },
            Self::RepeatUntilNonePass{rules, limit} => {
                for _ in 0..*limit {
                    let mut done=true;
                    for rule in rules {
                        match rule.apply_with_params(url, params) {
                            Err(RuleError::FailedCondition) => {},
                            Ok(()) => done=false,
                            e @ Err(_) => e?
                        }
                    }
                    if done {break}
                }
                Ok(())
            },
            Self::HostMap(map) => Ok(map.get(url.host_str().ok_or(RuleError::UrlHasNoHost)?).ok_or(RuleError::HostNotInMap)?.apply_with_params(url, params)?)
        }
    }
}

/// A thin wrapper around a vector of rules.
/// Exists mainly for convenience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {fn from(value: Vec<Rule>) -> Self {Self(value)}}
impl From<Rules> for Vec<Rule> {fn from(value: Rules    ) -> Self {value.0    }}

impl Deref for Rules {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(dead_code)]
impl Rules {
    /// Applies each rule to the provided [`Url`] in order.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`].
    /// If an error is returned, `url` is left unmodified.
    /// # Errors
    /// If a rule returns any error other than [`RuleError::FailedCondition`], that error is returned.
    /// If the error [`RuleError::FailedCondition`] is encountered, it is ignored.
    pub fn apply(&self, url: &mut Url) -> Result<(), RuleError> {
        self.apply_with_params(url, &config::Params::default())
    }

    /// Applies each rule to the provided [`Url`] in order.
    /// Bubbles up every unignored error except for [`RuleError::FailedCondition`].
    /// If an error is returned, `url` is left unmodified.
    /// # Errors
    /// If the error [`RuleError::FailedCondition`], [`RuleError::UrlHasNoHost`], or [`RuleError::HostNotInMap`] is encountered, it is ignored.
    pub fn apply_with_params(&self, url: &mut Url, params: &config::Params) -> Result<(), RuleError> {
        let mut temp_url=url.clone();
        for rule in &**self {
            match rule.apply_with_params(&mut temp_url, params) {
                Err(RuleError::FailedCondition | RuleError::UrlHasNoHost | RuleError::HostNotInMap) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        *url=temp_url;
        Ok(())
    }
}

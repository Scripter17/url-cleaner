//! The part of a config that actually modified URLs.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use thiserror::Error;

mod conditions;
pub use conditions::*;
mod mappers;
pub use mappers::*;

pub use crate::types::*;
use crate::util::*;

/// The main API for modifying URLs.
/// 
/// [`Rule::Normal`] is almost always what you want.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Rule {
    /// Gets a certain part of a URL then applies a [`Mapper`] depending on the returned value.
    /// # Errors
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    PartMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<Option<String>, Mapper>
    },
    /// Gets a certain part of a URL then applies a [`Rule`] depending on the returned value.
    /// # Errors
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    PartRuleMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Rule`] to apply.
        map: HashMap<Option<String>, Rule>
    },
    /// Gets a certain part of a URL then applies a [`Rules`] depending on the returned value.
    /// # Errors
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    PartRulesMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Rules`] to apply.
        map: HashMap<Option<String>, Rules>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Mapper`] depending on the returned value.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    StringSourceMap {
        /// The [`StringSource`] to get the string from.
        source: Option<StringSource>,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<Option<String>, Mapper>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Rule`] depending on the returned value.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    StringSourceRuleMap {
        /// The [`StringSource`] to get the string from.
        source: Option<StringSource>,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<Option<String>, Rule>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Rules`] depending on the returned value.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`HashMap::get`] returns [`None`], returns the error [`RuleError::ValueNotInMap`]. This error is ignored by [`Rules::apply`].
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    StringSourceRulesMap {
        /// The [`StringSource`] to get the string from.
        source: Option<StringSource>,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<Option<String>, Rules>
    },
    /// Runs all the contained rules in a loop until none of their conditions pass.
    /// 
    /// Runs at most `limit` times. (Defaults to 10).
    /// # Implementation details
    /// While [`Self::RepeatUntilNonePass`] is a [`Rules`], [`Rules::apply`] is not called due to needing to keep track of if any contained [`Rule`]s... pass. I need a better term for that/
    /// # Errors
    /// If any call to a [`Self`] contained in the specified [`Rules`] returns an error other than [`RuleError::FailedCondition`] and [`RuleError::ValueNotInMap`], the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// assert!(Rule::RepeatUntilNonePass {
    ///     rules: Rules(vec![
    ///         Rule::Normal {
    ///             condition: Condition::Always,
    ///             mapper: Mapper::SetPart {
    ///                 part: UrlPart::NextPathSegment,
    ///                 value: Some(FromStr::from_str("a").unwrap())
    ///             }
    ///         }
    ///     ]),
    ///     limit: 10
    /// }.apply(&mut JobState::new(&mut url)).is_ok());
    /// assert_eq!(url.as_str(), "https://example.com/a/a/a/a/a/a/a/a/a/a");
    /// ```
    RepeatUntilNonePass {
        /// The rules to repeat.
        rules: Rules,
        /// The max amount of times to repeat them.
        /// 
        /// Defaults to 10.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// When many rules share a common condition (such as `{"UnqualifiedAnySuffix": "amazon"}`), it often makes semantic and performance sense to merge them all into one.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Condition::satisfied_by`] returns `Some(false)`, returns the error [`RuleError::FailedCondition`].
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    CommonCondition {
        /// The condition they all share. Note that [`Condition::All`] and [`Condition::Any`] can be used to have multiple common conditions.
        condition: Condition,
        /// The rules to run if [`Self::CommonCondition::condition`] passes.
        rules: Rules
    },
    /// Runs the contained [`Self`] then, if no error is returned, returns the error [`RuleError::FailedCondition`],
    /// 
    /// Useful for use in [``]
    /// # Errors 
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// 
    /// If no error is returned, returns the error [`RuleError::FailedCondition`].
    PretendFailedCondition(Box<Self>),
    /// The most basic type of rule. If the call to [`Condition::satisfied_by`] returns `Ok(true)`, calls [`Mapper::apply`] on the provided URL.
    /// 
    /// This is the last variant because of the [`#[serde(untageed)]`](https://serde.rs/variant-attrs.html#untagged) macro.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Condition::satisfied_by`] returns `Some(false)`, returns the error [`RuleError::FailedCondition`].
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// // [`RuleError::FailedCondition`] is returned when the condition does not pass.
    /// // [`Rules`] just ignores them because it's a higher level API.
    /// assert!(Rule::Normal{condition: Condition::Never, mapper: Mapper::None}.apply(&mut JobState::new(&mut Url::parse("https://example.com").unwrap())).is_err());
    /// ```
    #[serde(untagged)]
    Normal {
        /// The condition under which the provided URL is modified.
        condition: Condition,
        /// The mapper used to modify the provided URL.
        mapper: Mapper
    }
}

/// Serde helper function. The default value of [`Rule::RepeatUntilNonePass::limit`].
const fn get_10_u8() -> u8 {10}

/// The errors that [`Rule`] can return.
#[derive(Debug, Error)]
pub enum RuleError {
    /// The URL does not meet the rule's condition.
    #[error("The URL does not meet the rule's condition.")]
    FailedCondition,
    /// The condition returned an error.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// The mapper returned an error.
    #[error(transparent)]
    MapperError(#[from] MapperError),
    /// Returned when the requested value isn't found in a rule's [`HashMap`].
    #[error("The requested value was not found in the rule's HashMap.")]
    ValueNotInMap,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError)
}

impl Rule {
    /// Apply the rule to the url in-place.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        #[cfg(feature = "debug")]
        println!("Rule: {self:?}");
        match self {
            Self::Normal{condition, mapper} => if condition.satisfied_by(job_state)? {
                mapper.apply(job_state)?;
                Ok(())
            } else {
                Err(RuleError::FailedCondition)
            },
            Self::PartMap      {part, map} => Ok(map.get(&part.get(job_state.url).map(|x| x.into_owned())).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::PartRuleMap  {part, map} => Ok(map.get(&part.get(job_state.url).map(|x| x.into_owned())).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::PartRulesMap {part, map} => Ok(map.get(&part.get(job_state.url).map(|x| x.into_owned())).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::StringSourceMap      {source, map} => Ok(map.get(&get_option_string!(source, job_state)).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::StringSourceRuleMap  {source, map} => Ok(map.get(&get_option_string!(source, job_state)).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::StringSourceRulesMap {source, map} => Ok(map.get(&get_option_string!(source, job_state)).ok_or(RuleError::ValueNotInMap)?.apply(job_state)?),
            Self::RepeatUntilNonePass{rules, limit} => {

                // MAKE SURE THIS IS ALWAYS SYNCED UP WITH [`Rules::apply`]!!!

                for _ in 0..*limit {
                    let mut done=true;
                    for rule in rules.0.iter() {
                        match rule.apply(job_state) {
                            Err(RuleError::FailedCondition | RuleError::ValueNotInMap) => {},
                            e @ Err(_) => e?,
                            _ => done=false
                        }
                    }
                    if done {break}
                }
                Ok(())
            },
            Self::CommonCondition{condition, rules} => {
                if condition.satisfied_by(job_state)? {
                    rules.apply(job_state)?;
                    Ok(())
                } else {
                    Err(RuleError::FailedCondition)
                }
            },
            Self::PretendFailedCondition(rule) => {
                rule.apply(job_state)?;
                Err(RuleError::FailedCondition)
            }
        }
    }
}

/// A wrapper around a vector of rules.
/// 
/// Exists mainly for convenience.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rules(pub Vec<Rule>);

impl Rules {
    /// Applies each contained [`Rule`] to the provided [`JobState::url`] in order.
    /// 
    /// If an error is returned, `job_state.url` and `job_state.string_vars` are left unmodified.
    /// # Errors
    /// If any contained [`Rule`] returns an error other than [`RuleError::FailedCondition`] or [`RuleError::ValueNotInMap`], that error is returned.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        #[cfg(feature = "debug")]
        println!("Rules: {self:?}");
        let mut temp_url = job_state.url.clone();
        let mut temp_job_state = JobState {
            url: &mut temp_url,
            params: job_state.params,
            string_vars: job_state.string_vars.clone()
        };
        for rule in &self.0 {
            match rule.apply(&mut temp_job_state) {
                Err(RuleError::FailedCondition | RuleError::ValueNotInMap) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        job_state.string_vars = temp_job_state.string_vars;
        *job_state.url = temp_url;
        Ok(())
    }
}

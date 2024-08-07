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
    /// Runs all the contained rules in a loop until the specified [`StopLoopCondition`] returns [`true`].
    /// 
    /// Runs at most `limit` times. (Defaults to 10).
    /// # Implementation details
    /// While [`Self::Repeat`] is a [`Rules`], [`Rules::apply`] is not called due to needing to keep track of if any contained [`Rule`]s... pass. I need a better term for that.
    /// 
    /// If a contained [`Rule`] never requires another loop, it's advised to put it in a [`Rule::DontTriggerLoop`].
    /// 
    /// Other rules can still trigger the loop.
    /// # Errors
    /// If any call to a [`Self`] contained in the specified [`Rules`] returns an error other than [`RuleError::DontTriggerLoop`], [`RuleError::FailedCondition`] and [`RuleError::ValueNotInMap`], the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// # use std::str::FromStr;
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// Rule::Repeat {
    ///     rules: Rules(vec![
    ///         Rule::Normal {
    ///             condition: Condition::Always,
    ///             mapper: Mapper::SetPart {
    ///                 part: UrlPart::NextPathSegment,
    ///                 value: Some(FromStr::from_str("a").unwrap())
    ///             }
    ///         }
    ///     ]),
    ///     stop_loop_condition: Default::default(),
    ///     limit: 10
    /// }.apply(&mut job_state).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com/a/a/a/a/a/a/a/a/a/a");
    /// ```
    Repeat {
        /// The rules to repeat.
        rules: Rules,
        #[serde(default)]
        /// Defaults to the value of [`StopLoopCondition::default`].
        stop_loop_condition: StopLoopCondition,
        /// The max amount of times to repeat them.
        /// 
        /// Defaults to 10.
        #[serde(default = "get_10_u8")]
        limit: u8
    },
    /// Runs the contained [`Self`] then, if no error is returned, returns the error [`RuleError::DontTriggerLoop`],
    /// 
    /// Intended for use in [`Self::Repeat`].
    /// 
    /// Other [`Rule`]s in the loop body can still trigger another loop.
    /// # Errors 
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    /// 
    /// If no error is returned, returns the error [`RuleError::DontTriggerLoop`].
    DontTriggerLoop(Box<Self>),
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
    /// Execites the contained [`Rules`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    Rules(Rules),
    /// If the call to [`Condition::satisfied_by`] returns `Ok(true)`, calls [`Self::IfElse::mapper`]'s [`Mapper::apply`] on the provided URL, otherwise use [`Self::IfElse::else_mapper`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Condition::satisfied_by`] returns `Ok(false)`, returns the error [`RuleError::FailedCondition`].
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    IfElse {
        /// The condition to decide which mapper to use.
        condition: Condition,
        /// The mapper to use if the condition passes.
        mapper: Mapper,
        /// The mapper to use if the consition fails.
        else_mapper: Mapper
    },
    /// The most basic type of rule. If the call to [`Condition::satisfied_by`] returns `Ok(true)`, calls [`Mapper::apply`] on the provided URL.
    /// 
    /// This is the last variant because of the [`#[serde(untageed)]`](https://serde.rs/variant-attrs.html#untagged) macro.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Condition::satisfied_by`] returns `Ok(false)`, returns the error [`RuleError::FailedCondition`].
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// # use url::Url;
    /// // [`RuleError::FailedCondition`] is returned when the condition does not pass.
    /// // [`Rules`] just ignores them because it's a higher level API.
    /// let mut url = Url::parse("https://example.com").unwrap();
    /// let params = Default::default();
    /// #[cfg(feature = "cache")]
    /// let cache_handler = std::path::PathBuf::from("test-cache.sqlite").as_path().try_into().unwrap();
    /// let mut job_state = url_cleaner::types::JobState {
    ///     url: &mut url,
    ///     params: &params,
    ///     vars: Default::default(),
    ///     #[cfg(feature = "cache")]
    ///     cache_handler: &cache_handler
    /// };
    /// 
    /// Rule::Normal {
    ///     condition: Condition::Always,
    ///     mapper: Mapper::None
    /// }.apply(&mut job_state).unwrap();
    /// 
    /// // If [`Condition::satisfied_by`] returns `Ok(false)`, `Rule::Normal.apply` returns `Err(ConditionError::FailedCondition)`.
    /// // That specific error is ignored by [`Rules::apply`].
    /// Rule::Normal {
    ///     condition: Condition::Never,
    ///     mapper: Mapper::None
    /// }.apply(&mut job_state).unwrap_err();
    /// ```
    #[serde(untagged)]
    Normal {
        /// The condition under which the provided URL is modified.
        condition: Condition,
        /// The mapper used to modify the provided URL.
        mapper: Mapper
    }
}

/// Serde helper function. The default value of [`Rule::Repeat::limit`].
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
    StringSourceError(#[from] StringSourceError),
    /// Not an error; Just tells Rule::Repeat to not loop just because of the rule that returned this.
    #[error("Not an error; Just tells Rule::Repeat to not loop just because of the rule that returned this.")]
    DontTriggerLoop
}

impl Rule {
    /// Apply the rule to the url in-place.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        debug!(Rule::apply, self, job_state);
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
            Self::Repeat{rules, stop_loop_condition, limit} => {

                // MAKE SURE THIS IS ALWAYS SYNCED UP WITH [`Rules::apply`]!!!

                let mut previous_url = job_state.url.clone();
                let mut previous_job_vars = job_state.vars.clone();
                for _ in 0..*limit {
                    let mut none_passed=true;
                    for rule in rules.0.iter() {
                        match rule.apply(job_state) {
                            Err(RuleError::DontTriggerLoop | RuleError::FailedCondition | RuleError::ValueNotInMap) => {},
                            e @ Err(_) => e?,
                            _ => none_passed=false
                        }
                    }
                    if stop_loop_condition.satisfied_by(job_state, none_passed, &previous_url, &previous_job_vars) {break;}
                    previous_url = job_state.url.clone();
                    previous_job_vars = job_state.vars.clone();
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
            Self::DontTriggerLoop(rule) => {
                rule.apply(job_state)?;
                Err(RuleError::DontTriggerLoop)
            },
            Self::Rules(rules) => Ok(rules.apply(job_state)?),
            Self::IfElse {condition, mapper, else_mapper} => Ok(if condition.satisfied_by(job_state)? {
                mapper.apply(job_state)?
            } else {
                else_mapper.apply(job_state)?
            })
        }
    }

    /// Internal method to make sure I don't accidetnally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used)]
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        match self {
            Self::PartMap {part, map} => part.is_suitable_for_release() && map.iter().all(|(_, mapper)| mapper.is_suitable_for_release()),
            Self::PartRuleMap {part, map} => part.is_suitable_for_release() && map.iter().all(|(_, rule)| rule.is_suitable_for_release()),
            Self::PartRulesMap {part, map} => part.is_suitable_for_release() && map.iter().all(|(_, rules)| rules.is_suitable_for_release()),
            Self::StringSourceMap {source, map} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release()) && map.iter().all(|(_, mapper)| mapper.is_suitable_for_release()),
            Self::StringSourceRuleMap {source, map} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release()) && map.iter().all(|(_, rule)| rule.is_suitable_for_release()),
            Self::StringSourceRulesMap {source, map} => (source.is_none() || source.as_ref().unwrap().is_suitable_for_release()) && map.iter().all(|(_, rules)| rules.is_suitable_for_release()),
            Self::Repeat {rules, ..} => rules.is_suitable_for_release(),
            Self::CommonCondition {condition, rules} => condition.is_suitable_for_release() && rules.is_suitable_for_release(),
            Self::DontTriggerLoop(rule) => rule.is_suitable_for_release(),
            Self::Rules(rules) => rules.is_suitable_for_release(),
            Self::IfElse {condition, mapper, else_mapper} => condition.is_suitable_for_release() && mapper.is_suitable_for_release() && else_mapper.is_suitable_for_release(),
            Self::Normal {condition, mapper} => condition.is_suitable_for_release() && mapper.is_suitable_for_release()
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
    /// If an error is returned, `job_state.url` and `job_state.vars` are left unmodified.
    /// 
    /// Caching may still happen and won't be reverted.
    /// # Errors
    /// If any contained [`Rule`] returns an error other than [`RuleError::FailedCondition`] or [`RuleError::ValueNotInMap`], that error is returned.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        debug!(Rules::apply, self, job_state);
        let mut temp_url = job_state.url.clone();
        let mut temp_job_state = JobState {
            url: &mut temp_url,
            params: job_state.params,
            vars: job_state.vars.clone(),
            #[cfg(feature = "cache")]
            cache_handler: job_state.cache_handler
        };

        // MAKE SURE THIS IS ALWAYS SYNCED UP WITH [`Rule::Repeat`]!!!

        for rule in &self.0 {
            match rule.apply(&mut temp_job_state) {
                Err(RuleError::DontTriggerLoop | RuleError::FailedCondition | RuleError::ValueNotInMap) => {},
                e @ Err(_) => e?,
                _ => {}
            }
        }
        job_state.vars = temp_job_state.vars;
        *job_state.url = temp_url;
        Ok(())
    }

    /// Internal method to make sure I don't accidetnally commit Debug variants and other stuff unsuitable for the default config.
    #[allow(clippy::unwrap_used)]
    pub(crate) fn is_suitable_for_release(&self) -> bool {
        self.0.iter().all(|rule| rule.is_suitable_for_release())
    }
}

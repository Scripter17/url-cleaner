//! Logic for when and how [`TaskState`]s should be modified.

use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};
use thiserror::Error;

pub mod conditions;
pub use conditions::*;
pub mod mappers;
pub use mappers::*;

use crate::types::*;
#[expect(unused_imports, reason = "Used in Rule::Custom")]
use crate::glue::*;
use crate::util::*;

/// When and how to modify a [`TaskState`].
///
/// I'm sorry for the dogshit diagnostics on typos. I tried setting a bounty for serde to fix it but they didn't let me.
///
/// For now, [`Self::Normal`] being `#[serde(untagged)]` is one of my many eternal torments.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Suitability)]
pub enum Rule {
    /// Gets the specified [`UrlPart`] and uses it to select a [`Mapper`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    PartMap {
        /// The part to get.
        part: UrlPart,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Mapper>
    },
    /// Gets the specified [`UrlPart`] and uses it to select a [`Rule`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    PartRuleMap {
        /// The part to get.
        part: UrlPart,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Gets the specified [`UrlPart`] and uses it to select a [`Rules`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    PartRulesMap {
        /// The part to get.
        part: UrlPart,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Rules>
    },
    /// Gets the specified [`UrlPart`] and uses it to select a [`Mapper`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    StringMap {
        /// The string to get.
        value: StringSource,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Mapper>
    },
    /// Gets the specified [`UrlPart`] and uses it to select a [`Rule`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Self::apply`] returns an error, that error is returned.
    StringRuleMap {
        /// The stringed get.
        value: StringSource,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Self>
    },
    /// Gets the specified [`UrlPart`] and uses it to select a [`Rules`].
    ///
    /// If the call to [`Map::get`] returns [`None`], does nothing.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    StringRulesMap {
        /// The stringed get.
        value: StringSource,
        /// The map to branch with.
        #[serde(flatten)]
        map: Map<Rules>
    },
    /// Repeat [`Self::Repeat::rules`] until either no changes happen or the rules were executed [`Self::Repeat::limit`] times.
    /// # Errors
    /// If any call to [`Rules::apply`] returns an error, that error is returned.
    Repeat {
        /// The [`Rules`] to repeat.
        rules: Rules,
        /// The maximum amount of times to repeat.
        ///
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64
    },
    /// If [`Self::SharedCondition::condition`] passes, apply [`Self::SharedCondition::rules`].
    SharedCondition {
        /// The [`Condition`] to share between [`Rules`].
        condition: Condition,
        /// The [`Rules`] to apply.
        rules: Rules
    },
    /// Apply a list of [`Rules`].
    Rules(Rules),
    /// Apply a [`Mapper`].
    Mapper(Mapper),
    /// Apply a [`Self`] from [`Commons::rules`].
    Common(CommonCall),
    /// Apply a custom [`Self`].
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    #[suitable(never)]
    #[serde(skip)]
    Custom(fn(&mut TaskState) -> Result<(), RuleError>),
    /// If [`Self::Normal::condition`] passes, apply [`Self::Normal::mapper`].
    ///
    /// If [`Self::Normal::condition`] fails and [`Self::Normal::else_mapper`] is [`Some`], apply [`Self::Normal::else_mapper`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    ///
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    #[serde(untagged)]
    Normal {
        /// The [`Condition`] to test to determine which [`Mapper`] to apply.
        condition: Condition,
        /// The [`Mapper`] that's applied if [`Self::Normal::condition`] passes.
        mapper: Mapper,
        /// If [`Some`], the [`Mapper`] that's applied if [`Self::Normal::condition`] fails.
        #[serde(default, skip_serializing_if = "is_default")]
        else_mapper: Option<Mapper>
    }
}

/// Helper function to get the default [`Rule::Repeat::limit`].
const fn get_10_u64() -> u64 {10}

/// The enum of errors [`Rule::apply`] can return.
#[derive(Debug, Error)]
pub enum RuleError {
    /// Returned when a [`ConditionError`] is encountered.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// Returned when a [`MapperError`] is encountered.
    #[error(transparent)]
    MapperError(#[from] MapperError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a call to [`StringSource::get`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when trying to call an unknown [`Commons::rules`].
    #[error("The common Rule was not found.")]
    CommonRuleNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Returned by [`Rule::Custom`] functions when their errors don't fit in any of the other variants of [`Self`].
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl Rule {
    /// See each variant of [`Self`] for behavior.
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// See each variant of [`Self`] for errors.
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), RuleError> {
        debug!(Rule::apply, self, task_state);
        Ok(match self {
            Self::Normal{condition, mapper, else_mapper} => if condition.satisfied_by(&task_state.to_view())? {
                mapper.apply(task_state)?;
            } else if let Some(else_mapper) = else_mapper {
                else_mapper.apply(task_state)?
            },
            Self::PartMap        {part , map} => if let Some(x) = map.get(part .get( task_state.url      ) ) {x.apply(task_state)?;},
            Self::PartRuleMap    {part , map} => if let Some(x) = map.get(part .get( task_state.url      ) ) {x.apply(task_state)?;},
            Self::PartRulesMap   {part , map} => if let Some(x) = map.get(part .get( task_state.url      ) ) {x.apply(task_state)?;},
            Self::StringMap      {value, map} => if let Some(x) = map.get(value.get(&task_state.to_view())?) {x.apply(task_state)?;},
            Self::StringRuleMap  {value, map} => if let Some(x) = map.get(value.get(&task_state.to_view())?) {x.apply(task_state)?;},
            Self::StringRulesMap {value, map} => if let Some(x) = map.get(value.get(&task_state.to_view())?) {x.apply(task_state)?;},
            Self::Repeat{rules, limit} => {
                let mut previous_url;
                let mut previous_scratchpad;
                for _ in 0..*limit {
                    previous_url = task_state.url.clone();
                    previous_scratchpad = task_state.scratchpad.clone();
                    rules.apply(task_state)?;
                    if task_state.url == &previous_url && task_state.scratchpad == &previous_scratchpad {break;}
                }
            },
            Self::SharedCondition{condition, rules} => if condition.satisfied_by(&task_state.to_view())? {
                rules.apply(task_state)?
            },
            Self::Rules(rules) => rules.apply(task_state)?,
            Self::Mapper(mapper) => mapper.apply(task_state)?,
            Self::Common(common_call) => {
                task_state.commons.rules.get(get_str!(common_call.name, task_state, RuleError)).ok_or(RuleError::CommonRuleNotFound)?.apply(&mut TaskState {
                    common_args: Some(&common_call.args.build(&task_state.to_view())?),
                    url: task_state.url,
                    context: task_state.context,
                    params: task_state.params,
                    scratchpad: task_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: task_state.cache,
                    commons: task_state.commons,
                    job_context: task_state.job_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(task_state)?
        })
    }
}

/// A list of [`Rule`]s
/// .
/// Exists mainly to provide [`Self::apply`] as a wrapper around [`Rule::apply`]ing each contained [`Rule`] in order.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Suitability)]
#[repr(transparent)]
pub struct Rules(pub Vec<Rule>);

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

impl Rules {
    /// Applies each contained [`Rule`] in order.
    ///
    /// If an error is returned, `task_state` may be left in a partially modified state.
    /// # Errors
    /// If any call to [`Rule::apply`] returns an error, that error is returned.
    pub fn apply(&self, task_state: &mut TaskState) -> Result<(), RuleError> {
        debug!(Rules::apply, self, task_state);
        for rule in &self.0 {
            rule.apply(task_state)?;
        }
        Ok(())
    }
}

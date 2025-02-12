//! The part of a config that actually modified URLs.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Errors
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    PartMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<String, Mapper>,
        /// The [`Mapper`] to use if [`Self::PartMap::part`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Mapper>,
        /// If the part isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Mapper>
    },
    /// Gets a certain part of a URL then applies a [`Rule`] depending on the returned value.
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Errors
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    PartRuleMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Rule`] to apply.
        map: HashMap<String, Rule>,
        /// The [`Rule`] to use if [`Self::PartRuleMap::part`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Box<Rule>>,
        /// If the part isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Rule>>
    },
    /// Gets a certain part of a URL then applies a [`Rules`] depending on the returned value.
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    PartRulesMap {
        /// The part to get.
        part: UrlPart,
        /// The map determining which [`Rules`] to apply.
        map: HashMap<String, Rules>,
        /// The [`Rules`] to use if [`Self::PartRulesMap::part`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Rules>,
        /// If the part isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Rules>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Mapper`] depending on the returned value.
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    StringMap {
        /// The [`StringSource`] to get the string from.
        value: StringSource,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<String, Mapper>,
        /// The [`Mapper`] to use if [`Self::StringMap::value`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Mapper>,
        /// If the string isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Mapper>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Rule`] depending on the returned value.
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Rule::apply`] returns an error, that error is returned.
    StringRuleMap {
        /// The [`StringSource`] to get the string from.
        value: StringSource,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<String, Rule>,
        /// The [`Rule`] to use if [`Self::StringRuleMap::value`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Box<Rule>>,
        /// If the string isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Box<Rule>>
    },
    /// Gets a string from a [`StringSource`] then applies a [`Rules`] depending on the returned value.
    ///
    /// If no [`Mapper`] is found, does nothing and doesn't return an error.
    /// # Rules
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    StringRulesMap {
        /// The [`StringSource`] to get the string from.
        value: StringSource,
        /// The map determining which [`Mapper`] to apply.
        map: HashMap<String, Rules>,
        /// The [`Rules`] to use if [`Self::StringRulesMap::value`] returns [`None`].
        #[serde(default, skip_serializing_if = "is_default")]
        if_null: Option<Rules>,
        /// If the string isn't in the map, use this.
        #[serde(default, skip_serializing_if = "is_default")]
        r#else: Option<Rules>
    },
    /// Repeatedly runs the contained [`Rules`] until neither [`JobState::url`] nor [`JobState::scratchpad`] change.
    /// 
    /// Runs at most `limit` times. (Defaults to 10).
    /// # Errors
    /// If call to [`Rules::apply`] returns an error, the error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Rule::Repeat {
    ///     rules: Rules(vec![
    ///         Rule::Normal {
    ///             condition: Condition::Always,
    ///             mapper: Mapper::SetPart {
    ///                 part: UrlPart::NextPathSegment,
    ///                 value: Some("a".into())
    ///             }
    ///         }
    ///     ]),
    ///     limit: 10
    /// }.apply(&mut job_state).unwrap();
    /// assert_eq!(job_state.url.as_str(), "https://example.com/a/a/a/a/a/a/a/a/a/a");
    /// ```
    Repeat {
        /// The rules to repeat.
        rules: Rules,
        /// The max amount of times to repeat them.
        /// 
        /// Defaults to 10.
        #[serde(default = "get_10_u64")]
        limit: u64
    },
    /// When many rules share a common condition (such as `{"UnqualifiedAnySuffix": "amazon"}`), it often makes semantic and performance sense to merge them all into one.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    SharedCondition {
        /// The condition they all share. Note that [`Condition::All`] and [`Condition::Any`] can be used to have multiple common conditions.
        condition: Condition,
        /// The rules to run if [`Self::SharedCondition::condition`] passes.
        rules: Rules
    },
    /// Applies the contained [`Rules`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    Rules(Rules),
    /// If the call to [`Condition::satisfied_by`] returns `Ok(true)`, calls [`Self::IfElse::mapper`]'s [`Mapper::apply`] on the provided URL, otherwise use [`Self::IfElse::else_mapper`].
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    IfElse {
        /// The condition to decide which mapper to use.
        condition: Condition,
        /// The mapper to use if the condition passes.
        mapper: Mapper,
        /// The mapper to use if the condition fails.
        else_mapper: Mapper
    },
    /// Uses a [`Self`] from the [`JobState::commons`]'s [`Commons::rules`].
    Common(CommonCall),
    /// Uses a function pointer.
    /// 
    /// Cannot be serialized or deserialized.
    #[expect(clippy::type_complexity, reason = "Who cares")]
    #[cfg(feature = "custom")]
    Custom(FnWrapper<fn(&mut JobState) -> Result<(), RuleError>>),
    /// The most basic type of rule. If the call to [`Condition::satisfied_by`] returns `Ok(true)`, calls [`Mapper::apply`] on the provided URL.
    /// 
    /// This is the last variant because of the [`#[serde(untageed)]`](https://serde.rs/variant-attrs.html#untagged) macro.
    /// # Errors
    /// If the call to [`Condition::satisfied_by`] returns an error, that error is returned.
    /// 
    /// If the call to [`Mapper::apply`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// # use url_cleaner::types::*;
    /// url_cleaner::job_state!(job_state;);
    /// 
    /// Rule::Normal {
    ///     condition: Condition::Always,
    ///     mapper: Mapper::None
    /// }.apply(&mut job_state).unwrap();
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
const fn get_10_u64() -> u64 {10}

/// The errors that [`Rule`] can return.
#[derive(Debug, Error)]
pub enum RuleError {
    /// The condition returned an error.
    #[error(transparent)]
    ConditionError(#[from] ConditionError),
    /// The mapper returned an error.
    #[error(transparent)]
    MapperError(#[from] MapperError),
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError),
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when the common [`Rule`] is not found.
    #[error("The common Rule was not found.")]
    CommonRuleNotFound,
    /// Returned when a [`CommonCallArgsError`] is encountered.
    #[error(transparent)]
    CommonCallArgsError(#[from] CommonCallArgsError),
    /// Custom error.
    #[error(transparent)]
    #[cfg(feature = "custom")]
    Custom(Box<dyn std::error::Error + Send>)
}

impl Rule {
    /// Apply the rule to the url in-place.
    /// # Errors
    /// See each of [`Self`]'s variant's documentation for details.
    ///
    /// If an error occurs, `job_state` is effectively unmodified, though the mutable parts may be clones.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        debug!(Rule::apply, self, job_state);
        Ok(match self {
            Self::Normal{condition, mapper} => if condition.satisfied_by(&job_state.to_view())? {
                mapper.apply(job_state)?;
            },
            Self::PartMap        {part , map, if_null, r#else} => if let Some(x) = part .get( job_state.url      ) .map(|x| map.get(&*x)).unwrap_or(if_null.as_ref  ()).or(r#else.as_ref  ()) {x.apply(job_state)?;},
            Self::PartRuleMap    {part , map, if_null, r#else} => if let Some(x) = part .get( job_state.url      ) .map(|x| map.get(&*x)).unwrap_or(if_null.as_deref()).or(r#else.as_deref()) {x.apply(job_state)?;},
            Self::PartRulesMap   {part , map, if_null, r#else} => if let Some(x) = part .get( job_state.url      ) .map(|x| map.get(&*x)).unwrap_or(if_null.as_ref  ()).or(r#else.as_ref  ()) {x.apply(job_state)?;},
            Self::StringMap      {value, map, if_null, r#else} => if let Some(x) = value.get(&job_state.to_view())?.map(|x| map.get(&*x)).unwrap_or(if_null.as_ref  ()).or(r#else.as_ref  ()) {x.apply(job_state)?;},
            Self::StringRuleMap  {value, map, if_null, r#else} => if let Some(x) = value.get(&job_state.to_view())?.map(|x| map.get(&*x)).unwrap_or(if_null.as_deref()).or(r#else.as_deref()) {x.apply(job_state)?;},
            Self::StringRulesMap {value, map, if_null, r#else} => if let Some(x) = value.get(&job_state.to_view())?.map(|x| map.get(&*x)).unwrap_or(if_null.as_ref  ()).or(r#else.as_ref  ()) {x.apply(job_state)?;},
            Self::Repeat{rules, limit} => {
                let original_url = job_state.url.clone();
                let original_scratchpad = job_state.scratchpad.clone();
                let mut previous_url;
                let mut previous_scratchpad;
                for _ in 0..*limit {
                    previous_url = job_state.url.clone();
                    previous_scratchpad = job_state.scratchpad.clone();
                    match rules.apply_no_revert(job_state) {
                        Ok(()) => if job_state.url == &previous_url && job_state.scratchpad == &previous_scratchpad {break;},
                        e @ Err(_) => {
                            *job_state.url = original_url;
                            *job_state.scratchpad = original_scratchpad;
                            return e;
                        }
                    }
                }
            },
            Self::SharedCondition{condition, rules} => if condition.satisfied_by(&job_state.to_view())? {
                rules.apply(job_state)?
            },
            Self::Rules(rules) => rules.apply(job_state)?,
            Self::IfElse {condition, mapper, else_mapper} => if condition.satisfied_by(&job_state.to_view())? {
                mapper.apply(job_state)?;
            } else {
                else_mapper.apply(job_state)?;
            },
            Self::Common(common_call) => {
                job_state.commons.rules.get(get_str!(common_call.name, job_state, RuleError)).ok_or(RuleError::CommonRuleNotFound)?.apply(&mut JobState {
                    common_args: Some(&common_call.args.make(&job_state.to_view())?),
                    url: job_state.url,
                    context: job_state.context,
                    params: job_state.params,
                    scratchpad: job_state.scratchpad,
                    #[cfg(feature = "cache")]
                    cache: job_state.cache,
                    commons: job_state.commons,
                    jobs_context: job_state.jobs_context
                })?
            },
            #[cfg(feature = "custom")]
            Self::Custom(function) => function(job_state)?
        })
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(match self {
            Self::PartMap        {part , map, if_null, r#else} => part .is_suitable_for_release(config) && map.iter().all(|(_, mapper)| mapper.is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::PartRuleMap    {part , map, if_null, r#else} => part .is_suitable_for_release(config) && map.iter().all(|(_, rule  )| rule  .is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::PartRulesMap   {part , map, if_null, r#else} => part .is_suitable_for_release(config) && map.iter().all(|(_, rules )| rules .is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::StringMap      {value, map, if_null, r#else} => value.is_suitable_for_release(config) && map.iter().all(|(_, mapper)| mapper.is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::StringRuleMap  {value, map, if_null, r#else} => value.is_suitable_for_release(config) && map.iter().all(|(_, rule  )| rule  .is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::StringRulesMap {value, map, if_null, r#else} => value.is_suitable_for_release(config) && map.iter().all(|(_, rules )| rules .is_suitable_for_release(config)) && if_null.as_ref().is_none_or(|x| x.is_suitable_for_release(config)) && r#else.as_ref().is_none_or(|x| x.is_suitable_for_release(config)),
            Self::Repeat {rules, ..} => rules.is_suitable_for_release(config),
            Self::SharedCondition {condition, rules} => condition.is_suitable_for_release(config) && rules.is_suitable_for_release(config),
            Self::Rules(rules) => rules.is_suitable_for_release(config),
            Self::IfElse {condition, mapper, else_mapper} => condition.is_suitable_for_release(config) && mapper.is_suitable_for_release(config) && else_mapper.is_suitable_for_release(config),
            Self::Common(common_call) => common_call.is_suitable_for_release(config),
            Self::Normal {condition, mapper} => condition.is_suitable_for_release(config) && mapper.is_suitable_for_release(config),
            #[cfg(feature = "custom")]
            Self::Custom(_) => false
        }, "Unsuitable Rule detected: {self:?}");
        let self_debug_string = format!("{self:?}");
        assert!(
            self_debug_string.contains("no-network") == (self_debug_string.contains("ExpandRedirect") || self_debug_string.contains("HttpRequest")),
            "Network call without no-network flag: {self_debug_string}"
        );
        true
    }
}

/// A wrapper around a vector of rules.
/// 
/// Exists mainly for convenience.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    /// Applies each contained [`Rule`] to the provided [`JobState::url`] in order.
    /// 
    /// If an error is returned, `job_state.url` and `job_state.scratchpad` are left unmodified.
    /// 
    /// Caching may still happen and won't be reverted.
    /// # Errors
    /// If any contained [`Rule`] returns an error, that error is returned.
    pub fn apply(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        debug!(Rules::apply, self, job_state);
        let old_url = job_state.url.clone();
        let old_scratchpad = job_state.scratchpad.clone();
        match self.apply_no_revert(job_state) {
            x @ Ok(_) => x,
            e @ Err(_) => {
                *job_state.scratchpad = old_scratchpad;
                *job_state.url = old_url;
                e
            }
        }
    }

    /// Applies each contained [`Rule`] to the provided [`JobState::url`] in order.
    /// 
    /// If an error is returned, `job_state.url` and `job_state.scratchpad` are not reverted.
    ///
    /// This is fine if you guarantee discarding the URL on an error, such as [`Job::do`], but can result in unpredictable and undefined outputs.
    /// # Errors
    /// If any contained [`Rule`] returns an error, that error is returned.
    pub fn apply_no_revert(&self, job_state: &mut JobState) -> Result<(), RuleError> {
        debug!(Rules::apply_no_revert, self, job_state);
        for rule in &self.0 {
            rule.apply(job_state)?;
        }
        Ok(())
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        self.0.iter().all(|rule| rule.is_suitable_for_release(config))
    }
}

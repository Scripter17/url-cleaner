//! [`CommonCallArgsConfig`].

use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// Instructions on how to make the [`CommonCallArgs`] to call a [`Commons`] thing.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgsConfig {
    /// The flags to set.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars to set.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, StringSource>,
    /// The [`Condition`]s to use with [`Condition::CommonCallArg`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// The [`Action`]s to use with [`Action::CommonCallArg`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// The [`StringSource`]s to use with [`StringSource::CommonCallArg`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// The [`StringModification`]s to use with [`StringModification::CommonCallArg`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// The [`StringMatcher`]s to use with [`StringMatcher::CommonCallArg`].
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}

/// The enum of errors [`CommonCallArgsConfig::make`] can return.
#[derive(Debug, Error)]
pub enum MakeCommonCallArgsError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>)
}

impl From<StringSourceError> for MakeCommonCallArgsError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl CommonCallArgsConfig {
    /// Builds the [`CommonCallArgs`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    pub fn make<'a>(&'a self, task_state: &TaskStateView) -> Result<CommonCallArgs<'a>, MakeCommonCallArgsError> {
        Ok(CommonCallArgs {
            flags: &self.flags,
            vars: self.vars.iter().filter_map(|(k, v)| match v.get(task_state) {Ok(Some(x)) => Some(Ok((&**k, x.into_owned()))), Ok(None) => None, Err(e) => Some(Err(e))}).collect::<Result<HashMap<_, _>, StringSourceError>>()?,
            conditions: &self.conditions,
            actions: &self.actions,
            string_sources: &self.string_sources,
            string_modifications: &self.string_modifications,
            string_matchers: &self.string_matchers
        })
    }
}


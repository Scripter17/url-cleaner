//! Details on how to call a [`Commons`] thing.

use std::str::FromStr;
use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// Instructions on how to call a [`Commons`] thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct CommonCall {
    /// The name of the [`Commons`] thing to call.
    pub name: Box<StringSource>,
    /// The args to call the [`Commons`] thing with.
    ///
    /// Defaults to [`CommonCallArgsSource::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Box<CommonCallArgsSource>
}

impl FromStr for CommonCall {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for CommonCall {
    fn from(value: &str) -> Self {
        StringSource::from(value).into()
    }
}

impl From<String> for CommonCall {
    fn from(value: String) -> Self {
        StringSource::from(value).into()
    }
}

impl From<StringSource> for CommonCall {
    fn from(value: StringSource) -> Self {
        Box::new(value).into()
    }
}

impl From<Box<StringSource>> for CommonCall {
    fn from(value: Box<StringSource>) -> Self {
        Self {
            name: value,
            args: Default::default()
        }
    }
}

string_or_struct_magic!(CommonCall);

/// Instructions on how to make the args to call a [`Commons`] thing.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct CommonCallArgsSource {
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
    /// The [`Condition`]s to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// The [`Action`]s to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// The [`StringSource`]s to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// The [`StringModification`]s to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// The [`StringMatcher`]s to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}

/// The enum of errors [`CommonCallArgsSource::build`] can return.
#[derive(Debug, Error)]
pub enum CommonCallArgsError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>)
}

impl From<StringSourceError> for CommonCallArgsError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl CommonCallArgsSource {
    /// Builds the [`CommonCallArgs`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource))]
    pub fn build<'a>(&'a self, task_state: &TaskStateView) -> Result<CommonCallArgs<'a>, CommonCallArgsError> {
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

/// The args a [`Commons`] thing is called with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommonCallArgs<'a> {
    /// The flags that are set.
    pub flags: &'a HashSet<String>,
    /// The vars that are set.
    pub vars: HashMap<&'a str, String>,
    /// The [`Condition`]s to use.
    pub conditions: &'a HashMap<String, Condition>,
    /// The [`Action`]s to use.
    pub actions: &'a HashMap<String, Action>,
    /// The [`StringSource`]s to use.
    pub string_sources: &'a HashMap<String, StringSource>,
    /// The [`StringModification`]s to use.
    pub string_modifications: &'a HashMap<String, StringModification>,
    /// The [`StringMatcher`]s to use.
    pub string_matchers: &'a HashMap<String, StringMatcher>
}

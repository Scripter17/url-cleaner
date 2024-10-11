//! Unified logic for calling commons.

use std::str::FromStr;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// The name of the common to call and the arguments to call it with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct CommonCall {
    /// The name of the common to call.
    pub name: Box<StringSource>,
    /// The arguments to call it with.
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: CommonCallArgs
}

impl FromStr for CommonCall {
    type Err = <StringSource as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: Box::new(FromStr::from_str(s)?),
            args: Default::default()
        })
    }
}

string_or_struct_magic!(CommonCall);

impl CommonCall {
    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(
            self.name.is_suitable_for_release(config) && self.args.is_suitable_for_release(config),
            "Unsuitable CommonCall detected: {self:?}"
        );
        true
    }
}

/// The rules used to make a [`CommonArgs`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonCallArgs {
    /// The variables for a common.
    pub vars: HashMap<String, StringSource>
}

/// The enum of errors that [`CommonCallArgs::make`] can return.
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

impl CommonCallArgs {
    /// Makes a [`CommonArgs`].
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make(&self, job_state: &JobStateView) -> Result<CommonArgs, CommonCallArgsError> {
        Ok(CommonArgs {
            vars: self.vars.iter().map(|(k, v)| Ok::<_, StringSourceError>((k.clone(), get_string!(v, job_state, StringSourceError)))).collect::<Result<HashMap<_, _>, _>>()?
        })
    }

    /// Internal method to make sure I don't accidentally commit Debug variants and other stuff unsuitable for the default config.
    pub(crate) fn is_suitable_for_release(&self, config: &Config) -> bool {
        assert!(
            self.vars.iter().all(|(_, v)| v.is_suitable_for_release(config)),
            "Unsuitable CommonCallArgs detected: {self:?}"
        );
        true
    }
}

/// The arguments for a common.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonArgs {
    /// The variables for a common.
    pub vars: HashMap<String, String>
}

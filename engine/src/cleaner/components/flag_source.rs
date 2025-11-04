//! [`FlagSource`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// Gets a flag from somewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum FlagSource {
    /// Get it from [`Params::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetFlagError))]
    Params(StringSource),
    /// Get it from [`Scratchpad::flags`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetFlagError))]
    Scratchpad(StringSource),
    /// Get it from [`CommonArgs::flags`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetFlagError::NotInCommonContext`].
    ///
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetFlagError))]
    CommonArg(StringSource)    
}

string_or_struct_magic!(FlagSource);

impl FlagSource {
    /// Get the flag.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get(&self, task_state: &TaskStateView) -> Result<bool, GetFlagError> {
        Ok(match self {
            Self::Params    (name) => task_state.params    .flags.contains(get_str!(name, task_state, GetFlagError)),
            Self::Scratchpad(name) => task_state.scratchpad.flags.contains(get_str!(name, task_state, GetFlagError)),
            Self::CommonArg (name) => task_state.common_args.ok_or(GetFlagError::NotInCommonContext)?.flags.contains(get_str!(name, task_state, GetFlagError))
        })
    }
}

impl FromStr for FlagSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Params(s.into()))
    }
}

impl From<&str> for FlagSource {
    fn from(value: &str) -> Self {
        Self::Params(value.into())
    }
}

impl From<String> for FlagSource {
    fn from(value: String) -> Self {
        Self::Params(value.into())
    }
}

impl From<StringSource> for FlagSource {
    fn from(value: StringSource) -> Self {
        Self::Params(value)
    }
}

/// The enum of errors [`FlagSource::get`] can return.
#[derive(Debug, Error)]
pub enum GetFlagError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when trying to use [`FlagSource::CommonArg`] outside of a common context.
    #[error("Tried to use FlagSource::CommonArg outside of a common context.")]
    NotInCommonContext
}

impl From<StringSourceError> for GetFlagError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

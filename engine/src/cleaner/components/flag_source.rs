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
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, FlagSourceError))]
    Params(StringSource),
    TaskContext(StringSource),
    JobContext(StringSource),
    CallArg(StringSource)
}

string_or_struct_magic!(FlagSource);

impl FlagSource {
    /// Get the flag.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j>(&'j self, task_state: &TaskState<'j>) -> Result<bool, FlagSourceError> {
        Ok(match self {
            Self::Params     (name) => task_state.job.cleaner.params.flags                                    .contains(get_str!(name, task_state, FlagSourceError)),
            Self::TaskContext(name) => task_state.context.flags                                               .contains(get_str!(name, task_state, FlagSourceError)),
            Self::JobContext (name) => task_state.job.context.flags                                           .contains(get_str!(name, task_state, FlagSourceError)),
            Self::CallArg    (name) => task_state.call_args.get().ok_or(FlagSourceError::NotInFunction)?.flags.contains(get_str!(name, task_state, FlagSourceError)),
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
pub enum FlagSourceError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,

    #[error("TOOD")]
    NotInFunction
}

impl From<StringSourceError> for FlagSourceError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

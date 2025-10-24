//! [`VarType`].

use std::env;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// The various parts of a [`TaskStateView`] vars exist.
///
/// Defaults to [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum VarType {
    /// Get it from [`TaskStateView::params`]'s [`Params::vars`].
    #[default]
    Params,
    /// Get it from [`TaskStateView::job_context`]'s [`JobContext::vars`].
    JobContext,
    /// Get it from [`TaskStateView::context`]'s [`TaskContext::vars`].
    TaskContext,
    /// Get it from [`TaskStateView::scratchpad`]'s [`Scratchpad::vars`].
    Scratchpad,
    /// Get it from [`TaskStateView::common_args`]'s [`CommonCallArgs::vars`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    CommonArg,
    /// Get it from [`std::env::var`].
    ///
    /// Even though [`std::env::var`] returns an [`Err`] when the environment variable isn't present, this instead returns [`None`].
    /// # Errors
    /// If the environment variable exists but isn't valid UTF-8, returns the error [`GetVarError::EnvVarIsNotUtf8`].
    Env
}

/// The enum of errors [`VarType::get`] and [`VarRef::get`] can return.
#[derive(Debug, Error)]
pub enum GetVarError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when trying to use [`VarType::CommonArg`] outside of a common context.
    #[error("Tried to use VarType::CommonArg outside of a common context.")]
    NotInCommonContext,
    /// Returned when the value of an environment variable isn't valid UTF-8.
    #[error("The value of the environment variable wasn't valid UTF-8")]
    EnvVarIsNotUtf8
}

impl From<StringSourceError> for GetVarError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl VarType {
    /// Get the var.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'j: 't, 't: 'c, 'c>(&self, name: &str, task_state: &TaskStateView<'j, 't, 'c>) -> Result<Option<TaskCow<'j, 't, 'c, str>>, GetVarError> {
        Ok(match self {
            Self::Params      => task_state.params     .vars.get(name).map(|x| TaskCow::Job(x.as_str())),
            Self::JobContext  => task_state.job_context.vars.get(name).map(|x| TaskCow::Job(x.as_str())),
            Self::TaskContext => task_state.context    .vars.get(name).map(|x| TaskCow::Job(x.as_str())),
            Self::Scratchpad  => task_state.scratchpad .vars.get(name).map(|x| TaskCow::Task(x.as_str())),
            Self::CommonArg   => task_state.common_args.ok_or(GetVarError::NotInCommonContext)?.vars.get(name).map(|x| TaskCow::Call(&**x)),
            Self::Env         => match env::var(name) {
                Ok(value) => Some(TaskCow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(GetVarError::EnvVarIsNotUtf8)?
            }
        })
    }
}

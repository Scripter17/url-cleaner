//! [`VarSource`].

use std::borrow::Cow;
use std::str::FromStr;
use std::env;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// Gets a var from somewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub enum VarSource {
    /// Get it from [`Params::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    Params(StringSource),
    /// Get it from [`JobContext::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    JobContext(StringSource),
    /// Get it from [`TaskContext::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    TaskContext(StringSource),
    /// Get it from [`Scratchpad::vars`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    Scratchpad(StringSource),
    /// Get it from [`CommonArgs::vars`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    ///
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    CommonArg(StringSource),
    /// Get it from [`std::env::var`].
    ///
    /// Even though [`std::env::var`] returns an [`Err`] when the environment variable isn't present, this instead returns [`None`].
    /// # Errors
    #[doc = edoc!(geterr(StringSource), getnone(StringSource, GetVarError))]
    ///
    /// If the environment variable exists but isn't valid UTF-8, returns the error [`GetVarError::EnvVarIsNotUtf8`].
    Env(StringSource)
}

string_or_struct_magic!(VarSource);

impl VarSource {
    /// Get the var.
    /// # Errors
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'a>(&self, task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, GetVarError> {
        Ok(match self {
            Self::Params     (name) => task_state.params     .vars.get(get_str!(name, task_state, GetVarError)).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobContext (name) => task_state.job_context.vars.get(get_str!(name, task_state, GetVarError)).map(|x| Cow::Borrowed(x.as_str())),
            Self::TaskContext(name) => task_state.context    .vars.get(get_str!(name, task_state, GetVarError)).map(|x| Cow::Borrowed(x.as_str())),
            Self::Scratchpad (name) => task_state.scratchpad .vars.get(get_str!(name, task_state, GetVarError)).map(|x| Cow::Borrowed(x.as_str())),
            Self::CommonArg  (name) => task_state.common_args.ok_or(GetVarError::NotInCommonContext)?.vars.get(get_str!(name, task_state, GetVarError)).map(|x| Cow::Borrowed(x.as_str())),
            Self::Env        (name) => match env::var(get_str!(name, task_state, GetVarError)) {
                Ok(value) => Some(Cow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(GetVarError::EnvVarIsNotUtf8)?
            }
        })
    }
}

impl FromStr for VarSource {
    type Err = std::convert::Infallible;

    fn from_str(name: &str) -> Result<VarSource, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for VarSource {
    fn from(name: StringSource) -> Self {
        Self::Params(name)
    }
}

impl From<String> for VarSource {
    fn from(name: String) -> Self {
        Self::Params(name.into())
    }
}

impl From<&str> for VarSource {
    fn from(name: &str) -> Self {
        Self::Params(name.into())
    }
}

impl Suitability for VarSource {
    fn assert_suitability(&self, config: &Cleaner) {
        match self {
            Self::Params     (StringSource::String(name)) => assert!(config.docs.vars             .contains_key(name), "Undocumented Var: {name}"),
            Self::JobContext (StringSource::String(name)) => assert!(config.docs.job_context.vars .contains_key(name), "Undocumented JobContext var: {name}"),
            Self::TaskContext(StringSource::String(name)) => assert!(config.docs.task_context.vars.contains_key(name), "Undocumented TaskContext var: {name}"),
            Self::Env        (StringSource::String(name)) => assert!(config.docs.environment_vars .contains_key(name), "Undocumented Env var: {name}"),
            Self::CommonArg(_) | Self::Scratchpad(_) => {},
            _ => panic!("Unsuitable VarSource: {self:?}")
        }
    }
}

/// The enum of errors [`VarSource::get`] can return.
#[derive(Debug, Error)]
pub enum GetVarError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when trying to use [`VarSource::CommonArg`] outside of a common context.
    #[error("Tried to use VarSource::CommonArg outside of a common context.")]
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

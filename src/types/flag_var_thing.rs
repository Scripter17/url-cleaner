

use std::borrow::Cow;
use std::str::FromStr;
use std::env;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

/// The various parts of a [`TaskStateView`] flags exist.
///
/// Defaults to [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum FlagType {
    /// Get it from [`TaskStateView::params`]'s [`Params::vars`].
    #[default]
    Params,
    /// Get it from [`TaskStateView::common_args`]'s [`CommonCallArgs::vars`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    Common,
    /// Get it from [`TaskStateView::scratchpad`]'s [`Scratchpad::vars`]
    Scratchpad
}

#[derive(Debug, Error)]
pub enum GetFlagError {
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[error("Not in a common context.")]
    NotInCommonContext
}

impl From<StringSourceError> for GetFlagError {
    fn from(value: StringSourceError) -> Self {
        Self::StringSourceError(Box::new(value))
    }
}

impl FlagType {
    /// Gets a flag.
    /// # Errors
    /// If `self` is [`Self::Common`] and `task_state`'s [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    pub fn get(&self, task_state: &TaskStateView, name: &str) -> Result<bool, GetFlagError> {
        Ok(match self {
            Self::Params     => task_state.params     .flags.contains(name),
            Self::Common     => task_state.common_args.ok_or(GetFlagError::NotInCommonContext)?.flags.contains(name),
            Self::Scratchpad => task_state.scratchpad .flags.contains(name)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct FlagRef {
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: FlagType,
    pub name: StringSource
}

impl Suitability for FlagRef {
    fn assert_suitability(&self, config: &Config) {
        match (&self.r#type, &self.name) {
            (FlagType::Params, StringSource::String(name)) => assert!(config.docs.flags.contains_key(name), "Undocumented Flag: {name}"),
            (FlagType::Common | FlagType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable FlagRef: {self:?}")
        }
    }
}

impl FlagRef {
    /// Get the flag.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`GetFlagFlagor::StringSourceIsNone`].
    ///
    /// If the call to [`FlagFlage::get`] returns an error, that error is returned.
    pub fn get(&self, task_state: &TaskStateView) -> Result<bool, GetFlagError> {
        self.r#type.get(task_state, get_str!(self.name, task_state, GetFlagError))
    }
}

impl FromStr for FlagRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<FlagRef, Self::Err> {
        Ok(Self {
            r#type: Default::default(),
            name: s.into()
        })
    }
}

string_or_struct_magic!(FlagRef);



/// The various parts of a [`TaskStateView`] vars exist.
///
/// Defaults to [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum VarType {
    /// Get it from [`TaskStateView::params`]'s [`Params::vars`].
    #[default]
    Params,
    /// Get it from [`TaskStateView::job_context`]'s [`JobContext::vars`].
    JobContext,
    /// Get it from [`TaskStateView::context`]'s [`TaskContext::vars`].
    TaskContext,
    /// Get it from [`TaskStateView::common_args`]'s [`CommonCallArgs::vars`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    Common,
    /// Get it from [`TaskStateView::scratchpad`]'s [`Scratchpad::vars`]
    Scratchpad,
    /// Get it from [`std::env::var`].
    Env
}

#[derive(Debug, Error)]
pub enum GetVarError {
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    #[error("Not in a common context.")]
    NotInCommonContext,
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
    /// If `self` is [`Self::Common`] and `task_state`'s [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    pub fn get<'a>(&self, task_state: &'a TaskStateView, name: &str) -> Result<Option<Cow<'a, str>>, GetVarError> {
        Ok(match self {
            Self::Params      => task_state.params      .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobContext  => task_state.job_context.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::TaskContext => task_state.context     .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Common      => task_state.common_args .ok_or(GetVarError::NotInCommonContext)?.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Scratchpad  => task_state.scratchpad  .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Env         => match env::var(name) {
                Ok(value) => Some(Cow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(GetVarError::EnvVarIsNotUtf8)?
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct VarRef {
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: VarType,
    pub name: StringSource
}

impl VarRef {
    /// Get the var.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`GetVarError::StringSourceIsNone`].
    ///
    /// If the call to [`VarType::get`] returns an error, that error is returned.
    pub fn get<'a>(&self, task_state: &'a TaskStateView) -> Result<Option<Cow<'a, str>>, GetVarError> {
        self.r#type.get(task_state, get_str!(self.name, task_state, GetVarError))
    }
}

impl FromStr for VarRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<VarRef, Self::Err> {
        Ok(Self {
            r#type: Default::default(),
            name: s.into()
        })
    }
}

string_or_struct_magic!(VarRef);

impl Suitability for VarRef {
    fn assert_suitability(&self, config: &Config) {
        match (&self.r#type, &self.name) {
            (VarType::Params     , StringSource::String(name)) => assert!(config.docs.vars             .contains_key(name), "Undocumented Var: {name}"),
            (VarType::JobContext , StringSource::String(name)) => assert!(config.docs.job_context.vars .contains_key(name), "Undocumented JobContext var: {name}"),
            (VarType::TaskContext, StringSource::String(name)) => assert!(config.docs.task_context.vars.contains_key(name), "Undocumented TaskContext var: {name}"),
            (VarType::Env        , StringSource::String(name)) => assert!(config.docs.environment_vars .contains_key(name), "Undocumented Env var: {name}"),
            (VarType::Common | VarType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable VarRef: {self:?}")
        }
    }
}

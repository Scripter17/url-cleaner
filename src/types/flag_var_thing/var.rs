//! Unified API for the various places vars exist.

use std::borrow::Cow;
use std::str::FromStr;
use std::env;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::types::*;
use crate::util::*;

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
    ///
    /// Even though [`std::env:;var`] returns an [`Err`] when the environment variable isn't present, this instead returns [`None`].
    /// # Errors
    /// If the environment variable exists but isn't valid UTF-8, returns the error [`GetVarError::EnvVarIsNotUtf8`]
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
    /// See each variant of [`Self`] for when each variant returns an error.
    pub fn get<'a>(&self, task_state: &'a TaskStateView, name: &str) -> Result<Option<Cow<'a, str>>, GetVarError> {
        Ok(match self {
            Self::Params      => task_state.params     .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobContext  => task_state.job_context.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::TaskContext => task_state.context    .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Common      => task_state.common_args.ok_or(GetVarError::NotInCommonContext)?.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Scratchpad  => task_state.scratchpad .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Env         => match env::var(name) {
                Ok(value) => Some(Cow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(GetVarError::EnvVarIsNotUtf8)?
            }
        })
    }
}

/// A "referene" to a variable.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner::types::*;
/// assert_eq!(serde_json::from_str::<VarRef>("\"name\"").unwrap(), VarRef {r#type: Default::default(), name: "name".into()});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct VarRef {
    /// The type of the varaible to get.
    ///
    /// Defaults to [`VarType::Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: VarType,
    /// The name of the variable to get.
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

    fn from_str(name: &str) -> Result<VarRef, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for VarRef {
    fn from(name: StringSource) -> Self {
        Self {
            r#type: Default::default(),
            name
        }
    }
}

impl From<String> for VarRef {
    fn from(name: String) -> Self {
        StringSource::String(name).into()
    }
}

impl From<&str> for VarRef {
    fn from(name: &str) -> Self {
        name.to_string().into()
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

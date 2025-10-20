//! Unified API for the various places vars exist.

use std::borrow::Cow;
use std::str::FromStr;
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
    pub fn get<'a>(&self, name: &str, task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, GetVarError> {
        Ok(match self {
            Self::Params      => task_state.params     .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::JobContext  => task_state.job_context.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::TaskContext => task_state.context    .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Scratchpad  => task_state.scratchpad .vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::CommonArg   => task_state.common_args.ok_or(GetVarError::NotInCommonContext)?.vars.get(name).map(|x| Cow::Borrowed(x.as_str())),
            Self::Env         => match env::var(name) {
                Ok(value) => Some(Cow::Owned(value)),
                Err(env::VarError::NotPresent) => None,
                Err(env::VarError::NotUnicode(_)) => Err(GetVarError::EnvVarIsNotUtf8)?
            }
        })
    }
}

/// A "reference" to a variable.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
/// assert_eq!(serde_json::from_str::<VarRef>("\"name\"").unwrap(), VarRef {r#type: Default::default(), name: "name".into()});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct VarRef {
    /// The type of the variable to get.
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
    pub fn get<'a>(&self, task_state: &TaskStateView<'a>) -> Result<Option<Cow<'a, str>>, GetVarError> {
        debug!(VarRef::get, self);
        self.r#type.get(get_str!(self.name, task_state, GetVarError), task_state)
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
    fn assert_suitability(&self, config: &Cleaner) {
        match (&self.r#type, &self.name) {
            (VarType::Params     , StringSource::String(name)) => assert!(config.docs.vars             .contains_key(name), "Undocumented Var: {name}"),
            (VarType::JobContext , StringSource::String(name)) => assert!(config.docs.job_context.vars .contains_key(name), "Undocumented JobContext var: {name}"),
            (VarType::TaskContext, StringSource::String(name)) => assert!(config.docs.task_context.vars.contains_key(name), "Undocumented TaskContext var: {name}"),
            (VarType::Env        , StringSource::String(name)) => assert!(config.docs.environment_vars .contains_key(name), "Undocumented Env var: {name}"),
            (VarType::CommonArg | VarType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable VarRef: {self:?}")
        }
    }
}

//! Unified API for the various places flags exist.

use std::str::FromStr;

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

/// A "referene" to a flag.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner::types::*;
/// assert_eq!(serde_json::from_str::<FlagRef>("\"name\"").unwrap(), FlagRef {r#type: Default::default(), name: "name".into()});
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct FlagRef {
    /// The type of the flag to get.
    ///
    /// Defaults to [`FlagType::Params`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub r#type: FlagType,
    /// The name of the flag to get.
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

    fn from_str(name: &str) -> Result<FlagRef, Self::Err> {
        Ok(name.into())
    }
}

impl From<StringSource> for FlagRef {
    fn from(name: StringSource) -> Self {
        Self {
            r#type: Default::default(),
            name
        }
    }
}

impl From<String> for FlagRef {
    fn from(name: String) -> Self {
        StringSource::String(name).into()
    }
}

impl From<&str> for FlagRef {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

string_or_struct_magic!(FlagRef);

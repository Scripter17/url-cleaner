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
    /// Get it from [`TaskStateView::cleaner`]'s [`Cleaner::params`]'s [`Params::vars`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, params = Params {
    ///     flags: ["abc".into()].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert!( FlagType::Params.get("abc", &task_state).unwrap());
    /// assert!(!FlagType::Params.get("def", &task_state).unwrap());
    /// ```
    #[default]
    Params,
    /// Get it from [`TaskStateView::scratchpad`]'s [`Scratchpad::vars`]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// url_cleaner_engine::task_state_view!(task_state, scratchpad = Scratchpad {
    ///     flags: ["abc".into()].into(),
    ///     ..Default::default()
    /// });
    ///
    /// assert!(FlagType::Scratchpad.get("abc", &task_state).unwrap())
    /// ```
    Scratchpad,
    /// Get it from [`TaskStateView::common_args`]'s [`CommonCallArgs::vars`].
    /// # Errors
    /// If the [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    CommonArg
}

/// The enum of erros [`FlagType::get`] and [`FlagRef::get`] can return.
#[derive(Debug, Error)]
pub enum GetFlagError {
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] Box<StringSourceError>),
    /// Returned when the specified [`StringSource`] returns [`None`] where it has to return [`Some`].
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when trying to use [`FlagType::CommonArg`] outside of a common context.
    #[error("Tried to use FlagType::CommonArg outside of a common context.")]
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
    /// If `self` is [`Self::CommonArg`] and `task_state`'s [`TaskStateView::common_args`] is [`None`], returns the error [`GetVarError::NotInCommonContext`].
    pub fn get(&self, name: &str, task_state: &TaskStateView) -> Result<bool, GetFlagError> {
        Ok(match self {
            Self::Params     => task_state.cleaner.params.flags.contains(name),
            Self::Scratchpad => task_state.scratchpad    .flags.contains(name),
            Self::CommonArg  => task_state.common_args.ok_or(GetFlagError::NotInCommonContext)?.flags.contains(name)
        })
    }
}

/// A "reference" to a flag.
///
/// Used mainly to allow deserializing `{"type": "Params", "name": "..."}` as `"..."`.
/// # Examples
/// ```
/// use url_cleaner_engine::types::*;
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
    fn assert_suitability(&self, config: &Cleaner) {
        match (&self.r#type, &self.name) {
            (FlagType::Params, StringSource::String(name)) => assert!(config.docs.flags.contains_key(name), "Undocumented Flag: {name}"),
            (FlagType::CommonArg | FlagType::Scratchpad, StringSource::String(_)) => {},
            _ => panic!("Unsuitable FlagRef: {self:?}")
        }
    }
}

impl FlagRef {
    /// Get the flag.
    /// # Errors
    /// If the call to [`StringSource::get`] returns an error, that error is returned.
    ///
    /// If the call to [`StringSource::get`] returns [`None`], returns the error [`GetFlagError::StringSourceIsNone`].
    ///
    /// If the call to [`FlagType::get`] returns an error, that error is returned.
    pub fn get(&self, task_state: &TaskStateView) -> Result<bool, GetFlagError> {
        debug!(FlagRef::get, self);
        match self {
            Self {r#type, name: StringSource::String(name)} => r#type.get(name, task_state),
            _ => self.r#type.get(get_str!(self.name, task_state, GetFlagError), task_state)
        }
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

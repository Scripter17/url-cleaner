//! [`FlagType`].

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// The various parts of a [`TaskStateView`] flags exist.
///
/// Defaults to [`Self::Params`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum FlagType {
    /// Get it from [`TaskStateView::params`]'s [`Params::vars`].
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, params = Params {
    ///     flags: Cow::Owned(["abc".into()].into()),
    ///     ..Default::default()
    /// });
    ///
    /// assert!( FlagType::Params.get("abc", &task_state).unwrap());
    /// assert!(!FlagType::Params.get("def", &task_state).unwrap());
    /// ```
    #[default]
    Params,
    /// Get it from [`TaskStateView::scratchpad`]'s [`Scratchpad::vars`].
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use url_cleaner_engine::prelude::*;
    ///
    /// tsv!(task_state, scratchpad = Scratchpad {
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

/// The enum of errors [`FlagType::get`] and [`FlagRef::get`] can return.
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
            Self::Params     => task_state.params    .flags.contains(name),
            Self::Scratchpad => task_state.scratchpad.flags.contains(name),
            Self::CommonArg  => task_state.common_args.ok_or(GetFlagError::NotInCommonContext)?.flags.contains(name)
        })
    }
}

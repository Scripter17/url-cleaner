//! Allows lazily making a [`Task`] from a [`LazyTaskConfig`].

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// Allows lazily making a [`Task`].
///
/// Returned by [`Job`]s to allow doing the expensive conversion into [`Task`]s in parallel worker threads.
#[derive(Debug, Clone)]
pub struct LazyTask<'a> {
    /// The [`LazyTaskConfig`].
    pub lazy_task_config: LazyTaskConfig,
    /// The [`JobContext`].
    pub job_context: &'a JobContext,
    /// The [`Config`].
    pub config: &'a Config,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

/// The enum of errors that can happen when making a [`TaskConfig`].
#[derive(Debug, Error)]
pub enum MakeTaskError {
    /// Returned when a [`MakeLazyTaskError`] is encountered.
    #[error(transparent)]
    MakeLazyTaskError(#[from] MakeLazyTaskError),
    /// Returned when a [`MakeTaskConfigError`] is encountered.
    #[error(transparent)]
    MakeTaskConfigError(#[from] MakeTaskConfigError)
}

impl<'a> TryFrom<LazyTask<'a>> for Task<'a> {
    type Error = MakeTaskError;

    /// Makes the [`Task`].
    /// # Errors
    /// If the call to [`LazyTaskConfig::make`] returns an error, that error is returned.
    fn try_from(value: LazyTask<'a>) -> Result<Self, Self::Error> {
        let TaskConfig {url, context} = value.lazy_task_config.make()?;
        Ok(Self {
            url,
            context,
            job_context: value.job_context,
            config: value.config,
            #[cfg(feature = "cache")]
            cache: value.cache
        })
    }
}

impl<'a> LazyTask<'a> {
    /// Makes the [`Task`].
    /// # Errors
    /// If the call to [`LazyTaskConfig::make`] returns an error, that error is returned.
    pub fn make(self) -> Result<Task<'a>, MakeTaskError> {
        self.try_into()
    }
}

impl<'a> From<Task<'a>> for LazyTask<'a> {
    fn from(value: Task<'a>) -> Self {
        Self {
            lazy_task_config: LazyTaskConfig::Made(TaskConfig {url: value.url, context: value.context}),
            job_context: value.job_context,
            config: value.config,
            #[cfg(feature = "cache")]
            cache: value.cache
        }
    }
}

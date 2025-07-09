//! Allows lazily making a [`Task`] from a [`LazyTaskConfig`].

use thiserror::Error;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Allows lazily making a [`Task`].
///
/// Returned by [`Job`]s to allow doing the expensive conversion into [`Task`]s in parallel worker threads.
#[derive(Debug, Clone)]
pub struct LazyTask<'a> {
    /// The [`LazyTaskConfig`].
    pub config: LazyTaskConfig,
    /// The [`JobContext`].
    pub job_context: &'a JobContext,
    /// The [`Cleaner`].
    pub cleaner: &'a Cleaner<'a>,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: CacheHandle<'a>
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
    #[doc = edoc!(callerr(LazyTaskConfig::make))]
    fn try_from(value: LazyTask<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            config: value.config.make()?,
            job_context: value.job_context,
            cleaner: value.cleaner,
            #[cfg(feature = "cache")]
            cache: value.cache
        })
    }
}

impl<'a> LazyTask<'a> {
    /// Makes the [`Task`].
    /// # Errors
    #[doc = edoc!(callerr(LazyTaskConfig::make))]
    pub fn make(self) -> Result<Task<'a>, MakeTaskError> {
        self.try_into()
    }
}

impl<'a> From<Task<'a>> for LazyTask<'a> {
    fn from(value: Task<'a>) -> Self {
        Self {
            config: value.config.into(),
            job_context: value.job_context,
            cleaner: value.cleaner,
            #[cfg(feature = "cache")]
            cache: value.cache
        }
    }
}

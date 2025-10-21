//! [`LazyTask`].

use thiserror::Error;

use crate::prelude::*;

/// Cheap intermediate step between [`LazyTaskConfig`] and [`Task`] to allow using worker threads.
///
/// Mainly used to make [`Task`]s via [`Self::make`].
#[derive(Debug, Clone)]
pub struct LazyTask<'j, 't> {
    /// The [`LazyTaskConfig`].
    ///
    /// Uses the lifetime `'t` to allow re-using buffers it borrows from between [`Self::make`] and [`Task::do`].
    pub config: LazyTaskConfig<'t>,
    /// The [`JobContext`].
    pub job_context: &'j JobContext,
    /// The [`Cleaner`].
    pub cleaner: &'j Cleaner<'j>,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`CacheHandle`].
    #[cfg(feature = "cache")]
    pub cache_handle: CacheHandle<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
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

impl<'j> TryFrom<LazyTask<'j, '_>> for Task<'j> {
    type Error = MakeTaskError;

    /// [`LazyTask::make`].
    /// # Errors
    #[doc = edoc!(callerr(LazyTask::make))]
    fn try_from(value: LazyTask<'j, '_>) -> Result<Self, Self::Error> {
        value.make()
    }
}

impl<'j> LazyTask<'j, '_> {
    /// Makes the [`Task`].
    ///
    /// Notably drops the `'t` lifetime to allow re-using buffers [`Self::config`] borrows from between [`Self::make`] and [`Task::do`].
    /// # Errors
    #[doc = edoc!(callerr(LazyTaskConfig::make))]
    pub fn make(self) -> Result<Task<'j>, MakeTaskError> {
        Ok(Task {
            config      : self.config.make()?,
            job_context : self.job_context,
            cleaner     : self.cleaner,
            unthreader  : self.unthreader,
            #[cfg(feature = "cache")]
            cache_handle: self.cache_handle,
            #[cfg(feature = "http")]
            http_client : self.http_client
        })
    }
}

impl<'j> From<Task<'j>> for LazyTask<'j, '_> {
    fn from(value: Task<'j>) -> Self {
        Self {
            config      : value.config.into(),
            job_context : value.job_context,
            cleaner     : value.cleaner,
            unthreader  : value.unthreader,
            #[cfg(feature = "cache")]
            cache_handle: value.cache_handle,
            #[cfg(feature = "http")]
            http_client : value.http_client
        }
    }
}

//! [`Task`] and co.

use thiserror::Error;

use crate::prelude::*;

pub mod lazy_task_config;
pub mod task_config;
pub mod task_context;
pub mod lazy_task;
pub mod task_state;
pub mod scratchpad;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::lazy_task_config::*;
    pub use super::task_config::*;
    pub use super::task_context::*;
    pub use super::lazy_task::*;
    pub use super::task_state::*;
    pub use super::scratchpad::*;

    pub use super::{Task, DoTaskError};
}

/// A task to be done with [`Self::do`].
///
/// Usually made via [`LazyTask::make`].
#[derive(Debug, Clone)]
pub struct Task<'j> {
    /// The [`TaskConfig`].
    pub config: TaskConfig,
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

impl Task<'_> {
    /// Do the task, returning the resulting [`BetterUrl`].
    /// # Errors
    #[doc = edoc!(applyerr(Cleaner))]
    pub fn r#do(mut self) -> Result<BetterUrl, DoTaskError> {
        self.cleaner.apply(&mut TaskState {
            url         : &mut self.config.url,
            scratchpad  : &mut Default::default(),
            common_args : None,
            context     : &self.config.context,
            job_context : self.job_context,
            params      : &self.cleaner.params,
            commons     : &self.cleaner.commons,
            unthreader  : self.unthreader,
            #[cfg(feature = "cache")]
            cache_handle: self.cache_handle,
            #[cfg(feature = "http")]
            http_client : self.http_client
        })?;
        Ok(self.config.url)
    }
}

/// The enums of errors that [`Task::do`] can return.
#[derive(Debug, Error)]
pub enum DoTaskError {
    /// Returned when an [`MakeTaskError`] is encountered.
    #[error(transparent)] MakeTaskError(#[from] MakeTaskError),
    /// Returned when an [`ApplyCleanerError`] is encountered.
    #[error(transparent)] ApplyCleanerError(#[from] ApplyCleanerError)
}

/// Helper macro to make docs briefer.
///
/// Not meant for public use.
#[macro_export]
macro_rules! task {
    ($config:expr, cleaner = $cleaner:expr) => {
        Task {
            config: $config.try_into().unwrap(),
            job_context: &Default::default(),
            cleaner: $cleaner,
            unthreader: &Default::default(),
            #[cfg(feature = "cache")]
            cache_handle: CacheHandle {
                cache: &Default::default(),
                config: Default::default()
            },
            #[cfg(feature = "http")]
            http_client: &Default::default()
        }
    }
}

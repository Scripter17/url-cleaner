//! [`Job`] and co.

use crate::prelude::*;

mod job_context;
mod secrets;
mod auth;
mod task;
mod task_context;
mod task_state;

pub use job_context::*;
pub use secrets::*;
pub use auth::*;
pub use task::*;
pub use task_context::*;
pub use task_state::*;

/// Configuration for a job.
#[derive(Debug)]
pub struct Job<'j> {
    /// The [`JobContext`].
    pub context: JobContext,
    /// The [`Cleaner`].
    pub cleaner: Cleaner<'j>,
    /// The [`Secrets`].
    pub secrets: &'j Secrets,
    /// The unthreader.
    pub unthreader: Option<parking_lot::ReentrantMutex<()>>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: Option<&'j HttpClient>,
    /// The [`CacheClient`].
    #[cfg(feature = "cache")]
    pub cache_client: &'j CacheClient,
    /// The [`CacheConfig`].
    #[cfg(feature = "cache")]
    pub cache_config: CacheConfig,
}

impl<'j> Job<'j> {
    /// Do a task.
    /// # Errors
    /// If [`TryInto::try_into`] returns an error, that error is turned into a [`MakeTaskError`] and returned.
    ///
    /// IF [`Cleaner::apply`] returns an error, that error is returned.
    /// # Panics
    /// If called inside a Tokio runtime, may or may not panic.
    pub fn r#do<T: TryInto<Task>>(&self, task: T) -> Result<(bool, BetterUrl), DoTaskError> where MakeTaskError: From<T::Error> {
        let Task {url, context} = task.try_into().map_err(MakeTaskError::from)?;

        let mut task_state = TaskState {
            url,
            context,
            job: self
        };

        Ok((self.cleaner.apply(&mut task_state)?, task_state.url))
    }
}

//! [`Job`] and co.

use thiserror::Error;
use crate::prelude::*;

mod job_context;
mod task;
mod task_context;
mod task_state;
mod unthreader;

pub use job_context::*;
pub use task::*;
pub use task_context::*;
pub use task_state::*;
pub use unthreader::*;

/// Configuration for a job.
#[derive(Debug, Clone)]
pub struct Job<'j> {
    /// The [`JobContext`].
    pub context: JobContext,
    /// The [`Cleaner`].
    pub cleaner: Cleaner<'j>,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: Cache<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

/// The enums of errors that [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoTaskError {
    /// [`MakeTaskError`].
    #[error(transparent)]
    MakeTaskError(#[from] MakeTaskError),
    /// [`ApplyCleanerError`].
    #[error(transparent)]
    ApplyCleanerError(#[from] ApplyCleanerError)
}

impl<'j> Job<'j> {
    /// Do a task.
    /// # Errors
    #[doc = edoc!(callerr(TaskConfig::make_task), callerr(Cleaner::apply))]
    pub fn r#do<T: TryInto<Task>>(&self, task: T) -> Result<BetterUrl, DoTaskError> where MakeTaskError: From<T::Error> {
        let Task {url, context} = task.try_into().map_err(MakeTaskError::from)?;

        let mut task_state = TaskState {
            url,
            context,
            job: self
        };

        self.cleaner.apply(&mut task_state)?;

        Ok(task_state.url)
    }
}

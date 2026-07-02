//! [`Job`] and co.

use crate::prelude::*;

mod job_context;
mod secrets;
mod auth;
mod task;
mod task_context;
mod task_state;
mod unthreader;

pub use job_context::*;
pub use secrets::*;
pub use auth::*;
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
    /// The [`Secrets`].
    pub secrets: &'j Secrets,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: Cache<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

impl<'j> Job<'j> {
    /// Do a task.
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is turned into a [`MakeTaskError`] and returned.
    ///
    /// IF the call to [`Cleaner::apply`] returns an error, that error is returned.
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

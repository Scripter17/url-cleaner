//! [`Job`] and co.

pub mod job_context;
pub mod task;
pub mod task_context;
pub mod task_config;
pub mod task_state;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::job_context::*;
    pub use super::task::*;
    pub use super::task_context::*;
    pub use super::task_config::*;
    pub use super::task_state::*;

    pub use super::Job;
}

use crate::prelude::*;

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

impl<'j> Job<'j> {
    /// Do a task.
    /// # Errors
    #[doc = edoc!(callerr(TaskConfig::make_task), callerr(Cleaner::apply))]
    pub fn r#do<T: TaskConfig>(&self, task: T) -> Result<BetterUrl, DoTaskError> {
        let Task {url, context} = task.make_task()?;

        let mut task_state = TaskState {
            url,
            context: &context,
            call_args: Default::default(),
            job: self
        };

        self.cleaner.apply(&mut task_state)?;

        Ok(task_state.url)
    }
}

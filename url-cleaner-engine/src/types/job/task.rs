//! A [`Task`] is an individual... task... from a [`Job`].

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// An individual job.
#[derive(Debug)]
pub struct Task<'a> {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The [`TaskContext`] to use.
    pub context: TaskContext,
    /// The [`JobContext`] to use.
    pub job_context: &'a JobContext,
    /// The [`Cleaner`] to use.
    pub cleaner: &'a Cleaner,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl Task<'_> {
    /// Do the job, returning the resulting [`BetterUrl`].
    /// # Errors
    /// If the call to [`Cleaner::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<BetterUrl, DoTaskError> {
        self.cleaner.apply(&mut TaskState {
            url: &mut self.url,
            params: &self.cleaner.params,
            scratchpad: &mut Default::default(),
            context: &self.context,
            job_context: self.job_context,
            #[cfg(feature = "cache")]
            cache: self.cache,
            commons: &self.cleaner.commons,
            common_args: None
        })?;
        Ok(self.url)
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

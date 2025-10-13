//! A [`Task`] is an individual... task... from a [`Job`].

use thiserror::Error;

use crate::types::*;
use crate::glue::prelude::*;
use crate::util::*;

/// An individual job.
#[derive(Debug, Clone)]
pub struct Task<'a> {
    /// The [`TaskConfig`] to use.
    pub config: TaskConfig,
    /// The [`JobContext`] to use.
    pub job_context: &'a JobContext,
    /// The [`Cleaner`] to use.
    pub cleaner: &'a Cleaner<'a>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: CacheHandle<'a>,
    /// The [`Unthreader`] to use.
    pub unthreader: &'a Unthreader
}

impl Task<'_> {
    /// Do the task, returning the resulting [`BetterUrl`].
    /// # Errors
    #[doc = edoc!(applyerr(Cleaner))]
    pub fn r#do(mut self) -> Result<BetterUrl, DoTaskError> {
        self.cleaner.apply(&mut TaskState {
            url        : &mut self.config.url,
            scratchpad : &mut Default::default(),
            common_args: None,
            context    : &self.config.context,
            job_context: self.job_context,
            params     : &self.cleaner.params,
            commons    : &self.cleaner.commons,
            #[cfg(feature = "cache")]
            cache      : &self.cache,
            unthreader : self.unthreader
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

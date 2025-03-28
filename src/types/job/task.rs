//! A [`Task`] is the unit cleaning is done to.

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// An individual job.
#[derive(Debug)]
pub struct Task<'a> {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The [`Config`] to use.
    pub config: &'a Config,
    /// The [`TaskContext`] to use.
    pub context: TaskContext,
    /// The [`JobContext`] to use.
    pub job_context: &'a JobContext,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl Task<'_> {
    /// Do the job, returning the resulting [`BetterUrl`].
    /// # Errors
    /// If the call to [`Config::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<BetterUrl, DoTaskError> {
        self.config.apply(&mut TaskState {
            url: &mut self.url,
            params: &self.config.params,
            scratchpad: &mut Default::default(),
            context: &self.context,
            job_context: self.job_context,
            #[cfg(feature = "cache")]
            cache: self.cache,
            commons: &self.config.commons,
            common_args: None
        })?;
        Ok(self.url)
    }
}

/// The enums of errros that [`Task::do`] can return.
#[derive(Debug, Error)]
pub enum DoTaskError {
    /// Returned when an [`ApplyConfigError`] is encountered.
    #[error(transparent)] ApplyConfigError(#[from] ApplyConfigError)
}

//! A [`Job`] is the unit cleaning is done to.

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// An individual job.
#[derive(Debug)]
pub struct Job<'a> {
    /// The [`BetterUrl`] to modify.
    pub url: BetterUrl,
    /// The [`Config`] to use.
    pub config: &'a Config,
    /// The [`JobContext`] to use.
    pub context: JobContext,
    /// The [`JobsContext`] to use.
    pub jobs_context: &'a JobsContext,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl Job<'_> {
    /// Do the job, returning the resulting [`BetterUrl`].
    /// # Errors
    /// If the call to [`Config::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<BetterUrl, DoJobError> {
        self.config.apply(&mut JobState {
            url: &mut self.url,
            params: &self.config.params,
            scratchpad: &mut Default::default(),
            context: &self.context,
            jobs_context: self.jobs_context,
            #[cfg(feature = "cache")]
            cache: self.cache,
            commons: &self.config.commons,
            common_args: None
        })?;
        Ok(self.url)
    }
}

/// The enums of errros that [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoJobError {
    /// Returned when an [`ApplyConfigError`] is encountered.
    #[error(transparent)] ApplyConfigError(#[from] ApplyConfigError)
}

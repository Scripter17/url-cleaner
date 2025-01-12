//! An individual job.

use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// An individual job.
#[derive(Debug)]
pub struct Job<'a> {
    /// The URL to modify.
    pub url: BetterUrl,
    /// The config to apply.
    pub config: &'a Config,
    /// The context of [`Self::url`].
    pub context: JobContext,
    /// The cache to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl Job<'_> {
    /// Does the job and returns the resulting [`Url`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<Url, DoJobError> {
        self.config.apply_no_revert(&mut JobState {
            url: &mut self.url,
            params: &self.config.params,
            scratchpad: &mut Default::default(),
            context: &self.context,
            #[cfg(feature = "cache")]
            cache: self.cache,
            commons: &self.config.commons,
            common_args: None
        })?;
        Ok(self.url.into())
    }
}

/// The enums of error [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoJobError {
    /// Returned when a [`ApplyConfigError`] is encountered.
    #[error(transparent)] ApplyConfigError(#[from] ApplyConfigError)
}

//! An individual job.

use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// A job.
#[derive(Debug)]
pub struct Job<'a> {
    /// The URL to modify.
    pub url: Url,
    /// The config to apply.
    pub config: &'a Config,
    /// The context of [`Self::url`].
    pub context: UrlContext,
    /// The cache to use.
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

impl Job<'_> {
    /// Does the job and returns the resulting [`Url`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<Url, DoJobError> {
        let mut scratchpad = Default::default();
        self.config.rules.apply(&mut JobState {
            url: &mut self.url,
            params: &self.config.params,
            scratchpad: &mut scratchpad,
            context: &self.context,
            #[cfg(feature = "cache")]
            cache_handler: self.cache_handler,
            commons: &self.config.commons,
            common_vars: None
        })?;
        Ok(self.url)
    }
}

/// The enums of error [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoJobError {
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)] RuleError(#[from] RuleError)
}

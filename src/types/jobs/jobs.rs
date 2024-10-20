//! Bulk jobs using common configs and cache handlers.

use std::borrow::Cow;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// A [`Job`] creator.
/// 
/// Arguably the main API you should build upon.
pub struct Jobs<'a> {
    /// The [`Config`] to use.
    pub config: Cow<'a, Config>,
    /// The cache handler.
    /// 
    /// Normally should be created via [`Self::config`]'s [`Config::cache_path`] but doesn't need to be.
    /// 
    /// This is intentional so you can override it using, for example, command line arguments.
    #[cfg(feature = "cache")]
    pub cache: Cache,
    /// The iterator [`JobConfig`]s are acquired from.
    pub job_config_source: Box<dyn Iterator<Item = Result<JobConfig, JobConfigSourceError>>>
}

impl ::core::fmt::Debug for Jobs<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Jobs");
        x.field("config", &self.config);
        #[cfg(feature = "cache")]
        x.field("cache", &self.cache);
        x.field("job_config_source", &"...");
        x.finish()
    }
}

impl Jobs<'_> {
    /// Iterates over [`Job`]s created from [`JobConfig`]s returned from [`Self::job_config_source`].
    pub fn iter(&mut self) -> impl Iterator<Item = Result<Job<'_>, GetJobError>> {
        (&mut self.job_config_source)
            .map(|job_config_result| match job_config_result {
                Ok(JobConfig {url, context}) => Ok(Job {
                    url,
                    config: &self.config,
                    context,
                    #[cfg(feature = "cache")]
                    cache: &self.cache
                }),
                Err(e) => Err(e.into())
            })
    }

    /// Creates a new [`Job`] with the provided [`JobConfig`].
    /// 
    /// Can be more convenient than [`Self::iter`].
    #[allow(dead_code, reason = "Public API.")]
    pub fn with_job_config(&self, job_config: JobConfig) -> Job<'_> {
        Job {
            url: job_config.url,
            config: &self.config,
            context: job_config.context,
            #[cfg(feature = "cache")]
            cache: &self.cache
        }
    }
}

/// The enum of errors [`Jobs::iter`] can return.
#[derive(Debug, Error)]
pub enum GetJobError {
    /// Returned when a [`JobConfigSourceError`] is encountered.
    #[error(transparent)]
    JobConfigSourceError(#[from] JobConfigSourceError)
}

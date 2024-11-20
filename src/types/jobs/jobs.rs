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
    pub job_configs_source: Box<dyn Iterator<Item = Result<JobConfig, MakeJobConfigError>>>
}

impl ::core::fmt::Debug for Jobs<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Jobs");
        x.field("config", &self.config);
        #[cfg(feature = "cache")]
        x.field("cache", &self.cache);
        x.field("job_configs_source", &"...");
        x.finish()
    }
}

impl<'a> Jobs<'a> {
    /// Iterates over [`Job`]s created from [`JobConfig`]s returned from [`Self::job_configs_source`].
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Job<'a>, MakeJobError>> {
        (&mut self.job_configs_source)
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
    pub fn with_job_config(&'a self, job_config: JobConfig) -> Job<'a> {
        Job {
            url: job_config.url,
            config: &self.config,
            context: job_config.context,
            #[cfg(feature = "cache")]
            cache: &self.cache
        }
    }
}

/// The enum of errors that can happen when [`Jobs::iter`] tries to get a URL.
#[derive(Debug, Error)]
pub enum MakeJobError {
    /// Returned when a [`MakeJobConfigError`] is encountered.
    #[error(transparent)]
    MakeJobConfigError(#[from] MakeJobConfigError)
}

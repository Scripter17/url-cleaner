//! Bulk jobs using common configs and cache handlers.

use std::error::Error;
use std::borrow::Cow;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// The enum of errors that can happen when [`Jobs::next_job`] tries to get a URL.
#[derive(Debug, Error)]
pub enum JobConfigSourceError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Catch-all for user-defined URL sources with errors not listed here.
    #[allow(dead_code, reason = "Public API for use in other people's code.")]
    #[error(transparent)]
    Other(Box<dyn Error>)
}

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
    pub cache_handler: CacheHandler,
    /// The iterator [`JobConfig`]s are acquired from.
    pub configs_source: Box<dyn Iterator<Item = Result<JobConfig, JobConfigSourceError>>>
}

impl ::core::fmt::Debug for Jobs<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Jobs");
        x.field("config", &self.config);
        #[cfg(feature = "cache")]
        x.field("cache_handler", &self.cache_handler);
        x.field("configs_source", &"...");
        x.finish()
    }
}

impl<'a> Jobs<'_> {
    /// Iterates over [`Job`]s created from [`JobConfig`]s returned from [`Self::configs_source`].
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Job<'a>, GetJobError>> {
        (&mut self.configs_source)
            .map(|job_config|
                job_config.map(|job_config| Job {
                    url: job_config.url,
                    config: &self.config,
                    context: job_config.context,
                    #[cfg(feature = "cache")]
                    cache_handler: &self.cache_handler
                })
                .map_err(Into::into)
            )
    }
}

/// The enum of errors [`Jobs::next_job`] can return.
#[derive(Debug, Error)]
pub enum GetJobError {
    /// Returned when a [`JobConfigSourceError`] is encountered.
    #[error(transparent)]
    JobConfigSourceError(#[from] JobConfigSourceError)
}

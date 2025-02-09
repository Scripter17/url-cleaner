//! Bulk jobs using common configs and cache handlers.

#![allow(dead_code, reason = "Public API partially not used by the CLI.")]

use std::borrow::Cow;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// The details of how to turn [`JobConfig`]s into [`Job`]s.
#[derive(Debug, Clone)]
pub struct JobsConfig<'a> {
    /// The [`Config`] to use.
    pub config: Cow<'a, Config>,
    /// The cache handler.
    /// 
    /// Normally should be created via [`Self::config`]'s [`Config::cache_path`] but doesn't need to be.
    /// 
    /// This is intentional so you can override it using, for example, command line arguments.
    #[cfg(feature = "cache")]
    pub cache: Cache,
}

impl<'a> JobsConfig<'a> {
    /// Creates a new [`Job`] with the provided [`JobConfig`].
    /// 
    /// Can be more convenient than [`Jobs::iter`].
    #[allow(dead_code, reason = "Public API.")]
    pub fn new_job(&'a self, job_config: JobConfig, jobs_context: &'a JobsContext) -> Job<'a> {
        Job {
            url: job_config.url,
            config: &self.config,
            context: job_config.context,
            jobs_context,
            #[cfg(feature = "cache")]
            cache: &self.cache
        }
    }
}

/// A [`Job`] creator.
/// 
/// Arguably the main API you should build upon.
/// 
/// For some tasks, like doing jobs in parallel, this is a bad API because it forces deserializing [`JobConfig`]s, a pretty complex task, in one thread.
pub struct Jobs<'a> {
    /// The [`JobsConfig`] to use.
    pub jobs_config: JobsConfig<'a>,
    /// The context.
    pub context: JobsContext,
    /// The iterator [`JobConfig`]s are acquired from.
    pub job_configs_source: Box<dyn Iterator<Item = Result<JobConfig, MakeJobConfigError>>>
}

impl ::core::fmt::Debug for Jobs<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Jobs");
        x.field("jobs_config", &self.jobs_config);
        x.field("job_configs_source", &"...");
        x.finish()
    }
}

impl<'a> Jobs<'a> {
    /// Iterates over [`Job`]s created from [`JobConfig`]s returned from [`Self::job_configs_source`].
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Job<'a>, MakeJobError>> {
        (&mut self.job_configs_source)
            .map(|job_config_result| match job_config_result {
                Ok(job_config) => Ok(self.jobs_config.new_job(job_config, &self.context)),
                Err(e) => Err(e.into())
            })
    }
}

/// The enum of errors that can happen when [`Jobs::iter`] tries to get a URL.
#[derive(Debug, Error)]
pub enum MakeJobError {
    /// Returned when a [`MakeJobConfigError`] is encountered.
    #[error(transparent)]
    MakeJobConfigError(#[from] MakeJobConfigError)
}

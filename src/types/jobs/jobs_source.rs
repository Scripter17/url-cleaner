//! Convenient wrapper to turn an iterator of [`JobConfig`]s into an iterator of [`Job`]s.

#![allow(dead_code, reason = "Public API partially not used by the CLI.")]

use std::borrow::Cow;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// Behavior shared by all [`Job`]s from a [`JobsSource`] and possibly mutliple [`JobsSource`]s.
#[derive(Debug, Clone)]
pub struct JobsConfig<'a> {
    /// The [`Config`] to use.
    pub config: Cow<'a, Config>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: Cache,
}

impl<'a> JobsConfig<'a> {
    /// Createsa new [`Job`] using the provided [`JobConfig`] and [`JobsContext`].
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

/// Source for [`Job`]s.
pub struct JobsSource<'a> {
    /// Configuration shared between [`Self`]s.
    pub jobs_config: JobsConfig<'a>,
    /// The context shared by this [`Self`]'s [`Job`]s.
    pub context: Cow<'a, JobsContext>,
    /// Source of [`JobConfig`]s.
    pub job_configs_source: Box<dyn Iterator<Item = Result<JobConfig, MakeJobConfigError>>>
}

impl ::core::fmt::Debug for JobsSource<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("JobsSource");
        x.field("jobs_config", &self.jobs_config);
        x.field("context", &self.context);
        x.field("job_configs_source", &"...");
        x.finish()
    }
}

impl<'a> JobsSource<'a> {
    /// Makes an iterator of [`Job`]s.
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Job<'a>, MakeJobError>> {
        (&mut self.job_configs_source)
            .map(|job_config_result| match job_config_result {
                Ok(job_config) => Ok(self.jobs_config.new_job(job_config, &self.context)),
                Err(e) => Err(e.into())
            })
    }
}

/// The enum of errors that can happen when trying to make a [`Job`].
#[derive(Debug, Error)]
pub enum MakeJobError {
    /// Returned when a [`MakeJobConfigError`] is encountered.
    #[error(transparent)]
    MakeJobConfigError(#[from] MakeJobConfigError)
}

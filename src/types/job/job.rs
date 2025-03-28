//! Convenient wrapper to turn an iterator of [`JobConfig`]s into an iterator of [`Task`]s.

#![allow(dead_code, reason = "Public API partially not used by the CLI.")]

use std::borrow::Cow;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// Behavior shared by all [`Task`]s from a [`Job`] and possibly mutliple [`Job`]s.
#[derive(Debug, Clone)]
pub struct JobConfig<'a> {
    /// The [`Config`] to use.
    pub config: Cow<'a, Config>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: Cache,
}

impl<'a> JobConfig<'a> {
    /// Createsa new [`Task`] using the provided [`JobConfig`] and [`JobContext`].
    #[allow(dead_code, reason = "Public API.")]
    pub fn new_task(&'a self, task_config: TaskConfig, job_context: &'a JobContext) -> Task<'a> {
        Task {
            url: task_config.url,
            config: &self.config,
            context: task_config.context,
            job_context,
            #[cfg(feature = "cache")]
            cache: &self.cache
        }
    }
}

/// Source for [`Task`]s.
pub struct Job<'a> {
    /// Configuration shared between [`Self`]s.
    pub config: JobConfig<'a>,
    /// The context shared by this [`Job`].
    pub context: Cow<'a, JobContext>,
    /// Source of [`JobConfig`]s.
    pub task_configs_source: Box<dyn Iterator<Item = Result<TaskConfig, MakeTaskConfigError>>>
}

impl ::core::fmt::Debug for Job<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Job");
        x.field("config", &self.config);
        x.field("context", &self.context);
        x.field("task_configs_source", &"...");
        x.finish()
    }
}

impl<'a> Job<'a> {
    /// Makes an iterator of [`Job`]s.
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Task<'a>, MakeTaskError>> {
        (&mut self.task_configs_source)
            .map(|task_config_result| match task_config_result {
                Ok(task_config) => Ok(self.config.new_task(task_config, &self.context)),
                Err(e) => Err(e.into())
            })
    }
}

/// The enum of errors that can happen when trying to make a [`Task`].
#[derive(Debug, Error)]
pub enum MakeTaskError {
    /// Returned when a [`MakeTaskConfigError`] is encountered.
    #[error(transparent)]
    MakeTaskConfigError(#[from] MakeTaskConfigError)
}

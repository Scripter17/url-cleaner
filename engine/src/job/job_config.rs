//! [`JobConfig`].

use crate::prelude::*;

/// Configuration for a [`Job`].
///
/// Mainly used in 3 ways:
///
/// 1. Inside a [`Job`].
///
/// 2. Manually converting [`LazyTaskConfig`]s into [`LazyTask`]s using [`Self::make_lazy_task`].
///
/// 3. Manually converting [`TaskConfig`]s into [`Task`]s using [`Self::make_task`].
#[derive(Debug, Clone, Copy)]
pub struct JobConfig<'j> {
    /// The context shared by this [`Job`].
    pub context: &'j JobContext,
    /// The [`Cleaner`].
    pub cleaner: &'j Cleaner<'j>,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: Cache<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

impl<'j> JobConfig<'j> {
    /// Make a [`LazyTask`] using the provided [`LazyTaskConfig`].
    pub fn make_lazy_task<'t, T: Into<LazyTaskConfig<'t>>>(&self, config: T) -> LazyTask<'j, 't> {
        LazyTask {
            config      : config.into(),
            job_context : self.context,
            cleaner     : self.cleaner,
            unthreader  : self.unthreader,
            #[cfg(feature = "cache")]
            cache       : self.cache,
            #[cfg(feature = "http")]
            http_client : self.http_client
        }
    }

    /// [`Self::make_lazy_task`], [`LazyTask::make`], then [`Task::do`].
    /// # Errors
    #[doc = edoc!(callerr(LazyTask::make), callerr(Task::r#do))]
    pub fn do_lazy_task_config<'t, T: Into<LazyTaskConfig<'t>>>(&self, config: T) -> Result<BetterUrl, DoTaskError> {
        self.make_lazy_task(config).make()?.r#do()
    }

    /// Make a [`Task`] using the provided [`TaskConfig`].
    pub fn make_task<T: Into<TaskConfig>>(&self, config: T) -> Task<'j> {
        Task {
            config      : config.into(),
            job_context : self.context,
            cleaner     : self.cleaner,
            unthreader  : self.unthreader,
            #[cfg(feature = "cache")]
            cache       : self.cache,
            #[cfg(feature = "http")]
            http_client : self.http_client
        }
    }

    /// [`Self::make_task`] then [`Task::do`].
    /// # Errors
    #[doc = edoc!(callerr(Task::r#do))]
    pub fn do_task_config<T: Into<TaskConfig>>(&self, config: T) -> Result<BetterUrl, DoTaskError> {
        self.make_task(config).r#do()
    }
}

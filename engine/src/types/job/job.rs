//! A transformer from [`LazyTaskConfig`]s to [`LazyTask`]s.

use std::io;
use std::error::Error;

use thiserror::Error;

use crate::types::*;
use crate::glue::prelude::*;

/// A transformer from [`LazyTaskConfig`]s to [`LazyTask`]s.
///
/// The laziness allows dividing the [`LazyTask`]s into worker threads with minimal (if any) bottlenecking.
/// # Examples
/// ```
/// use std::borrow::Cow;
/// use url_cleaner_engine::types::*;
///
/// let job = Job {
///     config: &JobConfig {
///         context: &Default::default(),
///         cleaner: &Cleaner {
///             actions: Cow::Owned(vec![
///                 Action::RemoveQueryParams(["utm_source".into()].into())
///             ]),
///             ..Default::default()
///         },
#[cfg_attr(feature = "cache", doc = "        cache: &Default::default(),")]
#[cfg_attr(feature = "cache", doc = "        cache_handle_config: Default::default(),")]
///         unthreader: &Default::default()
///     },
///     lazy_task_configs: Box::new([Ok("https://example.com?utm_source=url_cleaner".into())].into_iter())
/// };
///
/// let expectations = ["https://example.com/"];
///
/// for (task, expectation) in job.into_iter().zip(expectations) {
///     assert_eq!(task.unwrap().make().unwrap().r#do().unwrap().as_str(), expectation);
/// }
/// ```
pub struct Job<'c, 'a> {
    /// The [`JobConfig`] to use.
    pub config: &'c JobConfig<'a>,
    /// Source of [`LazyTaskConfig`]s.
    pub lazy_task_configs: Box<dyn Iterator<Item = Result<LazyTaskConfig<'a>, GetLazyTaskConfigError>> + 'a>
}

/// The config for a [`Job`].
///
/// Sometimes useful directly to allow for more efficient [`Task`] making than [`Job::lazy_task_configs`] by using [`Self::make_lazy_task`].
///
/// For example, URL Cleaner's CLI frontend uses this for its buffer system that lets it avoid allocating every STDIN line to a new [`Vec`].
#[derive(Debug)]
pub struct JobConfig<'a> {
    /// The context shared by this [`Job`].
    pub context: &'a JobContext,
    /// The [`Cleaner`] to use.
    pub cleaner: &'a Cleaner<'a>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache,
    /// The [`CacheHandleConfig`] to make a [`CacheHandle`] with [`Self::cache`].
    #[cfg(feature = "cache")]
    pub cache_handle_config: CacheHandleConfig,
    /// The [`Unthreader`] to use.
    pub unthreader: &'a Unthreader,
}

impl<'a> JobConfig<'a> {
    /// Make a [`LazyTask`] using the provided [`LazyTaskConfig`].
    pub fn make_lazy_task<'c>(&self, config: LazyTaskConfig<'c>) -> LazyTask<'c, 'a> {
        LazyTask {
            config,
            job_context: self.context,
            cleaner: self.cleaner,
            #[cfg(feature = "cache")]
            cache: CacheHandle {
                cache: self.cache,
                config: self.cache_handle_config
            },
            unthreader: self.unthreader
        }
    }
}

impl ::core::fmt::Debug for Job<'_, '_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Job");
        x.field("config", &self.config);
        x.field("lazy_task_configs", &"...");
        x.finish()
    }
}

impl<'a> Iterator for Job<'_, 'a> {
    type Item = Result<LazyTask<'a, 'a>, MakeLazyTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.lazy_task_configs.next()? {
            Ok(config) => Ok(self.config.make_lazy_task(config)),
            Err(e) => Err(e.into())
        })
    }
}

/// The enum of errors your code can pass to [`Job`]s to indicate why getting a [`LazyTaskConfig`] failed.
#[derive(Debug, Error)]
pub enum GetLazyTaskConfigError {
    /// Returned when an [`io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// Any other error that your [`TaskConfig`] source can return.
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send>)
}

/// The enum of errors that can happen when trying to make a [`Task`].
#[derive(Debug, Error)]
pub enum MakeLazyTaskError {
    /// Returned when a [`GetLazyTaskConfigError`] is encountered.
    #[error(transparent)]
    GetLazyTaskConfigError(#[from] GetLazyTaskConfigError)
}

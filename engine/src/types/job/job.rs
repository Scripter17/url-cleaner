//! A transformer from [`LazyTaskConfig`]s to [`LazyTask`]s.

use std::io;
use std::error::Error;

use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// A transformer from [`LazyTaskConfig`]s to [`LazyTask`]s.
///
/// The laziness allows dividing the [`LazyTask`]s into worker threads with minimal (if any) bottlenecking.
/// # Examples
/// ```
/// use std::borrow::Cow;
/// use url_cleaner_engine::types::*;
///
/// let job = Job {
///     context: &Default::default(),
///     cleaner: &Cleaner {
///         actions: Cow::Owned(vec![
///             Action::RemoveQueryParams(["utm_source".into()].into())
///         ]),
///         ..Default::default()
///     },
#[cfg_attr(feature = "cache", doc = "    cache: &Default::default(),")]
#[cfg_attr(feature = "cache", doc = "    cache_handle_config: Default::default(),")]
///     lazy_task_configs: Box::new([Ok("https://example.com?utm_source=url_cleaner".into())].into_iter())
/// };
///
/// let expectations = ["https://example.com/"];
///
/// for (task, expectation) in job.into_iter().zip(expectations) {
///     assert_eq!(task.unwrap().make().unwrap().r#do().unwrap().as_str(), expectation);
/// }
/// ```
pub struct Job<'a> {
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
    /// Source of [`LazyTaskConfig`]s.
    pub lazy_task_configs: Box<dyn Iterator<Item = Result<LazyTaskConfig, GetLazyTaskConfigError>> + 'a>
}

impl ::core::fmt::Debug for Job<'_> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Job");
        x.field("context", &self.context);
        x.field("cleaner" , &self.cleaner);
        #[cfg(feature = "cache")]
        x.field("cache"  , &self.cache);
        #[cfg(feature = "cache")]
        x.field("cache_handle_config", &self.cache_handle_config);
        x.field("lazy_task_configs", &"...");
        x.finish()
    }
}

impl<'a> Iterator for Job<'a> {
    type Item = Result<LazyTask<'a>, MakeLazyTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.lazy_task_configs.next()? {
            Ok(config) => Ok(LazyTask {
                config,
                job_context: self.context,
                cleaner: self.cleaner,
                #[cfg(feature = "cache")]
                cache: CacheHandle {
                    cache: self.cache,
                    config: self.cache_handle_config
                }
            }),
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

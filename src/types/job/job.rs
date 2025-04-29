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
/// use url_cleaner::types::*;
///
/// let job = Job {
///     context: &Default::default(),
///     cleaner: &Cleaner {
///         actions: vec![
///             Action::RemoveQueryParams(["utm_source".into()].into())
///         ],
///         ..Default::default()
///     },
#[cfg_attr(feature = "cache", doc = "    cache: &Default::default(),")]
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
    pub cleaner: &'a Cleaner,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache,
    /// Source of [`LazyTaskConfig`]s.
    pub lazy_task_configs: Box<dyn Iterator<Item = Result<LazyTaskConfig, GetLazyTaskConfigError>>>
}

impl ::core::fmt::Debug for Job<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Job");
        x.field("context", &self.context);
        x.field("cleaner" , &self.cleaner);
        #[cfg(feature = "cache")]
        x.field("cache"  , &self.cache);
        x.field("lazy_task_configs", &"...");
        x.finish()
    }
}

impl<'a> IntoIterator for Job<'a> {
    type IntoIter = JobIter<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        JobIter(self)
    }
}

/// An [`Iterator`] over [`Job`]s.
///
/// Yields [`LazyTask`]s.
#[derive(Debug)]
pub struct JobIter<'a>(Job<'a>);

impl<'a> Iterator for JobIter<'a> {
    type Item = Result<LazyTask<'a>, MakeLazyTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.lazy_task_configs.next()? {
            Ok(lazy_task_config) => Ok(LazyTask {
                lazy_task_config,
                job_context: self.0.context,
                cleaner: self.0.cleaner,
                #[cfg(feature = "cache")]
                cache: self.0.cache
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

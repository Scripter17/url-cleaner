//! Bulk jobs using common configs and cache handlers.

use std::error::Error;
use std::borrow::Cow;

use url::Url;
use thiserror::Error;

use crate::types::*;
use crate::glue::*;

/// The enum of errors that can happen when [`Jobs::next_job`] tries to get a URL.
#[derive(Debug, Error)]
pub enum UrlSourceError {
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
/// Note: [`Self::cache_handler`] should be created via `config.cache_handler.as_path().try_into()?` but does not need to be.
/// 
/// This is intentional as it means you can override it using, for example, command line arguments.
pub struct Jobs<'a> {
    /// The [`Config`] tp use.
    pub config: Cow<'a, Config>,
    /// The cache.
    #[cfg(feature = "cache")]
    pub cache_handler: CacheHandler,
    /// The iterator URLs are acquired from.
    pub job_source: Box<dyn Iterator<Item = Result<JobConfig, UrlSourceError>>>,
}

impl ::core::fmt::Debug for Jobs<'_> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut x = f.debug_struct("Jobs");
        x.field("config", &self.config);
        #[cfg(feature = "cache")]
        x.field("cache_handler", &self.cache_handler);
        x.field("job_source", &"...");
        x.finish()
    }
}

impl<'a> Jobs<'_> {
    /// Gets the next [`Job`].
    /// 
    /// Would be implemented as [`Iterator::next`] if not for the need of a `&'a mut self` in the type signature.
    /// # Errors
    /// If the call to [`Self::job_source`]'s [`Iterator::next`] returns an error, that error is returned.
    pub fn next_job(&'a mut self) -> Option<Result<Job<'a>, GetJobError>> {
        Some(match self.job_source.next()? {
            Ok(JobConfig {url, context}) => Ok(Job {
                url,
                config: &self.config,
                context,
                #[cfg(feature = "cache")]
                cache_handler: &mut self.cache_handler
            }),
            // `e @ Err(e) => e?` doesn't work because for some reason it thinks `e` is a `Url`.
            Err(e) => Err(e.into())
        })
    }

    /// Does all the jobs returned by [`Self::next_job`] until either `Ok(None)` or `Err(_)` are encountered.
    /// # Errors
    /// If a call to [`Self::next_job`] returns an error, that error is returned.
    /// 
    /// If a call to [`Jobs::do`] returns an error, that error is returned.
    /// # Panics
    /// If a call to [`Vec::push`] panics, that panic is... returned? Thrown? Not caught?
    /// 
    /// If you feed in infinite URLs you get a panic.
    #[allow(dead_code, reason = "For some reason, using expect here complains about no lint being thrown. But the link is thrown if this isn't allowed. Maybe it's because of the r#?")]
    pub fn r#do(mut self) -> Vec<Result<Result<Url, DoJobError>, GetJobError>> {
        // For reasons I don't fully understand, [`std::iter::from_fn`] doesn't work here.
        let mut ret = Vec::new();
        while let Some(maybe_job) = self.next_job() {
            ret.push(maybe_job.map(|job| job.r#do()));
        }
        ret
    }
}

/// The enum of errors [`Jobs::next_job`] can return.
#[derive(Debug, Error)]
pub enum GetJobError {
    /// Returned when a [`UrlSourceError`] is encountered.
    #[error(transparent)]
    UrlSourceError(#[from] UrlSourceError)
}

/// The enum of errors [`Jobs::do`] can return.
#[derive(Debug, Error)]
pub enum DoJobsError {
    /// Returned when a [`GetJobError`] is encountered.
    #[error(transparent)]
    GetJobError(#[from] GetJobError),
    /// Returned when a [`DoJobError`] is encountered.
    #[error(transparent)]
    DoJobError(#[from] DoJobError)
}

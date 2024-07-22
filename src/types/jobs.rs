//! Jobs.

use std::error::Error;

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
    #[allow(dead_code)]
    #[error(transparent)]
    Other(Box<dyn Error>)
}

/// A [`Job`] creator.
/// 
/// Note: [`Self::cache_handler`] should be created via `config.cache_handler.as_path().try_into()?` but does not need to be.
/// 
/// This is intentional as it means you can override it using, for example, command line arguments.
pub struct Jobs {
    /// The [`Config`] tp use.
    pub config: Config,
    /// The cache.
    #[cfg(feature = "cache")]
    pub cache_handler: CacheHandler,
    /// The iterator URLs are acquired from.
    pub url_source: Box<dyn Iterator<Item = Result<Url, UrlSourceError>>>,
}

impl ::core::fmt::Debug for Jobs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.debug_struct("Jobs")
            .field("config", &self.config)
            .field("cache_handler", &self.cache_handler)
            .field("url_source", &"...")
            .finish()
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

impl<'a> Jobs {
    /// Gets the next [`Job`].
    /// 
    /// Would be implemented as [`Iterator::next`] if not for the need of a `&'a mut self` in the type signature.
    /// # Errors
    /// If the call to [`Self::url_source`]'s [`Iterator::next`] returns an error, that error is returned.
    pub fn next_job(&'a mut self) -> Option<Result<Job<'a>, GetJobError>> {
        Some(match self.url_source.next()? {
            Ok(url) => Ok(Job {
                url,
                config: &self.config,
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
    /// If a call to [`Vec::push`] panics, that panic is... returned? Thrown?
    /// 
    /// If you feed in infinite URLs you get a panic.
    #[allow(dead_code)]
    pub fn r#do(mut self) -> Result<Vec<Url>, DoJobsError> {
        let mut ret = Vec::new();
        while let Some(job) = self.next_job() {
            ret.push(job?.r#do()?);
        }
        Ok(ret)
    }
}

/// The enums of error [`Job::do`] can return.
#[derive(Debug, Error)]
pub enum DoJobError {
    /// Returned when a [`RuleError`] is encountered.
    #[error(transparent)] RuleError(#[from] RuleError)
}

/// A job.
#[derive(Debug)]
pub struct Job<'a> {
    /// The URL to modify.
    pub url: Url,
    /// The config to apply.
    pub config: &'a Config,
    /// The cache to use.
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

impl Job<'_> {
    /// Does the job and returns the resulting [`Url`].
    /// # Errors
    /// If the call to [`Rules::apply`] returns an error, that error is returned.
    pub fn r#do(mut self) -> Result<Url, DoJobError> {
        self.config.rules.apply(&mut JobState {
            url: &mut self.url,
            params: &self.config.params,
            vars: Default::default(),
            cache_handler: self.cache_handler
        })?;
        Ok(self.url)
    }
}

/// The current state of the job.
#[derive(Debug)]
pub struct JobState<'a> {
    /// The URL being modified.
    pub url: &'a mut Url,
    /// The flags, variables, etc. defined by the job initiator.
    pub params: &'a Params,
    /// The string vars created and managed by the config.
    pub vars: HashMap<String, String>,
    /// The cache handler.
    pub cache_handler: &'a CacheHandler
}

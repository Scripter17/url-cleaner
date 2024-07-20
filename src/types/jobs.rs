use url::Url;

use thiserror::Error;
#[cfg(feature = "cache")]
use diesel::SqliteConnection;

use crate::types::*;
use crate::glue::*;

#[derive(Debug, Error)]
pub enum UrlSourceError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error)
}

/// A [`Job`] creator.
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

#[derive(Debug, Error)]
pub enum GetJobError {
    #[error("...")]
    NoNextUrl,
    #[error(transparent)]
    UrlSourceError(#[from] UrlSourceError)
}

impl<'a> Jobs {
    pub fn next_job(&'a mut self) -> Result<Job<'a>, GetJobError> {
        Ok(Job {
            url: self.url_source.next().ok_or(GetJobError::NoNextUrl)??,
            config: &self.config,
            cache_handler: &mut self.cache_handler
        })
    }
}

#[derive(Debug, Error)]
pub enum JobError {
    #[error(transparent)] RuleError(#[from] RuleError)
}

#[derive(Debug)]
pub struct Job<'a> {
    pub url: Url,
    pub config: &'a Config,
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

impl Job<'_> {
    pub fn r#do(mut self) -> Result<Url, JobError> {
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
    pub cache_handler: &'a CacheHandler
}

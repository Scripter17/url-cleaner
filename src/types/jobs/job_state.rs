//! The state of a job as it's happening.

use url::Url;

use crate::types::*;
use crate::glue::*;

/// The current state of a job.
#[derive(Debug)]
pub struct JobState<'a> {
    /// The URL being modified.
    pub url: &'a mut Url,
    /// Scratchpad space for [`Mapper`]s to store state in.
    pub scratchpad: &'a mut JobScratchpad,
    /// Vars used in common contexts.
    pub common_args: Option<&'a CommonArgs>,
    /// The context surrounding the URL.
    pub context: &'a UrlContext,
    /// The flags, variables, etc. defined by the job initiator.
    pub params: &'a Params,
    /// Various things that are used multiple times.
    pub commons: &'a Commons,
    /// The cache handler.
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

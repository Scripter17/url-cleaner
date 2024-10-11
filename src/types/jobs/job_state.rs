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
    pub context: &'a JobContext,
    /// The flags, variables, etc. defined by the job initiator.
    pub params: &'a Params,
    /// Various things that are used multiple times.
    pub commons: &'a Commons,
    /// The cache handler.
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

/// The current state of a job.
#[derive(Debug)]
pub struct JobStateView<'a> {
    /// The URL being modified.
    pub url: &'a Url,
    /// Scratchpad space for [`Mapper`]s to store state in.
    pub scratchpad: &'a JobScratchpad,
    /// Vars used in common contexts.
    pub common_args: Option<&'a CommonArgs>,
    /// The context surrounding the URL.
    pub context: &'a JobContext,
    /// The flags, variables, etc. defined by the job initiator.
    pub params: &'a Params,
    /// Various things that are used multiple times.
    pub commons: &'a Commons,
    /// The cache handler.
    #[cfg(feature = "cache")]
    pub cache_handler: &'a CacheHandler
}

impl<'a> JobState<'a> {
    /// For optimization purposes, functions that could take `&JobState` instead take `&JobStateView` to make [`Commons`] easier to handle.
    /// 
    /// Functions that don't have anything to do with [`Commons`] still take [`Self`] for the consistency.
    pub fn to_view(&'a self) -> JobStateView<'a> {
        JobStateView {
            url: self.url,
            scratchpad: self.scratchpad,
            common_args: self.common_args,
            context: self.context,
            params: self.params,
            commons: self.commons,
            #[cfg(feature = "cache")]
            cache_handler: self.cache_handler
        }
    }
}

impl<'a> JobStateView<'a> {
    /// Makes [`get_str`], [`get_option_str`], [`get_string`], and [`get_option_string`] calls shorter.
    pub(crate) const fn to_view(&'a self) -> &'a JobStateView<'a> {
        self
    }
}

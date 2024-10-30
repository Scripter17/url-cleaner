//! The state of a job as it's happening.

use url::Url;

use crate::types::*;
use crate::glue::*;

/// The current state of a [`Job`].
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
    pub cache: &'a Cache
}

impl<'a> JobState<'a> {
    /// For optimization purposes, functions that could take `&JobState` instead take `&JobStateView` to make [`Commons`] easier to handle.
    /// 
    /// Functions that don't have anything to do with [`Commons`] still take [`Self`] for the consistency.
    pub fn to_view(&'a self) -> JobStateView<'a> {
        JobStateView {
            url        : self.url,
            scratchpad : self.scratchpad,
            common_args: self.common_args,
            context    : self.context,
            params     : self.params,
            commons    : self.commons,
            #[cfg(feature = "cache")]
            cache      : self.cache
        }
    }
}

/// Helper macro to make doctests less noisy.
#[macro_export]
#[cfg(feature = "cache")]
macro_rules! job_state {
    ($job_state:ident; $(url = $url:expr;)? $(context = $context:expr;)? $(params = $params:expr;)? $(commons = $commons:expr;)?) => {
        let url = "https://example.com";
        $(let url = $url;)?
        let mut scratchpad = Default::default();
        let context: $crate::types::JobContext = Default::default();
        $(let context = $context;)?
        let params: $crate::types::Params = Default::default();
        $(let params = $params;)?
        let commons: $crate::types::Commons = Default::default();
        $(let commons = $commons;)?
        let cache = Default::default();
        let mut url = ::url::Url::parse(url).unwrap();
        let mut $job_state = url_cleaner::types::JobState {
            url: &mut url,
            scratchpad: &mut scratchpad,
            common_args: None,
            context: &context,
            params: &params,
            commons: &commons,
            cache: &cache
        };
    };
}

/// Helper macro to make doctests less noisy.
#[macro_export]
#[cfg(not(feature = "cache"))]
macro_rules! job_state {
    ($job_state:ident; $(url = $url:expr;)? $(context = $context:expr;)? $(params = $params:expr;)? $(commons = $commons:expr;)?) => {
        let url = "https://example.com";
        $(let url = $url;)?
        let mut scratchpad = Default::default();
        let context: $crate::types::JobContext = Default::default();
        $(let context = $context;)?
        let params: $crate::types::Params = Default::default();
        $(let params = $params;)?
        let commons: $crate::types::Commons = Default::default();
        $(let commons = $commons;)?
        let mut url = ::url::Url::parse(url).unwrap();
        let mut $job_state = url_cleaner::types::JobState {
            url: &mut url,
            scratchpad: &mut scratchpad,
            common_args: None,
            context: &context,
            params: &params,
            commons: &commons
        };
    };
}

/// An immutable view of a [`JobState`].
/// 
/// Exists for nuanced optimization reasons. Sorry for the added API complexity.
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
    pub cache: &'a Cache
}

impl<'a> JobStateView<'a> {
    /// Just returns itself.
    /// 
    /// Exists for internal ergonomics reasons.
    pub const fn to_view(&'a self) -> &'a JobStateView<'a> {
        self
    }
}

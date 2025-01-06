//! The state of a job as it's happening.

use crate::types::*;
use crate::glue::*;

/// The current state of a [`Job`].
#[derive(Debug)]
pub struct JobState<'a> {
    /// The URL being modified.
    pub url: &'a mut BetterUrl,
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
    /// Functions that don't have anything to do with [`Commons`] still take [`JobStateView`] for the consistency.
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
        let mut url = BetterUrl::parse(url).unwrap();
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
        let mut url = BetterUrl::parse(url).unwrap();
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
#[derive(Debug, Clone, Copy)]
pub struct JobStateView<'a> {
    /// The URL being modified.
    /// 
    /// See [`JobState::url`].
    pub url: &'a BetterUrl,
    /// Scratchpad space for [`Mapper`]s to store state in.
    /// 
    /// See [`JobState::scratchpad`].
    pub scratchpad: &'a JobScratchpad,
    /// Vars used in common contexts.
    /// 
    /// See [`JobState::common_args`].
    // One could argue this should be a `&'a Option<CommonArgs>`, but that'd break ABI compatibility or whatever it's called.
    // Transmuting a `JobState` to a `JobStateView` is effectively safe and that change would break that (I think?).
    pub common_args: Option<&'a CommonArgs>,
    /// The context surrounding the URL.
    /// 
    /// See [`JobState::context`].
    pub context: &'a JobContext,
    /// The flags, variables, etc. defined by the job initiator.
    /// 
    /// See [`JobState::params`].
    pub params: &'a Params,
    /// Various things that are used multiple times.
    /// 
    /// See [`JobState::commons`].
    pub commons: &'a Commons,
    /// The cache handler.
    /// 
    /// See [`JobState::cache`].
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl<'a> JobStateView<'a> {
    /// Just returns itself.
    /// 
    /// Exists for internal ergonomics reasons.
    #[allow(clippy::wrong_self_convention, reason = "Don't care.")]
    pub(crate) const fn to_view(&'a self) -> &'a JobStateView<'a> {
        self
    }
}

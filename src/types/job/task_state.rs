//! The current state of an in-progress [`Task`].

use std::borrow::Cow;

use crate::types::*;
use crate::util::*;
use crate::glue::*;

/// The state of a [`Task`] being done.
#[derive(Debug)]
pub struct TaskState<'a> {
    /// The [`BetterUrl`] being modified.
    pub url: &'a mut BetterUrl,
    /// The [`TaskScratchpad`] being used.
    pub scratchpad: &'a mut TaskScratchpad,
    /// The [`CommonCallArgs`] for the current [`Commons`] context, if applicable.
    pub common_args: Option<&'a CommonCallArgs<'a>>,
    /// The [`TaskContext`] of the [`Task`] this came form.
    pub context: &'a TaskContext,
    /// The [`JobContext`] of the [`Job`] this came from.
    pub job_context: &'a JobContext,
    /// The [`Params`] being used.
    pub params: &'a Params,
    /// The [`Commons`] that can be called.
    pub commons: &'a Commons,
    /// The [`Cache`] being used.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl<'a> TaskState<'a> {
    /// Converts `self` to a [`TaskStateView`], which just makes the references immutable.
    ///
    /// `&task_state.to_view()` should always effectively compile down to a [`std::mem::transmute`].
    ///
    /// Once safe transmutes are stabilized, I'll implement [`std::ops::Deref`] like that.
    pub fn to_view(&'a self) -> TaskStateView<'a> {
        TaskStateView {
            url        : self.url,
            scratchpad : self.scratchpad,
            common_args: self.common_args,
            context    : self.context,
            job_context: self.job_context,
            params     : self.params,
            commons    : self.commons,
            #[cfg(feature = "cache")]
            cache      : self.cache
        }
    }
}

/// Helper macro to make docs briefer.
#[macro_export]
macro_rules! task_state {
    ($task_state:ident $(, url = $url:expr)? $(, context = $context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)? $(, job_context = $job_context:expr)?) => {
        let url                                     = "https://example.com"; $(let url         = $url        ;)?
        let context    : $crate::types::TaskContext = Default::default();    $(let context     = $context    ;)?
        let job_context: $crate::types::JobContext  = Default::default();    $(let job_context = $job_context;)?
        let params     : $crate::types::Params      = Default::default();    $(let params      = $params     ;)?
        let commons    : $crate::types::Commons     = Default::default();    $(let commons     = $commons    ;)?

        let mut $task_state = url_cleaner::types::TaskState {
            url        : &mut BetterUrl::parse(url).unwrap(),
            scratchpad : &mut Default::default(),
            common_args: None,
            context    : &context,
            job_context: &job_context,
            params     : &params,
            commons    : &commons,
            #[cfg(feature = "cache")]
            cache      : &Default::default()
        };
    };
}

/// An immutable view of a [`TaskState`].
#[derive(Debug, Clone, Copy)]
pub struct TaskStateView<'a> {
    /// The [`BetterUrl`] being modified.
    pub url: &'a BetterUrl,
    /// The [`TaskScratchpad`] being used.
    pub scratchpad: &'a TaskScratchpad,
    /// The [`CommonCallArgs`] for the current [`Commons`] context, if applicable.
    pub common_args: Option<&'a CommonCallArgs<'a>>,
    /// The [`TaskContext`] of the [`Task`] this came form.
    pub context: &'a TaskContext,
    /// The [`JobContext`] of the [`Job`] this came from.
    pub job_context: &'a JobContext,
    /// The [`Params`] being used.
    pub params: &'a Params,
    /// The [`Commons`] that can be called.
    pub commons: &'a Commons,
    /// The [`Cache`] being used.
    #[cfg(feature = "cache")]
    pub cache: &'a Cache
}

impl<'a> TaskStateView<'a> {
    /// Makes an [`reqwest::blocking::Client`] using the relevant [`HttpClientConfig`] and [`HttpClientConfigDiff`]s.
    /// # Errors
    /// If the call to [`HttpClientConfig::make`] returns an error, that error is returned.
    /// 
    /// If the call to [`reqwest::blocking::ClientBuilder::build`] returns an error, that error is returned.
    #[cfg(feature = "http")]
    pub fn http_client(&self, http_client_config_diff: Option<&HttpClientConfigDiff>) -> reqwest::Result<reqwest::blocking::Client> {
        debug!(self, Params::http_client, self, http_client_config_diff);

        let mut http_client_config = Cow::Borrowed(&self.params.http_client_config);

        if let Some(diff) = http_client_config_diff {diff.apply(http_client_config.to_mut());}

        http_client_config.make()
    }

    /// No-op to make some internal macros more convenient.
    #[allow(clippy::wrong_self_convention, reason = "Don't care.")]
    pub(crate) const fn to_view(&'a self) -> &'a TaskStateView<'a> {
        self
    }
}

/// Helper macro to make docs briefer.
#[macro_export]
macro_rules! task_state_view {
    ($task_state_view:ident $(, url = $url:expr)? $(, context = $context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)? $(, job_context = $job_context:expr)?) => {
        let url                                     = "https://example.com"; $(let url         = $url        ;)?
        let context    : $crate::types::TaskContext = Default::default();    $(let context     = $context    ;)?
        let job_context: $crate::types::JobContext  = Default::default();    $(let job_context = $job_context;)?
        let params     : $crate::types::Params      = Default::default();    $(let params      = $params     ;)?
        let commons    : $crate::types::Commons     = Default::default();    $(let commons     = $commons    ;)?

        let mut $task_state_view = url_cleaner::types::TaskStateView {
            url        : &BetterUrl::parse(url).unwrap(),
            scratchpad : &Default::default(),
            common_args: None,
            context    : &context,
            job_context: &job_context,
            params     : &params,
            commons    : &commons,
            #[cfg(feature = "cache")]
            cache      : &Default::default()
        };
    };
}

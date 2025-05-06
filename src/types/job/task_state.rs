//! The current state of an in-progress [`Task`].

use std::borrow::Cow;

use serde::Serialize;

use crate::types::*;
use crate::util::*;
use crate::glue::*;

/// The state of a [`Task`] being done.
#[derive(Debug, Serialize)]
pub struct TaskState<'a> {
    /// The [`BetterUrl`] being modified.
    pub url: &'a mut BetterUrl,
    /// The [`Scratchpad`] being used.
    pub scratchpad: &'a mut Scratchpad,
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
    #[serde(skip)]
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
    ($task_state:ident $(, url = $url:expr)? $(, scratchpad = $scratchpad:expr)? $(, common_args: $common_args:expr)? $(, context = $context:expr)? $(, job_context = $job_context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)?) => {
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     url                                                = "https://example.com"; $(let url         = $url        ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let mut scratchpad :        $crate::types::Scratchpad      = Default::default();    $(let scratchpad  = $scratchpad ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     common_args: Option<$crate::types::CommonCallArgs> = Default::default();    $(let common_args = $common_args;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     context    :        $crate::types::TaskContext     = Default::default();    $(let context     = $context    ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     job_context:        $crate::types::JobContext      = Default::default();    $(let job_context = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     params     :        $crate::types::Params          = Default::default();    $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     commons    :        $crate::types::Commons         = Default::default();    $(let commons     = $commons    ;)?

        let mut $task_state = $crate::types::TaskState {
            url        : &mut url.try_into().unwrap(),
            scratchpad : &mut scratchpad,
            common_args: common_args.as_ref(),
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
#[derive(Debug, Clone, Copy, Serialize)]
pub struct TaskStateView<'a> {
    /// The [`BetterUrl`] being modified.
    pub url: &'a BetterUrl,
    /// The [`Scratchpad`] being used.
    pub scratchpad: &'a Scratchpad,
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
    #[serde(skip)]
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
    ($task_state_view:ident $(, url = $url:expr)? $(, scratchpad = $scratchpad:expr)? $(, common_args: $common_args:expr)? $(, context = $context:expr)? $(, job_context = $job_context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)?) => {
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let url                                                = "https://example.com"; $(let url         = $url        ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let scratchpad :        $crate::types::Scratchpad      = Default::default();    $(let scratchpad  = $scratchpad ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let common_args: Option<$crate::types::CommonCallArgs> = Default::default();    $(let common_args = $common_args;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let context    :        $crate::types::TaskContext     = Default::default();    $(let context     = $context    ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let job_context:        $crate::types::JobContext      = Default::default();    $(let job_context = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let params     :        $crate::types::Params          = Default::default();    $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let commons    :        $crate::types::Commons         = Default::default();    $(let commons     = $commons    ;)?

        let $task_state_view = $crate::types::TaskStateView {
            url        : &url.try_into().unwrap(),
            scratchpad : &scratchpad,
            common_args: common_args.as_ref(),
            context    : &context,
            job_context: &job_context,
            params     : &params,
            commons    : &commons,
            #[cfg(feature = "cache")]
            cache      : &Default::default()
        };
    };
}

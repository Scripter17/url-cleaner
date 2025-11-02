//! [`TaskStateView`].

use crate::prelude::*;

/// An immutable view of a [`TaskState`].
///
/// Used by components that don't require mutable access to [`Self::url`] and [`Self::scratchpad`], such as [`Condition`], [`StringSource`], etc..
///
/// For an mutable view, see [`TaskState`].
#[derive(Debug, Clone, Copy)]
pub struct TaskStateView<'a> {
    /// The [`BetterUrl`] being modified.
    pub url: &'a BetterUrl,
    /// The [`Scratchpad`] being used.
    pub scratchpad: &'a Scratchpad,
    /// The [`CommonArgs`] for the current [`Commons`] context, if applicable.
    pub common_args: Option<&'a CommonArgs<'a>>,
    /// The [`TaskContext`] of the [`Task`] this came form.
    pub context: &'a TaskContext,
    /// The [`JobContext`] of the [`Job`] this came from.
    pub job_context: &'a JobContext,
    /// The [`Unthreader`].
    pub unthreader: &'a Unthreader,
    /// The [`Params`].
    pub params: &'a Params<'a>,
    /// The [`Commons`] that can be called.
    pub commons: &'a Commons,
    /// The [`Cache`] being used.
    #[cfg(feature = "cache")]
    pub cache: Cache<'a>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'a HttpClient
}

impl<'a> TaskStateView<'a> {
    /// No-op to make some internal macros more convenient.
    pub(crate) const fn to_view(self) -> Self {
        self
    }

    /// Make a [`TaskStateDebugHelper`].
    pub fn debug_helper(&self) -> TaskStateDebugHelper<'_> {
        TaskStateDebugHelper {
            url: self.url,
            scratchpad: self.scratchpad,
            common_args: self.common_args
        }
    }
}

/// Helper macro to make docs briefer.
///
/// Not meant for public use.
#[macro_export]
macro_rules! task_state_view {
    ($task_state_view:ident $(, url = $url:expr)? $(, scratchpad = $scratchpad:expr)? $(, common_args: $common_args:expr)? $(, context = $context:expr)? $(, job_context = $job_context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)?) => {
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let url                                              = "https://example.com"; $(let url         = $url        ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let scratchpad :        $crate::prelude::Scratchpad  = Default::default();    $(let scratchpad  = $scratchpad ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let common_args: Option<$crate::prelude::CommonArgs> = Default::default();    $(let common_args = $common_args;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let context    :        $crate::prelude::TaskContext = Default::default();    $(let context     = $context    ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let job_context:        $crate::prelude::JobContext  = Default::default();    $(let job_context = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let params     :        $crate::prelude::Params      = Default::default();    $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let commons    :        $crate::prelude::Commons     = Default::default();    $(let commons     = $commons    ;)?

        let $task_state_view = {
            $crate::prelude::TaskStateView {
                url        : &url.try_into().unwrap(),
                scratchpad : &scratchpad,
                common_args: common_args.as_ref(),
                context    : &context,
                job_context: &job_context,
                unthreader : &Default::default(),
                params     : &params,
                commons    : &commons,
                #[cfg(feature = "cache")]
                cache: $crate::prelude::Cache {
                    config: Default::default(),
                    inner: &Default::default()
                },
                #[cfg(feature = "http")]
                http_client: &Default::default()
            }
        };
    };
}

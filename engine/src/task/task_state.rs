//! [`TaskState`] and [`TaskStateView`].

use crate::prelude::*;

/// The state of a [`Task`] being done.
///
/// Used by components that require mutable access to [`Self::url`] and [`Self::scratchpad`], such as [`Action`].
///
/// For an immutable view, see [`TaskStateView`].
#[derive(Debug)]
pub struct TaskState<'j: 't, 't: 'c, 'c> {
    /// The [`BetterUrl`] being modified.
    pub url: &'t mut BetterUrl,
    /// The [`Scratchpad`] being used.
    pub scratchpad: &'t mut Scratchpad,
    /// The [`CommonCallArgs`] for the current [`Commons`] context, if applicable.
    pub common_args: Option<&'c CommonCallArgs<'j>>,
    /// The [`TaskContext`] of the [`Task`] this came form.
    pub context: &'j TaskContext,
    /// The [`JobContext`] of the [`Job`] this came from.
    pub job_context: &'j JobContext,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`Params`].
    pub params: &'j Params<'j>,
    /// The [`Commons`] that can be called.
    pub commons: &'j Commons,
    /// The [`CacheHandle`] being used.
    #[cfg(feature = "cache")]
    pub cache_handle: CacheHandle<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

impl<'j: 't, 't: 'c, 'c> TaskState<'j, 't, 'c> {
    /// Converts `self` to a [`TaskStateView`], which just makes the references immutable.
    ///
    /// `&task_state.to_view()` should always effectively compile down to a [`std::mem::transmute`].
    ///
    /// Once safe transmutes are stabilized, I'll implement [`std::ops::Deref`] like that.
    pub fn to_view(&'t self) -> TaskStateView<'j, 't, 'c> {
        TaskStateView {
            url         : self.url,
            scratchpad  : self.scratchpad,
            common_args : self.common_args,
            context     : self.context,
            job_context : self.job_context,
            unthreader  : self.unthreader,
            params      : self.params,
            commons     : self.commons,
            #[cfg(feature = "cache")]
            cache_handle: self.cache_handle,
            #[cfg(feature = "http")]
            http_client : self.http_client
        }
    }

    /// Make a [`TaskStateDebugHelper`].
    pub fn debug_helper(&'t self) -> TaskStateDebugHelper<'t, 'c> {
        TaskStateDebugHelper {
            url: self.url,
            scratchpad: self.scratchpad,
            common_args: self.common_args
        }
    }
}

/// Used by the `debug` feature to only print parts of a [`TaskState`]/[`TaskStateView`] that can change.
#[derive(Debug, Clone, Copy)]
pub struct TaskStateDebugHelper<'t: 'c, 'c> {
    /// [`TaskState::url`].
    pub url: &'t BetterUrl,
    /// [`TaskState::scratchpad`].
    pub scratchpad: &'t Scratchpad,
    /// [`TaskState::common_args`]
    pub common_args: Option<&'c CommonCallArgs<'t>>
}

/// Helper macro to make docs briefer.
///
/// Not meant for public use.
#[macro_export]
macro_rules! task_state {
    ($task_state:ident $(, url = $url:expr)? $(, scratchpad = $scratchpad:expr)? $(, common_args: $common_args:expr)? $(, context = $context:expr)? $(, job_context = $job_context:expr)? $(, params = $params:expr)? $(, commons = $commons:expr)?) => {
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     url                                                  = "https://example.com"; $(let url         = $url        ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let mut scratchpad :        $crate::prelude::Scratchpad      = Default::default();    $(let scratchpad  = $scratchpad ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     common_args: Option<$crate::prelude::CommonCallArgs> = Default::default();    $(let common_args = $common_args;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     context    :        $crate::prelude::TaskContext     = Default::default();    $(let context     = $context    ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     job_context:        $crate::prelude::JobContext      = Default::default();    $(let job_context = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     params     :        $crate::prelude::Params          = Default::default();    $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let     commons    :        $crate::prelude::Commons         = Default::default();    $(let commons     = $commons    ;)?

        let mut $task_state = {
            $crate::prelude::TaskState {
                url         : &mut url.try_into().unwrap(),
                scratchpad  : &mut scratchpad,
                common_args : common_args.as_ref(),
                context     : &context,
                job_context : &job_context,
                unthreader  : &Default::default(),
                params      : &params,
                commons     : &commons,
                #[cfg(feature = "cache")]
                cache_handle: url_cleaner_engine::prelude::CacheHandle {
                    cache: &Default::default(),
                    config: Default::default()
                },
                #[cfg(feature = "http")]
                http_client : &Default::default()
            }
        };
    };
}

/// An immutable view of a [`TaskState`].
///
/// Used by components that don't require mutable access to [`Self::url`] and [`Self::scratchpad`], such as [`Condition`], [`StringSource`], etc..
///
/// For an mutable view, see [`TaskState`].
#[derive(Debug, Clone, Copy)]
pub struct TaskStateView<'j: 't, 't: 'c, 'c> {
    /// The [`BetterUrl`] being modified.
    pub url: &'t BetterUrl,
    /// The [`Scratchpad`] being used.
    pub scratchpad: &'t Scratchpad,
    /// The [`CommonCallArgs`] for the current [`Commons`] context, if applicable.
    pub common_args: Option<&'c CommonCallArgs<'j>>,
    /// The [`TaskContext`] of the [`Task`] this came form.
    pub context: &'j TaskContext,
    /// The [`JobContext`] of the [`Job`] this came from.
    pub job_context: &'j JobContext,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`Params`].
    pub params: &'j Params<'j>,
    /// The [`Commons`] that can be called.
    pub commons: &'j Commons,
    /// The [`CacheHandle`] being used.
    #[cfg(feature = "cache")]
    pub cache_handle: CacheHandle<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

impl<'j: 't, 't: 'c, 'c> TaskStateView<'j, 't, 'c> {
    /// No-op to make some internal macros more convenient.
    pub(crate) const fn to_view(self) -> Self {
        self
    }

    /// Make a [`TaskStateDebugHelper`].
    pub fn debug_helper(&self) -> TaskStateDebugHelper<'t, 'c> {
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
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let url                                                  = "https://example.com"; $(let url         = $url        ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let scratchpad :        $crate::prelude::Scratchpad      = Default::default();    $(let scratchpad  = $scratchpad ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let common_args: Option<$crate::prelude::CommonCallArgs> = Default::default();    $(let common_args = $common_args;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let context    :        $crate::prelude::TaskContext     = Default::default();    $(let context     = $context    ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let job_context:        $crate::prelude::JobContext      = Default::default();    $(let job_context = $job_context;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let params     :        $crate::prelude::Params          = Default::default();    $(let params      = $params     ;)?
        #[allow(unused_variables, reason = "You're a macro. Shut up.")] let commons    :        $crate::prelude::Commons         = Default::default();    $(let commons     = $commons    ;)?

        let $task_state_view = {
            $crate::prelude::TaskStateView {
                url         : &url.try_into().unwrap(),
                scratchpad  : &scratchpad,
                common_args : common_args.as_ref(),
                context     : &context,
                job_context : &job_context,
                unthreader  : &Default::default(),
                params      : &params,
                commons     : &commons,
                #[cfg(feature = "cache")]
                cache_handle: url_cleaner_engine::prelude::CacheHandle {
                    cache: &Default::default(),
                    config: Default::default()
                },
                #[cfg(feature = "http")]
                http_client : &Default::default()
            }
        };
    };
}

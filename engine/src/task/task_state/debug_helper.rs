//! [`TaskStateDebugHelper`].

use crate::prelude::*;

/// Used by the `debug` feature to only print parts of a [`TaskState`]/[`TaskStateView`] that can change.
#[derive(Debug, Clone, Copy)]
pub struct TaskStateDebugHelper<'a> {
    /// [`TaskState::url`].
    pub url: &'a BetterUrl,
    /// [`TaskState::scratchpad`].
    pub scratchpad: &'a Scratchpad,
    /// [`TaskState::common_args`]
    pub common_args: Option<&'a CommonArgs<'a>>
}


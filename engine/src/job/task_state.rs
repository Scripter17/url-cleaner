//! [`TaskState`].

use std::cell::Cell;

use crate::prelude::*;

/// The state of a [`Task`] being done.
#[derive(Debug)]
pub struct TaskState<'j> {
    /// [`Task::url`].
    pub url: BetterUrl,
    /// [`Task::context`].
    pub context: &'j TaskContext,
    /// The [`CallArgs`] for the current function call.
    pub call_args: Cell<Option<&'j CallArgs>>,
    /// The [`Job`].
    pub job: &'j Job<'j>
}

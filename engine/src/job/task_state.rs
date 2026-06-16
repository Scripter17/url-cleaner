//! [`TaskState`].

use crate::prelude::*;

/// The state of a [`Task`] being done.
#[derive(Debug)]
pub struct TaskState<'j> {
    /// [`Task::url`].
    pub url: BetterUrl,
    /// [`Task::context`].
    pub context: TaskContext,
    /// The [`Job`].
    pub job: &'j Job<'j>
}

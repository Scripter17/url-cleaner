//! # [`TaskState`] and [`TaskStateView`]
//!
//! [`TaskState`] is a mutable view of the state of a [`Task`] being [`Task::do`]ne.
//!
//! It is used only by components that need to modify [`TaskState::url`] and [`TaskState::scratchpad`]. Currently this is just [`Action`].
//!
//! [`TaskStateView`] is an immutable view of the state of a [`Task`] being [`Task::do`]ne.
//!
//! It is used by components that only need to read the task's state. Having an immutable view allows for certain optimizations.
//!
//! Notably, [`TaskState::to_view`] is literally free because [`TaskState`] and [`TaskStateView`] have the same ABI.

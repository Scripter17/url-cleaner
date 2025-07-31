//! # [`Job`]
//!
//! A [`Job`] is the bulk unit of cleaning URLs.
//!
//! The design of [`Job`] is a bit bizarre but well reasoned.
//!
//! - [`Job::lazy_task_configs`] contains an [`Iterator`] of [`Result`]s of [`LazyTaskConfig`]s.
//!
//! - The [`Result`] is because some inputs of getting [`LazyTaskConfig`]s, such as reading from STDIN, may return an error for some but not other inputs.
//!
//! - [`LazyTaskConfig`]s are very cheap to make from common sources of tasks, such as strings, bytes, JSON values, and so on.
//!
//! - Iterating over a [`Job`] produces [`LazyTask`]s, which is also very cheap.
//!
//! - [`LazyTask`]s can then be sent across threads then [`LazyTask::make`]d into [`Task`]s, which is good because that's a fairly expensive process.
//!
//! - [`Task`]s can then be [`Task::do`]ne to return either the cleaned URL or an error.
//!
//! This design maximizes versatility with minimal performance sacrifices.
//!
//! - URL Cleaner's CLI reads tasks from STDIN, which may at any point start or stop returning errors. [`Job::lazy_task_configs`] taking an [`Iterator`] of [`Result`]s allows handling those errors relatively nicely.
//! 
//! - URL Cleaner Site accepts a list of [`LazyTaskConfig`]s so that only the cost to turn the string into JSON is single threaded.
//!   This also allows for individual tasks to be misformed (because [`LazyTaskConfig::JsonValue`] accepts any JSON value) without stopping the rest of the tasks from being done.
//!
//! - URL Cleaner Discord App uses [`LazyTaskConfig::Str`] to use regex captures from a message without allocating each URL into a [`String`] then (due to the URL spec being inherently complicated) re-allocating those [`String`]s into [`Url`]s.
//!   This is largely irrelevant because the vast majority of the time between wanting to clean a message and getting the result is in UI and network latency, but it helps in other (currently only theoretical) cases.

pub(crate) use super::*;

pub mod job_context;
pub(crate) use job_context::*;
pub mod task_context;
pub(crate) use task_context::*;

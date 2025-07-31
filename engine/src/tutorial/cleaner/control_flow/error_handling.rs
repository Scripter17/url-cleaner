//! # Error handling
//!
//! Various components can return errors. Usually these errors are caused by logic errors, malformed input, or network issues.
//!
//! When a component returns an error, usually the error is then returned by all parent components until being returned as the result of the task.
//!
//! However, sometimes errors can be recovered from and/or safely ignored.

pub(crate) use super::*;

pub mod try_else;
pub(crate) use try_else::*;
pub mod first_not_error;
pub(crate) use first_not_error::*;

//! # Try else
//!
//! "Try else" is one of the most popular and primitive forms of error handling.
//!
//! In the following example, the `Error` variant returns an `ExplicitError`, but the `TryElse` variant "catches" the error, silences it, and instead does its `else` field.
//!
//! ```Json
//! {"TryElse": {
//!   "try": {"Error": "Something that returns an error"},
//!   "else": ...
//! }}
//! ```

pub(crate) use super::*;

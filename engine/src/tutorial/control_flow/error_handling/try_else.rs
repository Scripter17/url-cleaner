//! # Try else
//!
//! "Try else" is one of the most popular and primitive forms of error handling.
//!
//! In the following example, the `Error` variant returns an `ExplicitError`, but the `TryElse` variant "catches" the error, silences it, and instead does its `else` field.
//!
//! ```Json
//! {"TryElse": {
//!   "try": "Something that may or may not return an error.",
//!   "else": "Something to do if the try thing returned an error."
//! }}
//! ```

pub(crate) use super::*;

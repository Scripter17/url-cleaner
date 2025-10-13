//! # First not error
//!
//! `FirstNotError` variants are effectively a sequence of [try else](try_else) variants with each successive component in the previous component's `else` field.
//!
//! For example, these two are functionally identical:
//!
//! ```Json
//! {"TryElse": {
//!   "try": "Thing1",
//!   "else": {"TryElse": {
//!     "try": "Thing2",
//!     "else": {"TryElse": {
//!       "try": "Thing3",
//!       "else": "Thing4"
//!     }}
//!   }}
//! }}
//! ```
//!
//! ```Json
//! {"FirstNotError": [
//!   "Thing1",
//!   "Thing2",
//!   "Thing3",
//!   "Thing4"
//! ]}
//! ```
//!
//! The main difference is that when all attempted things return an error, the error variant is named `FirstNotErrorErrors` with a [`Vec`] of all the errors instead of `TryElseErrors` with an equivalently nested chain of errors.

pub(crate) use super::*;

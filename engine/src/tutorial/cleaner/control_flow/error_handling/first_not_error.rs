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

pub(crate) use super::*;

//! General utility functions.

mod macros;
pub(crate) use macros::*;
mod docs;
mod suitability;
pub(crate) use suitability::*;
mod serde_helpers;
pub(crate) use serde_helpers::*;
mod indexing;
pub(crate) use indexing::*;
mod segments;
pub(crate) use segments::*;

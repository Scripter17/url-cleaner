//! General utility functions.

mod macros;
pub(crate) use macros::*;
mod suitability;
pub(crate) use suitability::*;
#[cfg(feature = "debug")] mod debug;
#[cfg(feature = "debug")] pub(crate) use debug::*;

/// Dud debug macro.
#[cfg(not(feature = "debug"))]
macro_rules! debug {($($_:tt)*) => {}}
#[cfg(not(feature = "debug"))]
pub(crate) use debug;

mod serde_helpers;
pub(crate) use serde_helpers::*;
mod indexing;
pub(crate) use indexing::*;
mod segments;
pub(crate) use segments::*;

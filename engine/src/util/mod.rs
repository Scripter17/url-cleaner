//! General utility functions.

pub(crate) use url_cleaner_macros::*;

mod ext_traits;
mod docs;
mod suitability;
mod serde_helpers;
mod indexing;

pub(crate) use ext_traits::*;
pub(crate) use suitability::*;
pub(crate) use serde_helpers::*;
pub(crate) use indexing::*;

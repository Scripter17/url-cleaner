//! A wrapper around the [`url`] crate that provides higher level operations.

pub mod errors;
pub mod url;
pub mod host;
pub mod ref_host;
pub mod position;
pub mod host_details;
pub mod path;
pub mod query;
pub mod parse;
pub(crate) mod util;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::errors::*;
    pub use super::url::*;
    pub use super::host::*;
    pub use super::ref_host::*;
    pub use super::position::*;
    pub use super::host_details::*;
    pub use super::path::prelude::*;
    pub use super::query::prelude::*;
    pub use super::parse::*;
    pub(crate) use super::util::prelude::*;
}

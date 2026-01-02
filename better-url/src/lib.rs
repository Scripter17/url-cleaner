//! A wrapper around the [`url`] crate that provides higher level operations.

pub mod url;
pub mod host;
pub mod position;
pub mod host_details;
pub mod query;
pub mod parse;
pub(crate) mod util;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::url::*;
    pub use super::host::*;
    pub use super::position::*;
    pub use super::host_details::*;
    pub use super::query::*;
    pub use super::parse::*;
    pub(crate) use super::util::*;
}

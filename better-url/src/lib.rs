//! A wrapper around the [`url`](::url) crate that provides higher level operations.

mod url;
mod parse;
mod position;
pub mod host;
pub mod path;
pub mod query;
pub mod errors;
pub mod util;

pub use url::*;
pub use parse::*;
pub use position::*;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::url::*;
    pub use super::position::*;
    pub use super::parse::*;
    pub use super::host::prelude::*;
    pub use super::path::*;
    pub use super::query::*;
    pub use super::errors::*;
    pub use super::util::prelude::*;
}

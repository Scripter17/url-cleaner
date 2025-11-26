//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

pub mod clean;
pub mod info;
pub(crate) mod util;

#[cfg(feature = "rocket")]
pub mod rocket;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::clean::*;
    pub use super::info::*;

    pub(crate) use super::util::*;
}

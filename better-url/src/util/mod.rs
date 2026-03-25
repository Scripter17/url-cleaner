//! General utility functions.

pub(crate) mod ext_traits;

pub mod percent_encoding;
pub mod host;

/// Prelude module for importing everything here better.
pub(crate) mod prelude {
    pub(crate) use super::ext_traits::prelude::*;

    pub use super::percent_encoding::*;
    pub use super::host::*;
}

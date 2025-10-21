//! Unified APIs for the various places flags and vars exist.

pub mod flag;
pub mod var;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::flag::prelude::*;
    pub use super::var::prelude::*;
}

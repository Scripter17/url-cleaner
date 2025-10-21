//! Unified API for the various places flags exist.

pub mod r#type;
pub mod r#ref;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::r#type::*;
    pub use super::r#ref::*;
}

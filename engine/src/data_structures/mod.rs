//! Data structures like [`Set`], [`Map`] and [`NamedPartitioning`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use prelude::*;

pub mod named_partitioning;
pub mod set;
pub mod map;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::named_partitioning::*;
    pub use super::set::*;
    pub use super::map::*;
}

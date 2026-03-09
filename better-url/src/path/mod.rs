//! Path stuff.

pub mod cow;
pub mod cow_segments;
pub mod r#ref;
pub mod ref_segments;

pub mod raw_segment;
pub mod encode;
pub mod decode;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::cow::*;
    pub use super::cow_segments::*;
    pub use super::r#ref::*;
    pub use super::ref_segments::*;

    pub use super::raw_segment::*;
    pub use super::encode::*;
    pub use super::decode::*;
}

//! Entire paths.

mod either;
mod opaque;
mod non_special;
mod segmented;

pub use either::*;
pub use opaque::*;
pub use non_special::*;
pub use segmented::*;

/// Symmetry with [`NonSpecialPath`].
pub type SpecialNotFilePath<'a> = SpecialNotFileSegmentedPath<'a>;

/// Symmetry with [`NonSpecialPath`].
pub type FilePath<'a> = FileSegmentedPath<'a>;

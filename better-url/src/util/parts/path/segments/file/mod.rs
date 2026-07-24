//! [`FilePathSegments`].

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`FilePathSegments`].
pub fn encode_file_path_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_path_segments(value)
}

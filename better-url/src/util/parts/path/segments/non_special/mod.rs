//! [`NonSpecialPathSegments`].

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`NonSpecialPathSegments`].
pub fn encode_non_special_path_segments<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH)
}

/// Encode a [`NonSpecialPathSegments`] from bytes.
pub fn encode_non_special_path_segments_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, PATH)
}

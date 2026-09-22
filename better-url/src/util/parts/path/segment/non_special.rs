//! [`NonSpecialPathSegment`].

use crate::prelude::*;

/// Encode a [`NonSpecialPathSegment`].
pub fn encode_non_special_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH_SEGMENT)
}

/// Encode a [`NonSpecialPathSegment`] from bytes.
pub fn encode_non_special_path_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, PATH_SEGMENT)
}

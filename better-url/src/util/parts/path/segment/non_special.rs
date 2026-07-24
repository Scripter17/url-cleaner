//! [`NonSpecialPathSegment`].

use crate::prelude::*;

/// Encode a [`NonSpecialPathSegment`].
pub fn encode_non_special_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH_SEGMENT)
}

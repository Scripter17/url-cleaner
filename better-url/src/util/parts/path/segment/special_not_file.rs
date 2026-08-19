//! [`SpecialNotFilePathSegment`].

use crate::prelude::*;

/// Encode a [`SpecialNotFilePathSegment`].
pub fn encode_special_not_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH_SEGMENT)
}

/// Convert a [`NonSpecialPathSegment`] into a [`SpecialNotFilePathSegment`].
pub fn non_special_path_segment_to_special_not_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unsafe {
        percent_encode_one(value, b'\\')
    }
}

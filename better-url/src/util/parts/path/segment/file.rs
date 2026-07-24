//! [`FilePathSegment`].

use crate::prelude::*;

/// Encode a [`FilePathSegment`].
pub fn encode_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, PATH_SEGMENT)
}

/// Convert a [`SpecialNotFilePathSegment`] into a [`FilePathSegment`].
pub fn special_not_file_path_segment_to_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    (false, value.into())
}

/// Convert a [`NonSpecialPathSegment`] into a [`FilePathSegment`].
pub fn non_special_path_segment_to_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = non_special_path_segment_to_special_not_file_path_segment(value);
    let (b, value) = special_not_file_path_segment_to_file_path_segment(value);

    (a || b, value)
}

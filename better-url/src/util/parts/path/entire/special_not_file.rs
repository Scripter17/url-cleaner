//! Special not file paths.

use crate::prelude::*;

/// Encode a [`SpecialNotFilePath`].
pub fn encode_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_segmented_path(value)
}

/// Encode a [`SpecialNotFileSegmentedPath`].
pub fn encode_special_not_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let (mut changed, mut value) = percent_encode::<'_, _, false, true, false>(cow_str_to_bytes(value), PATH);

    if !value.starts_with("/") {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    let (x, value) = resolve_path(value, false);
    changed |= x;
    (changed, value)
}

/// Convert a [`NonSpecialSegmentedPath`] into a [`SpecialNotFileSegmentedPath`].
pub fn non_special_segmented_path_to_special_not_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    for i in 0..value.len() {
        if value.as_bytes()[i] == b'\\' {
            // SAFETY: Replacing ASCII with ASCII is always valid.
            unsafe {
                value.to_mut().as_mut_vec()[i] = b'/';
            }
            changed = true;
        }
    }

    (changed, value)
}

/// Convert an [`OpaquePath`] into a [`SpecialNotFileSegmentedPath`].
pub fn opaque_path_to_special_not_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = opaque_path_to_non_special_segmented_path(value);
    let (b, value) = non_special_segmented_path_to_special_not_file_segmented_path(value);

    (a || b, value)
}

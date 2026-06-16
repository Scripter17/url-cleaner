//! File paths.

use crate::prelude::*;

/// Encode a [`FilePath`].
pub fn encode_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_file_segmented_path(value)
}

/// Encode a [`FileSegmentedPath`].
pub fn encode_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (mut changed, mut value) = percent_encode(cow_str_to_bytes(value.into()), false, true, false, PATH);

    if !value.starts_with("/") {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    if let [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] = value.as_bytes() && x.is_ascii_alphabetic() {
        // SAFETY: Replacing ASCII with ASCII is always valid.
        unsafe {
            value.to_mut().as_mut_vec()[2] = b':';
        }
        changed = true;
    }

    let (x, value) = resolve_path(value, true);
    changed |= x;
    (changed, value)
}

/// Convert a [`SpecialNotFileSegmentedPath`] into a [`FileSegmentedPath`].
pub fn special_not_file_segmented_path_to_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if matches!(value.as_bytes(), [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic()) {
        // SAFETY: Replacing ASCII with ASCII is always valid.
        unsafe {
            value.to_mut().as_mut_vec()[2] = b':';
        }
        changed = false;
    }

    (changed, value)
}

/// Convert a [`NonSpecialSegmentedPath`] into a [`FileSegmentedPath`].
pub fn non_special_segmented_path_to_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = non_special_segmented_path_to_special_not_file_segmented_path(value);
    let (b, value) = special_not_file_segmented_path_to_file_segmented_path(value);

    (a || b, value)
}

/// Convert an [`OpaquePath`] into a [`FileSegmentedPath`].
pub fn opaque_path_to_file_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = opaque_path_to_special_not_file_segmented_path(value);
    let (b, value) = special_not_file_segmented_path_to_file_segmented_path(value);

    (a || b, value)
}

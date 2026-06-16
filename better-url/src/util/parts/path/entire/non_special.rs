//! Non-specail paths.

use crate::prelude::*;

/// Encode a [`NonSpecialPath`].
pub fn encode_non_special_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.is_empty() {
        true  => (false, value),
        false => encode_non_special_segmented_path(value)
    }
}

/// Encode a [`NonSpecialSegmentedPath`].
pub fn encode_non_special_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (mut changed, mut value) = percent_encode(cow_str_to_bytes(value.into()), false, false, false, PATH);

    if !value.starts_with("/") {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    let (x, value) = resolve_path(value, false);
    changed |= x;
    (changed, value)
}

/// Convert an [`OpaquePath`] into a [`NonSpecialPath`].
pub fn opaque_path_to_non_special_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    match value.is_empty() {
        true  => (false, value),
        false => {value.to_mut().insert(0, '/'); (true, value)}
    }
}

/// Convert an [`OpaquePath`] into a [`NonSpecialSegmentedPath`].
pub fn opaque_path_to_non_special_segmented_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    value.to_mut().insert(0, '/');
    (true, value)
}

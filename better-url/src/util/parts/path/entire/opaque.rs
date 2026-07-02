//! Opaque path stuff.

use crate::prelude::*;

/// Encode an [`OpaquePath`].
pub fn encode_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    if value.starts_with("//") {
        value.replace_range(..=0, "%2F");
    }

    if value.ends_with(' ') {
        value.replace_range(value.len() - 1 .., "%20");
    }

    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value), OPAQUE_PATH)
}

/// Turn a [`SegmentedPath`] into an [`OpaquePath`].
pub fn segmented_path_to_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    value.replace_range(..=0, "%2F");
    (true, value)
}

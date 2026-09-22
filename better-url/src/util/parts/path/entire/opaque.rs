//! [`OpaquePath`].

use crate::prelude::*;

/// Make an [`OpaquePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_opaque_path("/abc   def   "), (true, "%2Fabc   def  %20".into()));
/// ```
pub fn encode_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_opaque_path_bytes(cow_str_to_bytes(value))
}

/// Do just the percent encoding for an [`OpaquePath`].
///
/// For the full process, see [`encode_opaque_path`].
pub fn percent_encode_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, OPAQUE_PATH)
}



/// Make an [`OpaquePath`] from bytes.
pub fn encode_opaque_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if value.starts_with(b"/") {
        value.replace_range(..=0, b"%2F");
        changed = true;
    }

    if value.ends_with(b" ") {
        value.replace_range(value.len() - 1 .., b"%20");
        changed = true;
    }

    let (a, value) = percent_encode_opaque_path_bytes(value);

    changed |= a;

    (changed, value)
}

/// Do just the percent encoding for an [`OpaquePath`].
///
/// For the full process, see [`encode_opaque_path_bytes`].
pub fn percent_encode_opaque_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, OPAQUE_PATH)
}



/// Turn a [`SegmentedPath`] into an [`OpaquePath`].
pub fn segmented_path_to_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    value.replace_range(..=0, "%2F");
    (true, value)
}

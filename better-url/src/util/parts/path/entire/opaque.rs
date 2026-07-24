//! [`OpaquePath`].

use crate::prelude::*;

/// Encode an [`OpaquePath`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(make_opaque_path("/abc   def   "), (true, "%2Fabc   def  %20".into()));
/// ```
pub fn make_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if value.starts_with('/') {
        value.replace_range(..=0, "%2F");
        changed = true;
    }

    if value.ends_with(' ') {
        value.replace_range(value.len() - 1 .., "%20");
        changed = true;
    }

    let (a, value) = encode_opaque_path(value);

    changed |= a;

    (changed, value)
}

/// Do just the percent encoding for an [`OpaquePath`].
///
/// See [`make_opaque_path`] for the full process.
pub fn encode_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, OPAQUE_PATH)
}

/// Turn a [`SegmentedPath`] into an [`OpaquePath`].
pub fn segmented_path_to_opaque_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    value.replace_range(..=0, "%2F");
    (true, value)
}

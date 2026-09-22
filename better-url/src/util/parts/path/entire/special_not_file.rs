//! [`SpecialNotFilePath`].

use crate::prelude::*;

/// encode a [`SpecialNotFilePath`].
///
/// Specifically, [`percent_encode_special_not_file_path`] + [`resolve_special_not_file_path`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(encode_special_not_file_path(""            ), (true, "/"        .into()));
///
/// assert_eq!(encode_special_not_file_path("abc"         ), (true, "/abc"     .into()));
///
/// assert_eq!(encode_special_not_file_path("/abc/."      ), (true, "/abc/"    .into()));
/// assert_eq!(encode_special_not_file_path("/abc/.."     ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/abc/./ghi/" ), (true, "/abc/ghi/".into()));
/// assert_eq!(encode_special_not_file_path("/abc/../ghi/"), (true, "/ghi/"    .into()));
///
/// assert_eq!(encode_special_not_file_path("/."          ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/.."         ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/./ghi/"     ), (true, "/ghi/"    .into()));
/// assert_eq!(encode_special_not_file_path("/../ghi/"    ), (true, "/ghi/"    .into()));
///
/// assert_eq!(encode_special_not_file_path("/c:/."       ), (true, "/c:/"     .into()));
/// assert_eq!(encode_special_not_file_path("/c:/.."      ), (true, "/"        .into()));
/// assert_eq!(encode_special_not_file_path("/c:/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(encode_special_not_file_path("/c:/../ghi/" ), (true, "/ghi/"    .into()));
/// ```
pub fn encode_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = percent_encode_special_not_file_path(value);
    let (b, value) = resolve_special_not_file_path       (value);

    (a || b, value)
}

/// Do just the percent encoding and slash unbacking for a [`SpecialNotFilePath`].
///
/// For the full process, see [`encode_special_not_file_path`].
pub fn percent_encode_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_path_segments(value)
}



/// encode a [`SpecialNotFilePath`] from bytes.
///
/// Specifically, [`percent_encode_special_not_file_path`] + [`resolve_special_not_file_path`].
pub fn encode_special_not_file_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = percent_encode_special_not_file_path_bytes(value);
    let (b, value) = resolve_special_not_file_path             (value);

    (a || b, value)
}

/// Do just the percent encoding and slash unbacking for a [`SpecialNotFilePath`].
///
/// For the full process, see [`encode_special_not_file_path_bytes`].
pub fn percent_encode_special_not_file_path_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_path_segments_bytes(value)
}



/// Convert a [`NonSpecialPath`] into a [`SpecialNotFilePath`].
pub fn non_special_path_to_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
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

    if changed {
        value = resolve_special_not_file_path_range(value, ..).1;
    }

    (changed, value)
}



/// Resolve an encoded special not file path.
///
/// Ensures a leading `/`.
pub fn resolve_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if !matches!(value.as_bytes(), [b'/', ..]) {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    let (a, value) = resolve_special_not_file_path_range(value, ..);

    changed |= a;

    (changed, value)
}

/// Resolve an encoded special not file path using only the segments in `range`.
/// # Panics
/// May or may not panic if the range does not begin with a `/` and/or does not end after the end of a segment.
pub fn resolve_special_not_file_path_range<'a, T: Into<Cow<'a, str>>, B: RangeBounds<usize>>(value: T, range: B) -> (bool, Cow<'a, str>) {
    // Every valid special path literal is a valid non-special path literal.
    resolve_non_special_path_range(value, range)
}



/// Convert an [`OpaquePath`] into a [`SpecialNotFilePath`].
pub fn opaque_path_to_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    value.to_mut().insert(0, '/');

    let (_, value) = forward_slashes(value);

    let (_, value) = resolve_special_not_file_path_range(value, ..);

    (true, value)
}

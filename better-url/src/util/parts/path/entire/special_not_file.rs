//! [`SpecialNotFilePath`].

use crate::prelude::*;

/// Make a [`SpecialNotFilePath`].
///
/// Specifically, [`encode_special_not_file_path`] + [`resolve_special_not_file_path`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(make_special_not_file_path(""            ), (true, "/"        .into()));
///
/// assert_eq!(make_special_not_file_path("abc"         ), (true, "/abc"     .into()));
///
/// assert_eq!(make_special_not_file_path("/abc/."      ), (true, "/abc/"    .into()));
/// assert_eq!(make_special_not_file_path("/abc/.."     ), (true, "/"        .into()));
/// assert_eq!(make_special_not_file_path("/abc/./ghi/" ), (true, "/abc/ghi/".into()));
/// assert_eq!(make_special_not_file_path("/abc/../ghi/"), (true, "/ghi/"    .into()));
///
/// assert_eq!(make_special_not_file_path("/."          ), (true, "/"        .into()));
/// assert_eq!(make_special_not_file_path("/.."         ), (true, "/"        .into()));
/// assert_eq!(make_special_not_file_path("/./ghi/"     ), (true, "/ghi/"    .into()));
/// assert_eq!(make_special_not_file_path("/../ghi/"    ), (true, "/ghi/"    .into()));
///
/// assert_eq!(make_special_not_file_path("/c:/."       ), (true, "/c:/"     .into()));
/// assert_eq!(make_special_not_file_path("/c:/.."      ), (true, "/"        .into()));
/// assert_eq!(make_special_not_file_path("/c:/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(make_special_not_file_path("/c:/../ghi/" ), (true, "/ghi/"    .into()));
/// ```
pub fn make_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = encode_special_not_file_path (value);
    let (b, value) = resolve_special_not_file_path(value);

    (a || b, value)
}

/// Do just the percent encoding and slash unbacking for a [`SpecialNotFilePath`].
///
/// For the full process, see [`make_special_not_file_path`].
pub fn encode_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_not_file_path_segments(value)
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

    (changed, value)
}

/// Convert an [`OpaquePath`] into a [`SpecialNotFilePath`].
pub fn opaque_path_to_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = opaque_path_to_non_special_path          (value);
    let (b, value) = non_special_path_to_special_not_file_path(value);

    (a || b, value)
}



/// Resolve an encoded special not file path.
///
/// Ensures a leading `/`.
pub fn resolve_special_not_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if !value.starts_with('/') {
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

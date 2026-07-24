//! [`FilePath`].

use crate::prelude::*;

/// Make a [`FilePath`].
///
/// Specifically, [`encode_file_path`] + [`resolve_file_path`].
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(make_file_path(""            ), (true, "/"        .into()));
///
/// assert_eq!(make_file_path("abc"         ), (true, "/abc"     .into()));
///
/// assert_eq!(make_file_path("/abc/."      ), (true, "/abc/"    .into()));
/// assert_eq!(make_file_path("/abc/.."     ), (true, "/"        .into()));
/// assert_eq!(make_file_path("/abc/./ghi/" ), (true, "/abc/ghi/".into()));
/// assert_eq!(make_file_path("/abc/../ghi/"), (true, "/ghi/"    .into()));
///
/// assert_eq!(make_file_path("/c:/."       ), (true, "/c:/"     .into()));
/// assert_eq!(make_file_path("/c:/.."      ), (true, "/c:/"     .into()));
/// assert_eq!(make_file_path("/c:/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(make_file_path("/c:/../ghi/" ), (true, "/c:/ghi/" .into()));
///
/// assert_eq!(make_file_path("/c|/."       ), (true, "/c:/"     .into()));
/// assert_eq!(make_file_path("/c|/.."      ), (true, "/c:/"     .into()));
/// assert_eq!(make_file_path("/c|/./ghi/"  ), (true, "/c:/ghi/" .into()));
/// assert_eq!(make_file_path("/c|/../ghi/" ), (true, "/c:/ghi/" .into()));
/// ```
pub fn make_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = encode_file_path (value);
    let (b, value) = resolve_file_path(value);

    (a || b, value)
}


/// Do just the percent encoding and slash unbacking for a [`FilePath`].
///
/// For the full process, see [`make_file_path`].
pub fn encode_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_file_path_segments(value)
}



/// Convert a [`SpecialNotFilePath`] into a [`FilePath`].
pub fn special_not_file_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if matches!(value.as_bytes(), [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic()) {
        // SAFETY: Replacing ASCII with ASCII is always valid.
        unsafe {
            value.to_mut().as_mut_vec()[2] = b':';
        }
        changed = true;
    }

    (changed, value)
}

/// Convert a [`NonSpecialPath`] into a [`FilePath`].
pub fn non_special_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = non_special_path_to_special_not_file_path(value);
    let (b, value) = special_not_file_path_to_file_path       (value);

    (a || b, value)
}

/// Convert an [`OpaquePath`] into a [`FilePath`].
pub fn opaque_path_to_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = opaque_path_to_special_not_file_path(value);
    let (b, value) = special_not_file_path_to_file_path  (value);

    (a || b, value)
}


/// Resolve an encoded file path from the start.
///
/// Ensures a leading `/`.
pub fn resolve_file_path<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if !value.starts_with('/') {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    let (a, value) = resolve_file_path_range(value, ..);

    changed |= a;

    (changed, value)
}

/// Resolve an encoded file path using only the segments in `range`.
/// # Panics
/// May or may not panic if the range does not begin with a `/` and/or does not end after the end of a segment.
pub fn resolve_file_path_range<'a, T: Into<Cow<'a, str>>, B: RangeBounds<usize>>(value: T, range: B) -> (bool, Cow<'a, str>) {
    let mut value = cow_str_to_bytes(value.into());
    let mut changed = false;

    let start = match range.start_bound() {
        Bound::Unbounded    => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };

    let mut after = match range.end_bound() {
        Bound::Unbounded    => value.len(),
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };

    assert!(start <= after && after <= value.len());

    debug_assert_eq!(value[start], b'/');
    debug_assert!(after == value.len() || value[after] == b'/');

    let mut i = start;

    if i == 0 && matches!(&*value, [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic()) {
        value.to_mut()[2] = b':';
        changed = true;
    }

    while i < after {
        let left = unsafe {value.get_unchecked(..i)};
        let rest = unsafe {value.get_unchecked(i..)};

        debug_assert_eq!(value[i], b'/');
        debug_assert!(after == value.len() || value[after] == b'/');

        if let Some(x) = munch_single_dot_segment(rest) {
            changed = true;

            if x.is_empty() {
                unsafe {
                    value.truncate_unchecked(i + 1);
                }
                break;
            } else {
                let l = rest.len() - x.len();

                value.to_mut().drain(i .. i + l);

                after -= l;
            }
        } else if let Some(x) = munch_double_dot_segment(rest) {
            changed = true;

            let j = match left {
                [b'/', b'a'..=b'z' | b'A'..=b'Z', b':'] => i,
                _                                       => left.memrchr(b'/').unwrap_or(0),
            };

            if x.is_empty() {
                unsafe {
                    value.truncate_unchecked(j + 1);
                }
                break;
            } else {
                let l = rest.len() - x.len();

                value.to_mut().drain(j .. i + l);

                after -= i + l - j;
            }

            i = j;
        } else if let Some(j) = unsafe {value.get_unchecked(i + 1 .. after)}.memchr(b'/') {
            i += j + 1;
        } else {
            break;
        }
    }

    (changed, unsafe {cow_bytes_to_str_unchecked(value)})
}

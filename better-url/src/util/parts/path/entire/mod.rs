//! Entire path stuff.

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;
mod opaque;

pub use file::*;
pub use special_not_file::*;
pub use non_special::*;
pub use opaque::*;

/// Resolve a segmented path.
///
/// Assumes `value` is a sequence of `/` and [`PathSegment`] pairs.
///
/// If `file` is [`true`], protect leading windows drive letters.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// // assert_eq!(resolve_path("/abc/def", false), (false, "/abc/def".into()));
///
/// assert_eq!(resolve_path("/abc/."      , false), (true, "/abc/"    .into()));
/// assert_eq!(resolve_path("/abc/.."     , false), (true, "/"        .into()));
/// assert_eq!(resolve_path("/abc/./ghi/" , false), (true, "/abc/ghi/".into()));
/// assert_eq!(resolve_path("/abc/../ghi/", false), (true, "/ghi/"    .into()));
///
/// assert_eq!(resolve_path("/c:/."       , false), (true, "/c:/"     .into()));
/// assert_eq!(resolve_path("/c:/.."      , false), (true, "/"        .into()));
/// assert_eq!(resolve_path("/c:/./ghi/"  , false), (true, "/c:/ghi/" .into()));
/// assert_eq!(resolve_path("/c:/../ghi/" , false), (true, "/ghi/"    .into()));
///
/// assert_eq!(resolve_path("/abc/."      , true ), (true, "/abc/"    .into()));
/// assert_eq!(resolve_path("/abc/.."     , true ), (true, "/"        .into()));
/// assert_eq!(resolve_path("/abc/./ghi/" , true ), (true, "/abc/ghi/".into()));
/// assert_eq!(resolve_path("/abc/../ghi/", true ), (true, "/ghi/"    .into()));
///
/// assert_eq!(resolve_path("/c:/."       , true ), (true, "/c:/"     .into()));
/// assert_eq!(resolve_path("/c:/.."      , true ), (true, "/c:/"     .into()));
/// assert_eq!(resolve_path("/c:/./ghi/"  , true ), (true, "/c:/ghi/" .into()));
/// assert_eq!(resolve_path("/c:/../ghi/" , true ), (true, "/c:/ghi/" .into()));
/// ```
pub fn resolve_path<'a, T: Into<Cow<'a, str>>>(value: T, file: bool) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if !value.starts_with('/') {
        value.to_mut().insert(0, '/');
        changed = true;
    }

    let mut value = cow_str_to_bytes(value);

    let mut i = 0;

    while i + 1 < value.len() {
        if value[i + 1] != b'.' && value[i + 1] != b'%' {
            match value[i + 1..].iter().position(|&b| b == b'/') {
                Some(a) => {
                    i += a + 1;
                    continue;
                },
                None => break
            }
        }

        match &value[i + 1..] {
            x if path_segment_bytes_is_double_dot(x) => {
                if file && matches!(value[..i], [b'/', b'a'..=b'z' | b'A'..=b'Z', b':']) {
                    value.to_mut().truncate(4);
                } else {
                    let start_prev_seg = value[..i].iter().rposition(|&b| b == b'/').unwrap_or(0) + 1;
                    value.to_mut().truncate(start_prev_seg);
                }
                changed = true;
            },
            x if let Some(j) = x.iter().position(|&b| b == b'/') && path_segment_bytes_is_double_dot(&x[..j]) => {
                if file && matches!(value[..i], [b'/', b'a'..=b'z' | b'A'..=b'Z', b':']) {
                    value.to_mut().drain(i ..= i + j);
                } else {
                    let start_prev_seg = value[..i].iter().rposition(|&b| b == b'/').unwrap_or(0);
                    value.to_mut().drain(start_prev_seg ..= i + j);
                    i = start_prev_seg;
                }
                changed = true;
            },
            x if path_segment_bytes_is_dot(x) => {
                value.to_mut().truncate(i + 1);
                changed = true;
            },
            x if let Some(j) = x.iter().position(|&b| b == b'/') && path_segment_bytes_is_dot(&x[..j]) => {
                let _ = value.to_mut().drain(i + 1 ..= i + j + 1);
                changed = true;
            },
            x => match x.iter().position(|&b| b == b'/') {
                Some(a) => i += a + 1,
                None => break
            }
        }
    }

    (changed, unsafe {cow_bytes_to_str(value)})
}

/// Extend path segments.
///
/// Assumes `value` is a [`SegmentedPath`] and `T` are valid [`PathSegment`]s of the corrosponding type.
///
/// - If `file` is [`true`], `/c:/..` resolves to `/c:/` instead of `/`.
pub fn extend_path_segments<T: AsRef<str>, I: IntoIterator<Item = T>>(value: &mut String, file: bool, iter: I) -> bool {
    let mut iter = iter.into_iter().peekable();
    let mut changed = false;

    while let Some(segment) = iter.next() {
        let segment = segment.as_ref();

        if path_segment_is_dot(segment) {
            if iter.peek().is_none() {
                value.push('/');
                changed = true;
            }
        } else if path_segment_is_double_dot(segment) {
            if file && matches!(&value.as_bytes(), [b'/', b'a'..=b'z' | b'A'..=b'Z', b':']) {
                if iter.peek().is_none() {
                    value.push('/');
                    changed = true;
                }
            } else if let Some(mut x) = value.rfind('/') {
                if iter.peek().is_none() {
                    x += 1;
                }
                value.replace_range(x .., "");
                changed = true;
            }
        } else {
            if file && value.is_empty() && let [x @ (b'a'..=b'z' | b'A'..=b'Z'), b'|'] = segment.as_bytes() {
                value.insert_str(0, &format!("/{}:", *x as char));
            } else {
                value.extend(["/", segment]);
            }
            changed = true;
        }
    }

    changed
}

/// Insert path segments.
///
/// Assumes `value` is a [`SegmentedPath`] and `T` are valid [`PathSegment`]s of the corrosponding type.
///
/// - If `file` is [`true`], `/c:/..` resolves to `/c:/` instead of `/`.
///
/// - `insert` is the index in `value` of the `/` of the first segment to insert.
/// # Panics
/// Panics if `insert` is greater than `value`'s length.
pub fn insert_path_segments<T: AsRef<str>, I: IntoIterator<Item = T>>(value: &mut String, file: bool, mut insert: usize, iter: I) -> bool {
    assert!(insert <= value.len());

    let mut iter = iter.into_iter().peekable();
    let mut changed = false;

    while let Some(segment) = iter.next() {
        let segment = segment.as_ref();

        if path_segment_is_dot(segment) {
            if insert == value.len() && iter.peek().is_none() {
                value.push('/');
                changed = true;
            }
        } else if path_segment_is_double_dot(segment) {
            if file && matches!(&value.as_bytes()[..insert], [b'/', b'a'..=b'z' | b'A'..=b'Z', b':']) {
                if insert == value.len() && iter.peek().is_none() {
                    value.push('/');
                    changed = true;
                }
            } else if let Some(mut x) = value[..insert].rfind('/') {
                if insert == value.len() && iter.peek().is_none() {
                    x += 1;
                }
                value.replace_range(x .. insert, "");
                insert = x;
                changed = true;
            }
        } else {
            if file && insert == 0 && let [x @ (b'a'..=b'z' | b'A'..=b'Z'), b'|'] = segment.as_bytes() {
                value.insert_str(0, &format!("/{}:", *x as char));
            } else {
                value.insert_with(insert, &["/", segment]);
            }
            insert += 1 + segment.len();
            changed = true;
        }
    }

    changed
}

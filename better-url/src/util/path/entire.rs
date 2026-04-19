//! Entire path stuff.

use crate::prelude::*;

/// Convert a [`SpecialNotFileSegmentedPath`] into a [`FileSegmentedPath`].
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn path_snf_2_f(mut value: Cow<'_, str>) -> Cow<'_, str> {
    if matches!(value.as_bytes(), [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] if x.is_ascii_alphabetic()) {
        // SAFETY: Replacing ASCII with ASCII is always valid.
        unsafe {
            value.to_mut().as_mut_vec()[2] = b':';
        }
    }

    value
}

/// Convert a [`NonSpecialSegmentedPath`] into a [`SpecialNotFileSegmentedPath`].
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn path_nss_2_snf(mut value: Cow<'_, str>) -> Cow<'_, str> {
    for i in 0..value.len() {
        if value.as_bytes()[i] == b'\\' {
            // SAFETY: Replacing ASCII with ASCII is always valid.
            unsafe {
                value.to_mut().as_mut_vec()[i] = b'/';
            }
        }
    }

    value
}

/// Convert an [`OpaquePath`] into a [`NonSpecialSegmentedPath`].
pub fn path_o_2_nss(mut value: Cow<'_, str>) -> Cow<'_, str> {
    value.to_mut().insert(0, '/');
    value
}




/// Convert a [`NonSpecialSegmentedPath`] into a [`FileSegmentedPath`].
pub fn path_nss_2_f(value: Cow<'_, str>) -> Cow<'_, str> {
    path_snf_2_f(path_nss_2_snf(value))
}

/// Convert an [`OpaquePath`] into a [`FileSegmentedPath`].
pub fn path_o_2_f(value: Cow<'_, str>) -> Cow<'_, str> {
    path_snf_2_f(path_o_2_snf(value))
}

/// Convert an [`OpaquePath`] into a [`SpecialNotFileSegmentedPath`].
pub fn path_o_2_snf(value: Cow<'_, str>) -> Cow<'_, str> {
    path_nss_2_snf(path_o_2_nss(value))
}



/// Extend path segments.
///
/// `file` is [`true`] if leading drive letter segments should be protected from `..`.
pub fn extend_path_segments<T: AsRef<str>, I: IntoIterator<Item = T>>(path: &mut String, file: bool, iter: I) -> bool {
    insert_path_segments(path, file, path.len(), iter)
}

/// Insert path segments.
///
/// `file` is [`true`] if leading drive letter segments should be protected from `..`.
///
/// `insert` is the location at which the next path segment's leading `/` will be inserted at.
/// # Panics
/// Panics if `insert` is greater than `path`'s length.
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn insert_path_segments<T: AsRef<str>, I: IntoIterator<Item = T>>(path: &mut String, file: bool, mut insert: usize, iter: I) -> bool {
    assert!(insert <= path.len());

    let mut iter = iter.into_iter().peekable();
    let mut changed = false;

    while let Some(segment) = iter.next() {
        let segment = segment.as_ref();

        if path_segment_is_dot(segment) {
            if insert == path.len() && iter.peek().is_none() {
                path.push('/');
                changed = true;
            }
        } else if path_segment_is_double_dot(segment) {
            if file && matches!(&path.as_bytes()[..insert], [b'/', b'a'..=b'z' | b'A'..=b'Z', b':']) {
                if insert == path.len() && iter.peek().is_none() {
                    path.push('/');
                    changed = true;
                }
            } else if let Some(mut x) = path[..insert].rfind('/') {
                if insert == path.len() && iter.peek().is_none() {
                    x += 1;
                }
                path.replace_range(x .. insert, "");
                insert = x;
                changed = true;
            }
        } else {
            if file && insert == 0 && let [x @ (b'a'..=b'z' | b'A'..=b'Z'), b'|'] = segment.as_bytes() {
                path.insert_str(0, &format!("/{}:", *x as char));
            } else {
                path.insert_with(insert, ["/", segment]);
            }
            insert += 1 + segment.len();
            changed = true;
        }
    }

    changed
}

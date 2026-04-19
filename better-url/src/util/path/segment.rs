//! Path segment stuff.

use crate::prelude::*;

/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter).
pub fn path_segment_is_windows_drive_letter(s: &str) -> bool {
    matches!(s.as_bytes(), [x, b':' | b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_is_normalized_windows_drive_letter(s: &str) -> bool {
    matches!(s.as_bytes(), [x, b':'] if x.is_ascii_alphabetic())
}

/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter) but not a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_is_non_normalized_windows_drive_letter(s: &str) -> bool {
    matches!(s.as_bytes(), [x, b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [single-dot path segment](https://url.spec.whatwg.org/#single-dot-path-segment).
pub fn path_segment_is_dot(s: &str) -> bool {
    matches!(s, "." | "%2e" | "%2E")
}

/// If it's a [double-dot path segment](https://url.spec.whatwg.org/#double-dot-path-segment).
pub fn path_segment_is_double_dot(s: &str) -> bool {
    matches!(s, ".." | ".%2e" | ".%2E" | "%2e." | "%2e%2e" | "%2e%2E" | "%2E." | "%2E%2e" | "%2E%2E")
}



/// Convert a [`SpecialNotFilePathSegment`] into a [`FilePathSegment`].
pub fn path_segment_snf_2_f(value: Cow<'_, str>) -> Cow<'_, str> {
    value
}

/// Convert a [`NonSpecialPathSegment`] into a [`FilePathSegment`].
pub fn path_segment_ns_2_f(value: Cow<'_, str>) -> Cow<'_, str> {
    path_segment_snf_2_f(path_segment_ns_2_snf(value))
}

/// Convert a [`NonSpecialPathSegment`] into a [`SpecialNotFilePathSegment`].
#[allow(clippy::indexing_slicing, reason = "Can't happen.")]
pub fn path_segment_ns_2_snf(mut value: Cow<'_, str>) -> Cow<'_, str> {
    for i in (0..value.len()).rev() {
        if value.as_bytes()[i] == b'\\' {
            value.to_mut().replace_range(i..=i, "%5C");
        }
    }

    value
}

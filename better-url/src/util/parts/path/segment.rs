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



/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter).
pub fn path_segment_bytes_is_windows_drive_letter(s: &[u8]) -> bool {
    matches!(s, [x, b':' | b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_bytes_is_normalized_windows_drive_letter(s: &[u8]) -> bool {
    matches!(s, [x, b':'] if x.is_ascii_alphabetic())
}

/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter) but not a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_bytes_is_non_normalized_windows_drive_letter(s: &[u8]) -> bool {
    matches!(s, [x, b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [single-dot path segment](https://url.spec.whatwg.org/#single-dot-path-segment).
pub fn path_segment_bytes_is_dot(s: &[u8]) -> bool {
    matches!(s, b"." | b"%2e" | b"%2E")
}

/// If it's a [double-dot path segment](https://url.spec.whatwg.org/#double-dot-path-segment).
pub fn path_segment_bytes_is_double_dot(s: &[u8]) -> bool {
    matches!(s, b".." | b".%2e" | b".%2E" | b"%2e." | b"%2e%2e" | b"%2e%2E" | b"%2E." | b"%2E%2e" | b"%2E%2E")
}



/// Encode a [`PathSegment`].
pub fn encode_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_special_path_segment(value.into())
}

/// Encode a [`SpecialNotFilePathSegment`]/[`FilePathSegment`].
pub fn encode_special_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), PATH_SEGMENT)
}

/// Encode a [`NonSpecialPathSegment`].
pub fn encode_non_special_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), PATH_SEGMENT)
}



/// Convert a [`SpecialNotFilePathSegment`] into a [`FilePathSegment`].
pub fn special_not_file_path_segment_to_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    (false, value.into())
}

/// Convert a [`NonSpecialPathSegment`] into a [`FilePathSegment`].
pub fn non_special_path_segment_to_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = non_special_path_segment_to_special_not_file_path_segment(value);
    let (b, value) = special_not_file_path_segment_to_file_path_segment(value);

    (a || b, value)
}

/// Convert a [`NonSpecialPathSegment`] into a [`SpecialNotFilePathSegment`].
pub fn non_special_path_segment_to_special_not_file_path_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    for i in (0..value.len()).rev() {
        if value.as_bytes()[i] == b'\\' {
            value.to_mut().replace_range(i..=i, "%5C");
            changed = true;
        }
    }

    (changed, value)
}

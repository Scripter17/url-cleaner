//! Single path segments.

mod file;
mod special_not_file;
mod non_special;

pub use file::*;
pub use special_not_file::*;
pub use non_special::*;

/// Munch a single dot segment from the start of `x`.
pub(crate) fn munch_single_dot_segment(x: &[u8]) -> Option<&[u8]> {
    match munch_single_dot(munch_slash(x)?)? {
        x @ ([b'/', ..] | []) => Some(x),
        _ => None
    }
}

/// Munch a double dot segment from the start of `x`.
pub(crate) fn munch_double_dot_segment(x: &[u8]) -> Option<&[u8]> {
    match munch_double_dot(munch_slash(x)?)? {
        x @ ([b'/', ..] | []) => Some(x),
        _ => None
    }
}

/// Munch a `/` from the start of `x`.
pub(crate) fn munch_slash(x: &[u8]) -> Option<&[u8]> {
    match x {
        [b'/', x @ ..] => Some(x),
        _ => None
    }
}

/// [`munch_single_dot`] twice.
pub(crate) fn munch_double_dot(x: &[u8]) -> Option<&[u8]> {
    munch_single_dot(munch_single_dot(x)?)
}

/// Munch a `.`, `%2e`, or `%2E` from the start of `x`.
pub(crate) fn munch_single_dot(x: &[u8]) -> Option<&[u8]> {
    match x {
        [b'.', x @ ..] | [b'%', b'2', b'e' | b'E', x @ ..] => Some(x),
        _ => None
    }
}



/** [`path_segment_bytes_is_drive_letter`].                **/ pub fn path_segment_is_drive_letter               (value: &str) -> bool {path_segment_bytes_is_drive_letter               (value.as_bytes())}
/** [`path_segment_bytes_is_normalized_drive_letter`].     **/ pub fn path_segment_is_normalized_drive_letter    (value: &str) -> bool {path_segment_bytes_is_normalized_drive_letter    (value.as_bytes())}
/** [`path_segment_bytes_is_non_normalized_drive_letter`]. **/ pub fn path_segment_is_non_normalized_drive_letter(value: &str) -> bool {path_segment_bytes_is_non_normalized_drive_letter(value.as_bytes())}
/** [`path_segment_bytes_is_single_dot`].                  **/ pub fn path_segment_is_single_dot                 (value: &str) -> bool {path_segment_bytes_is_single_dot                 (value.as_bytes())}
/** [`path_segment_bytes_is_double_dot`].                  **/ pub fn path_segment_is_double_dot                 (value: &str) -> bool {path_segment_bytes_is_double_dot                 (value.as_bytes())}



/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter).
pub fn path_segment_bytes_is_drive_letter(value: &[u8]) -> bool {
    matches!(value, [x, b':' | b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_bytes_is_normalized_drive_letter(value: &[u8]) -> bool {
    matches!(value, [x, b':'] if x.is_ascii_alphabetic())
}

/// If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter) but not a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter).
pub fn path_segment_bytes_is_non_normalized_drive_letter(value: &[u8]) -> bool {
    matches!(value, [x, b'|'] if x.is_ascii_alphabetic())
}

/// If it's a [single-dot path segment](https://url.spec.whatwg.org/#single-dot-path-segment).
pub fn path_segment_bytes_is_single_dot(value: &[u8]) -> bool {
    matches!(value, b"." | b"%2e" | b"%2E")
}

/// If it's a [double-dot path segment](https://url.spec.whatwg.org/#double-dot-path-segment).
pub fn path_segment_bytes_is_double_dot(value: &[u8]) -> bool {
    matches!(value, b".." | b".%2e" | b".%2E" | b"%2e." | b"%2e%2e" | b"%2e%2E" | b"%2E." | b"%2E%2e" | b"%2E%2E")
}

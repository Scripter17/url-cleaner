//! Single path segments.

mod file;
mod special_not_file;
mod non_special;

pub use file::*;
pub use special_not_file::*;
pub use non_special::*;



/** [`path_segment_bytes_is_drive_letter`].                **/ pub fn path_segment_is_drive_letter               (value: &str) -> bool {path_segment_bytes_is_drive_letter               (value.as_bytes())}
/** [`path_segment_bytes_is_normalized_drive_letter`].     **/ pub fn path_segment_is_normalized_drive_letter    (value: &str) -> bool {path_segment_bytes_is_normalized_drive_letter    (value.as_bytes())}
/** [`path_segment_bytes_is_non_normalized_drive_letter`]. **/ pub fn path_segment_is_non_normalized_drive_letter(value: &str) -> bool {path_segment_bytes_is_non_normalized_drive_letter(value.as_bytes())}
/** [`path_segment_bytes_is_single_dot`].                  **/ pub fn path_segment_is_single_dot                 (value: &str) -> bool {path_segment_bytes_is_single_dot                 (value.as_bytes())}
/** [`path_segment_bytes_is_double_dot`].                  **/ pub fn path_segment_is_double_dot                 (value: &str) -> bool {path_segment_bytes_is_double_dot                 (value.as_bytes())}

/** If it's a [windows drive letter](https://url.spec.whatwg.org/#windows-drive-letter).                       **/ pub fn path_segment_bytes_is_drive_letter               (value: &[u8]) -> bool {matches!(value, [x, b':' | b'|'] if x.is_ascii_alphabetic())}
/** If it's a [normalized windows drive letter](https://url.spec.whatwg.org/#normalized-windows-drive-letter). **/ pub fn path_segment_bytes_is_normalized_drive_letter    (value: &[u8]) -> bool {matches!(value, [x, b':'       ] if x.is_ascii_alphabetic())}
/** [`path_segment_bytes_is_drive_letter`] but not [`path_segment_bytes_is_normalized_drive_letter`].          **/ pub fn path_segment_bytes_is_non_normalized_drive_letter(value: &[u8]) -> bool {matches!(value, [x,        b'|'] if x.is_ascii_alphabetic())}
/** If it's a [single-dot path segment](https://url.spec.whatwg.org/#single-dot-path-segment).                 **/ pub fn path_segment_bytes_is_single_dot                 (value: &[u8]) -> bool {matches!(value, b"." | b"%2e" | b"%2E")}
/** If it's a [double-dot path segment](https://url.spec.whatwg.org/#double-dot-path-segment).                 **/ pub fn path_segment_bytes_is_double_dot                 (value: &[u8]) -> bool {matches!(value, b".." | b".%2e" | b".%2E" | b"%2e." | b"%2e%2e" | b"%2e%2E" | b"%2E." | b"%2E%2e" | b"%2E%2E")}

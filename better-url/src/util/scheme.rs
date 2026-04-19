//! Scheme stuff.

/// If it's a valid scheme start.
pub fn is_valid_scheme_start(b: u8) -> bool {
    b.is_ascii_alphabetic()
}

/// If it's a valid scheme continue.
pub fn is_valid_scheme_continue(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'.' | b'-' | b'+')
}

/// If it's a valid scheme.
pub fn is_valid_scheme(scheme: &str) -> bool {
    let mut bytes = scheme.bytes();

    bytes.next().is_some_and(is_valid_scheme_start) && bytes.all(is_valid_scheme_continue)
}

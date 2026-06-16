//! Scheme stuff.

use crate::prelude::*;

/// If it's a valid scheme start.
pub fn is_valid_scheme_start(b: u8) -> bool {
    b.is_ascii_alphabetic()
}

/// If it's a valid scheme continue.
pub fn is_valid_scheme_continue(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'+')
}

/// If it's a valid scheme.
pub fn is_valid_scheme(scheme: &str) -> bool {
    let mut bytes = scheme.bytes();

    bytes.next().is_some_and(is_valid_scheme_start) && bytes.all(is_valid_scheme_continue)
}

/// Encode a scheme.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
pub fn encode_scheme<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidScheme> {
    let mut value = value.into();
    let mut to_lowercase = false;

    let mut bytes = value.bytes();

    match bytes.next() {
        Some(b'a'..=b'z') => {},
        Some(b'A'..=b'Z') => to_lowercase = true,
        _ => Err(InvalidScheme)?
    }

    for b in bytes {
        match b {
            b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'+'  => {},
            b'A'..=b'Z' => to_lowercase = true,
            _ => Err(InvalidScheme)?
        }
    }

    if to_lowercase {
        value.to_mut().make_ascii_lowercase();
    }

    Ok((to_lowercase, value))
}

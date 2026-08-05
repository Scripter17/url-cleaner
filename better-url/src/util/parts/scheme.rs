//! Scheme stuff.

use crate::prelude::*;

/// If it's a valid scheme input.
pub fn is_valid_scheme(value: &str) -> bool {
    let mut x = value.bytes();

    x.next().is_some_and(|b| START_DATA[b as usize] != 2) && x.all(|b| CONTINUE_DATA[b as usize] != 2)
}

/// Get [`START_DATA`].
const fn get_start_data() -> [u8; 256] {
    let mut ret = [2; 256];

    let mut i = b'a';
    while i <= b'z' {
        ret[i as usize] = 0;
        i += 1;
    }

    let mut i = b'A';
    while i <= b'Z' {
        ret[i as usize] = 1;
        i += 1;
    }

    ret
}

/// Get [`CONTINUE_DATA`].
const fn get_continue_data() -> [u8; 256] {
    let mut ret = START_DATA;

    let mut i = b'0';
    while i <= b'9' {
        ret[i as usize] = 0;
        i += 1;
    }

    ret[b'.' as usize] = 0;
    ret[b'-' as usize] = 0;
    ret[b'+' as usize] = 0;

    ret
}

/// For each byte, `0` if a valid scheme start literal, `1` if a valid scheme start input, `2` if an invalid scheem start input.
const START_DATA   : [u8; 256] = get_start_data   ();
/// For each byte, `0` if a valid scheme continue literal, `1` if a valid scheme continue input, `2` if an invalid scheem continue input.
const CONTINUE_DATA: [u8; 256] = get_continue_data();

/// Encode a scheme.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
pub fn encode_scheme<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidScheme> {
    let mut value = value.into();

    let mut bytes = value.bytes();

    let mut class = START_DATA[bytes.next().ok_or(InvalidScheme)? as usize];

    for byte in bytes {
        class |= CONTINUE_DATA[byte as usize];
    }

    Ok(match class {
        0 => (false, value),
        1 => {
            value.to_mut().make_ascii_lowercase();
            (true, value)
        },
        _ => Err(InvalidScheme)?
    })
}

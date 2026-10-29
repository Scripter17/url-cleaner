//! [`SCHEME_PATH_BYTE_CLASS_DATA`].

use crate::prelude::*;

/// A classification of each byte whose bits, from lowest to highest, are:
///
/// 1. If it's in [`PATH`].
///
/// 2. If it's `\`.
///
/// 3. If it's `%` or `.`.
///
/// 4. If it's `|`.
///
/// 5. If it can't start a scheme input (not `[a-zA-Z]`).
///
/// 6. If it can't continue a scheme input (not `[a-zA-Z0-9.+-]`).
///
/// 7. If it requires the scheme be lowercased (`[A-Z]`).
///
/// 8. Unused.
pub const SCHEME_PATH_BYTE_CLASS_DATA: [u8; 256] = get_scheme_path_byte_class_data();

/// Get [`SCHEME_PATH_BYTE_CLASS_DATA`].
const fn get_scheme_path_byte_class_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        if PATH.contains(i as u8) {ret[i as usize] |= 1;}

        match i as u8 {
            b'a'..=b'z'                      => ret[i as usize] |= 0b0000_0000,
            b'A'..=b'Z'                      => ret[i as usize] |= 0b0100_0000,
            b'.' | b'-' | b'+' | b'0'..=b'9' => ret[i as usize] |= 0b0001_0000,
            _                                => ret[i as usize] |= 0b0011_0000,
        }

        i += 1;
    }

    ret[b'\\' as usize] |= 2;
    ret[b'%'  as usize] |= 4;
    ret[b'.'  as usize] |= 4;
    ret[b'|'  as usize] |= 8;

    ret
}

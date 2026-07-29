//! Percent decode normalizing.

use crate::prelude::*;

/// Percent decode except for bytes in `set`.
pub fn percent_decode_normalize<'a, T: Into<Cow<'a, str>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    if value.memchr(b'%').is_none() {
        return (false, value);
    }

    _percent_decode_normalize_bytes(unsafe {value.to_mut().as_mut_vec()}, set);

    (true, value)
}

/// Percent decode except for bytes in `set`.
pub fn percent_decode_normalize_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();

    if value.memchr(b'%').is_none() {
        return (false, value);
    }

    _percent_decode_normalize_bytes(value.to_mut(), set);

    (true, value)
}

/// Percent decode except for bytes in `set`.
fn _percent_decode_normalize_bytes(value: &mut Vec<u8>, set: AsciiSet) {
    let mut r = 0;
    let mut w = 0;

    unsafe {
        loop {
            match *value.get_unchecked(r..) {
                [b'%', h, l, ..] if let Some(b) = decode_hex_byte(h, l) && !set.contains(b) => {
                    *value.get_unchecked_mut(w) = b;
                    w += 1;
                    r += 3;
                },
                [b, ..] => {
                    *value.get_unchecked_mut(w) = b;
                    w += 1;
                    r += 1;
                },
                [] => break,
            }
        }

        value.set_len(w);
    }
}

//! Percent encoding.

use crate::prelude::*;

/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";

/// Percent encode a string.
pub fn percent_encode<'a, T: Into<Cow<'a, str>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let mut to_reserve = 0;

    for &b in value.as_bytes() {
        if set.contains(b) {
            to_reserve += 2;
        }
    }

    if to_reserve == 0 {
        return (false, value);
    }

    (true, _percent_encode_bytes(value.as_bytes(), set, to_reserve).into())
}

/// Percent encode bytes.
pub fn percent_encode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let mut to_reserve = 0;

    for &b in &*value {
        if set.contains(b) {
            to_reserve += 2;
        }
    }

    if to_reserve == 0 {
        // SAFETY: AsciiSet::get always triggers on non-ASCII bytes, so if it found no matches then it's ASCII.
        return (false, unsafe {cow_bytes_to_str_unchecked(value)});
    }

    (true, _percent_encode_bytes(&value, set, to_reserve).into())
}

/// Percent encode bytes.
fn _percent_encode_bytes(value: &[u8], set: AsciiSet, to_reserve: usize) -> String {
    let mut ret = String::with_capacity(value.len() + to_reserve);

    let mut w = 0;

    unsafe {
        for &b in value {
            if set.contains(b) {
                *ret.as_mut_ptr().add(w    ) = b'%';
                *ret.as_mut_ptr().add(w + 1) = NIBBLES[b as usize >> 4];
                *ret.as_mut_ptr().add(w + 2) = NIBBLES[b as usize & 15];

                w += 3;
            } else {
                *ret.as_mut_ptr().add(w) = b;

                w += 1;
            }
        }

        ret.as_mut_vec().set_len(value.len() + to_reserve);
    }

    ret
}

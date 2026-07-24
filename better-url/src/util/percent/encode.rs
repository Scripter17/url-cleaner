//! Percent encoding.

use crate::prelude::*;

/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";

/// Percent encode a string.
pub fn percent_encode<'a, T: Into<Cow<'a, str>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(cow_str_to_bytes(value), set)
}

/// Percent encode a bytes.
pub fn percent_encode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let mut to_reserve = 0;

    for &b in value.iter() {
        if set.contains(b) {
            to_reserve += 2;
        }
    }

    if to_reserve == 0 {
        // AsciiSet::get always triggers on non-ASCII bytes, so if it found no matches then it's ASCII.
        return (false, unsafe {cow_bytes_to_str_unchecked(value)});
    }

    let mut ret = Vec::<u8>::with_capacity(value.len() + to_reserve);

    let mut i = value.len();
    let mut j = value.len() + to_reserve;

    unsafe {
        while i > 0 {
            i -= 1;
            j -= 1;

            match *value.get_unchecked(i) {
                b if set.contains(b) => {
                    *ret.as_mut_ptr().add(j - 2) = b'%';
                    *ret.as_mut_ptr().add(j - 1) = NIBBLES[b as usize >> 4];
                    *ret.as_mut_ptr().add(j    ) = NIBBLES[b as usize & 15];

                    j -= 2;
                },
                b => *ret.as_mut_ptr().add(j) = b
            }
        }

        ret.set_len(value.len() + to_reserve);
    }

    (true, unsafe {cow_bytes_to_str_unchecked(ret)})
}

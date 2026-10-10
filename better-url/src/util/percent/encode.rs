//! Percent encoding.

use crate::prelude::*;

/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";

/// Percent encode a string.
pub fn percent_encode<'a, T: Into<Cow<'a, str>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(cow_str_to_bytes(value), set)
}

/// Percent encode bytes.
pub fn percent_encode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.iter().filter(|&&b| set.contains(b)).count() {
        0 => (false, unsafe {cow_bytes_to_str_unchecked(value)}),
        to_encode => {
            let mut ret = String::with_capacity(value.len() + to_encode * 2);

            let mut w = 0;

            unsafe {
                for &b in &*value {
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

                ret.as_mut_vec().set_len(value.len() + to_encode * 2);
            }

            (true, ret.into())
        },
    }
}

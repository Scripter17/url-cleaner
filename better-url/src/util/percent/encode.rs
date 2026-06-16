//! Percent encoding.

use crate::prelude::*;

/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";

/// Percent encode.
pub fn percent_encode<'a, T: Into<Cow<'a, [u8]>>>(value: T, space_to_plus: bool, backslash_to_slash: bool, ignore_first_colon: bool, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    let mut has_spaces_to_plus = false;
    let mut has_backslashes_to_forward = false;
    let mut colon_to_ignore = None;
    let mut to_reserve = 0;

    for (i, &b) in value.iter().enumerate() {
        match b {
            b' '  if space_to_plus                                   => has_spaces_to_plus = true,
            b'\\' if backslash_to_slash                              => has_backslashes_to_forward = true,
            b':'  if ignore_first_colon && colon_to_ignore.is_none() => colon_to_ignore = Some(i),
            b     if set.contains(b)                                 => to_reserve += 2,
            _ => {},
        }
    }

    if to_reserve == 0 && !has_spaces_to_plus && !has_backslashes_to_forward {
        // SAFETY: `value` contains only ASCII bytes.
        return (false, unsafe {cow_bytes_to_str(value)});
    }

    let x = value.to_mut();

    x.reserve(to_reserve);

    let mut i = x.len();
    let mut j = x.len() + to_reserve;

    unsafe {
        std::ptr::write_bytes(x.as_mut_ptr().add(i), 0, to_reserve);
        x.set_len(j);
    }

    while i > 0 {
        i -= 1;
        j -= 1;

        match x[i] {
            b' '  if space_to_plus              => {x[j] = b'+';},
            b'\\' if backslash_to_slash         => {x[j] = b'/';},
            b':'  if Some(i) == colon_to_ignore => {x[j] = b':';},
            b     if set.contains(b) => {
                x[j - 2] = b'%';
                x[j - 1] = NIBBLES[b as usize >> 4];
                x[j    ] = NIBBLES[b as usize & 15];

                j -= 2;
            },
            b => x[j] = b
        }
    }

    // SAFETY: `value` contains only ASCII bytes.
    (true, unsafe {cow_bytes_to_str(value)})
}

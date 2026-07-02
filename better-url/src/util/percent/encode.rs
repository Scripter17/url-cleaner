//! Percent encoding.

use crate::prelude::*;

/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";

/// Percent encode.
pub fn percent_encode<'a, T: Into<Cow<'a, [u8]>>, const S2P: bool, const BS2FS: bool, const IFC: bool>(value: T, set: AsciiSet) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    let mut has_spaces_to_plus = false;
    let mut has_backslashes_to_forward = false;
    let mut colon_to_ignore = None;
    let mut to_reserve = 0;

    for i in 0..value.len() {
        match value[i] {
            b' '  if S2P                                => has_spaces_to_plus = true,
            b'\\' if BS2FS                              => has_backslashes_to_forward = true,
            b':'  if IFC   && colon_to_ignore.is_none() => colon_to_ignore = Some(i),
            b     if set.contains(b)                    => to_reserve += 2,
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
            b' '  if S2P                                 => {x[j] = b'+';},
            b'\\' if BS2FS                               => {x[j] = b'/';},
            b':'  if IFC   && Some(i) == colon_to_ignore => {x[j] = b':';},
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

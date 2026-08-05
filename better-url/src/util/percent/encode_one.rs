//! Percent encoding one byte.

use crate::prelude::*;

/// Percent encode just `byte`.
/// # Safety
/// `byte` must be ASCII.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(unsafe {percent_encode_one("abc", b'a')}, (true , "%61bc".into()));
/// assert_eq!(unsafe {percent_encode_one("abc", b'b')}, (true , "a%62c".into()));
/// assert_eq!(unsafe {percent_encode_one("abc", b'c')}, (true , "ab%63".into()));
/// assert_eq!(unsafe {percent_encode_one("abc", b'd')}, (false, "abc"  .into()));
/// ```
pub unsafe fn percent_encode_one<'a, T: Into<Cow<'a, str>>>(value: T, byte: u8) -> (bool, Cow<'a, str>) {
    debug_assert!(byte.is_ascii());

    let value = value.into();

    // TODO: Make faster.
    let to_reserve = value.bytes().filter(|&b| b == byte).count() * 2;

    match to_reserve {
        0 => (false, value),
        _ => (true, unsafe {cow_bytes_to_str_unchecked(_percent_encode_one(value.as_bytes(), byte, to_reserve))}),
    }
}

/// Percent encode just `byte`.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(percent_encode_one_bytes(b"abc", b'a'), (true , b"%61bc".into()));
/// assert_eq!(percent_encode_one_bytes(b"abc", b'b'), (true , b"a%62c".into()));
/// assert_eq!(percent_encode_one_bytes(b"abc", b'c'), (true , b"ab%63".into()));
/// assert_eq!(percent_encode_one_bytes(b"abc", b'd'), (false, b"abc"  .into()));
/// ```
pub fn percent_encode_one_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, byte: u8) -> (bool, Cow<'a, [u8]>) {
    let value = value.into();

    // TODO: Make faster.
    let to_reserve = value.iter().filter(|&&b| b == byte).count() * 2;

    match to_reserve {
        0 => (false, value),
        _ => (true, _percent_encode_one(&value, byte, to_reserve).into()),
    }
}

/// Percent encode just `byte`.
fn _percent_encode_one(value: &[u8], byte: u8, to_reserve: usize) -> Vec<u8> {
    let mut ret = Vec::<u8>::with_capacity(value.len() + to_reserve);

    let h = NIBBLES[byte as usize >> 4];
    let l = NIBBLES[byte as usize & 15];

    unsafe {
        let mut w = 0;

        for &b in value {
            if b == byte {
                *ret.as_mut_ptr().add(w    ) = b'%';
                *ret.as_mut_ptr().add(w + 1) = h;
                *ret.as_mut_ptr().add(w + 2) = l;
                w += 3;
            } else {
                *ret.as_mut_ptr().add(w) = b;
                w += 1;
            }
        }

        ret.set_len(value.len() + to_reserve)
    }

    ret
}

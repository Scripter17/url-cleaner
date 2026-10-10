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
    let (changed, value) = percent_encode_one_bytes(cow_str_to_bytes(value), byte);

    (changed, unsafe {cow_bytes_to_str_unchecked(value)})
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

    match value.memchr(byte) {
        None => (false, value),
        Some(i) => {
            let mut to_encode = 1;

            for &b in unsafe {value.get_unchecked(i+1..)} {
                if b == byte {
                    to_encode += 1;
                }
            }

            let mut ret = Vec::<u8>::with_capacity(value.len() + to_encode * 2);

            let h = NIBBLES[byte as usize >> 4];
            let l = NIBBLES[byte as usize & 15];

            unsafe {
                let mut w = 0;

                for &b in &*value {
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

                ret.set_len(value.len() + to_encode * 2)
            }

            (true, ret.into())
        },
    }
}

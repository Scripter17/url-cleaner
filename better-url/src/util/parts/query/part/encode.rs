//! Encoding.

use crate::prelude::*;

/// [`QUERY_PART`] without space, allowing a cleaner "check each byte" loop.
const THING: AsciiSet = QUERY_PART.remove(b' ');

/// [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoding.
pub fn encode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_query_part_bytes(cow_str_to_bytes(value))
}

/// [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoding.
pub fn encode_query_part_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let (a, value) = percent_encode_bytes(value, THING);
    let (b, value) = space_to_plus(value);

    (a || b, value)
}

/// Replace spaces with `+`.
pub fn space_to_plus<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();

    match value.memchr(b' ') {
        Some(mut i) => unsafe {
            let x = value.to_mut().as_mut_vec();

            *x.get_unchecked_mut(i) = b'+';

            while let Some(j) = x.get_unchecked(i + 1..).memchr(b' ') {
                i += j + 1;

                *x.get_unchecked_mut(i) = b'+';
            }

            (true, value)
        },
        None => (false, value)
    }
}

//! Encoding.

use crate::prelude::*;

/// [`QUERY_PART`] without space, allowing a cleaner "check each byte" loop.
const THING: AsciiSet = QUERY_PART.remove(b' ');

// TODO: Do percent and space encoding in two separate passes?

/// [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoding.
pub fn encode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_query_part_bytes(cow_str_to_bytes(value))
}

/// [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoding.
pub fn encode_query_part_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    let mut to_reserve = 0;

    for &b in value.iter() {
        if THING.contains(b) {
            to_reserve += 2;
        }
    }

    if to_reserve == 0 && value.memchr(b' ').is_none() {
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
                b' ' => *ret.as_mut_ptr().add(j) = b'+',
                b if THING.contains(b) => {
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

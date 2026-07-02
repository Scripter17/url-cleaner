//! Query parts.

use crate::prelude::*;

/// [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoding.
pub fn encode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, true, false, false>(cow_str_to_bytes(value.into()), QUERY_PART)
}

/// Try to decode a [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded string.
/// # Errors
/// If the call to [`try_cow_bytes_to_str`] returns an error, that error is returned.
pub fn try_decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), std::str::Utf8Error> {
    Ok(match decode_query_part(value) {
        (true , value) => (true , try_cow_bytes_to_str(value)?),
        (false, value) => (false, unsafe {cow_bytes_to_str(value)}),
    })
}

/// Lossily decode a [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded string.
pub fn lossy_decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    match decode_query_part(value) {
        (true , value) => (true , decode_utf8_cow_lossy(value)),
        (false, value) => (false, unsafe {cow_bytes_to_str(value)}),
    }
}

/// [`decode_query_part_bytes`] but accepting a string.
pub fn decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    decode_query_part_bytes(cow_str_to_bytes(value.into()))
}

/// Decode [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded bytes to bytes.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(decode_query_part_bytes(b"abc"     ), (false, b"abc" .into()));
/// assert_eq!(decode_query_part_bytes(b"%20"     ), (true , b" "   .into()));
/// assert_eq!(decode_query_part_bytes(b"%41=%61" ), (true , b"A=a" .into()));
/// assert_eq!(decode_query_part_bytes(b"%41=%61+"), (true , b"A=a ".into()));
/// ```
pub fn decode_query_part_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();

    // Technically `%XY` does nothing since X and Y aren't valid nibbles.
    // Though that's rare and checking if each % is succeeded by 2 valid nibbles is expensive.
    if !value.iter().any(|&b| b == b'%' || b == b'+') {
        return (false, value);
    }

    let x = value.to_mut();

    let mut i = 0;
    let mut j = 0;

    while j < x.len() {
        match x[j..] {
            [b'%', h, l, ..] if let Some(b) = decode_hex_byte(h, l) => {
                x[i] = b;
                j += 2;
            },
            [b'+'] => x[i] = b' ',
            _      => x[i] = x[j],
        }
        i += 1;
        j += 1;
    }

    let changed = i != x.len();

    x.truncate(i);

    (changed, value)
}

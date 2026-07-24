//! Decoding.

use crate::prelude::*;

// Note: The memchr2 calls are in each of the top level functions because apparently [`cow_bytes_to_str_unchecked`] is pretty expensive.

/// Try to decode a [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded string.
/// # Errors
/// If the call to [`try_cow_bytes_to_str`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn try_decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    let value = value.into();

    if value.memchr2(b'%', b'+').is_none() {
        return Ok((false, value));
    }

    Ok(match _decode_query_part(value) {
        (true , value) => (true , try_cow_bytes_to_str(value)?),
        (false, value) => (false, unsafe {cow_bytes_to_str_unchecked(value)}),
    })
}

/// Lossily decode a [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded string.
pub fn lossy_decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    if value.memchr2(b'%', b'+').is_none() {
        return (false, value);
    }

    match _decode_query_part(value) {
        (true , value) => (true , lossy_cow_bytes_to_str(value)),
        (false, value) => (false, unsafe {cow_bytes_to_str_unchecked(value)}),
    }
}

/// [`decode_query_part_bytes`] but accepting a string.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(decode_query_part("abc"     ), (false, b"abc" .into()));
/// assert_eq!(decode_query_part("a%20b"   ), (true , b"a b" .into()));
/// assert_eq!(decode_query_part("a+b"     ), (true , b"a b" .into()));
/// assert_eq!(decode_query_part("%41=%61" ), (true , b"A=a" .into()));
/// assert_eq!(decode_query_part("%41=%61+"), (true , b"A=a ".into()));
/// ```
pub fn decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let value = value.into();

    if value.memchr2(b'%', b'+').is_none() {
        return (false, cow_str_to_bytes(value));
    }

    _decode_query_part(value)
}

/// Decode [`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded) encoded bytes to bytes.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(decode_query_part_bytes(b"abc"     ), (false, b"abc" .into()));
/// assert_eq!(decode_query_part_bytes(b"a%20b"   ), (true , b"a b" .into()));
/// assert_eq!(decode_query_part_bytes(b"a+b"     ), (true , b"a b" .into()));
/// assert_eq!(decode_query_part_bytes(b"%41=%61" ), (true , b"A=a" .into()));
/// assert_eq!(decode_query_part_bytes(b"%41=%61+"), (true , b"A=a ".into()));
/// ```
pub fn decode_query_part_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let value = value.into();

    if value.memchr2(b'%', b'+').is_none() {
        return (false, value);
    }

    _decode_query_part_bytes(value)
}

/// Decode a query part str without a fast path.
fn _decode_query_part<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    _decode_query_part_bytes(cow_str_to_bytes(value))
}

/// Decode a query part bytes without a fast path.
fn _decode_query_part_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();

    let x = value.to_mut();

    let mut r = 0;
    let mut w = 0;

    unsafe {
        loop {
            match *x.get_unchecked(r..) {
                [b'%', h, l, ..] if let Some(b) = decode_hex_byte(h, l) => {
                    *x.get_unchecked_mut(w) = b;
                    r += 3;
                    w += 1
                },
                [b'+', ..] => {*x.get_unchecked_mut(w) = b' '; r += 1; w += 1;},
                [b   , ..] => {*x.get_unchecked_mut(w) = b   ; r += 1; w += 1;},
                []         => break,
            }
        }

        x.set_len(w);
    }

    (true, value)
}

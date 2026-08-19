//! Percent decoding.

use crate::prelude::*;

/// Decode a pair of ASCII hex nibbles.
pub(crate) fn decode_hex_byte(h: u8, l: u8) -> Option<u8> {
    Some(decode_hex_nibble(h)? * 16 + decode_hex_nibble(l)?)
}

/// Decode an ASCII hex nibble.
pub(crate) fn decode_hex_nibble(x: u8) -> Option<u8> {
    match x {
        b'0'..=b'9' => Some(x - b'0'),
        b'a'..=b'f' => Some(x - b'a' + 10),
        b'A'..=b'F' => Some(x - b'A' + 10),
        _ => None
    }
}

/// Try to losslessly percent decode.
/// # Errors
/// If [`try_cow_bytes_to_str`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn try_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    let value = value.into();

    if value.memchr(b'%').is_none() {
        return Ok((false, value));
    }

    Ok((true, try_cow_bytes_to_str(_percent_decode(value))?))
}

/// Lossily percent decode.
pub fn lossy_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    if value.memchr(b'%').is_none() {
        return (false, value);
    }

    (true, lossy_cow_bytes_to_str(_percent_decode(value)))
}

/// [`percent_decode_bytes`] but accepting a string.
pub fn percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = cow_str_to_bytes(value);

    if value.memchr(b'%').is_none() {
        return (false, value);
    }

    _percent_decode_bytes(value.to_mut());

    (true, value)
}

/// Percent decode bytes.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert_eq!(percent_decode_bytes(b"abc"     ), (false, b"abc" .into()));
/// assert_eq!(percent_decode_bytes(b"%20"     ), (true , b" "   .into()));
/// assert_eq!(percent_decode_bytes(b"%41=%61" ), (true , b"A=a" .into()));
/// assert_eq!(percent_decode_bytes(b"%41=%61+"), (true , b"A=a+".into()));
/// ```
pub fn percent_decode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();

    if value.memchr(b'%').is_none() {
        return (false, value);
    }

    _percent_decode_bytes(value.to_mut());

    (true, value)
}

/// [`percent_decode`] without a fast path for not containing a `%`.
fn _percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> Cow<'a, [u8]> {
    let mut value = cow_str_to_bytes(value);

    _percent_decode_bytes(value.to_mut());

    value
}

/// [`percent_decode_bytes`] without a fast path for not containing a `%`.
fn _percent_decode_bytes(value: &mut Vec<u8>) {
    let mut i = 0;
    let mut j = 0;

    unsafe {
        loop {
            match *value.get_unchecked(j..) {
                [b'%', h, l, ..] if let Some(b) = decode_hex_byte(h, l) => {
                    *value.get_unchecked_mut(i) = b;
                    i += 1;
                    j += 3;
                },
                [b, ..] => {
                    *value.get_unchecked_mut(i) = b;
                    i += 1;
                    j += 1;
                },
                [] => break,
            }
        }

        value.set_len(i);
    }
}

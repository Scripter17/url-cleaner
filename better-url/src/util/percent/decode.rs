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
/// If the call to [`try_cow_bytes_to_str`] returns an error, that error is returned.
pub fn try_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), std::str::Utf8Error> {
    Ok(match percent_decode(value) {
        (true , value) => (true , try_cow_bytes_to_str(value)?),
        (false, value) => (false, unsafe {cow_bytes_to_str(value)}),
    })
}

/// Lossily percent decode.
pub fn lossy_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    match percent_decode(value) {
        (true , value) => (true , decode_utf8_cow_lossy(value)),
        (false, value) => (false, unsafe {cow_bytes_to_str(value)}),
    }
}

/// [`percent_decode_bytes`] but accepting a string.
pub fn percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    percent_decode_bytes(cow_str_to_bytes(value.into()))
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

    if !value.contains(&b'%') {
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
            _ => x[i] = x[j]
        }
        i += 1;
        j += 1;
    }

    x.truncate(i);

    (true, value)
}

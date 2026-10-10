//! Percent decoding.

use crate::prelude::*;



/// Try to losslessly percent decode.
/// # Errors
/// If [`try_cow_bytes_to_str`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
#[inline]
pub fn try_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    let value = value.into();

    Ok(match value.memchr(b'%') {
        Some(r) => (true , try_cow_bytes_to_str(_percent_decode(value, r))?),
        None    => (false, value),
    })
}

/// Try to losslessly percent decode bytes.
/// # Errors
/// If [`try_cow_bytes_to_str`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
#[inline]
pub fn try_percent_decode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), (std::str::Utf8Error, Cow<'a, [u8]>)> {
    let value = value.into();

    Ok(match value.memchr(b'%') {
        Some(r) => (true , try_cow_bytes_to_str(_percent_decode_bytes(value, r))?),
        None    => (false, try_cow_bytes_to_str(value)?),
    })
}



/// Lossily percent decode.
#[inline]
pub fn lossy_percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.memchr(b'%') {
        Some(r) => (true , lossy_cow_bytes_to_str(_percent_decode(value, r))),
        None    => (false, value),
    }
}

/// Lossily percent decode bytes.
#[inline]
pub fn lossy_percent_decode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    let value = value.into();

    match value.memchr(b'%') {
        Some(r) => (true , lossy_cow_bytes_to_str(_percent_decode_bytes(value, r))),
        None    => (false, lossy_cow_bytes_to_str(value)),
    }
}



/// [`percent_decode_bytes`] but accepting a string.
#[inline]
pub fn percent_decode<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    percent_decode_bytes(cow_str_to_bytes(value))
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
#[inline]
pub fn percent_decode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let value = value.into();

    match value.memchr(b'%') {
        Some(r) => (true , _percent_decode_bytes(value, r)),
        None    => (false, value)
    }
}



/// [`percent_decode`] without a fast path for not containing a `%`.
fn _percent_decode<'a, T: Into<Cow<'a, str>>>(value: T, r: usize) -> Cow<'a, [u8]> {
    _percent_decode_bytes(cow_str_to_bytes(value), r)
}

/// [`percent_decode`] without a fast path for not containing a `%`.
fn _percent_decode_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T, mut r: usize) -> Cow<'a, [u8]> {
    let mut value = value.into();
    let mut w = r;

    let x = value.to_mut();

    unsafe {
        loop {
            match *x.get_unchecked(r..) {
                [b'%', h, l, ..] if let Some(b) = decode_hex_byte(h, l) => {
                    *x.get_unchecked_mut(w) = b;
                    r += 3;
                    w += 1;
                },
                [b, ..] => {
                    *x.get_unchecked_mut(w) = b;
                    r += 1;
                    w += 1;
                },
                [] => break,
            }
        }

        x.set_len(w);
    }

    value
}

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

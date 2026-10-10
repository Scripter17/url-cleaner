//! Scheme stuff.

use crate::prelude::*;

/// Get [`DATA`].
const fn get_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        match i as u8 {
            b'a'..=b'z'                      => ret[i as usize] = 0b000,
            b'A'..=b'Z'                      => ret[i as usize] = 0b100,
            b'.' | b'-' | b'+' | b'0'..=b'9' => ret[i as usize] = 0b001,
            _                                => ret[i as usize] = 0b011,
        }
        i += 1;
    }

    ret
}

/// A set of bitfields where the bits are, from lowest to highest:
///
/// - If the byte can't start a scheme.
///
/// - If the byte can't continue a scheme.
///
/// - If the byte requires the scheme be lowercased.
const DATA: [u8; 256] = get_data();

/// If it's a valid scheme input.
pub fn is_valid_scheme(value: &str) -> bool {
    let mut bytes = value.bytes();

    bytes.next().is_some_and(|b| DATA[b as usize] & 0b101 == 0b000)
        && bytes.fold(0, |acc, b| acc | DATA[b as usize]) & 0b110 == 0b000
}

/// Encode a scheme.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
#[inline]
pub fn encode_scheme<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidScheme> {
    encode_scheme_bytes(cow_str_to_bytes(value))
}

/// Encode a scheme from bytes.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
#[inline]
pub fn encode_scheme_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidScheme> {
    let value = value.into();

    if matches!(&*value, b"http" | b"https" | b"ws" | b"wss" | b"file" | b"ftp") {
        return Ok((false, unsafe {cow_bytes_to_str_unchecked(value)}));
    }

    #[cold]
    fn outline<'a>(mut value: Cow<'a, [u8]>) -> Result<(bool, Cow<'a, str>), InvalidScheme> {
        let mut bytes = value.iter().copied();

        let class1 = DATA[bytes.next().ok_or(InvalidScheme)? as usize];
        let class2 = bytes.fold(0, |acc, b| acc | DATA[b as usize]);

        Ok(match (class1 & 0b101, class2 & 0b110) {
            (0b000, 0b000) => (false, unsafe {cow_bytes_to_str_unchecked(value)}),
            (0b000, 0b100) | (0b100, 0b000 | 0b100) => {
                value.to_mut().make_ascii_lowercase();
                (true, unsafe {cow_bytes_to_str_unchecked(value)})
            },
            _ => Err(InvalidScheme)?
        })
    }

    outline(value)
}

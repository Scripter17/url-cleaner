//! Scheme stuff.

use crate::prelude::*;

/// If it's a valid scheme literal.
pub fn is_valid_scheme_literal(value: &str) -> bool {
    match value.as_bytes() {
        [] => false,
        [first, rest @ ..] => (SCHEME_PATH_BYTE_CLASS_DATA[*first as usize] & 0b0101_0000 == 0b0000_0000) && (rest.iter().fold(0, |acc, &b| acc | SCHEME_PATH_BYTE_CLASS_DATA[b as usize]) & 0b0110_0000 == 0b0000_0000)
    }
}

/// Make a scheme.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
#[inline]
pub fn make_scheme<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, SchemeDetails), InvalidScheme> {
    make_scheme_bytes(cow_str_to_bytes(value))
}

/// Make a scheme from bytes.
/// # Errors
/// If the scheme is invalid, returns the error [`InvalidScheme`].
#[inline]
pub fn make_scheme_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, SchemeDetails), InvalidScheme> {
    let value = value.into();

    Ok(match &*value {
        b"http"  => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::HTTP ),
        b"https" => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::HTTPS),
        b"ws"    => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::WS   ),
        b"wss"   => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::WSS  ),
        b"file"  => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::FILE ),
        b"ftp"   => (false, unsafe {cow_bytes_to_str_unchecked(value)}, SchemeDetails::FTP  ),
        _ => {
            #[cold]
            fn outline<'a>(value: Cow<'a, [u8]>) -> Result<(bool, Cow<'a, str>, SchemeDetails), InvalidScheme> {
                match &*value {
                    [] => Err(InvalidScheme),
                    [first, rest @ ..] => {
                        let class1 =                                     SCHEME_PATH_BYTE_CLASS_DATA[*first as usize] ;
                        let class2 = rest.iter().fold(0, |acc, &b| acc | SCHEME_PATH_BYTE_CLASS_DATA[ b     as usize]);

                        Ok(match (class1 & 0b0101_0000, class2 & 0b0110_0000) {
                            (0, 0) => {
                                let value = unsafe {cow_bytes_to_str_unchecked(value)};

                                let details = SchemeDetails::new_unchecked(&value);

                                (false, value, details)
                            }
                            (0 | 64, 0 | 64) => {
                                let mut value = unsafe {cow_bytes_to_str_unchecked(value)};

                                value.to_mut().make_ascii_lowercase();

                                let details = SchemeDetails::new_unchecked(&value);

                                (true, value, details)
                            },
                            _ => Err(InvalidScheme)?
                        })
                    }
                }
            }

            outline(value)?
        }
    })
}

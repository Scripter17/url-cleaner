//! [`get_js_string`] and co.

use crate::prelude::*;

/// Given a [`str`] that starts with a javascript string literal, return the value of that string.
///
/// TODO: Handle template strings.
/// # Errors
/// If a syntax error happens, returns the error [`SyntaxError`].
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(get_js_string("\"abc\\n\\u000Adef\"other stuff"                                ).unwrap(), "abc\n\ndef"          );
/// assert_eq!(get_js_string("\"1\\u{a}2\\u{0a}3\\u{00a}4\\u{000a}5\\u{0000a}6\\u000a7\\\n8\"").unwrap(), "1\n2\n3\n4\n5\n6\n78");
/// assert_eq!(get_js_string("\"'\\\"\"outside"                                               ).unwrap(), "'\""                 );
/// assert_eq!(get_js_string("'\"\\''outside"                                                 ).unwrap(), "\"'"                 );
/// assert_eq!(get_js_string("'a\\na'"                                                        ).unwrap(), "a\na"                );
/// assert_eq!(get_js_string("'a\\\na'"                                                       ).unwrap(), "aa"                  );
/// ```
pub fn get_js_string<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<Cow<'a, str>, GetJsStringError> {
    let value = value.into();

    match value.as_bytes().split_first() {
        Some((&q @ (b'"' | b'\''), mut rest)) => {
            // TODO: Reject unescaped line break literals.

            let mut ret = Vec::<u8>::new();

            while let Some(i) = rest.memchrn([b'\\', q]) {
                match rest[i] {
                    b'\\' => {
                        let before = unsafe {rest.get_unchecked(..i)};
                        let after  = unsafe {rest.get_unchecked(i+1..)};

                        ret.extend_from_slice(before);

                        let (c, x) = munch_escape(after).ok_or(SyntaxError)?;

                        if let Some(c) = c {
                            ret.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes());
                        }

                        rest = x;
                    },
                    _ => {
                        ret.extend_from_slice(unsafe {rest.get_unchecked(..i)});

                        return Ok(unsafe {String::from_utf8_unchecked(ret)}.into());
                    }
                }
            }

            Err(SyntaxError)?
        },
        _ => Err(SyntaxError)?
    }
}

/// Munch an escape sequence's value.
fn munch_escape(x: &[u8]) -> Option<(Option<char>, &[u8])> {
    Some(match *x {
        [b'0'         , ref x @ ..] => (Some('\0'  ), x),
        [b'\''        , ref x @ ..] => (Some('\''  ), x),
        [b'"'         , ref x @ ..] => (Some('"'   ), x),
        [b'\\'        , ref x @ ..] => (Some('\\'  ), x),
        [b'n'         , ref x @ ..] => (Some('\n'  ), x),
        [b'r'         , ref x @ ..] => (Some('\r'  ), x),
        [b'v'         , ref x @ ..] => (Some('\x0B'), x),
        [b't'         , ref x @ ..] => (Some('\t'  ), x),
        [b'b'         , ref x @ ..] => (Some('\x08'), x),
        [b'f'         , ref x @ ..] => (Some('\x0C'), x),

        [b'\r'        , ref x @ ..] => (None, x),
        [b'\n'        , ref x @ ..] => (None, x),
        [226, 128, 168, ref x @ ..] => (None, x),
        [226, 128, 169, ref x @ ..] => (None, x),

        [b'x', h, l, ref x @ ..] => (Some(thing(&[h, l])?), x),

        [b'u', b'{', ref x @ ..] => {
            let i = x.memchr(b'}')?;

            let a = unsafe {x.get_unchecked(      .. i)};
            let b = unsafe {x.get_unchecked(i + 1 ..  )};

            (Some(thing(a)?), b)
        },

        // TODO: Surrogate pairs, somehow

        [b'u', a, b, c, d, ref x @ ..] => (Some(thing(&[a, b, c, d])?), x),

        _ => (None, x)
    })
}

/// Decode a sequence of hex nibbles.
pub(crate) fn thing(x: &[u8]) -> Option<char> {
    let mut ret = 0u32;

    for &n in x {
        ret *= 16;
        ret = ret.checked_add(decode_hex_nibble(n)? as u32)?;
    }

    ret.try_into().ok()
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

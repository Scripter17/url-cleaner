//! Query stuff.

use std::borrow::Cow;

use percent_encoding::percent_decode_str;

/// Decode a query parameter name/value.
pub fn decode_query_part(input: &str) -> Cow<'_, str> {
    let mut plus    = false;
    let mut percent = false;
    for b in input.bytes() {
        if b == b'+' {plus    = true;}
        if b == b'%' {percent = true;}
    }
    match (plus, percent) {
        (false, false) => Cow::Borrowed(input),
        (false, true ) => percent_decode_str(input).decode_utf8_lossy(),
        (true , false) => Cow::Owned(input.replace("+", " ")),
        (true , true ) => Cow::Owned(percent_decode_str(&input.replace("+", " ")).decode_utf8_lossy().into_owned())
    }
}

/// A lazy iterator over the bytes of a decoded query part.
pub fn iter_decode_query_part(input: &str) -> IterDecodeQueryPart<'_> {
    IterDecodeQueryPart(input.bytes())
}

/// A lazy iterator over the bytes of a decoded query part.
#[derive(Debug, Clone)]
pub struct IterDecodeQueryPart<'a>(std::str::Bytes<'a>);

impl Iterator for IterDecodeQueryPart<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.next()? {
            b'+' => b' ',
            b'%' => after_percent_sign(self).unwrap_or(b'%'),
            x => x
        })
    }
}

/// Helper function stolen from [`percent_encoding`].
fn after_percent_sign(iter: &mut IterDecodeQueryPart<'_>) -> Option<u8> {
    let mut cloned_iter = iter.clone();
    let h = char::from(cloned_iter.next()?).to_digit(16)?;
    let l = char::from(cloned_iter.next()?).to_digit(16)?;
    *iter = cloned_iter;
    Some(h as u8 * 0x10 + l as u8)
}


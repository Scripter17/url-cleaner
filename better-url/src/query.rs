//! Query stuff.

use std::borrow::Cow;
use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use percent_encoding::percent_decode_str;

/// A query string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterQuery<'a>(pub &'a str);

impl<'a> BetterQuery<'a> {
    /// An iterator over each unparsed segment.
    pub fn raw_segments(&self) -> impl Iterator<Item = &'a str> {
        self.0.split('&')
    }

    /// An iterator over each unparsed key/value pair.
    pub fn raw_pairs(&self) -> impl Iterator<Item = (&'a str, Option<&'a str>)> {
        self.raw_segments().map(|kev| match kev.split_once('=') {
            Some((k, v)) => (k  , Some(v)),
            None         => (kev, None   )
        })
    }

    /// An iterator over each unparsed name.
    pub fn raw_names(&self) -> impl Iterator<Item = &'a str> {
        self.raw_pairs().map(|(k, _)| k)
    }

    /// An iterator over each unparsed value.
    pub fn raw_values(&self) -> impl Iterator<Item = Option<&'a str>> {
        self.raw_pairs().map(|(_, v)| v)
    }

    /// An iterator over each key/value pair.
    pub fn pairs(&self) -> impl Iterator<Item = (Cow<'a, str>, Option<Cow<'a, str>>)> {
        self.raw_pairs().map(|(k, v)| (decode_query_part(k), v.map(decode_query_part)))
    }

    /// An iterator over each name.
    pub fn names(&self) -> impl Iterator<Item = Cow<'a, str>> {
        self.raw_names().map(decode_query_part)
    }

    /// An iterator over each value.
    pub fn values(&self) -> impl Iterator<Item = Option<Cow<'a, str>>> {
        self.raw_values().map(|x| x.map(decode_query_part))
    }

    /// An iterator over each lazily decodable key/value pair
    pub fn lazy_pairs(&self) -> impl Iterator<Item = (QueryPartDecodedBytes<'a>, Option<QueryPartDecodedBytes<'a>>)> {
        self.raw_pairs().map(|(k, v)| (query_part_decoded_bytes(k), v.map(query_part_decoded_bytes)))
    }

    /// An iterator over each lazily decodable name.
    pub fn lazy_names(&self) -> impl Iterator<Item = QueryPartDecodedBytes<'a>> {
        self.lazy_pairs().map(|(k, _)| k)
    }

    /// An iterator over each lazily decodable value.
    pub fn lazy_values(&self) -> impl Iterator<Item = Option<QueryPartDecodedBytes<'a>>> {
        self.lazy_pairs().map(|(_, v)| v)
    }
}

impl AsRef<str> for BetterQuery<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Deref for BetterQuery<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

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
pub fn query_part_decoded_bytes(input: &str) -> QueryPartDecodedBytes<'_> {
    QueryPartDecodedBytes(input.bytes())
}

/// A lazy iterator over the bytes of a decoded query part.
#[derive(Debug, Clone)]
pub struct QueryPartDecodedBytes<'a>(pub std::str::Bytes<'a>);

impl Iterator for QueryPartDecodedBytes<'_> {
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
fn after_percent_sign(iter: &mut QueryPartDecodedBytes<'_>) -> Option<u8> {
    let mut cloned_iter = iter.clone();
    let h = char::from(cloned_iter.next()?).to_digit(16)?;
    let l = char::from(cloned_iter.next()?).to_digit(16)?;
    *iter = cloned_iter;
    Some(h as u8 * 0x10 + l as u8)
}


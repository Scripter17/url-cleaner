//! [`QueryPartDecoder`].

use std::borrow::Cow;

use crate::prelude::*;

/// An [`Iterator`] yielding bytes from decoding [`Self::0`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryPartDecoder<'a>(pub &'a [u8]);

impl<'a> QueryPartDecoder<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(part: T) -> Self {
        part.into()
    }

    /// Decode to UTF8.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(QueryPartDecoder::new("abc%E2%80%AEdef").decode(), "abc\u{202e}def");
    /// assert_eq!(QueryPartDecoder::new("abc%F0%90%80def").decode(), "abc\u{FFFD}def");
    /// ```
    pub fn decode(self) -> Cow<'a, str> {
        match self.decode_bytes() {
            Cow::Owned(bytes) => match String::from_utf8_lossy(&bytes) {
                Cow::Owned(s) => Cow::Owned(s),
                Cow::Borrowed(_) => Cow::Owned(unsafe {String::from_utf8_unchecked(bytes)})
            },
            Cow::Borrowed(bytes) => String::from_utf8_lossy(bytes)
        }
    }

    /// Decode to bytes.
    pub fn decode_bytes(self) -> Cow<'a, [u8]> {
        if self.0.contains(&b'+') || self.0.contains(&b'%') {
            Cow::Owned(self.collect())
        } else {
            Cow::Borrowed(self.0)
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> From<&'a T> for QueryPartDecoder<'a> {
    fn from(value: &'a T) -> Self {
        Self(value.as_ref())
    }
}

impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for QueryPartDecoder<'_> {
    fn eq(&self, other: &T) -> bool {
        if self.0.contains(&b'+') || self.0.contains(&b'%') {
            Iterator::eq(*self, other.as_ref().iter().copied())
        } else {
            self.0 == other.as_ref()
        }
    }
}

impl<'a> Iterator for QueryPartDecoder<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let (&b, rest) = self.0.split_first()?;

        self.0 = rest;

        Some(match b {
            b'+' => b' ',
            b'%' => after_percent_sign(self).unwrap_or(b'%'),
            b => b
        })
    }
}

//! [`PathSegmentDecoder`].

use std::borrow::Cow;

use crate::prelude::*;

/// An [`Iterator`] yielding bytes from decoding [`Self::0`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathSegmentDecoder<'a>(pub &'a [u8]);

impl<'a> PathSegmentDecoder<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(segment: T) -> Self {
        segment.into()
    }

    /// Decode to UTF-8.
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
        if self.0.contains(&b'%') {
            Cow::Owned(self.collect())
        } else {
            Cow::Borrowed(self.0)
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> From<&'a T> for PathSegmentDecoder<'a> {
    fn from(value: &'a T) -> Self {
        Self(value.as_ref())
    }
}

impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for PathSegmentDecoder<'_> {
    fn eq(&self, other: &T) -> bool {
        Iterator::eq(*self, other.as_ref().iter().copied())
    }
}

impl<'a> Iterator for PathSegmentDecoder<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let (&b, rest) = self.0.split_first()?;

        self.0 = rest;

        Some(match b {
            b'%' => after_percent_sign(self).unwrap_or(b'%'),
            b => b
        })
    }
}

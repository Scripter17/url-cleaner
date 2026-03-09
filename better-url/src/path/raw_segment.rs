//! [`RawPathSegment`].

use std::borrow::Cow;

use crate::prelude::*;

/// A raw path segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawPathSegment<'a>(pub &'a str);

impl<'a> RawPathSegment<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(segment: T) -> Self {
        segment.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Make a [`PathSegmentDecoder`].
    pub fn decoder(self) -> PathSegmentDecoder<'a> {
        PathSegmentDecoder::new(self.0)
    }

    /// Decode to UTF-8.
    pub fn decode(self) -> Cow<'a, str> {
        self.decoder().decode()
    }

    /// Decode to bytes.
    pub fn decode_bytes(self) -> Cow<'a, [u8]> {
        self.decoder().decode_bytes()
    }
}

impl<'a> From<&'a str> for RawPathSegment<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl PartialEq<&str> for RawPathSegment<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

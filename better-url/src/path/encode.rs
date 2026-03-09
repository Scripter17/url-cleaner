//! [`PathSegmentEncoder`].

use crate::prelude::*;

/// An [`Iterator`] yielding [`str`] slices from encoding [`Self::0`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathSegmentEncoder<'a>(pub &'a [u8]);

impl<'a> PathSegmentEncoder<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(segment: T) -> Self {
        segment.into()
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> From<&'a T> for PathSegmentEncoder<'a> {
    fn from(value: &'a T) -> Self {
        Self(value.as_ref())
    }
}

impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for PathSegmentEncoder<'_> {
    fn eq(&self, other: &T) -> bool {
        self.flat_map(str::bytes).eq(other.as_ref().iter().copied())
    }
}

/// Returns [`true`] if `b` should be encoded.
fn changed(b: u8) -> bool {
    PercentEncodeSet::PathSegment.matches(b)
}

/// Encodes `b`.
fn change(b: u8) -> &'static str {
    percent_encode(b)
}

impl<'a> Iterator for PathSegmentEncoder<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.split_first()? {
            (&b, rest) if changed(b) => {self.0 = rest; change(b)},
            _ => {
                let split = self.0.iter().copied().position(changed).unwrap_or(self.0.len());
                let (ret, rest) = self.0.split_at(split);
                self.0 = rest;
                unsafe {str::from_utf8_unchecked(ret)}
            }
        })
    }
}

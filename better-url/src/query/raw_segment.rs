//! [`RawQuerySegment`].

use std::borrow::Cow;

use crate::prelude::*;

/// A raw query segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawQuerySegment<'a>(pub &'a str);

impl<'a> RawQuerySegment<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(segment: T) -> Self {
        segment.into()
    }

    /// Get [`Self::0`].
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Get the raw name and value.
    pub fn raw_pair(self) -> (&'a str, Option<&'a str>) {
        match self.0.split_once('=') {
            Some((name, value)) => (name, Some(value)),
            None => (self.0, None)
        }
    }

    /// Get the raw name.
    pub fn raw_name(self) -> &'a str {
        self.raw_pair().0
    }

    /// Get the raw value.
    pub fn raw_value(self) -> Option<&'a str> {
        self.raw_pair().1
    }

    /// Get [`QueryPartDecoder`]s for the name and value.
    pub fn lazy_pair(self) -> (QueryPartDecoder<'a>, Option<QueryPartDecoder<'a>>) {
        let (name, value) = self.raw_pair();
        (QueryPartDecoder::new(name), value.map(QueryPartDecoder::new))
    }

    /// Get a [`QueryPartDecoder`] for the name.
    pub fn lazy_name(self) -> QueryPartDecoder<'a> {
        self.lazy_pair().0
    }

    /// Get a [`QueryPartDecoder`] for the value.
    pub fn lazy_value(self) -> Option<QueryPartDecoder<'a>> {
        self.lazy_pair().1
    }

    /// Decode the name and value.
    pub fn pair(self) -> (Cow<'a, str>, Option<Cow<'a, str>>) {
        let (name, value) = self.lazy_pair();
        (name.decode(), value.map(QueryPartDecoder::decode))
    }

    /// Decode the name.
    pub fn name(self) -> Cow<'a, str> {
        self.lazy_name().decode()
    }

    /// Decode the value.
    pub fn value(self) -> Option<Cow<'a, str>> {
        self.lazy_value().map(QueryPartDecoder::decode)
    }
}

impl<'a> From<&'a str> for RawQuerySegment<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

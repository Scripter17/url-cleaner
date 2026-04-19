//! [`SpecialQuerySegment`].

use crate::prelude::*;

mod name;
mod value;

/// A special query segment.
#[derive(Debug, Clone)]
pub struct SpecialQuerySegment<'a> {
    /// The raw segment.
    pub(crate) raw: Cow<'a, str>,
    /// If non-zero, the start of the value.
    pub(crate) vs: usize,
}

impl<'a> SpecialQuerySegment<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(segment: T) -> Self {
        let raw = segment.into();

        Self {
            vs: raw.find("=").map_or(0, |x| x + 1),
            raw
        }
    }
    
    /// Make a new [`Self`] from a pair.
    pub fn from_pair<T: Into<Cow<'a, str>>>(name: T, value: Option<&str>) -> Self {
        let mut ret = name.into();

        if let Some(value) = value {
            ret.to_mut().extend(["=", value]);
        }

        ret = PartTranscoder::QueryPart.encode(ret);

        match ret.find("%3D") {
            Some(x) => {
                let vs = x + 1;
                ret.replace_range(x..=x+2, "=");
                Self {raw: ret, vs}
            },
            None => Self {raw: ret, vs: 0}
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Turn into an owned [`SpecialQuerySegment`].
    pub fn into_owned(self) -> SpecialQuerySegment<'static> {
        SpecialQuerySegment {
            raw: self.raw.into_owned().into(),
            vs: self.vs
        }
    }

    /// Turn into the inner [`Self`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.raw
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialQuerySegment<'_> {
        SpecialQuerySegment {
            raw: Cow::Borrowed(&self.raw),
            vs: self.vs
        }
    }
}

impl<'a> From<Cow<'a, str>> for SpecialQuerySegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let value = PartTranscoder::SpecialQuery.encode(value);

        Self {
            vs: value.split_once('=').map_or(0, |(x, _)| x.len() + 1),
            raw: value
        }
    }
}

impl<'a> From<QuerySegment<'a>> for SpecialQuerySegment<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x,
            QuerySegment::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<NonSpecialQuerySegment<'a>> for SpecialQuerySegment<'a> {
    fn from(value: NonSpecialQuerySegment<'a>) -> Self {
        let raw = specialize_query(value.into_inner());
        Self {
            vs: raw.find('=').map_or(0, |x| x + 1),
            raw,
        }
    }
}

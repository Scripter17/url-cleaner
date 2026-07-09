//! [`FragmentQuerySegment`].

use crate::prelude::*;

mod name;
mod value;

/// A special query segment.
#[derive(Debug, Clone)]
pub struct FragmentQuerySegment<'a> {
    /// The raw segment.
    pub(crate) raw: Cow<'a, str>,
    /// If non-zero, the start of the value.
    pub(crate) vs: Option<NonZero<usize>>,
}

impl<'a> FragmentQuerySegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(segment: T) -> Self {
        let raw = segment.into();

        Self {
            vs: raw.bytes().position(|b| b == b'=').and_then(|x| NonZero::new(x + 1)),
            raw
        }
    }

    /// Make a new [`Self`] from a pair.
    pub fn from_pair<'b, T: Into<Cow<'a, str>>, U: Into<Cow<'b, str>>>(name: T, value: Option<U>) -> Self {
        let (_, mut raw) = encode_query_part(name);

        match value {
            Some(value) => {
                let vs = raw.len() + 1;
                raw.to_mut().extend(["=", &encode_query_part(value).1]);
                Self {raw, vs: NonZero::new(vs)}
            },
            None => Self {raw, vs: None}
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FragmentQuerySegment<'_> {
        FragmentQuerySegment {
            raw: Cow::Borrowed(&self.raw),
            vs: self.vs
        }
    }

    /// Turn into an owned [`FragmentQuerySegment`].
    pub fn into_owned(self) -> FragmentQuerySegment<'static> {
        FragmentQuerySegment {
            raw: self.raw.into_owned().into(),
            vs: self.vs
        }
    }

    /// Turn into the inner [`Self`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.raw
    }
}



impl<'a> From<Cow<'a, str>> for FragmentQuerySegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, raw, vs) = encode_fragment_query_segment(value);

        Self {raw, vs}
    }
}

impl<'a> From<QueryLikeSegment<'a>> for FragmentQuerySegment<'a> {
    fn from(value: QueryLikeSegment<'a>) -> Self {
        match value {
            QueryLikeSegment::Query   (x) => x.into(),
            QueryLikeSegment::Fragment(x) => x,
        }
    }
}

impl<'a> From<QuerySegment<'a>> for FragmentQuerySegment<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment<'a>> for FragmentQuerySegment<'a> {
    fn from(value: SpecialQuerySegment<'a>) -> Self {
        let old_vs = value.vs;

        match special_query_to_fragment(value.into_inner()) {
            (true , raw) => Self {vs: raw.bytes().position(|b| b == b'=').and_then(|x| NonZero::new(x + 1)), raw},
            (false, raw) => Self {vs: old_vs, raw}
        }
    }
}

impl<'a> From<NonSpecialQuerySegment<'a>> for FragmentQuerySegment<'a> {
    fn from(value: NonSpecialQuerySegment<'a>) -> Self {
        let old_vs = value.vs;

        match non_special_query_to_fragment(value.into_inner()) {
            (true , raw) => Self {vs: raw.bytes().position(|b| b == b'=').and_then(|x| NonZero::new(x + 1)), raw},
            (false, raw) => Self {vs: old_vs, raw}
        }
    }
}

impl<'a, 'b, T: Into<Cow<'a, str>>, U: Into<Cow<'b, str>>> From<(T, Option<U>)> for FragmentQuerySegment<'a> {fn from(value: (T, Option<U>)) -> Self {Self::from_pair(value.0, value.1)}}

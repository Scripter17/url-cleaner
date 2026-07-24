//! [`NonSpecialQuerySegment`].

use crate::prelude::*;

mod name;
mod value;

/// A non-special query segment.
#[derive(Debug, Clone)]
pub struct NonSpecialQuerySegment<'a> {
    /// The raw segment.
    pub(crate) raw: Cow<'a, str>,
    /// If non-zero, the start of the value.
    pub(crate) vs: Option<NonZero<usize>>,
}

impl<'a> NonSpecialQuerySegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(segment: T) -> Self {
        let raw = segment.into();

        Self {
            vs: raw.memchr(b'=').and_then(|x| NonZero::new(x + 1)),
            raw
        }
    }

    /// Make a new [`Self`] from a pair.
    pub fn from_pair<'b, T: Into<Cow<'a, str>>, U: Into<Cow<'b, str>>>(name: T, value: Option<U>) -> Self {
        let (_, mut raw) = encode_query_part(name);

        match value {
            Some(value) => {
                let vs = raw.len() + 1;
                raw.extend(["=", &encode_query_part(value).1]);
                Self {raw, vs: NonZero::new(vs)}
            },
            None => Self {raw, vs: None}
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialQuerySegment<'_> {
        NonSpecialQuerySegment {
            raw: Cow::Borrowed(&self.raw),
            vs: self.vs
        }
    }

    /// Turn into an owned [`NonSpecialQuerySegment`].
    pub fn into_owned(self) -> NonSpecialQuerySegment<'static> {
        NonSpecialQuerySegment {
            raw: self.raw.into_owned().into(),
            vs: self.vs
        }
    }

    /// Turn into the inner [`Self`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.raw
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialQuerySegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, raw, vs) = encode_non_special_query_segment(value);

        Self {raw, vs}
    }
}

impl<'a> From<QueryLikeSegment<'a>> for NonSpecialQuerySegment<'a> {
    fn from(value: QueryLikeSegment<'a>) -> Self {
        match value {
            QueryLikeSegment::Query   (x) => x.into(),
            QueryLikeSegment::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<QuerySegment<'a>> for NonSpecialQuerySegment<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x,
        }
    }
}

impl<'a> From<SpecialQuerySegment<'a>> for NonSpecialQuerySegment<'a> {
    fn from(value: SpecialQuerySegment<'a>) -> Self {
        Self {
            raw: value.raw,
            vs: value.vs,
        }
    }
}

impl<'a> From<FragmentQuerySegment<'a>> for NonSpecialQuerySegment<'a> {
    fn from(value: FragmentQuerySegment<'a>) -> Self {
        let old_vs = value.vs;

        match fragment_to_non_special_query(value.into_inner()) {
            (true , raw) => Self {vs: raw.memchr(b'=').and_then(|x| NonZero::new(x + 1)), raw},
            (false, raw) => Self {vs: old_vs, raw}
        }
    }
}

impl<'a, 'b, T: Into<Cow<'a, str>>, U: Into<Cow<'b, str>>> From<(T, Option<U>)> for NonSpecialQuerySegment<'a> {fn from(value: (T, Option<U>)) -> Self {Self::from_pair(value.0, value.1)}}

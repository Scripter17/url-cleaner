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
    pub(crate) value_start: Option<NonZero<usize>>,
}

impl<'a> SpecialQuerySegment<'a> {
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
            value_start: raw.memchr(b'=').and_then(|x| NonZero::new(x + 1)),
            raw
        }
    }

    /// Make a new [`Self`] from a pair.
    pub fn from_pair<'b, T: Into<SpecialQueryName<'a>>, U: Into<MaybeSpecialQueryValue<'b>>>(name: T, value: U) -> Self {
        let mut raw = name.into().into_inner();

        match value.into().as_str() {
            Some(value) => {
                let value_start = raw.len() + 1;
                raw.extend(["=", value]);
                Self {raw, value_start: NonZero::new(value_start)}
            },
            None => Self {raw, value_start: None}
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialQuerySegment<'_> {
        SpecialQuerySegment {
            raw: Cow::Borrowed(&self.raw),
            value_start: self.value_start
        }
    }

    /// Turn into an owned [`SpecialQuerySegment`].
    pub fn into_owned(self) -> SpecialQuerySegment<'static> {
        SpecialQuerySegment {
            raw: self.raw.into_owned().into(),
            value_start: self.value_start
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.raw
    }
}



impl<'a> From<Cow<'a, str>> for SpecialQuerySegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, raw, value_start) = encode_special_query_segment(value);

        Self {raw, value_start}
    }
}

impl<'a> From<QueryLikeSegment<'a>> for SpecialQuerySegment<'a> {
    fn from(value: QueryLikeSegment<'a>) -> Self {
        match value {
            QueryLikeSegment::Query   (x) => x.into(),
            QueryLikeSegment::Fragment(x) => x.into(),
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
        let old_vs = value.value_start;

        match non_special_query_to_special_query(value.into_inner()) {
            (true , raw) => Self {value_start: raw.memchr(b'=').and_then(|x| NonZero::new(x + 1)), raw},
            (false, raw) => Self {value_start: old_vs, raw}
        }
    }
}

impl<'a> From<FragmentQuerySegment<'a>> for SpecialQuerySegment<'a> {
    fn from(value: FragmentQuerySegment<'a>) -> Self {
        let old_vs = value.value_start;

        match fragment_to_special_query(value.into_inner()) {
            (true , raw) => Self {value_start: raw.memchr(b'=').and_then(|x| NonZero::new(x + 1)), raw},
            (false, raw) => Self {value_start: old_vs, raw}
        }
    }
}

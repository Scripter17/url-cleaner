//! [`FragmentQueryValue`].

use crate::prelude::*;

/// A name of a [`FragmentQuerySegment`].
///
/// [`Self::new`] uses [`encode_query_part`] becasue using just the special query percent encode set would result in `+` and `%` not being percent encoded, which would make [`Self::decode`] not the inverse of [`Self::new`].
///
/// However, the only invariant is that this is a substring of a [`FragmentQuery`] that contains no `&` literals.
#[derive(Debug, Clone)]
pub struct FragmentQueryValue<'a>(Cow<'a, str>);

impl<'a> FragmentQueryValue<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// [`decode_query_part`].
    pub fn decode(self) -> Cow<'a, [u8]> {
        let (_, value) = decode_query_part(self.0);

        value
    }

    /// [`try_decode_query_part`].
    /// # Errors
    /// If [`decode_query_part`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
        let (_, value) = try_decode_query_part(self.0)?;

        Ok(value)
    }

    /// [`lossy_decode_query_part`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        let (_, value) = lossy_decode_query_part(self.0);

        value
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FragmentQueryValue<'_> {
        FragmentQueryValue(self.as_str().into())
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FragmentQueryValue<'static> {
        FragmentQueryValue(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, [u8]>> for FragmentQueryValue<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        let (_, value) = encode_query_part_bytes(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<QueryLikeValue<'a>> for FragmentQueryValue<'a> {
    fn from(value: QueryLikeValue<'a>) -> Self {
        match value {
            QueryLikeValue::Query   (x) => x.into(),
            QueryLikeValue::Fragment(x) => x,
        }
    }
}

impl<'a> From<QueryValue<'a>> for FragmentQueryValue<'a> {
    fn from(value: QueryValue<'a>) -> Self {
        match value {
            QueryValue::Special   (x) => x.into(),
            QueryValue::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQueryValue   <'a>> for FragmentQueryValue<'a> {fn from(value: SpecialQueryValue   <'a>) -> Self {unsafe {Self::new_unchecked(special_query_to_fragment    (value.into_inner()).1)}}}
impl<'a> From<NonSpecialQueryValue<'a>> for FragmentQueryValue<'a> {fn from(value: NonSpecialQueryValue<'a>) -> Self {unsafe {Self::new_unchecked(non_special_query_to_fragment(value.into_inner()).1)}}}

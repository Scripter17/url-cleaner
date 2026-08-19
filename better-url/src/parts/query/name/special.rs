//! [`SpecialQueryName`].

use crate::prelude::*;

/// A name of a [`SpecialQuerySegment`].
///
/// [`Self::new`] uses [`encode_query_part`] becasue using just the special query percent encode set would result in `+` and `%` not being percent encoded, which would make [`Self::decode`] not the inverse of [`Self::new`].
///
/// However, the only invariant is that this is a substring of a [`SpecialQuery`] that contains no `=` or `&` literals.
#[derive(Debug, Clone)]
pub struct SpecialQueryName<'a>(Cow<'a, str>);

impl<'a> SpecialQueryName<'a> {
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
    pub fn borrowed(&self) -> SpecialQueryName<'_> {
        SpecialQueryName(self.as_str().into())
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialQueryName<'static> {
        SpecialQueryName(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for SpecialQueryName<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_query_part(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<QueryLikeName<'a>> for SpecialQueryName<'a> {
    fn from(value: QueryLikeName<'a>) -> Self {
        match value {
            QueryLikeName::Query   (x) => x.into(),
            QueryLikeName::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<QueryName<'a>> for SpecialQueryName<'a> {
    fn from(value: QueryName<'a>) -> Self {
        match value {
            QueryName::Special   (x) => x,
            QueryName::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<FragmentQueryName  <'a>> for SpecialQueryName<'a> {fn from(value: FragmentQueryName  <'a>) -> Self {unsafe {Self::new_unchecked(fragment_to_non_special_query     (value.into_inner()).1)}}}
impl<'a> From<NonSpecialQueryName<'a>> for SpecialQueryName<'a> {fn from(value: NonSpecialQueryName<'a>) -> Self {unsafe {Self::new_unchecked(non_special_query_to_special_query(value.into_inner()).1)}}}

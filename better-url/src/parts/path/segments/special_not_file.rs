//! [`SpecialNotFilePathSegments`].

use crate::prelude::*;

/// Special path segments.
#[derive(Debug, Clone)]
pub struct SpecialNotFilePathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFilePathSegments<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `details` must be its details.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// The [`SpecialNotFilePathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = SpecialNotFilePathSegment<'_>> {
        SplitSlashes(Some(self.as_str())).map(|x| SpecialNotFilePathSegment(x.into()))
    }

    /// The `index`th [`SpecialNotFilePathSegment`].
    pub fn get(&self, index: isize) -> Option<SpecialNotFilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialNotFilePathSegments<'static> {
        SpecialNotFilePathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFilePathSegments<'_> {
        SpecialNotFilePathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> TryFrom<PathSegments<'a>> for SpecialNotFilePathSegments<'a> {
    type Error = PathSegments<'a>;

    fn try_from(value: PathSegments<'a>) -> Result<Self, Self::Error> {
        match value {
            PathSegments::SpecialNotFile(x) => Ok(x),
            x => Err(x)
        }
    }
}

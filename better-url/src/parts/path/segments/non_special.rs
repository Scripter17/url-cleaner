//! [`NonSpecialPathSegments`].

use crate::prelude::*;

/// Non-special path segments.
#[derive(Debug, Clone)]
pub struct NonSpecialPathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> NonSpecialPathSegments<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// The [`NonSpecialPathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = NonSpecialPathSegment<'_>> {
        SplitSlashes(Some(self.as_str())).map(|x| NonSpecialPathSegment(x.into()))
    }

    /// The `index`th [`NonSpecialPathSegment`].
    pub fn get(&self, index: isize) -> Option<NonSpecialPathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialPathSegments<'static> {
        NonSpecialPathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialPathSegments<'_> {
        NonSpecialPathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> TryFrom<PathSegments<'a>> for NonSpecialPathSegments<'a> {
    type Error = PathSegments<'a>;

    fn try_from(value: PathSegments<'a>) -> Result<Self, Self::Error> {
        match value {
            PathSegments::NonSpecial(x) => Ok(x),
            x => Err(x)
        }
    }
}

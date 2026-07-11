//! [`FilePathSegments`].

use crate::prelude::*;

/// File path segments.
#[derive(Debug, Clone)]
pub struct FilePathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> FilePathSegments<'a> {
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



    /// The [`FilePathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = FilePathSegment<'_>> {
        SplitSlashes(Some(self.as_str())).map(|x| FilePathSegment(x.into()))
    }

    /// The `index`th [`FilePathSegment`].
    pub fn get(&self, index: isize) -> Option<FilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FilePathSegments<'static> {
        FilePathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FilePathSegments<'_> {
        FilePathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> TryFrom<PathSegments<'a>> for FilePathSegments<'a> {
    type Error = PathSegments<'a>;

    fn try_from(value: PathSegments<'a>) -> Result<Self, Self::Error> {
        match value {
            PathSegments::File(x) => Ok(x),
            x => Err(x)
        }
    }
}

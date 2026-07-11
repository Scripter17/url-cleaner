//! [`NonSpecialSegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A non-special segmented path.
#[derive(Debug, Clone)]
pub struct NonSpecialSegmentedPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> NonSpecialSegmentedPath<'a> {
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



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialSegmentedPath<'static> {
        NonSpecialSegmentedPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialSegmentedPath<'_> {
        NonSpecialSegmentedPath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialSegmentedPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_non_special_segmented_path(value).1)
    }
}



impl<'a> From<Path<'a>> for NonSpecialSegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x.into(),
            Path::SpecialNotFile(x) => x.into(),
            Path::NonSpecial    (x) => x.into(),
            Path::Opaque        (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for NonSpecialSegmentedPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x,
        }
    }
}

impl<'a> From<FileSegmentedPath          <'a>> for NonSpecialSegmentedPath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self(                                          value.into_inner()   )}}
impl<'a> From<SpecialNotFileSegmentedPath<'a>> for NonSpecialSegmentedPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self(                                          value.into_inner()   )}}
impl<'a> From<NonSpecialEmptyPath        <'a>> for NonSpecialSegmentedPath<'a> {fn from(_    : NonSpecialEmptyPath        <'a>) -> Self {unsafe {Self::new_unchecked("/")}}}
impl<'a> From<OpaquePath                 <'a>> for NonSpecialSegmentedPath<'a> {fn from(value: OpaquePath                 <'a>) -> Self {Self(opaque_path_to_non_special_segmented_path(value.into_inner()).1)}}

impl<'a> From<NonSpecialPath<'a>> for NonSpecialSegmentedPath<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x,
            NonSpecialPath::Empty    (x) => x.into(),
        }
    }
}



impl<'b, T: Into<NonSpecialPathSegment<'b>>> Extend<T> for NonSpecialSegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        extend_path_segments(self.0.to_mut(), false, iter.into_iter().map(Into::into));
    }
}

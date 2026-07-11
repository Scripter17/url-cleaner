//! [`SpecialNotFileSegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// Symmetry with [`NonSpecialPath`].
pub type SpecialNotFilePath<'a> = SpecialNotFileSegmentedPath<'a>;

/// A special but not file segmented path.
#[derive(Debug, Clone)]
pub struct SpecialNotFileSegmentedPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFileSegmentedPath<'a> {
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
    pub fn into_owned(self) -> SpecialNotFileSegmentedPath<'static> {
        SpecialNotFileSegmentedPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFileSegmentedPath<'_> {
        SpecialNotFileSegmentedPath(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<Cow<'a, str>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_special_not_file_path(value).1)
    }
}



impl<'a> From<Path<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x.into(),
            Path::SpecialNotFile(x) => x,
            Path::NonSpecial    (x) => x.into(),
            Path::Opaque        (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::SpecialNotFile(x) => x,
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<FileSegmentedPath      <'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: FileSegmentedPath      <'a>) -> Self {Self(                                                              value.into_inner()   )}}
impl<'a> From<NonSpecialSegmentedPath<'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: NonSpecialSegmentedPath<'a>) -> Self {Self(non_special_segmented_path_to_special_not_file_segmented_path(value.into_inner()).1)}}
impl<'a> From<OpaquePath             <'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: OpaquePath             <'a>) -> Self {Self(opaque_path_to_special_not_file_segmented_path               (value.into_inner()).1)}}

impl<'a> From<NonSpecialEmptyPath<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(_: NonSpecialEmptyPath<'a>) -> Self {
        unsafe {
            Self::new_unchecked("/")
        }
    }
}

impl<'a> From<NonSpecialPath<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x.into(),
            NonSpecialPath::Empty    (x) => x.into(),
        }
    }
}



impl<'b, T: Into<SpecialNotFilePathSegment<'b>>> Extend<T> for SpecialNotFileSegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        extend_path_segments(self.0.to_mut(), false, iter.into_iter().map(Into::into));
    }
}

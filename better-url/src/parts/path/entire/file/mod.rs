//! [`FileSegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// Symmetry with [`NonSpecialPath`].
pub type FilePath<'a> = FileSegmentedPath<'a>;

/// A file segmented path.
#[derive(Debug, Clone)]
pub struct FileSegmentedPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> FileSegmentedPath<'a> {
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
    pub fn into_owned(self) -> FileSegmentedPath<'static> {
        FileSegmentedPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FileSegmentedPath<'_> {
        FileSegmentedPath(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<Cow<'a, str>> for FileSegmentedPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_file_path(value).1)
    }
}



impl<'a> From<Path<'a>> for FileSegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x,
            Path::SpecialNotFile(x) => x.into(),
            Path::NonSpecial    (x) => x.into(),
            Path::Opaque        (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for FileSegmentedPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x,
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFileSegmentedPath<'a>> for FileSegmentedPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self(special_not_file_segmented_path_to_file_segmented_path(value.into_inner()).1)}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for FileSegmentedPath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self(non_special_segmented_path_to_file_segmented_path     (value.into_inner()).1)}}
impl<'a> From<NonSpecialEmptyPath        <'a>> for FileSegmentedPath<'a> {fn from(_    : NonSpecialEmptyPath        <'a>) -> Self {Self("/".into())}}
impl<'a> From<OpaquePath                 <'a>> for FileSegmentedPath<'a> {fn from(value: OpaquePath                 <'a>) -> Self {Self(opaque_path_to_file_segmented_path                    (value.into_inner()).1)}}

impl<'a> From<NonSpecialPath<'a>> for FileSegmentedPath<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x.into(),
            NonSpecialPath::Empty    (x) => x.into(),
        }
    }
}



impl<'b, T: Into<FilePathSegment<'b>>> Extend<T> for FileSegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        extend_path_segments(self.0.to_mut(), true, iter.into_iter().map(Into::into));
    }
}

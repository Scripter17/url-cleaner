//! [`FilePath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A [`SchemeType::File`] path.
#[derive(Debug, Clone)]
pub struct FilePath<'a>(pub(crate) Cow<'a, str>);

impl<'a> FilePath<'a> {
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

    /// The [`FilePathSegments`].
    pub fn segments(&self) -> FilePathSegments<'_> {
        unsafe {FilePathSegments::new_unchecked(self.as_str().get_unchecked(1..))}
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FilePath<'static> {
        FilePath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FilePath<'_> {
        FilePath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for FilePath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = make_file_path(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<Path<'a>> for FilePath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for FilePath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x,
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}



impl<'a> From<SpecialNotFilePath<'a>> for FilePath<'a> {fn from(value: SpecialNotFilePath<'a>) -> Self {Self(special_not_file_path_to_file_path(value.into_inner()).1)}}
impl<'a> From<NonSpecialPath    <'a>> for FilePath<'a> {fn from(value: NonSpecialPath    <'a>) -> Self {Self(non_special_path_to_file_path     (value.into_inner()).1)}}
impl<'a> From<OpaquePath        <'a>> for FilePath<'a> {fn from(value: OpaquePath        <'a>) -> Self {Self(opaque_path_to_file_path          (value.into_inner()).1)}}

impl From<PathSegments              <'_>> for FilePath<'static> {fn from(value: PathSegments              <'_>) -> Self {Self(resolve_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<FilePathSegments          <'_>> for FilePath<'static> {fn from(value: FilePathSegments          <'_>) -> Self {Self(resolve_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<SpecialNotFilePathSegments<'_>> for FilePath<'static> {fn from(value: SpecialNotFilePathSegments<'_>) -> Self {Self(resolve_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<NonSpecialPathSegments    <'_>> for FilePath<'static> {fn from(value: NonSpecialPathSegments    <'_>) -> Self {Self(resolve_file_path(forward_slashes(value.into_inner().with_insert_str(0, "/")).1).1)}}

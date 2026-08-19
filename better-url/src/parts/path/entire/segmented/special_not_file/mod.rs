//! [`SpecialNotFilePath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A [`SchemeType::SpecialNotFile`] path.
#[derive(Debug, Clone)]
pub struct SpecialNotFilePath<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFilePath<'a> {
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

    /// The [`SpecialNotFilePathSegments`].
    pub fn segments(&self) -> SpecialNotFilePathSegments<'_> {
        unsafe {SpecialNotFilePathSegments::new_unchecked(self.as_str().get_unchecked(1..))}
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialNotFilePath<'static> {
        SpecialNotFilePath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFilePath<'_> {
        SpecialNotFilePath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for SpecialNotFilePath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = make_special_not_file_path(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}



impl<'a> From<Path<'a>> for SpecialNotFilePath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for SpecialNotFilePath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::SpecialNotFile(x) => x,
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}



impl<'a> From<FilePath      <'a>> for SpecialNotFilePath<'a> {fn from(value: FilePath      <'a>) -> Self {Self(                                          value.into_inner()   )}}
impl<'a> From<NonSpecialPath<'a>> for SpecialNotFilePath<'a> {fn from(value: NonSpecialPath<'a>) -> Self {Self(non_special_path_to_special_not_file_path(value.into_inner()).1)}}
impl<'a> From<OpaquePath    <'a>> for SpecialNotFilePath<'a> {fn from(value: OpaquePath    <'a>) -> Self {Self(opaque_path_to_special_not_file_path     (value.into_inner()).1)}}

impl From<PathSegments              <'_>> for SpecialNotFilePath<'static> {fn from(value: PathSegments              <'_>) -> Self {Self(resolve_special_not_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<FilePathSegments          <'_>> for SpecialNotFilePath<'static> {fn from(value: FilePathSegments          <'_>) -> Self {Self(resolve_special_not_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<SpecialNotFilePathSegments<'_>> for SpecialNotFilePath<'static> {fn from(value: SpecialNotFilePathSegments<'_>) -> Self {Self(resolve_special_not_file_path(                value.into_inner().with_insert_str(0, "/")   ).1)}}
impl From<NonSpecialPathSegments    <'_>> for SpecialNotFilePath<'static> {fn from(value: NonSpecialPathSegments    <'_>) -> Self {Self(resolve_special_not_file_path(forward_slashes(value.into_inner().with_insert_str(0, "/")).1).1)}}

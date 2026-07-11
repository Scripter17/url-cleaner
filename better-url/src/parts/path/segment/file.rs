//! [`FilePathSegment`].

use crate::prelude::*;

/// A special path segment.
#[derive(Debug, Clone)]
pub struct FilePathSegment<'a>(pub(crate) Cow<'a, str>);

impl<'a> FilePathSegment<'a> {
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

    /// The decoded value.
    pub fn decode(self) -> Cow<'a, str> {
        lossy_percent_decode(self.0).1
    }



    /// [`path_segment_is_windows_drive_letter`].
    pub fn is_windows_drive_letter(&self) -> bool {
        path_segment_is_windows_drive_letter(self.as_str())
    }

    /// [`path_segment_is_normalized_windows_drive_letter`].
    pub fn is_normalized_windows_drive_letter(&self) -> bool {
        path_segment_is_normalized_windows_drive_letter(self.as_str())
    }

    /// [`path_segment_is_non_normalized_windows_drive_letter`].
    pub fn is_non_normalized_windows_drive_letter(&self) -> bool {
        path_segment_is_non_normalized_windows_drive_letter(self.as_str())
    }

    /// [`path_segment_is_dot`].
    pub fn is_dot(&self) -> bool {
        path_segment_is_dot(self.as_str())
    }

    /// [`path_segment_is_double_dot`].
    pub fn is_double_dot(&self) -> bool {
        path_segment_is_double_dot(self.as_str())
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FilePathSegment<'static> {
        FilePathSegment(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FilePathSegment<'_> {
        FilePathSegment(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for FilePathSegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_special_path_segment(value).1)
    }
}



impl<'a> From<PathSegment<'a>> for FilePathSegment<'a> {
    fn from(value: PathSegment<'a>) -> Self {
        match value {
            PathSegment::SpecialNotFile(x) => x.into(),
            PathSegment::File          (x) => x,
            PathSegment::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFilePathSegment<'a>> for FilePathSegment<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self(special_not_file_path_segment_to_file_path_segment(value.into_inner()).1)}}
impl<'a> From<NonSpecialPathSegment    <'a>> for FilePathSegment<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self(non_special_path_segment_to_file_path_segment     (value.into_inner()).1)}}

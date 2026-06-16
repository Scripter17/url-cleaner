//! [`SpecialNotFilePathSegment`].

use crate::prelude::*;

/// A special path segment.
#[derive(Debug, Clone)]
pub struct SpecialNotFilePathSegment<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFilePathSegment<'a> {
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
    pub fn into_owned(self) -> SpecialNotFilePathSegment<'static> {
        SpecialNotFilePathSegment(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFilePathSegment<'_> {
        SpecialNotFilePathSegment(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for SpecialNotFilePathSegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_special_path_segment(value).1)
    }
}



impl<'a> From<PathSegment<'a>> for SpecialNotFilePathSegment<'a> {
    fn from(value: PathSegment<'a>) -> Self {
        match value {
            PathSegment::SpecialNotFile(x) => x,
            PathSegment::File          (x) => x.into(),
            PathSegment::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<FilePathSegment      <'a>> for SpecialNotFilePathSegment<'a> {fn from(value: FilePathSegment      <'a>) -> Self {Self(                                                          value.into_inner()   )}}
impl<'a> From<NonSpecialPathSegment<'a>> for SpecialNotFilePathSegment<'a> {fn from(value: NonSpecialPathSegment<'a>) -> Self {Self(non_special_path_segment_to_special_not_file_path_segment(value.into_inner()).1)}}

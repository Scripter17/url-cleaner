//! [`FilePathSegment`].

use crate::prelude::*;

/// A special path segment.
#[derive(Debug, Clone)]
pub struct FilePathSegment<'a>(pub(crate) Cow<'a, str>);

impl<'a> FilePathSegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The decoded value.
    pub fn decode(self) -> Cow<'a, str> {
        PartTranscoder::SpecialPathSegment.decode_lossy(self.0)
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
        Self(PartTranscoder::SpecialPathSegment.encode(value))
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

impl<'a> From<SpecialNotFilePathSegment<'a>> for FilePathSegment<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self(path_segment_snf_2_f(value.into_inner()))}}
impl<'a> From<NonSpecialPathSegment    <'a>> for FilePathSegment<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self(path_segment_ns_2_f (value.into_inner()))}}

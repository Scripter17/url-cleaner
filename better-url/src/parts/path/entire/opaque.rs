//! [`OpaquePath`].

use crate::prelude::*;

/// An opaque path.
#[derive(Debug, Clone)]
pub struct OpaquePath<'a>(pub(crate) Cow<'a, str>);

impl<'a> OpaquePath<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }




    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> OpaquePath<'static> {
        OpaquePath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> OpaquePath<'_> {
        OpaquePath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for OpaquePath<'a> {
    fn from(mut value: Cow<'a, str>) -> Self {
        if value.starts_with("/") {
            value.to_mut().replace_range(0..=0, "%2F");
        }

        Self(PartTranscoder::NonSpecialPath.encode(value))
    }
}



impl<'a> From<Path<'a>> for OpaquePath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x,
        }
    }
}

impl<'a> From<SegmentedPath              <'a>> for OpaquePath<'a> {fn from(value: SegmentedPath              <'a>) -> Self {let mut value = value.into_inner(); value.to_mut().replace_range(0..=0, "%27"); Self(value)}}
impl<'a> From<SpecialNotFileSegmentedPath<'a>> for OpaquePath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {let mut value = value.into_inner(); value.to_mut().replace_range(0..=0, "%27"); Self(value)}}
impl<'a> From<FileSegmentedPath          <'a>> for OpaquePath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {let mut value = value.into_inner(); value.to_mut().replace_range(0..=0, "%27"); Self(value)}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for OpaquePath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {let mut value = value.into_inner(); value.to_mut().replace_range(0..=0, "%27"); Self(value)}}

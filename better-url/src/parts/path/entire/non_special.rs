//! [`NonSpecialPath`].

use crate::prelude::*;

/// Either a [`NonSpecialSegmentedPath`] or a [`Self::Empty`].
#[derive(Debug, Clone)]
pub enum NonSpecialPath<'a> {
    /// [`NonSpecialSegmentedPath`].
    Segmented(NonSpecialSegmentedPath<'a>),
    /// The empty string.
    Empty
}

impl<'a> NonSpecialPath<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Segmented(x) => x.as_str(),
            Self::Empty        => ""
        }
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Segmented(x) => x.into_inner(),
            Self::Empty        => "".into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialPath<'_> {
        match self {
            Self::Segmented(x) => x.borrowed().into(),
            Self::Empty        => NonSpecialPath::Empty,
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialPath<'static> {
        match self {
            Self::Segmented(x) => x.into_owned().into(),
            Self::Empty        => NonSpecialPath::Empty,
        }
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value.is_empty() {
            true  => Self::Empty,
            false => Self::Segmented(value.into()),
        }
    }
}



impl<'a> From<Path<'a>> for NonSpecialPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for NonSpecialPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFileSegmentedPath<'a>> for NonSpecialPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<FileSegmentedPath          <'a>> for NonSpecialPath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for NonSpecialPath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self::Segmented(value)}}

impl<'a> From<OpaquePath<'a>> for NonSpecialPath<'a> {
    fn from(value: OpaquePath<'a>) -> Self {
        match value.is_empty() {
            true  => Self::Empty,
            false => Self::Segmented(value.into())
        }
    }
}

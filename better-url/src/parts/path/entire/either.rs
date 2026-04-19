//! [`Path`].

use crate::prelude::*;

/// Either a [`SegmentedPath`] or am [`OpaquePath`].
#[derive(Debug, Clone)]
pub enum Path<'a> {
    /// [`SegmentedPath`].
    Segmented(SegmentedPath<'a>),
    /// [`OpaquePath`].
    Opaque(OpaquePath<'a>),
}

impl<'a> Path<'a> {
    /// Make from a new [`SpecialNotFilePath`].
    pub fn new_special_not_file<T: Into<SpecialNotFilePath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// Make from a new [`FilePath`].
    pub fn new_file<T: Into<FilePath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// Make from a new [`NonSpecialPath`].
    pub fn new_non_special<T: Into<NonSpecialPath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// Make from a new [`SegmentedPath`].
    pub fn new_segmented<T: Into<SegmentedPath<'a>>>(path: T) -> Self {
        Self::Segmented(path.into())
    }

    /// make from a new [`OpaquePath`].
    pub fn new_opaque<T: Into<OpaquePath<'a>>>(path: T) -> Self {
        Self::Opaque(path.into())
    }



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Segmented(x) => x.as_str(),
            Self::Opaque   (x) => x.as_str(),
        }
    }



    /// If it's [`Self::Segmented`].
    pub fn is_segmented(&self) -> bool {
        matches!(self, Self::Segmented(_))
    }

    /// If it's [`Self::Opaque`].
    pub fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }



    /// The [`SegmentedPath`].
    pub fn segmented(self) -> Option<SegmentedPath<'a>> {
        match self {
            Self::Segmented(x) => Some(x),
            Self::Opaque   (_) => None,
        }
    }

    /// The [`OpaquePath`].
    pub fn opaque(self) -> Option<OpaquePath<'a>> {
        match self {
            Self::Segmented(_) => None,
            Self::Opaque   (x) => Some(x)
        }
    }



    /// Turn into a [`SegmentedPath`].
    pub fn into_segmented(self) -> SegmentedPath<'a> {
        self.into()
    }

    /// Turn into a [`OpaquePath`].
    pub fn into_opaque(self) -> OpaquePath<'a> {
        self.into()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Segmented(x) => x.into_inner(),
            Self::Opaque   (x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Path<'static> {
        match self {
            Self::Segmented(x) => x.into_owned().into(),
            Self::Opaque   (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Path<'_> {
        match self {
            Self::Segmented(x) => x.borrowed().into(),
            Self::Opaque   (x) => x.borrowed().into(),
        }
    }
}

impl<'a> From<Cow<'a, str>> for Path<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value.starts_with("/") {
            true  => Self::new_segmented(value),
            false => Self::new_opaque   (value),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for Path<'a> {fn from(value: SegmentedPath<'a>) -> Self {Self::Segmented(value)}}
impl<'a> From<OpaquePath   <'a>> for Path<'a> {fn from(value: OpaquePath   <'a>) -> Self {Self::Opaque   (value)}}

impl<'a> From<SpecialNotFileSegmentedPath<'a>> for Path<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<FileSegmentedPath          <'a>> for Path<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for Path<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self::Segmented(value.into())}}

impl<'a> From<NonSpecialPath<'a>> for Path<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x.into(),
            NonSpecialPath::Empty        => OpaquePath("".into()).into(),
        }
    }
}

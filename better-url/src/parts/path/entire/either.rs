//! [`Path`].

use crate::prelude::*;

/// Either a [`FilePath`], [`SpecialNotFilePath`], [`NonSpecialPath`], or [`OpaquePath`].
#[derive(Debug, Clone)]
pub enum Path<'a> {
    /// [`FilePath`].
    File(FilePath<'a>),
    /// [`SpecialNotFilePath`].
    SpecialNotFile(SpecialNotFilePath<'a>),
    /// [`NonSpecialPath`].
    NonSpecial(NonSpecialPath<'a>),
    /// [`OpaquePath`].
    Opaque(OpaquePath<'a>),
}

impl<'a> Path<'a> {
    /// [`FilePath`].
    pub fn new_file<T: Into<FilePath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// [`SpecialNotFilePath`].
    pub fn new_special_not_file<T: Into<SpecialNotFilePath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// [`NonSpecialPath`].
    pub fn new_non_special<T: Into<NonSpecialPath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// [`NonSpecialSegmentedPath`].
    pub fn new_non_special_segmented<T: Into<NonSpecialSegmentedPath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// [`NonSpecialEmptyPath`].
    pub fn new_non_special_empty<T: Into<NonSpecialEmptyPath<'a>>>(path: T) -> Self {
        path.into().into()
    }

    /// [`OpaquePath`].
    pub fn new_opaque<T: Into<OpaquePath<'a>>>(path: T) -> Self {
        path.into().into()
    }



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::File          (x) => x.as_str(),
            Self::SpecialNotFile(x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str(),
            Self::Opaque        (x) => x.as_str(),
        }
    }



    /// If it's [`Self::File`].
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// If it's [`Self::SpecialNotFile`].
    pub fn is_special_not_file(&self) -> bool {
        matches!(self, Self::SpecialNotFile(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }

    /// If it's [`NonSpecialPath::Segmented`].
    pub fn is_non_special_segmented(&self) -> bool {
        matches!(self, Self::NonSpecial(NonSpecialPath::Segmented(_)))
    }

    /// If it's [`NonSpecialPath::Empty`]
    pub fn is_non_special_empty(&self) -> bool {
        matches!(self, Self::NonSpecial(NonSpecialPath::Empty(_)))
    }

    /// If it's [`Self::Opaque`].
    pub fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }



    /// The [`SegmentedPath`].
    pub fn segmented(self) -> Option<SegmentedPath<'a>> {
        match self {
            Self::File          (x) => Some(x.into()),
            Self::SpecialNotFile(x) => Some(x.into()),
            Self::NonSpecial    (x) => x.segmented().map(Into::into),
            Self::Opaque        (_) => None
        }
    }

    /// The [`FilePath`].
    pub fn file(self) -> Option<FilePath<'a>> {
        match self {
            Self::File          (x) => Some(x),
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (_) => None,
            Self::Opaque        (_) => None,
        }
    }

    /// The [`SpecialNotFilePath`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x),
            Self::NonSpecial    (_) => None,
            Self::Opaque        (_) => None,
        }
    }

    /// The [`NonSpecialPath`].
    pub fn non_special(self) -> Option<NonSpecialPath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (x) => Some(x),
            Self::Opaque        (_) => None,
        }
    }

    /// The [`NonSpecialSegmentedPath`].
    pub fn non_special_segmented(self) -> Option<NonSpecialSegmentedPath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (x) => x.segmented(),
            Self::Opaque        (_) => None,
        }
    }

    /// The [`NonSpecialEmptyPath`].
    pub fn non_special_empty(self) -> Option<NonSpecialEmptyPath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (x) => x.empty(),
            Self::Opaque        (_) => None,
        }
    }

    /// The [`OpaquePath`].
    pub fn opaque(self) -> Option<OpaquePath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (_) => None,
            Self::Opaque        (x) => Some(x),
        }
    }



    /// Turn into a [`OpaquePath`].
    pub fn into_opaque(self) -> OpaquePath<'a> {
        match self {
            Self::File          (x) => x.into(),
            Self::SpecialNotFile(x) => x.into(),
            Self::NonSpecial    (x) => x.into(),
            Self::Opaque        (x) => x,
        }
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::File          (x) => x.into_inner(),
            Self::SpecialNotFile(x) => x.into_inner(),
            Self::NonSpecial    (x) => x.into_inner(),
            Self::Opaque        (x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Path<'static> {
        match self {
            Self::File          (x) => x.into_owned().into(),
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
            Self::Opaque        (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Path<'_> {
        match self {
            Self::File          (x) => x.borrowed().into() ,
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
            Self::Opaque        (x) => x.borrowed().into(),
        }
    }
}

impl<'a> From<FilePath               <'a>> for Path<'a> {fn from(value: FilePath               <'a>) -> Self {Self::File          (value       )}}
impl<'a> From<SpecialNotFilePath     <'a>> for Path<'a> {fn from(value: SpecialNotFilePath     <'a>) -> Self {Self::SpecialNotFile(value       )}}
impl<'a> From<NonSpecialPath         <'a>> for Path<'a> {fn from(value: NonSpecialPath         <'a>) -> Self {Self::NonSpecial    (value       )}}
impl<'a> From<NonSpecialSegmentedPath<'a>> for Path<'a> {fn from(value: NonSpecialSegmentedPath<'a>) -> Self {Self::NonSpecial    (value.into())}}
impl<'a> From<NonSpecialEmptyPath    <'a>> for Path<'a> {fn from(value: NonSpecialEmptyPath    <'a>) -> Self {Self::NonSpecial    (value.into())}}
impl<'a> From<OpaquePath             <'a>> for Path<'a> {fn from(value: OpaquePath             <'a>) -> Self {Self::Opaque        (value       )}}

impl<'a> From<SegmentedPath<'a>> for Path<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

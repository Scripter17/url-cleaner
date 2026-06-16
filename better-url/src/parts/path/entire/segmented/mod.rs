//! [`SegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// Either [`SpecialNotFileSegmentedPath`], [`FileSegmentedPath`], or [`NonSpecialSegmentedPath`].
///
/// Constructors prefer [`Self::SpecialNotFile`].
#[derive(Debug, Clone)]
pub enum SegmentedPath<'a> {
    /// [`SpecialNotFileSegmentedPath`].
    SpecialNotFile(SpecialNotFileSegmentedPath<'a>),
    /// [`FileSegmentedPath`].
    File(FileSegmentedPath<'a>),
    /// [`NonSpecialSegmentedPath`].
    NonSpecial(NonSpecialSegmentedPath<'a>),
}

impl<'a> SegmentedPath<'a> {
    /// Make from a new [`SpecialNotFileSegmentedPath`].
    pub fn new_special_not_file<T: Into<SpecialNotFileSegmentedPath<'a>>>(value: T) -> Self {
        Self::SpecialNotFile(value.into())
    }

    /// Make from a new [`FileSegmentedPath`].
    pub fn new_file<T: Into<FileSegmentedPath<'a>>>(value: T) -> Self {
        Self::File(value.into())
    }

    /// Make from a new [`NonSpecialSegmentedPath`].
    pub fn new_non_special<T: Into<NonSpecialSegmentedPath<'a>>>(value: T) -> Self {
        Self::NonSpecial(value.into())
    }
    
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::SpecialNotFile(x) => x.as_str(),
            Self::File          (x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str()
        }
    }



    /// If it's [`Self::SpecialNotFile`] or [`Self::File`].
    pub fn is_special(&self) -> bool {
        matches!(self, Self::SpecialNotFile(_) | Self::File(_))
    }

    /// If it's [`Self::SpecialNotFile`].
    pub fn is_special_not_file(&self) -> bool {
        matches!(self, Self::SpecialNotFile(_))
    }

    /// If it's [`Self::File`].
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }



    /// The [`SpecialNotFileSegmentedPath`].
    pub fn special_not_file(self) -> Option<SpecialNotFileSegmentedPath<'a>> {
        match self {
            Self::SpecialNotFile(x) => Some(x),
            Self::File          (_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// If it's [`Self::File`].
    pub fn file(self) -> Option<FileSegmentedPath<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// The [`NonSpecialSegmentedPath`].
    pub fn non_special(self) -> Option<NonSpecialSegmentedPath<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (_) => None,
            Self::NonSpecial    (x) => Some(x),
        }
    }



    /// Turn into a [`SpecialNotFileSegmentedPath`].
    pub fn into_special_not_file(self) -> SpecialNotFileSegmentedPath<'a> {
        self.into()
    }

    /// Turn into a [`FileSegmentedPath`].
    pub fn into_file(self) -> FileSegmentedPath<'a> {
        self.into()
    }

    /// Turn into a [`NonSpecialSegmentedPath`].
    pub fn into_non_special(self) -> NonSpecialSegmentedPath<'a> {
        self.into()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::SpecialNotFile(x) => x.into_inner(),
            Self::File          (x) => x.into_inner(),
            Self::NonSpecial    (x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SegmentedPath<'static> {
        match self {
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::File          (x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SegmentedPath<'_> {
        match self {
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::File          (x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
        }
    }
}



impl<'a> From<Cow<'a, str>> for SegmentedPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::new_special_not_file(value)
    }
}



impl<'a> From<Path<'a>> for SegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x.into(),
            Path::SpecialNotFile(x) => x.into(),
            Path::NonSpecial    (x) => x.into(),
            Path::Opaque        (x) => x.into(),
        }
    }
}

impl<'a> From<FileSegmentedPath          <'a>> for SegmentedPath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self::File          (value       )}}
impl<'a> From<SpecialNotFileSegmentedPath<'a>> for SegmentedPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self::SpecialNotFile(value       )}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for SegmentedPath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self::NonSpecial    (value       )}}
impl<'a> From<NonSpecialEmptyPath        <'a>> for SegmentedPath<'a> {fn from(value: NonSpecialEmptyPath        <'a>) -> Self {Self::NonSpecial    (value.into())}}
impl<'a> From<OpaquePath                 <'a>> for SegmentedPath<'a> {fn from(value: OpaquePath                 <'a>) -> Self {Self::NonSpecial    (value.into())}}

impl<'a> From<NonSpecialPath<'a>> for SegmentedPath<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x.into(),
            NonSpecialPath::Empty    (x) => x.into(),
        }
    }
}


impl<'a, T: Into<FilePathSegment<'a>> + Into<SpecialNotFilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>> Extend<T> for SegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        match self {
            Self::File          (x) => x.extend(iter),
            Self::SpecialNotFile(x) => x.extend(iter),
            Self::NonSpecial    (x) => x.extend(iter),
        }
    }
}

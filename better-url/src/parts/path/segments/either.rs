//! [`PathSegments`].

use crate::prelude::*;

/// Either [`SpecialNotFilePathSegments`], [`FilePathSegments`] or [`NonSpecialPathSegments`].
#[derive(Debug, Clone)]
pub enum PathSegments<'a> {
    /// [`SpecialNotFilePathSegments`].
    SpecialNotFile(SpecialNotFilePathSegments<'a>),
    /// [`FilePathSegments`].
    File(FilePathSegments<'a>),
    /// [`NonSpecialPathSegments`].
    NonSpecial(NonSpecialPathSegments<'a>),
}

impl<'a> PathSegments<'a> {
    /// The [`SchemeType`].
    pub(crate) fn r#type(&self) -> SchemeType {
        match self {
            Self::SpecialNotFile(_) => SchemeType::SpecialNotFile,
            Self::File          (_) => SchemeType::File,
            Self::NonSpecial    (_) => SchemeType::NonSpecial,
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::SpecialNotFile(x) => x.as_str(),
            Self::File          (x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str(),
        }
    }



    /// The [`PathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = PathSegment<'_>> {
        let r#type = self.r#type();

        self.as_str().split('/').map(move |x| match r#type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegment(x.into()).into(),
            SchemeType::File           => FilePathSegment          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegment    (x.into()).into(),
        })
    }

    /// The `index`th [`PathSegment`].
    pub fn get(&self, index: isize) -> Option<PathSegment<'_>> {
        self.iter().neg_nth(index)
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



    /// The [`SpecialNotFilePathSegments`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePathSegments<'a>> {
        match self {
            Self::SpecialNotFile(x) => Some(x),
            Self::File          (_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// The [`FilePathSegments`].
    pub fn file(self) -> Option<FilePathSegments<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// The [`NonSpecialPathSegments`].
    pub fn non_special(self) -> Option<NonSpecialPathSegments<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (_) => None,
            Self::NonSpecial    (x) => Some(x),
        }
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::SpecialNotFile(x) => x.into_inner(),
            Self::File          (x) => x.into_inner(),
            Self::NonSpecial    (x) => x.into_inner(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_owned(self) -> PathSegments<'static> {
        match self {
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::File          (x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn borrowed(&self) -> PathSegments<'_> {
        match self {
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::File          (x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
        }
    }
}



impl<'a> From<SpecialNotFilePathSegments<'a>> for PathSegments<'a> {fn from(value: SpecialNotFilePathSegments<'a>) -> Self {Self::SpecialNotFile(value)}}
impl<'a> From<FilePathSegments          <'a>> for PathSegments<'a> {fn from(value: FilePathSegments          <'a>) -> Self {Self::File          (value)}}
impl<'a> From<NonSpecialPathSegments    <'a>> for PathSegments<'a> {fn from(value: NonSpecialPathSegments    <'a>) -> Self {Self::NonSpecial    (value)}}

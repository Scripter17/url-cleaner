//! [`PathSegment`].

use crate::prelude::*;

/// Either [`SpecialNotFilePathSegment`], [`FilePathSegment`] or [`NonSpecialPathSegment`].
#[derive(Debug, Clone)]
pub enum PathSegment<'a> {
    /// [`SpecialNotFilePathSegment`].
    SpecialNotFile(SpecialNotFilePathSegment<'a>),
    /// [`FilePathSegment`].
    File(FilePathSegment<'a>),
    /// [`NonSpecialPathSegment`].
    NonSpecial(NonSpecialPathSegment<'a>)
}

impl<'a> PathSegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::SpecialNotFile(x) => x.as_str(),
            Self::File          (x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str(),
        }
    }

    /// The decoded value.
    pub fn decode(self) -> Cow<'a, str> {
        match self {
            Self::SpecialNotFile(x) => x.decode(),
            Self::File          (x) => x.decode(),
            Self::NonSpecial    (x) => x.decode(),
        }
    }



    /// Either [`SpecialNotFilePathSegment::is_windows_drive_letter`], [`FilePathSegment::is_windows_drive_letter`], or [`NonSpecialPathSegment::is_windows_drive_letter`]
    pub fn is_windows_drive_letter(&self) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.is_windows_drive_letter(),
            Self::File          (x) => x.is_windows_drive_letter(),
            Self::NonSpecial    (x) => x.is_windows_drive_letter(),
        }
    }

    /// Either [`SpecialNotFilePathSegment::is_normalized_windows_drive_letter`], [`FilePathSegment::is_normalized_windows_drive_letter`], or [`NonSpecialPathSegment::is_normalized_windows_drive_letter`]
    pub fn is_normalized_windows_drive_letter(&self) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.is_normalized_windows_drive_letter(),
            Self::File          (x) => x.is_normalized_windows_drive_letter(),
            Self::NonSpecial    (x) => x.is_normalized_windows_drive_letter(),
        }
    }

    /// Either [`SpecialNotFilePathSegment::is_non_normalized_windows_drive_letter`], [`FilePathSegment::is_non_normalized_windows_drive_letter`], or [`NonSpecialPathSegment::is_non_normalized_windows_drive_letter`]
    pub fn is_non_normalized_windows_drive_letter(&self) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.is_non_normalized_windows_drive_letter(),
            Self::File          (x) => x.is_non_normalized_windows_drive_letter(),
            Self::NonSpecial    (x) => x.is_non_normalized_windows_drive_letter(),
        }
    }

    /// Either [`SpecialNotFilePathSegment::is_dot`], [`FilePathSegment::is_dot`], or [`NonSpecialPathSegment::is_dot`]
    pub fn is_dot(&self) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.is_dot(),
            Self::File          (x) => x.is_dot(),
            Self::NonSpecial    (x) => x.is_dot(),
        }
    }

    /// Either [`SpecialNotFilePathSegment::is_double_dot`], [`FilePathSegment::is_double_dot`], or [`NonSpecialPathSegment::is_double_dot`]
    pub fn is_double_dot(&self) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.is_double_dot(),
            Self::File          (x) => x.is_double_dot(),
            Self::NonSpecial    (x) => x.is_double_dot(),
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

    /// if it's [`Self::File`].
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }



    /// The [`SpecialNotFilePathSegment`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePathSegment<'a>> {
        match self {
            Self::SpecialNotFile(x) => Some(x),
            Self::File          (_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// The [`FilePathSegment`].
    pub fn file(self) -> Option<FilePathSegment<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// The [`NonSpecialPathSegment`].
    pub fn non_special(self) -> Option<NonSpecialPathSegment<'a>> {
        match self {
            Self::SpecialNotFile(_) => None,
            Self::File          (_) => None,
            Self::NonSpecial    (x) => Some(x),
        }
    }



    /// Turn into a [`SpecialNotFilePathSegment`].
    pub fn into_special_not_file(self) -> SpecialNotFilePathSegment<'a> {
        self.into()
    }

    /// Turn into a [`FilePathSegment`]
    pub fn into_file(self) -> FilePathSegment<'a> {
        self.into()
    }

    /// Turn into a [`NonSpecialPathSegment`].
    pub fn into_non_special(self) -> NonSpecialPathSegment<'a> {
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

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> PathSegment<'_> {
        match self {
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::File          (x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`]
    pub fn into_owned(self) -> PathSegment<'static> {
        match self {
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::File          (x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
        }
    }
}



impl<'a> From<SpecialNotFilePathSegment<'a>> for PathSegment<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self::SpecialNotFile(value)}}
impl<'a> From<FilePathSegment          <'a>> for PathSegment<'a> {fn from(value: FilePathSegment          <'a>) -> Self {Self::File          (value)}}
impl<'a> From<NonSpecialPathSegment    <'a>> for PathSegment<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self::NonSpecial    (value)}}

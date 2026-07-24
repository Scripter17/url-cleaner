//! [`PathSegment`].

use crate::prelude::*;

/// Either [`FilePathSegment`], [`SpecialNotFilePathSegment`], or [`NonSpecialPathSegment`].
#[derive(Debug, Clone)]
pub enum PathSegment<'a> {
    /** [`FilePathSegment`].           **/ File          (FilePathSegment          <'a>),
    /** [`SpecialNotFilePathSegment`]. **/ SpecialNotFile(SpecialNotFilePathSegment<'a>),
    /** [`NonSpecialPathSegment`].     **/ NonSpecial    (NonSpecialPathSegment    <'a>),
}

impl<'a> PathSegment<'a> {
    /// Make a new [`Self`] of the [`SegmentedPathType`].
    pub fn new<T: Into<FilePathSegment<'a>> + Into<SpecialNotFilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(value: T, r#type: SegmentedPathType) -> Self {
        match r#type {
            SegmentedPathType::File           => Self::new_file            (value),
            SegmentedPathType::SpecialNotFile => Self::new_special_not_file(value),
            SegmentedPathType::NonSpecial     => Self::new_non_special     (value),
        }
    }

    /** [`FilePathSegment::new`].           **/ pub fn new_file            <T: Into<FilePathSegment          <'a>>>(value: T) -> Self {FilePathSegment          ::new(value).into()}
    /** [`SpecialNotFilePathSegment::new`]. **/ pub fn new_special_not_file<T: Into<SpecialNotFilePathSegment<'a>>>(value: T) -> Self {SpecialNotFilePathSegment::new(value).into()}
    /** [`NonSpecialPathSegment::new`].     **/ pub fn new_non_special     <T: Into<NonSpecialPathSegment    <'a>>>(value: T) -> Self {NonSpecialPathSegment    ::new(value).into()}



    /// Either [`Self::new_file_unchecked`], [`Self::new_special_not_file_unchecked`], or [`Self::new_non_special_unchecked`].
    /// # Safety
    /// Either [`Self::new_file_unchecked`], [`Self::new_special_not_file_unchecked`], or [`Self::new_non_special_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: SegmentedPathType) -> Self {
        match r#type {
            SegmentedPathType::File           => unsafe {Self::new_file_unchecked            (value)},
            SegmentedPathType::SpecialNotFile => unsafe {Self::new_special_not_file_unchecked(value)},
            SegmentedPathType::NonSpecial     => unsafe {Self::new_non_special_unchecked     (value)},
        }
    }

    /// [`FilePathSegment::new`].
    /// # Safety
    /// [`FilePathSegment::new`].
    pub unsafe fn new_file_unchecked            <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {FilePathSegment          ::new_unchecked(value)}.into()}

    /// [`SpecialNotFilePathSegment::new`].
    /// # Safety
    /// [`SpecialNotFilePathSegment::new`].
    pub unsafe fn new_special_not_file_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialNotFilePathSegment::new_unchecked(value)}.into()}

    /// [`NonSpecialPathSegment::new`].
    /// # Safety
    /// [`NonSpecialPathSegment::new`].
    pub unsafe fn new_non_special_unchecked     <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialPathSegment    ::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::File          (x) => x.as_str(),
            Self::SpecialNotFile(x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str(),
        }
    }

    /// The decoded value.
    pub fn decode(self) -> Cow<'a, str> {
        match self {
            Self::File          (x) => x.decode(),
            Self::SpecialNotFile(x) => x.decode(),
            Self::NonSpecial    (x) => x.decode(),
        }
    }

    /// The [`SegmentedPathType`].
    pub fn r#type(&self) -> SegmentedPathType {
        match self {
            Self::File          (_) => SegmentedPathType::File          ,
            Self::SpecialNotFile(_) => SegmentedPathType::SpecialNotFile,
            Self::NonSpecial    (_) => SegmentedPathType::NonSpecial    ,
        }
    }



    /** [`path_segment_is_drive_letter`].                **/ pub fn is_drive_letter               (&self) -> bool {path_segment_is_drive_letter               (self.as_str())}
    /** [`path_segment_is_normalized_drive_letter`].     **/ pub fn is_normalized_drive_letter    (&self) -> bool {path_segment_is_normalized_drive_letter    (self.as_str())}
    /** [`path_segment_is_non_normalized_drive_letter`]. **/ pub fn is_non_normalized_drive_letter(&self) -> bool {path_segment_is_non_normalized_drive_letter(self.as_str())}
    /** [`path_segment_is_single_dot`].                  **/ pub fn is_single_dot                 (&self) -> bool {path_segment_is_single_dot                 (self.as_str())}
    /** [`path_segment_is_double_dot`].                  **/ pub fn is_double_dot                 (&self) -> bool {path_segment_is_double_dot                 (self.as_str())}



    /// [`Self::File`].
    pub fn file(self) -> Option<FilePathSegment<'a>> {
        match self {
            Self::File          (x) => Some(x),
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::SpecialNotFile`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePathSegment<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::NonSpecial`].
    pub fn non_special(self) -> Option<NonSpecialPathSegment<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (x) => Some(x),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> PathSegment<'_> {
        match self {
            Self::File          (x) => x.borrowed().into(),
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`]
    pub fn into_owned(self) -> PathSegment<'static> {
        match self {
            Self::File          (x) => x.into_owned().into(),
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::File          (x) => x.into_inner(),
            Self::SpecialNotFile(x) => x.into_inner(),
            Self::NonSpecial    (x) => x.into_inner(),
        }
    }
}



impl<'a> From<FilePathSegment          <'a>> for PathSegment<'a> {fn from(value: FilePathSegment          <'a>) -> Self {Self::File          (value)}}
impl<'a> From<SpecialNotFilePathSegment<'a>> for PathSegment<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self::SpecialNotFile(value)}}
impl<'a> From<NonSpecialPathSegment    <'a>> for PathSegment<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self::NonSpecial    (value)}}

//! [`PathSegments`].

use crate::prelude::*;

/// Either [`FilePathSegments`], [`SpecialNotFilePathSegments`], or [`NonSpecialPathSegments`].
#[derive(Debug, Clone)]
pub enum PathSegments<'a> {
    /** [`FilePathSegments`].           **/ File          (FilePathSegments          <'a>),
    /** [`SpecialNotFilePathSegments`]. **/ SpecialNotFile(SpecialNotFilePathSegments<'a>),
    /** [`NonSpecialPathSegments`].     **/ NonSpecial    (NonSpecialPathSegments    <'a>),
}

impl<'a> PathSegments<'a> {
    /// Make a new [`Self`] of the [`SegmentedPathType`].
    pub fn new<T: Into<FilePathSegments<'a>> + Into<SpecialNotFilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(value: T, r#type: SegmentedPathType) -> Self {
        match r#type {
            SegmentedPathType::File           => Self::new_file            (value),
            SegmentedPathType::SpecialNotFile => Self::new_special_not_file(value),
            SegmentedPathType::NonSpecial     => Self::new_non_special     (value),
        }
    }

    /** [`FilePathSegments::new`].           **/ pub fn new_file            <T: Into<FilePathSegments          <'a>>>(value: T) -> Self {FilePathSegments          ::new(value).into()}
    /** [`SpecialNotFilePathSegments::new`]. **/ pub fn new_special_not_file<T: Into<SpecialNotFilePathSegments<'a>>>(value: T) -> Self {SpecialNotFilePathSegments::new(value).into()}
    /** [`NonSpecialPathSegments::new`].     **/ pub fn new_non_special     <T: Into<NonSpecialPathSegments    <'a>>>(value: T) -> Self {NonSpecialPathSegments    ::new(value).into()}



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

    /// [`FilePathSegments::new_unchecked`].
    /// # Safety
    /// [`FilePathSegments::new_unchecked`].
    pub unsafe fn new_file_unchecked            <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {FilePathSegments          ::new_unchecked(value)}.into()}

    /// [`SpecialNotFilePathSegments::new_unchecked`].
    /// # Safety
    /// [`SpecialNotFilePathSegments::new_unchecked`].
    pub unsafe fn new_special_not_file_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialNotFilePathSegments::new_unchecked(value)}.into()}

    /// [`NonSpecialPathSegments::new_unchecked`].
    /// # Safety
    /// [`NonSpecialPathSegments::new_unchecked`].
    pub unsafe fn new_non_special_unchecked     <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialPathSegments    ::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::SpecialNotFile(x) => x.as_str(),
            Self::File          (x) => x.as_str(),
            Self::NonSpecial    (x) => x.as_str(),
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



    /// The [`PathSegmentsIter`].
    pub fn iter(&self) -> PathSegmentsIter<'_> {
        self.into_iter()
    }

    /// The `index`th [`PathSegment`].
    pub fn get(&self, index: isize) -> Option<PathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// [`Self::File`].
    pub fn file(self) -> Option<FilePathSegments<'a>> {
        match self {
            Self::File          (x) => Some(x),
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::SpecialNotFile`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePathSegments<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::NonSpecial`].
    pub fn non_special(self) -> Option<NonSpecialPathSegments<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (x) => Some(x),
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

    /// Turn into the inner [`Cow`].
    pub fn into_owned(self) -> PathSegments<'static> {
        match self {
            Self::File          (x) => x.into_owned().into(),
            Self::SpecialNotFile(x) => x.into_owned().into(),
            Self::NonSpecial    (x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn borrowed(&self) -> PathSegments<'_> {
        match self {
            Self::File          (x) => x.borrowed().into(),
            Self::SpecialNotFile(x) => x.borrowed().into(),
            Self::NonSpecial    (x) => x.borrowed().into(),
        }
    }
}



impl<'a> From<FilePathSegments          <'a>> for PathSegments<'a> {fn from(value: FilePathSegments          <'a>) -> Self {Self::File          (value)}}
impl<'a> From<SpecialNotFilePathSegments<'a>> for PathSegments<'a> {fn from(value: SpecialNotFilePathSegments<'a>) -> Self {Self::SpecialNotFile(value)}}
impl<'a> From<NonSpecialPathSegments    <'a>> for PathSegments<'a> {fn from(value: NonSpecialPathSegments    <'a>) -> Self {Self::NonSpecial    (value)}}

impl<'a> From<FilePathSegment           <'a>> for PathSegments<'a> {fn from(value: FilePathSegment           <'a>) -> Self {Self::File          (value.into())}}
impl<'a> From<SpecialNotFilePathSegment <'a>> for PathSegments<'a> {fn from(value: SpecialNotFilePathSegment <'a>) -> Self {Self::SpecialNotFile(value.into())}}
impl<'a> From<NonSpecialPathSegment     <'a>> for PathSegments<'a> {fn from(value: NonSpecialPathSegment     <'a>) -> Self {Self::NonSpecial    (value.into())}}

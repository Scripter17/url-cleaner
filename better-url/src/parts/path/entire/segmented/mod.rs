//! [`SegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// Either a [`FilePath`], [`SpecialNotFilePath`], or [`NonSpecialPath`].
#[derive(Debug, Clone)]
pub enum SegmentedPath<'a> {
    /** [`FilePath`].           **/ File          (FilePath          <'a>),
    /** [`SpecialNotFilePath`]. **/ SpecialNotFile(SpecialNotFilePath<'a>),
    /** [`NonSpecialPath`].     **/ NonSpecial    (NonSpecialPath    <'a>),
}

impl<'a> SegmentedPath<'a> {
    /// Make a new [`Self`] of the [`SegmentedPathType`].
    pub fn new<T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>>>(value: T, r#type: SegmentedPathType) -> Self {
        match r#type {
            SegmentedPathType::File           => Self::new_file            (value),
            SegmentedPathType::SpecialNotFile => Self::new_special_not_file(value),
            SegmentedPathType::NonSpecial     => Self::new_non_special     (value),
        }
    }

    /** [`FilePath::new`].           **/ pub fn new_file            <T: Into<FilePath          <'a>>>(value: T) -> Self {FilePath          ::new(value).into()}
    /** [`SpecialNotFilePath::new`]. **/ pub fn new_special_not_file<T: Into<SpecialNotFilePath<'a>>>(value: T) -> Self {SpecialNotFilePath::new(value).into()}
    /** [`NonSpecialPath::new`].     **/ pub fn new_non_special     <T: Into<NonSpecialPath    <'a>>>(value: T) -> Self {NonSpecialPath    ::new(value).into()}



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

    /// [`FilePath::new_unchecked`].
    /// # Safety
    /// [`FilePath::new_unchecked`].
    pub unsafe fn new_file_unchecked            <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {FilePath          ::new_unchecked(value)}.into()}

    /// [`SpecialNotFilePath::new_unchecked`].
    /// # Safety
    /// [`SpecialNotFilePath::new_unchecked`].
    pub unsafe fn new_special_not_file_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialNotFilePath::new_unchecked(value)}.into()}

    /// [`NonSpecialPath::new_unchecked`].
    /// # Safety
    /// [`NonSpecialPath::new_unchecked`].
    pub unsafe fn new_non_special_unchecked     <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialPath    ::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::File          (x) => x.as_str(),
            Self::SpecialNotFile(x) => x.as_str(),
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

    /// The [`PathSegments`].
    ///
    /// This is an [`Option`] because of [`NonSpecialPath`].
    pub fn segments(&self) -> Option<PathSegments<'_>> {
        Some(unsafe {PathSegments::new_unchecked(self.as_str().strip_prefix('/')?, self.r#type())})
    }



    /** If it's [`Self::SpecialNotFile`] or [`Self::File`]. **/ pub fn is_special         (&self) -> bool {matches!(self, Self::SpecialNotFile(_) | Self::File(_))}
    /** If it's [`Self::SpecialNotFile`].                   **/ pub fn is_special_not_file(&self) -> bool {matches!(self, Self::SpecialNotFile(_)                )}
    /** If it's [`Self::File`].                             **/ pub fn is_file            (&self) -> bool {matches!(self, Self::File          (_)                )}
    /** If it's [`Self::NonSpecial`].                       **/ pub fn is_non_special     (&self) -> bool {matches!(self, Self::NonSpecial    (_)                )}



    /// [`Self::File`].
    pub fn file(self) -> Option<FilePath<'a>> {
        match self {
            Self::File          (x) => Some(x),
            Self::SpecialNotFile(_) => None,
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::SpecialNotFile`].
    pub fn special_not_file(self) -> Option<SpecialNotFilePath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x),
            Self::NonSpecial    (_) => None,
        }
    }

    /// [`Self::NonSpecial`].
    pub fn non_special(self) -> Option<NonSpecialPath<'a>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(_) => None,
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



impl<'a> TryFrom<Path<'a>> for SegmentedPath<'a> {
    type Error = OpaquePath<'a>;

    fn try_from(value: Path<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Path::Segmented(x) => x,
            Path::Opaque   (x) => Err(x)?,
        })
    }
}

impl<'a> From<FilePath          <'a>> for SegmentedPath<'a> {fn from(value: FilePath          <'a>) -> Self {Self::File          (value)}}
impl<'a> From<SpecialNotFilePath<'a>> for SegmentedPath<'a> {fn from(value: SpecialNotFilePath<'a>) -> Self {Self::SpecialNotFile(value)}}
impl<'a> From<NonSpecialPath    <'a>> for SegmentedPath<'a> {fn from(value: NonSpecialPath    <'a>) -> Self {Self::NonSpecial    (value)}}

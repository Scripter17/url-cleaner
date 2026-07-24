//! [`Path`].

use crate::prelude::*;

/// Either a [`SegmentedPath`] or an [`OpaquePath`].
#[derive(Debug, Clone)]
pub enum Path<'a> {
    /** [`SegmentedPath`]. **/ Segmented(SegmentedPath<'a>),
    /** [`OpaquePath`].    **/ Opaque   (OpaquePath   <'a>),
}

impl<'a> Path<'a> {
    /// Either [`Self::new_segmented`] or [`Self::new_opaque`].
    pub fn new<T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(value: T, r#type: PathType) -> Self {
        match r#type {
            PathType::Segmented(x) => Self::new_segmented(value, x),
            PathType::Opaque       => Self::new_opaque   (value   ),
        }
    }

    /** [`SegmentedPath::new`].      **/ pub fn new_segmented       <T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>>>(value: T, r#type: SegmentedPathType) -> Self {SegmentedPath::new(value, r#type).into()}
    /** [`FilePath::new`].           **/ pub fn new_file            <T: Into<FilePath          <'a>>>(value: T) -> Self {FilePath          ::new(value).into()}
    /** [`SpecialNotFilePath::new`]. **/ pub fn new_special_not_file<T: Into<SpecialNotFilePath<'a>>>(value: T) -> Self {SpecialNotFilePath::new(value).into()}
    /** [`NonSpecialPath::new`].     **/ pub fn new_non_special     <T: Into<NonSpecialPath    <'a>>>(value: T) -> Self {NonSpecialPath    ::new(value).into()}
    /** [`OpaquePath::new`].         **/ pub fn new_opaque          <T: Into<OpaquePath        <'a>>>(value: T) -> Self {OpaquePath        ::new(value).into()}



    /// Either [`Self::new_segmented_unchecked`] or [`Self::new_opaque_unchecked`].
    /// # Safety
    /// Either [`Self::new_segmented_unchecked`] or [`Self::new_opaque_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: PathType) -> Self {
        match r#type {
            PathType::Segmented(x) => unsafe {Self::new_segmented_unchecked(value, x)},
            PathType::Opaque       => unsafe {Self::new_opaque_unchecked   (value   )},
        }
    }

    /// [`SegmentedPath::new_unchecked`].
    /// # Safety
    /// [`SegmentedPath::new_unchecked`].
    pub unsafe fn new_segmented_unchecked       <T: Into<Cow<'a, str>>>(value: T, r#type: SegmentedPathType) -> Self {unsafe {SegmentedPath::new_unchecked(value, r#type)}.into()}

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

    /// [`OpaquePath::new_unchecked`].
    /// # Safety
    /// [`OpaquePath::new_unchecked`].
    pub unsafe fn new_opaque_unchecked          <T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {OpaquePath        ::new_unchecked(value)}.into()}




    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Segmented(x) => x.as_str(),
            Self::Opaque   (x) => x.as_str(),
        }
    }

    /// The [`PathType`].
    pub fn r#type(&self) -> PathType {
        match self {
            Self::Segmented(x) => x.r#type().into(),
            Self::Opaque   (_) => PathType::Opaque,
        }
    }



    /** If it's [`Self::Segmented`].               **/ pub fn is_segmnted        (&self) -> bool {matches!(self, Self::Segmented(_))}
    /** If it's [`SegmentedPath::File`].           **/ pub fn is_file            (&self) -> bool {matches!(self, Self::Segmented(SegmentedPath::File          (_)))}
    /** If it's [`SegmentedPath::SpecialNotFile`]. **/ pub fn is_special_not_file(&self) -> bool {matches!(self, Self::Segmented(SegmentedPath::SpecialNotFile(_)))}
    /** If it's [`SegmentedPath::NonSpecial`].     **/ pub fn is_non_special     (&self) -> bool {matches!(self, Self::Segmented(SegmentedPath::NonSpecial    (_)))}
    /** If it's [`Self::Opaque`].                  **/ pub fn is_opaque          (&self) -> bool {matches!(self, Self::Opaque  (_))}



    /// The [`SegmentedPath`].
    pub fn segmented(self) -> Option<SegmentedPath<'a>> {
        match self {
            Self::Segmented(x) => Some(x),
            Self::Opaque   (_) => None
        }
    }

    /** The [`FilePath`].           **/ pub fn file            (self) -> Option<FilePath          <'a>> {self.segmented()?.file            ()}
    /** The [`SpecialNotFilePath`]. **/ pub fn special_not_file(self) -> Option<SpecialNotFilePath<'a>> {self.segmented()?.special_not_file()}
    /** The [`NonSpecialPath`].     **/ pub fn non_special     (self) -> Option<NonSpecialPath    <'a>> {self.segmented()?.non_special     ()}

    /// The [`OpaquePath`].
    pub fn opaque(self) -> Option<OpaquePath<'a>> {
        match self {
            Self::Segmented(_) => None,
            Self::Opaque   (x) => Some(x),
        }
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
            Self::Segmented(x) => x.borrowed().into() ,
            Self::Opaque   (x) => x.borrowed().into() ,
        }
    }
}



impl<'a> From<SegmentedPath     <'a>> for Path<'a> {fn from(value: SegmentedPath     <'a>) -> Self {Self::Segmented(value       )}}
impl<'a> From<FilePath          <'a>> for Path<'a> {fn from(value: FilePath          <'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<SpecialNotFilePath<'a>> for Path<'a> {fn from(value: SpecialNotFilePath<'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<NonSpecialPath    <'a>> for Path<'a> {fn from(value: NonSpecialPath    <'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<OpaquePath        <'a>> for Path<'a> {fn from(value: OpaquePath        <'a>) -> Self {Self::Opaque   (value       )}}

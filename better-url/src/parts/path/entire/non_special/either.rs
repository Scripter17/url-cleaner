//! [`NonSpecialPath`].

use crate::prelude::*;

/// Either a [`NonSpecialSegmentedPath`] or a [`Self::Empty`].
#[derive(Debug, Clone)]
pub enum NonSpecialPath<'a> {
    /// [`NonSpecialSegmentedPath`].
    Segmented(NonSpecialSegmentedPath<'a>),
    /// [`NonSpecialEmptyPath`].
    Empty(NonSpecialEmptyPath<'a>),
}

impl<'a> NonSpecialPath<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        let value = value.into();

        match value.is_empty() {
            true  =>         NonSpecialEmptyPath    ::default      (     ) .into(),
            false => unsafe {NonSpecialSegmentedPath::new_unchecked(value)}.into(),
        }
    }

    /// Make a new [`Self::Segmented`].
    pub fn new_segmented<T: Into<NonSpecialSegmentedPath<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// Make a new [`Self::Empty`].
    pub fn new_empty() -> Self {
        NonSpecialEmptyPath::default().into()
    }



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Segmented(x) => x.as_str(),
            Self::Empty    (x) => x.as_str(),
        }
    }

    /// The [`Self::Segmented`].
    pub fn segmented(self) -> Option<NonSpecialSegmentedPath<'a>> {
        match self {
            Self::Segmented(x) => Some(x),
            Self::Empty    (_) => None,
        }
    }

    /// The [`Self::Empty`].
    pub fn empty(self) -> Option<NonSpecialEmptyPath<'a>> {
        match self {
            Self::Segmented(_) => None,
            Self::Empty    (x) => Some(x),
        }
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Segmented(x) => x.into_inner(),
            Self::Empty    (x) => x.into_inner(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialPath<'_> {
        match self {
            Self::Segmented(x) => x.borrowed().into(),
            Self::Empty    (x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialPath<'static> {
        match self {
            Self::Segmented(x) => x.into_owned().into(),
            Self::Empty    (x) => x.into_owned().into(),
        }
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value.is_empty() {
            false => Self::new_segmented(value),
            true  => Self::new_empty    (     ),
        }
    }
}



impl<'a> From<Path<'a>> for NonSpecialPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x.into(),
            Path::SpecialNotFile(x) => x.into(),
            Path::NonSpecial    (x) => x,
            Path::Opaque        (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for NonSpecialPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<FileSegmentedPath          <'a>> for NonSpecialPath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<SpecialNotFileSegmentedPath<'a>> for NonSpecialPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self::Segmented(value.into())}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for NonSpecialPath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self::Segmented(value)}}
impl<'a> From<NonSpecialEmptyPath        <'a>> for NonSpecialPath<'a> {fn from(value: NonSpecialEmptyPath        <'a>) -> Self {Self::Empty    (value)}}

impl<'a> From<OpaquePath<'a>> for NonSpecialPath<'a> {
    fn from(value: OpaquePath<'a>) -> Self {
        match value.is_empty() {
            true  => Self::Empty    (Default::default()),
            false => Self::Segmented(value.into()),
        }
    }
}

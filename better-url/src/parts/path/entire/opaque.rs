//! [`OpaquePath`].

use crate::prelude::*;

/// An opaque path.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert_eq!(OpaquePath::new("  "), " %20");
/// ```
#[derive(Debug, Clone)]
pub struct OpaquePath<'a>(pub(crate) Cow<'a, str>);

impl<'a> OpaquePath<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }




    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> OpaquePath<'static> {
        OpaquePath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> OpaquePath<'_> {
        OpaquePath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for OpaquePath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_opaque_path(value).1)
    }
}



impl<'a> From<Path<'a>> for OpaquePath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::File          (x) => x.into(),
            Path::SpecialNotFile(x) => x.into(),
            Path::NonSpecial    (x) => x.into(),
            Path::Opaque        (x) => x,
        }
    }
}

impl<'a> From<SegmentedPath              <'a>> for OpaquePath<'a> {fn from(value: SegmentedPath              <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<FileSegmentedPath          <'a>> for OpaquePath<'a> {fn from(value: FileSegmentedPath          <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<SpecialNotFileSegmentedPath<'a>> for OpaquePath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for OpaquePath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<NonSpecialEmptyPath        <'a>> for OpaquePath<'a> {fn from(value: NonSpecialEmptyPath        <'a>) -> Self {Self(                              value.into_inner()   )}}

impl<'a> From<NonSpecialPath<'a>> for OpaquePath<'a> {
    fn from(value: NonSpecialPath<'a>) -> Self {
        match value {
            NonSpecialPath::Segmented(x) => x.into(),
            NonSpecialPath::Empty    (x) => x.into(),
        }
    }
}

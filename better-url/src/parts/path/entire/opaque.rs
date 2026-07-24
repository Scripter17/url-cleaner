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
        let (_, value) = make_opaque_path(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}



impl<'a> From<Path<'a>> for OpaquePath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x,
        }
    }
}

impl<'a> From<SegmentedPath     <'a>> for OpaquePath<'a> {fn from(value: SegmentedPath     <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<FilePath          <'a>> for OpaquePath<'a> {fn from(value: FilePath          <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<SpecialNotFilePath<'a>> for OpaquePath<'a> {fn from(value: SpecialNotFilePath<'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}
impl<'a> From<NonSpecialPath    <'a>> for OpaquePath<'a> {fn from(value: NonSpecialPath    <'a>) -> Self {Self(segmented_path_to_opaque_path(value.into_inner()).1)}}

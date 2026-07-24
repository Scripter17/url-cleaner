//! [`NonSpecialPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A [`SchemeType::NonSpecial`] segmented (non-opaque) path.
///
/// For opaque paths, see [`OpaquePath`].
///
/// Please note that this can be empty, in which case it has zero segments.
///
/// This is an [intentional](https://github.com/whatwg/url/issues/926), though often forgotten, detail of how paths exist in the URL spec's canon.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let url = BetterUrl::new("non-special://example.com?empty#path").unwrap();
///
/// assert_eq!(url.as_str(), "non-special://example.com?empty#path");
///
/// assert_eq!(url.segmented_path().unwrap().iter().count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct NonSpecialPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> NonSpecialPath<'a> {
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

    /// The [`NonSpecialPathSegments`].
    pub fn segments(&self) -> Option<NonSpecialPathSegments<'_>> {
        Some(unsafe {NonSpecialPathSegments::new_unchecked(self.as_str().strip_prefix('/')?)})
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialPath<'static> {
        NonSpecialPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialPath<'_> {
        NonSpecialPath(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialPath<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = make_non_special_path(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}



impl<'a> From<Path<'a>> for NonSpecialPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}



impl<'a> From<SegmentedPath     <'a>> for NonSpecialPath<'a> {fn from(value: SegmentedPath     <'a>) -> Self {Self(                                value.into_inner()   )}}
impl<'a> From<FilePath          <'a>> for NonSpecialPath<'a> {fn from(value: FilePath          <'a>) -> Self {Self(                                value.into_inner()   )}}
impl<'a> From<SpecialNotFilePath<'a>> for NonSpecialPath<'a> {fn from(value: SpecialNotFilePath<'a>) -> Self {Self(                                value.into_inner()   )}}
impl<'a> From<OpaquePath        <'a>> for NonSpecialPath<'a> {fn from(value: OpaquePath        <'a>) -> Self {Self(opaque_path_to_non_special_path(value.into_inner()).1)}}

impl From<PathSegments              <'_>> for NonSpecialPath<'static> {fn from(value: PathSegments              <'_>) -> Self {Self(resolve_non_special_path(value.into_inner().with_insert_str(0, "/")).1)}}
impl From<FilePathSegments          <'_>> for NonSpecialPath<'static> {fn from(value: FilePathSegments          <'_>) -> Self {Self(resolve_non_special_path(value.into_inner().with_insert_str(0, "/")).1)}}
impl From<SpecialNotFilePathSegments<'_>> for NonSpecialPath<'static> {fn from(value: SpecialNotFilePathSegments<'_>) -> Self {Self(resolve_non_special_path(value.into_inner().with_insert_str(0, "/")).1)}}
impl From<NonSpecialPathSegments    <'_>> for NonSpecialPath<'static> {fn from(value: NonSpecialPathSegments    <'_>) -> Self {Self(resolve_non_special_path(value.into_inner().with_insert_str(0, "/")).1)}}

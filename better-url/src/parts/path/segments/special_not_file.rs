//! [`SpecialNotFilePathSegments`].

use crate::prelude::*;

/// One or more [`SpecialNotFilePathSegment`]s.
///
/// Please note that single and double dot segments are not resolved until placed into a [`SpecialNotFilePath`].
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let segments = SpecialNotFilePathSegments::new("abc/def/../ghi");
///
/// assert_eq!(segments, "abc/def/../ghi");
///
/// let path = SpecialNotFilePath::new(segments);
///
/// assert_eq!(path, "/abc/ghi");
/// ```
#[derive(Debug, Clone)]
pub struct SpecialNotFilePathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFilePathSegments<'a> {
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



    /// The [`SpecialNotFilePathSegmentsIter`].
    pub fn iter(&self) -> SpecialNotFilePathSegmentsIter<'_> {
        self.into_iter()
    }

    /// The `index`th [`SpecialNotFilePathSegment`].
    pub fn get(&self, index: isize) -> Option<SpecialNotFilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialNotFilePathSegments<'static> {
        SpecialNotFilePathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFilePathSegments<'_> {
        SpecialNotFilePathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for SpecialNotFilePathSegments<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_special_not_file_path_segments(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<PathSegments<'a>> for SpecialNotFilePathSegments<'a> {
    fn from(value: PathSegments<'a>) -> Self {
        match value {
            PathSegments::File          (x) => x.into(),
            PathSegments::SpecialNotFile(x) => x,
            PathSegments::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<FilePathSegments      <'a>> for SpecialNotFilePathSegments<'a> {fn from(value: FilePathSegments      <'a>) -> Self {Self(                value.0   )}}
impl<'a> From<NonSpecialPathSegments<'a>> for SpecialNotFilePathSegments<'a> {fn from(value: NonSpecialPathSegments<'a>) -> Self {Self(forward_slashes(value.0).1)}}

impl<'a> From<PathSegment              <'a>> for SpecialNotFilePathSegments<'a> {fn from(value: PathSegment              <'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<FilePathSegment          <'a>> for SpecialNotFilePathSegments<'a> {fn from(value: FilePathSegment          <'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<SpecialNotFilePathSegment<'a>> for SpecialNotFilePathSegments<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<NonSpecialPathSegment    <'a>> for SpecialNotFilePathSegments<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self(forward_slashes(value.into_inner()).1)}}

impl<'a, 'b> Extend<SpecialNotFilePathSegment<'b>> for SpecialNotFilePathSegments<'a> {
    fn extend<I: IntoIterator<Item = SpecialNotFilePathSegment<'b>>>(&mut self, iter: I) {
        self.0.to_mut().extend(iter.into_iter().flat_map(|x| ["/".into(), x.into_inner()]))
    }
}

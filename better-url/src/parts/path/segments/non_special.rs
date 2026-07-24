//! [`NonSpecialPathSegments`].

use crate::prelude::*;

/// One or more [`NonSpecialPathSegment`]s.
///
/// Please note that single and double dot segments are not resolved until placed into a [`NonSpecialPath`].
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let segments = NonSpecialPathSegments::new("abc/def/../ghi");
///
/// assert_eq!(segments, "abc/def/../ghi");
///
/// let path = NonSpecialPath::new(segments);
///
/// assert_eq!(path, "/abc/ghi");
/// ```
#[derive(Debug, Clone)]
pub struct NonSpecialPathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> NonSpecialPathSegments<'a> {
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



    /// The [`NonSpecialPathSegmentsIter`].
    pub fn iter(&self) -> NonSpecialPathSegmentsIter<'_> {
        self.into_iter()
    }

    /// The `index`th [`NonSpecialPathSegment`].
    pub fn get(&self, index: isize) -> Option<NonSpecialPathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialPathSegments<'static> {
        NonSpecialPathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialPathSegments<'_> {
        NonSpecialPathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialPathSegments<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_non_special_path_segments(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<PathSegments<'a>> for NonSpecialPathSegments<'a> {
    fn from(value: PathSegments<'a>) -> Self {
        match value {
            PathSegments::File          (x) => x.into(),
            PathSegments::SpecialNotFile(x) => x.into(),
            PathSegments::NonSpecial    (x) => x,
        }
    }
}

impl<'a> From<FilePathSegments          <'a>> for NonSpecialPathSegments<'a> {fn from(value: FilePathSegments          <'a>) -> Self {Self(value.0)}}
impl<'a> From<SpecialNotFilePathSegments<'a>> for NonSpecialPathSegments<'a> {fn from(value: SpecialNotFilePathSegments<'a>) -> Self {Self(value.0)}}

impl<'a> From<PathSegment              <'a>> for NonSpecialPathSegments<'a> {fn from(value: PathSegment              <'a>) -> Self {Self(value.into_inner())}}
impl<'a> From<FilePathSegment          <'a>> for NonSpecialPathSegments<'a> {fn from(value: FilePathSegment          <'a>) -> Self {Self(value.into_inner())}}
impl<'a> From<SpecialNotFilePathSegment<'a>> for NonSpecialPathSegments<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self(value.into_inner())}}
impl<'a> From<NonSpecialPathSegment    <'a>> for NonSpecialPathSegments<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self(value.into_inner())}}

impl<'a, 'b> Extend<NonSpecialPathSegment<'b>> for NonSpecialPathSegments<'a> {
    fn extend<I: IntoIterator<Item = NonSpecialPathSegment<'b>>>(&mut self, iter: I) {
        self.0.to_mut().extend(iter.into_iter().flat_map(|x| ["/".into(), x.into_inner()]))
    }
}

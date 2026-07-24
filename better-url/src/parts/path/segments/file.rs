//! [`FilePathSegments`].

use crate::prelude::*;

/// One or more [`FilePathSegment`]s.
///
/// Please note that single and double dot segments are not resolved until placed into a [`FilePath`].
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let segments = FilePathSegments::new("abc/def/../ghi");
///
/// assert_eq!(segments, "abc/def/../ghi");
///
/// let path = FilePath::new(segments);
///
/// assert_eq!(path, "/abc/ghi");
/// ```
#[derive(Debug, Clone)]
pub struct FilePathSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> FilePathSegments<'a> {
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



    /// The [`FilePathSegmentsIter`].
    pub fn iter(&self) -> FilePathSegmentsIter<'_> {
        self.into_iter()
    }

    /// The `index`th [`FilePathSegment`].
    pub fn get(&self, index: isize) -> Option<FilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FilePathSegments<'static> {
        FilePathSegments(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FilePathSegments<'_> {
        FilePathSegments(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for FilePathSegments<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_file_path_segments(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}

impl<'a> From<PathSegments<'a>> for FilePathSegments<'a> {
    fn from(value: PathSegments<'a>) -> Self {
        match value {
            PathSegments::File          (x) => x,
            PathSegments::SpecialNotFile(x) => x.into(),
            PathSegments::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFilePathSegments<'a>> for FilePathSegments<'a> {fn from(value: SpecialNotFilePathSegments<'a>) -> Self {Self(                value.0   )}}
impl<'a> From<NonSpecialPathSegments    <'a>> for FilePathSegments<'a> {fn from(value: NonSpecialPathSegments    <'a>) -> Self {Self(forward_slashes(value.0).1)}}

impl<'a> From<PathSegment              <'a>> for FilePathSegments<'a> {fn from(value: PathSegment              <'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<FilePathSegment          <'a>> for FilePathSegments<'a> {fn from(value: FilePathSegment          <'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<SpecialNotFilePathSegment<'a>> for FilePathSegments<'a> {fn from(value: SpecialNotFilePathSegment<'a>) -> Self {Self(                value.into_inner()   )}}
impl<'a> From<NonSpecialPathSegment    <'a>> for FilePathSegments<'a> {fn from(value: NonSpecialPathSegment    <'a>) -> Self {Self(forward_slashes(value.into_inner()).1)}}

impl<'a, 'b> Extend<FilePathSegment<'b>> for FilePathSegments<'a> {
    fn extend<I: IntoIterator<Item = FilePathSegment<'b>>>(&mut self, iter: I) {
        self.0.to_mut().extend(iter.into_iter().flat_map(|x| ["/".into(), x.into_inner()]))
    }
}

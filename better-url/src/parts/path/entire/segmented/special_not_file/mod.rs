//! [`SpecialNotFileSegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A special but not file segmented path.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let paths = [
///     "/"     , "/abc"     , "/abc/"     , "/abc/def"     , "/abc/def/"     ,
///     "/."    , "/abc."    , "/abc/."    , "/abc/def."    , "/abc/def/."    ,
///     "/.."   , "/abc.."   , "/abc/.."   , "/abc/def.."   , "/abc/def/.."   ,
///     "/./."  , "/abc./."  , "/abc/./."  , "/abc/def./."  , "/abc/def/./."  ,
///     "/../." , "/abc../." , "/abc/../." , "/abc/def../." , "/abc/def/../." ,
///     "/./.." , "/abc./.." , "/abc/./.." , "/abc/def./.." , "/abc/def/./.." ,
///     "/../..", "/abc../..", "/abc/../..", "/abc/def../..", "/abc/def/../..",
///
///     ""      , "abc"      , "abc/"      , "abc/def"      , "abc/def/"      ,
///     "."     , "abc."     , "abc/."     , "abc/def."     , "abc/def/."     ,
///     ".."    , "abc.."    , "abc/.."    , "abc/def.."    , "abc/def/.."    ,
///     "./."   , "abc./."   , "abc/./."   , "abc/def./."   , "abc/def/./."   ,
///     "../."  , "abc../."  , "abc/../."  , "abc/def../."  , "abc/def/../."  ,
///     "./.."  , "abc./.."  , "abc/./.."  , "abc/def./.."  , "abc/def/./.."  ,
///     "../.." , "abc../.." , "abc/../.." , "abc/def../.." , "abc/def/../.." ,
/// ];
///
/// let mut url = url::Url::parse("https://example.com").unwrap();
///
/// for path in paths {
///     url.set_path(path);
///     let mine = SpecialNotFileSegmentedPath::new(path);
///
///     assert_eq!(url.path(), mine, "{path}");
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SpecialNotFileSegmentedPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialNotFileSegmentedPath<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialNotFileSegmentedPath<'static> {
        SpecialNotFileSegmentedPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialNotFileSegmentedPath<'_> {
        SpecialNotFileSegmentedPath(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<Cow<'a, str>> for SpecialNotFileSegmentedPath<'a> {
    fn from(mut value: Cow<'a, str>) -> Self {
        value = PartTranscoder::SpecialPath.encode(value);

        if !value.starts_with("/") {
            value.to_mut().insert(0, '/');
        }

        let mut segments = value.split('/').skip(1).map(|x| SpecialNotFilePathSegment(x.into()));

        while let Some(segment) = segments.next() {
            if segment.is_dot() || segment.is_double_dot() {
                let mut ret = value[.. segment.0.addr() - value.addr() - 1].to_string();
                extend_path_segments(&mut ret, false, std::iter::once(segment).chain(segments));
                if ret.is_empty() {
                    return Self("/".into());
                }
                return Self(ret.into());
            }
        }

        Self(value)
    }
}



impl<'a> From<Path<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for SpecialNotFileSegmentedPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::SpecialNotFile(x) => x,
            SegmentedPath::File          (x) => x.into(),
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<FileSegmentedPath      <'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: FileSegmentedPath      <'a>) -> Self {Self(               value.into_inner() )}}
impl<'a> From<NonSpecialSegmentedPath<'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: NonSpecialSegmentedPath<'a>) -> Self {Self(path_nss_2_snf(value.into_inner()))}}
impl<'a> From<OpaquePath             <'a>> for SpecialNotFileSegmentedPath<'a> {fn from(value: OpaquePath             <'a>) -> Self {Self(path_o_2_snf  (value.into_inner()))}}



impl<'b, T: Into<SpecialNotFilePathSegment<'b>>> Extend<T> for SpecialNotFileSegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        extend_path_segments(self.0.to_mut(), false, iter.into_iter().map(Into::into));
    }
}

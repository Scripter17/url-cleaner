//! [`FileSegmentedPath`].

use crate::prelude::*;

mod get;
mod set;
mod remove;

/// A file segmented path.
///
/// <https://jsdom.github.io/whatwg-url/> was used to make the test.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let tests = [
///     ("/"     , "/"  ), ("/abc"     , "/abc"   ), ("/abc/"     , "/abc/"  ), ("/abc/def"     , "/abc/def"   ), ("/abc/def/"     , "/abc/def/"),
///     ("/."    , "/"  ), ("/abc."    , "/abc."  ), ("/abc/."    , "/abc/"  ), ("/abc/def."    , "/abc/def."  ), ("/abc/def/."    , "/abc/def/"),
///     ("/.."   , "/"  ), ("/abc.."   , "/abc.." ), ("/abc/.."   , "/"      ), ("/abc/def.."   , "/abc/def.." ), ("/abc/def/.."   , "/abc/"    ),
///     ("/./."  , "/"  ), ("/abc./."  , "/abc./" ), ("/abc/./."  , "/abc/"  ), ("/abc/def./."  , "/abc/def./" ), ("/abc/def/./."  , "/abc/def/"),
///     ("/../." , "/"  ), ("/abc../." , "/abc../"), ("/abc/../." , "/"      ), ("/abc/def../." , "/abc/def../"), ("/abc/def/../." , "/abc/"    ),
///     ("/./.." , "/"  ), ("/abc./.." , "/"      ), ("/abc/./.." , "/"      ), ("/abc/def./.." , "/abc/"      ), ("/abc/def/./.." , "/abc/"    ),
///     ("/../..", "/"  ), ("/abc../..", "/"      ), ("/abc/../..", "/"      ), ("/abc/def../..", "/abc/"      ), ("/abc/def/../..", "/"        ),
///
///     (""      , "/"  ), ("abc"      , "/abc"   ), ("abc/"      , "/abc/"  ), ("abc/def"      , "/abc/def"   ), ("abc/def/"      , "/abc/def/"),
///     ("."     , "/"  ), ("abc."     , "/abc."  ), ("abc/."     , "/abc/"  ), ("abc/def."     , "/abc/def."  ), ("abc/def/."     , "/abc/def/"),
///     (".."    , "/"  ), ("abc.."    , "/abc.." ), ("abc/.."    , "/"      ), ("abc/def.."    , "/abc/def.." ), ("abc/def/.."    , "/abc/"    ),
///     ("./."   , "/"  ), ("abc./."   , "/abc./" ), ("abc/./."   , "/abc/"  ), ("abc/def./."   , "/abc/def./" ), ("abc/def/./."   , "/abc/def/"),
///     ("../."  , "/"  ), ("abc../."  , "/abc../"), ("abc/../."  , "/"      ), ("abc/def../."  , "/abc/def../"), ("abc/def/../."  , "/abc/"    ),
///     ("./.."  , "/"  ), ("abc./.."  , "/"      ), ("abc/./.."  , "/"      ), ("abc/def./.."  , "/abc/"      ), ("abc/def/./.."  , "/abc/"    ),
///     ("../.." , "/"  ), ("abc../.." , "/"      ), ("abc/../.." , "/"      ), ("abc/def../.." , "/abc/"      ), ("abc/def/../.." , "/"        ),
///
///     ("/c:"   , "/c:"), ("/c:/"     , "/c:/"   ), ("/c:/abc"   , "/c:/abc"), ("/c:/."        , "/c:/"       ), ("/c:/.."        , "/c:/"     ),
///     ("c:"    , "/c:"), ("c:/"      , "/c:/"   ), ("c:/abc"    , "/c:/abc"), ("c:/."         , "/c:/"       ), ("c:/.."         , "/c:/"     ),
///
///     ("/c|"   , "/c:"), ("/c|/"     , "/c:/"   ), ("/c|/abc"   , "/c:/abc"), ("/c|/."        , "/c:/"       ), ("/c|/.."        , "/c:/"     ),
///     ("c|"    , "/c:"), ("c|/"      , "/c:/"   ), ("c|/abc"    , "/c:/abc"), ("c|/."         , "/c:/"       ), ("c|/.."         , "/c:/"     ),
/// ];
///
/// for x in tests {
///     assert_eq!(FileSegmentedPath::new(x.0), x.1, "{}", x.0);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FileSegmentedPath<'a>(pub(crate) Cow<'a, str>);

impl<'a> FileSegmentedPath<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FileSegmentedPath<'static> {
        FileSegmentedPath(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FileSegmentedPath<'_> {
        FileSegmentedPath(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<Cow<'a, str>> for FileSegmentedPath<'a> {
    fn from(mut value: Cow<'a, str>) -> Self {
        value = PartTranscoder::SpecialPath.encode(value);

        if !value.starts_with("/") {
            value.to_mut().insert(0, '/');
        }

        if let [b'/', x, b'|'] | [b'/', x, b'|', b'/', ..] = value.as_bytes() && x.is_ascii_alphabetic() {
            // SAFETY: Replacing ASCII with ASCII is always valid.
            #[allow(clippy::indexing_slicing, reason = "Can't happen.")]
            unsafe {
                value.to_mut().as_mut_vec()[2] = b':';
            }
        }

        let mut segments = value.split('/').skip(1).map(|x| FilePathSegment(x.into())).peekable();

        while let Some(segment) = segments.next() {
            if segment.is_dot() || segment.is_double_dot() {
                let mut ret = value[..segment.0.addr() - value.addr() - 1].to_string();
                extend_path_segments(&mut ret, true, std::iter::once(segment).chain(segments));
                if ret.is_empty() {
                    return Self("/".into());
                }
                return Self(ret.into());
            }
        }

        Self(value)
    }
}



impl<'a> From<Path<'a>> for FileSegmentedPath<'a> {
    fn from(value: Path<'a>) -> Self {
        match value {
            Path::Segmented(x) => x.into(),
            Path::Opaque   (x) => x.into(),
        }
    }
}

impl<'a> From<SegmentedPath<'a>> for FileSegmentedPath<'a> {
    fn from(value: SegmentedPath<'a>) -> Self {
        match value {
            SegmentedPath::SpecialNotFile(x) => x.into(),
            SegmentedPath::File          (x) => x,
            SegmentedPath::NonSpecial    (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFileSegmentedPath<'a>> for FileSegmentedPath<'a> {fn from(value: SpecialNotFileSegmentedPath<'a>) -> Self {Self(path_snf_2_f(value.into_inner()))}}
impl<'a> From<NonSpecialSegmentedPath    <'a>> for FileSegmentedPath<'a> {fn from(value: NonSpecialSegmentedPath    <'a>) -> Self {Self(path_nss_2_f(value.into_inner()))}}
impl<'a> From<OpaquePath                 <'a>> for FileSegmentedPath<'a> {fn from(value: OpaquePath                 <'a>) -> Self {Self(path_o_2_f  (value.into_inner()))}}



impl<'b, T: Into<FilePathSegment<'b>>> Extend<T> for FileSegmentedPath<'_> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        extend_path_segments(self.0.to_mut(), true, iter.into_iter().map(Into::into));
    }
}

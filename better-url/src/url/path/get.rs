//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Path`].
    pub fn path(&self) -> Path<'_> {
        let ret = self.path_str();

        match ret.starts_with("/") {
            false => OpaquePath(ret.into()).into(),
            true  => Path::Segmented(match self.scheme_type() {
                SchemeType::SpecialNotFile => SpecialNotFileSegmentedPath(ret.into()).into(),
                SchemeType::File           => FileSegmentedPath          (ret.into()).into(),
                SchemeType::NonSpecial     => NonSpecialSegmentedPath    (ret.into()).into(),
            })
        }
    }

    /// The [`Path`] as a [`str`].
    pub fn path_str(&self) -> &str {
        self.url.path()
    }



    /// If the path is segmented.
    pub fn path_is_segmented(&self) -> bool {
        self.path_str().starts_with("/")
    }

    /// If the path is opaque.
    pub fn path_is_opaque(&self) -> bool {
        !self.path_str().starts_with("/")
    }



    /// The [`SegmentedPath`].
    pub fn segmented_path(&self) -> Option<SegmentedPath<'_>> {
        self.path().segmented()
    }

    /// The [`SegmentedPath`] as a [`str`].
    pub fn segmented_path_str(&self) -> Option<&str> {
        let ret = self.path_str();

        match ret.starts_with("/") {
            false => None,
            true  => Some(ret)
        }
    }



    /// The [`OpaquePath`].
    pub fn opaque_path(&self) -> Option<OpaquePath<'_>> {
        self.path().opaque()
    }

    /// The [`OpaquePath`] as a [`str`].
    pub fn opaque_path_str(&self) -> Option<&str> {
        let ret = self.path_str();

        match ret.starts_with("/") {
            false => Some(ret),
            true  => None
        }
    }



    /// The [`PathSegment`]s.
    pub fn path_segments(&self) -> Option<impl DoubleEndedIterator<Item = PathSegment<'_>>> {
        let scheme_type = self.scheme_type();

        Some(self.path_segments_str()?.map(move |x| match scheme_type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegment(x.into()).into(),
            SchemeType::File           => FilePathSegment          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegment    (x.into()).into(),
        }))
    }

    /// The path segments as [`str`]s.
    pub fn path_segments_str(&self) -> Option<impl DoubleEndedIterator<Item = &str>> {
        Some(self.segmented_path_str()?[1..].split('/'))
    }



    /// The `index`th [`PathSegment`].
    pub fn path_segment(&self, index: isize) -> Option<PathSegment<'_>> {
        let scheme_type = self.scheme_type();

        self.path_segment_str(index).map(|x| match scheme_type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegment(x.into()).into(),
            SchemeType::File           => FilePathSegment          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegment    (x.into()).into(),
        })
    }

    /// The `index`th path segment as a [`str`].
    pub fn path_segment_str(&self, index: isize) -> Option<&str> {
        self.path_segments_str()?.neg_nth(index)
    }



    /// The [`PathSegments`].
    pub fn path_segment_range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'_>> {
        let r#type = self.scheme_type();
    
        self.path_segment_range_str(range).map(|x| match r#type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegments(x.into()).into(),
            SchemeType::File           => FilePathSegments          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegments    (x.into()).into(),
        })
    }

    /// The range of path segments as a [`str`].
    /// # Example
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(url.path_segment_range_str(1..=2), Some("b/c"));
    /// ```
    pub fn path_segment_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        Some(&self.as_str()[self.as_str().my_substr_range(self.segmented_path()?.get_range(range)?.as_str())])
    }
}

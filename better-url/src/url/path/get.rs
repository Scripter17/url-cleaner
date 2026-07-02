//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Path`] as a [`str`].
    pub fn path_str(&self) -> &str {
        self.url.path()
    }

    /// The [`Path`].
    pub fn path(&self) -> Path<'_> {
        let ret = self.path_str();

        match self.cannot_be_a_base() {
            true  => match ret.starts_with('/') {
                true  => NonSpecialSegmentedPath::new_unchecked(ret).into(),
                false => OpaquePath             ::new_unchecked(ret).into(),
            },
            false => match self.scheme_type() {
                SchemeType::File           => FilePath          ::new_unchecked(ret).into(),
                SchemeType::SpecialNotFile => SpecialNotFilePath::new_unchecked(ret).into(),
                SchemeType::NonSpecial     => NonSpecialPath    ::new_unchecked(ret).into(),
            }
        }
    }



    /// If the path is segmented.
    pub fn path_is_segmented(&self) -> bool {
        self.path_str().starts_with("/")
    }

    /// If the path is opaque.
    pub fn path_is_opaque(&self) -> bool {
        !self.path_str().starts_with("/")
    }

    /// If [`Self::path_segment`] returns [`Some`].
    pub fn has_path_segment(&self, index: isize) -> bool {
        self.path_segment(index).is_some()
    }



    /// The [`SegmentedPath`] as a [`str`].
    pub fn segmented_path_str(&self) -> Option<&str> {
        let ret = self.path_str();

        match ret.starts_with('/') {
            true  => Some(ret),
            false => None,
        }
    }

    /// The [`SegmentedPath`].
    pub fn segmented_path(&self) -> Option<SegmentedPath<'_>> {
        self.path().segmented()
    }



    /// The [`OpaquePath`] as a [`str`].
    pub fn opaque_path_str(&self) -> Option<&str> {
        let ret = self.path_str();

        match ret.starts_with('/') {
            true  => None,
            false => Some(ret),
        }
    }

    /// The [`OpaquePath`].
    pub fn opaque_path(&self) -> Option<OpaquePath<'_>> {
        self.path().opaque()
    }



    /// The path segments as [`str`]s.
    pub fn path_segment_strs(&self) -> Option<SplitSlashes<'_>> {
        Some(SplitSlashes(Some(&self.segmented_path_str()?[1..])))
    }

    /// The path segments as [`PathSegment`]s.
    pub fn path_segments(&self) -> Option<impl DoubleEndedIterator<Item = PathSegment<'_>>> {
        let r#type = self.scheme_type();

        Some(self.path_segment_strs()?.map(move |x| match r#type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegment(x.into()).into(),
            SchemeType::File           => FilePathSegment          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegment    (x.into()).into(),
        }))
    }



    /// The `index`th path segment as a [`str`].
    pub fn path_segment_str(&self, index: isize) -> Option<&str> {
        self.path_segment_strs()?.neg_nth(index)
    }

    /// The `index`th [`PathSegment`].
    pub fn path_segment(&self, index: isize) -> Option<PathSegment<'_>> {
        self.path_segments()?.neg_nth(index)
    }



    /// The range of path segments as a [`str`].
    pub fn path_segment_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        Some(&self.as_str()[self.as_str().my_substr_range(self.segmented_path()?.get_range(range)?.as_str())])
    }

    /// The range of path segments as a [`PathSegments`].
    pub fn path_segment_range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'_>> {
        let r#type = self.scheme_type();

        self.path_segment_range_str(range).map(|x| match r#type {
            SchemeType::SpecialNotFile => SpecialNotFilePathSegments(x.into()).into(),
            SchemeType::File           => FilePathSegments          (x.into()).into(),
            SchemeType::NonSpecial     => NonSpecialPathSegments    (x.into()).into(),
        })
    }
}

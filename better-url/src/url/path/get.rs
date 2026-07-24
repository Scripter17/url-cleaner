//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the path.
    pub(crate) fn path_start(&self) -> usize {
        self.details.path_start as usize
    }

    /// The [`Range::end`] of the path.
    pub(crate) fn path_after(&self) -> usize {
        self.details.query_mark.or(self.details.fragment_mark).map_or(self.len(), |x| x.get() as usize)
    }

    /// The [`Range`] of the path.
    pub(crate) fn path_range(&self) -> Range<usize> {
        self.path_start() .. self.path_after()
    }



    /// The path as a [`str`].
    pub fn path_str(&self) -> &str {
        unsafe {
            self.serialization.get_unchecked(self.path_range())
        }
    }

    /// The [`PathType`].
    pub fn path_type(&self) -> PathType {
        match self.cannot_be_a_base() {
            true => PathType::Opaque,
            false => match self.scheme_type() {
                SchemeType::File           => SegmentedPathType::File          .into(),
                SchemeType::SpecialNotFile => SegmentedPathType::SpecialNotFile.into(),
                SchemeType::NonSpecial     => SegmentedPathType::NonSpecial    .into(),
            }
        }
    }

    /// The [`Path`].
    pub fn path(&self) -> Path<'_> {
        unsafe {
            Path::new_unchecked(self.path_str(), self.path_type())
        }
    }



    /// If the path is segmented.
    pub fn path_is_segmented(&self) -> bool {
        self.can_be_a_base()
    }

    /// If [`Self::path_is_segmented`], [`Self::path_str`].
    pub fn segmented_path_str(&self) -> Option<&str> {
        self.path_is_segmented().then(|| self.path_str())
    }

    /// The [`SegmentedPath`].
    pub fn segmented_path(&self) -> Option<SegmentedPath<'_>> {
        Some(unsafe {SegmentedPath::new_unchecked(self.segmented_path_str()?, self.scheme_type().into())})
    }



    /// If the path is opaque.
    pub fn path_is_opaque(&self) -> bool {
        self.cannot_be_a_base()
    }

    /// If [`Self::path_is_opaque`], [`Self::path_str`].
    pub fn opaque_path_str(&self) -> Option<&str> {
        self.path_is_opaque().then(|| self.path_str())
    }

    /// The [`OpaquePath`].
    pub fn opaque_path(&self) -> Option<OpaquePath<'_>> {
        Some(unsafe {OpaquePath::new_unchecked(self.opaque_path_str()?)})
    }



    /// The path segments as [`str`]s.
    pub fn path_segment_strs(&self) -> Option<SplitSlashes<'_>> {
        Some(SplitSlashes(self.segmented_path_str()?.strip_prefix('/')))
    }

    /// The [`PathSegmentsIter`].
    pub fn path_segments(&self) -> Option<PathSegmentsIter<'_>> {
        Some(PathSegmentsIter {iter: self.path_segment_strs()?, r#type: self.scheme_type().into()})
    }



    /// The `index`th path segment as a [`str`].
    pub fn path_segment_str(&self, index: isize) -> Option<&str> {
        self.path_segment_strs()?.neg_nth(index)
    }

    /// The `index`th [`PathSegment`].
    pub fn path_segment(&self, index: isize) -> Option<PathSegment<'_>> {
        self.path_segments()?.neg_nth(index)
    }

    /// If [`Self::path_segment`] returns [`Some`].
    pub fn has_path_segment(&self, index: isize) -> bool {
        self.path_segment_strs().is_some_and(|mut x| x.neg_nth(index).is_some())
    }



    /// The range of path segments as a [`str`].
    pub fn path_segment_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        SplitSlashes(self.segmented_path_str()?.strip_prefix('/')).range(range)
    }

    /// The range of path segments as a [`PathSegments`].
    pub fn path_segment_range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'_>> {
        self.path_segment_range_str(range).map(|x| unsafe {PathSegments::new_unchecked(x, self.scheme_type().into())})
    }
}

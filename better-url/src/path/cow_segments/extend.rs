//! Extenders.

use crate::prelude::*;

impl BetterPathSegments<'_> {
    /// Push a new thing.
    pub fn push<T>(&mut self, x: T) where Self: Extend<T> {
        self.extend([x]);
    }

    /// Push a new raw segment.
    pub fn push_raw_segment(&mut self, segment: &str) {
        self.push(RawPathSegment::new(segment));
    }

    /// Push a new segment.
    pub fn push_segment(&mut self, segment: &str) {
        self.push(PathSegmentEncoder::new(segment));
    }

    /// Insert a new raw segment.
    /// # Errors
    /// If the call to [`Self::split`] returns [`None`], returns the error [`InsertNotFound`].
    pub fn insert_raw_segment(&mut self, index: isize, segment: &str) -> Result<(), InsertNotFound> {
        let (before, after) = self.split(index).ok_or(InsertNotFound)?;

        if after.is_some() {
            let x = before.map_or(0, str::len);
            self.0.to_mut().insert_with(x, [segment, "/"]);
        } else {
            self.push_raw_segment(segment);
        }

        Ok(())
    }

    /// Insert a new segment.
    /// # Errors
    /// If the call to [`Self::split`] returns [`None`], returns the error [`InsertNotFound`].
    pub fn insert_segment(&mut self, index: isize, segment: &str) -> Result<(), InsertNotFound> {
        let (before, after) = self.split(index).ok_or(InsertNotFound)?;

        if after.is_some() {
            let x = before.map_or(0, str::len);
            self.0.to_mut().insert_with(x, PathSegmentEncoder::new(segment).chain(["/"]));
        } else {
            self.push_segment(segment);
        }

        Ok(())
    }
}

impl<'a> Extend<RawPathSegment<'a>> for BetterPathSegments<'_> {
    fn extend<T: IntoIterator<Item = RawPathSegment<'a>>>(&mut self, iter: T) {
        self.0.to_mut().extend(iter.into_iter().flat_map(|RawPathSegment(segment)| ["/", segment]))
    }
}

impl<'a> Extend<PathSegmentEncoder<'a>> for BetterPathSegments<'_> {
    fn extend<T: IntoIterator<Item = PathSegmentEncoder<'a>>>(&mut self, iter: T) {
        self.0.to_mut().extend(iter.into_iter().flat_map(|x| std::iter::once("/").chain(x)))
    }
}

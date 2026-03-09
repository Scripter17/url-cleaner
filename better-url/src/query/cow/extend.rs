//! Extenders.

use std::iter::once;

use crate::prelude::*;

impl BetterQuery<'_> {
    /// Push a thing.
    pub fn push<T>(&mut self, x: T) where Self: Extend<T> {
        self.extend([x]);
    }

    /// Push a raw segment.
    pub fn push_raw_segment(&mut self, segment: &str) {
        self.0.to_mut().extend(["&", segment]);
    }

    /// Push a pair without encoding.
    pub fn push_raw_pair(&mut self, name: &str, value: Option<&str>) {
        match value {
            Some(value) => self.0.to_mut().extend(["&", name, "=", value]),
            None        => self.0.to_mut().extend(["&", name]),
        }
    }

    /// Push a pair with encoding.
    pub fn push_pair(&mut self, name: &str, value: Option<&str>) {
        match value {
            Some(value) => self.0.to_mut().extend(once("&").chain(QueryPartEncoder::new(name)).chain(once("=")).chain(QueryPartEncoder::new(value))),
            None        => self.0.to_mut().extend(once("&").chain(QueryPartEncoder::new(name))),
        }
    }

    /// Insert a raw segment.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_raw_segment(&mut self, index: isize, segment: &str) -> Result<(), InsertNotFound> {
        let (before, after) = self.split(index).ok_or(InsertNotFound)?;

        match (before.map_or(0, str::len), after.is_some()) {
            (_, false) => self.0.to_mut().extend     (   ["&", segment]),
            (x, true ) => self.0.to_mut().insert_with(x, [segment, "&"]),
        }

        Ok(())
    }

    /// Insert a pair without encoding.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_raw_pair(&mut self, index: isize, name: &str, value: Option<&str>) -> Result<(), InsertNotFound> {
        let (before, after) = self.split(index).ok_or(InsertNotFound)?;

        match (before.map_or(0, str::len), after.is_some(), value) {
            (_, false, None) => self.0.to_mut().extend   (   ["&", name]),
            (x, true , None) => self.0.to_mut().insert_with(x, [name, "&"]),

            (_, false, Some(value)) => self.0.to_mut().extend      (   ["&", name, "=", value]),
            (x, true , Some(value)) => self.0.to_mut().insert_with(x, [name, "=", value, "&"]),
        }

        Ok(())
    }

    /// Insert a pair with encoding.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_pair(&mut self, index: isize, name: &str, value: Option<&str>) -> Result<(), InsertNotFound> {
        let (before, after) = self.split(index).ok_or(InsertNotFound)?;

        let name = QueryPartEncoder::new(name);
        let value = value.map(QueryPartEncoder::new);

        match (before.map_or(0, str::len), after.is_some(), value) {
            (_, false, None) => self.0.to_mut().extend     (   once("&").chain(name)),
            (x, true , None) => self.0.to_mut().insert_with(x, name.chain(once("&"))),

            (_, false, Some(value)) => self.0.to_mut().extend     (   once("&").chain(name).chain(once("=")).chain(value)),
            (x, true , Some(value)) => self.0.to_mut().insert_with(x, name.chain(once("=")).chain(value).chain(once("&"))),
        }

        Ok(())
    }
}

impl<'a> Extend<RawQuerySegment<'a>> for BetterQuery<'_> {
    fn extend<T: IntoIterator<Item = RawQuerySegment<'a>>>(&mut self, iter: T) {
        for RawQuerySegment(segment) in iter {
            self.push_raw_segment(segment);
        }
    }
}

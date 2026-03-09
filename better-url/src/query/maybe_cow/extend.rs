//! Extenders.

use crate::prelude::*;

impl BetterMaybeQuery<'_> {
    /// Push a raw segment.
    pub fn push_raw_segment(&mut self, segment: &str) {
        match self.0 {
            Some(ref mut query) => query.push_raw_segment(segment),
            None => self.0 = Some(segment.to_string().into())
        }
    }

    /// Push a pair without encoding.
    pub fn push_raw_pair(&mut self, name: &str, value: Option<&str>) {
        match self.0 {
            Some(ref mut query) => query.push_raw_pair(name, value),
            None => self.0 = Some(match value {
                Some(value) => format!("{name}={value}"),
                None        => name.to_string()
            }.into())
        }
    }

    /// Push a pair with encoding.
    pub fn push_pair(&mut self, name: &str, value: Option<&str>) {
        match self.0 {
            Some(ref mut query) => query.push_pair(name, value),
            None => self.0 = Some(match value {
                Some(value) => format!("{}={}", QueryPartEncoder::new(name), QueryPartEncoder::new(value)),
                None        => QueryPartEncoder::new(name).to_string()
            }.into())
        }
    }

    /// Insert a raw segment.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_raw_segment(&mut self, index: isize, segment: &str) -> Result<(), InsertNotFound> {
        match self.0 {
            Some(ref mut query) => query.insert_raw_segment(index, segment)?,
            None => self.0 = Some(segment.to_string().into())
        }

        Ok(())
    }

    /// Insert a pair without encoding.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_raw_pair(&mut self, index: isize, name: &str, value: Option<&str>) -> Result<(), InsertNotFound>{
        match self.0 {
            Some(ref mut query) => query.insert_raw_pair(index, name, value)?,
            None => match index {
                0 | -1 => self.push_raw_pair(name, value),
                _ => Err(InsertNotFound)?
            }
        }

        Ok(())
    }

    /// Insert a pair with encoding.
    /// # Errors
    /// If the split isn't found, returns the error [`InsertNotFound`]
    pub fn insert_pair(&mut self, index: isize, name: &str, value: Option<&str>) -> Result<(), InsertNotFound>{
        match self.0 {
            Some(ref mut query) => query.insert_pair(index, name, value)?,
            None => match index {
                0 | -1 => self.push_pair(name, value),
                _ => Err(InsertNotFound)?
            }
        }

        Ok(())
    }
}

impl<'a> Extend<RawQuerySegment<'a>> for BetterMaybeQuery<'_> {
    fn extend<T: IntoIterator<Item = RawQuerySegment<'a>>>(&mut self, iter: T) {
        for RawQuerySegment(raw_segment) in iter {
            self.push_raw_segment(raw_segment);
        }
    }
}

impl<A> FromIterator<A> for BetterMaybeQuery<'_> where Self: Extend<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

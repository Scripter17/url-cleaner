//! Setters.

use std::iter::once;

use crate::prelude::*;

impl BetterQuery<'_> {
    /// Sets a the `index`th `name` query pair without encoding.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`]
    pub fn set_raw_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), SegmentNotFound> {
        let range = self.0.my_substr_range(self.find(name, index).ok_or(SegmentNotFound)?.0);

        let pair = once(name).chain(value.into_iter().flat_map(|v| ["=", v]));

        self.0.to_mut().replace_range_with(range.clone(), pair);

        Ok(())
    }

    /// Sets a the `index`th `name` query pair with encoding.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`]
    pub fn set_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), SegmentNotFound> {
        let range = self.0.my_substr_range(self.find(name, index).ok_or(SegmentNotFound)?.0);

        let pair = QueryPartEncoder::new(name).chain(value.into_iter().flat_map(|v| once("=").chain(QueryPartEncoder::new(v))));

        self.0.to_mut().replace_range_with(range.clone(), pair);

        Ok(())
    }

    /// Set ot insert the `index`th `name` query pair without encoding.
    /// # Errors
    /// If no segment or insert location is found, returns the error [`InsertNotFound`].
    pub fn set_or_insert_raw_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), InsertNotFound> {
        let temp = self.iter().filter(|segment| segment.lazy_name() == name).try_neg_nth(index).map(|segment| self.0.my_substr_range(segment.0));

        let pair = once(name).chain(value.into_iter().flat_map(|v| ["=", v]));

        match temp {
            Ok(range) => self.0.to_mut().replace_range_with(range, pair),
            Err(0) => match index {
                0.. => self.0.to_mut().extend     (   once("&").chain(pair)),
                ..0 => self.0.to_mut().insert_with(0, pair.chain(once("&"))),
            },
            Err(_) => Err(InsertNotFound)?
        }

        Ok(())
    }

    /// Set ot insert the `index`th `name` query pair with encoding.
    /// # Errors
    /// If no segment or insert location is found, returns the error [`InsertNotFound`].
    pub fn set_or_insert_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), InsertNotFound> {
        let temp = self.iter().filter(|segment| segment.lazy_name() == name).try_neg_nth(index).map(|segment| self.0.my_substr_range(segment.0));

        let pair = QueryPartEncoder::new(name).chain(value.into_iter().flat_map(|v| once("=").chain(QueryPartEncoder::new(v))));

        match temp {
            Ok(range) => {
                self.0.to_mut().replace_range(range.clone(), "");
                self.0.to_mut().insert_with(range.start, pair);
            },
            Err(0) => match index {
                0.. => self.0.to_mut().extend     (   once("&").chain(pair)),
                ..0 => self.0.to_mut().insert_with(0, pair.chain(once("&"))),
            },
            Err(_) => Err(InsertNotFound)?
        }

        Ok(())
    }

    /// Set or insert or remove the `index`th `name` query pair without encoding.
    /// # Errors
    /// If the call to [`Self::set_raw_pair`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::find_remove`] returns an error, that error is returned.
    pub fn set_or_remove_raw_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrRemoveError> {
        match value {
            Some(value) => self.set_raw_pair(name, index, value)?,
            None => self.find_remove(name, index)?
        }

        Ok(())
    }

    /// Set or insert or remove the `index`th `name` query pair with encoding.
    /// # Errors
    /// If the call to [`Self::set_pair`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::find_remove`] returns an error, that error is returned.
    pub fn set_or_remove_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrRemoveError> {
        match value {
            Some(value) => self.set_pair(name, index, value)?,
            None => self.find_remove(name, index)?
        }

        Ok(())
    }

    /// Set or insert or remove the `index`th `name` query pair without encoding.
    /// # Errors
    /// If the call to [`Self::set_or_insert_raw_pair`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::find_remove`] returns an error, that error is returned.
    pub fn set_or_insert_or_remove_raw_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrInsertOrRemoveError> {
        match value {
            Some(value) => self.set_or_insert_raw_pair(name, index, value)?,
            None => self.find_remove(name, index)?
        }

        Ok(())
    }

    /// Set or insert or remove the `index`th `name` query pair with encoding.
    /// # Errors
    /// If the call to [`Self::set_or_insert_pair`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::find_remove`] returns an error, that error is returned.
    pub fn set_or_insert_or_remove_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrInsertOrRemoveError> {
        match value {
            Some(value) => self.set_or_insert_pair(name, index, value)?,
            None => self.find_remove(name, index)?
        }

        Ok(())
    }
}

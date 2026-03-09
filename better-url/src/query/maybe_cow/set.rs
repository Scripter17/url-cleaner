//! Setters.

use crate::prelude::*;

impl BetterMaybeQuery<'_> {
    /// Sets a the `index`th `name` query pair without encoding.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`]
    pub fn set_raw_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), SegmentNotFound> {
        self.0.as_mut().ok_or(SegmentNotFound)?.set_raw_pair(name, index, value)
    }

    /// Sets a the `index`th `name` query pair with encoding.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`]
    pub fn set_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), SegmentNotFound> {
        self.0.as_mut().ok_or(SegmentNotFound)?.set_pair(name, index, value)
    }

    /// Set ot insert the `index`th `name` query pair without encoding.
    /// # Errors
    /// If no segment or insert location is found, returns the error [`InsertNotFound`].
    pub fn set_or_insert_raw_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), InsertNotFound> {
        match self.0 {
            Some(ref mut query) => query.set_or_insert_raw_pair(name, index, value)?,
            None => match index {
                0 | -1 => self.push_raw_pair(name, value),
                _ => Err(InsertNotFound)?
            }
        }

        Ok(())
    }

    /// Set ot insert the `index`th `name` query pair with encoding.
    /// # Errors
    /// If no segment or insert location is found, returns the error [`InsertNotFound`].
    pub fn set_or_insert_pair(&mut self, name: &str, index: isize, value: Option<&str>) -> Result<(), InsertNotFound> {
        match self.0 {
            Some(ref mut query) => query.set_or_insert_pair(name, index, value)?,
            None => match index {
                0 | -1 => self.push_pair(name, value),
                _ => Err(InsertNotFound)?
            }
        }

        Ok(())
    }

    /// Set or insert or remove the `index`th `name` query pair without encoding.
    /// # Errors
    /// If the call to [`Self::set_raw_pair`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::find_remove`] returns an error, that error is returned.
    pub fn set_or_remove_raw_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SegmentNotFound> {
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
    pub fn set_or_remove_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SegmentNotFound> {
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
    pub fn set_or_insert_or_remove_raw_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrInsertOrRemoveMaybeError> {
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
    pub fn set_or_insert_or_remove_pair(&mut self, name: &str, index: isize, value: Option<Option<&str>>) -> Result<(), SetOrInsertOrRemoveMaybeError> {
        match value {
            Some(value) => self.set_or_insert_pair(name, index, value)?,
            None => self.find_remove(name, index)?
        }

        Ok(())
    }
}

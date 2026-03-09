//! Setters.

use crate::prelude::*;

impl BetterPathSegments<'_> {
    /// Set the `index`th segment.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`].
    pub fn set_segment(&mut self, index: isize, value: &str) -> Result<(), SegmentNotFound> {
        let range = self.0.my_substr_range(self.get(index).ok_or(SegmentNotFound)?.as_str());

        self.0.to_mut().replace_range_with(range, PathSegmentEncoder::new(value));

        Ok(())
    }

    /// Set or remove the `index`th segment.
    /// # Errors
    /// If the segment isn't found, returns the error [`SegmentNotFound`].
    pub fn set_or_remove_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetOrRemoveError> {
        match value {
            Some(value) => self.set_segment(index, value)?,
            None        => self.remove(index)?
        }

        Ok(())
    }

    /// Set or insert the `index`th segment.
    /// # Errors
    /// If both an equivalent [`Self::set_segment`] and [`Self::insert_segment`] would return an error, returns the error [`InsertNotFound`].
    pub fn set_or_insert_segment(&mut self, index: isize, value: &str) -> Result<(), InsertNotFound> {
        let temp = self.iter().try_neg_nth(index).map(|old| self.0.my_substr_range(old.0));

        match temp {
            Ok(range) => self.0.to_mut().replace_range_with(range, PathSegmentEncoder::new(value)),
            Err(0) => match index {
                0.. => self.push_segment(value),
                ..0 => self.0.to_mut().insert_with(0, PathSegmentEncoder::new(value).chain(["/"]))
            },
            Err(_) => Err(InsertNotFound)?
        }

        Ok(())
    }

    /// If `value` is [`Some`], [`Self::set_or_insert_segment`], otherwise [`Self::remove`]
    /// # Errors
    /// If the call to [`Self::set_or_insert_segment`] returns an erorr, that error is returned.
    ///
    /// If the call to [`Self::remove`] returns an error, that error is returned.
    pub fn set_or_insert_or_remove_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetOrInsertOrRemoveError> {
        match value {
            Some(value) => self.set_or_insert_segment(index, value)?,
            None        => self.remove(index)?
        }

        Ok(())
    }
}

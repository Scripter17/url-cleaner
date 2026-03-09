//! Removers.

use std::borrow::Cow;

use crate::prelude::*;

impl BetterQuery<'_> {
    /// Removes the last segment.
    /// # Errors
    /// If there is only one segment, returns the error [`CantBeNone`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = BetterQuery::new("0&1&2&3");
    ///
    /// query.pop().unwrap    (); assert_eq!(query, "0&1&2");
    /// query.pop().unwrap    (); assert_eq!(query, "0&1");
    /// query.pop().unwrap    (); assert_eq!(query, "0");
    /// query.pop().unwrap_err(); assert_eq!(query, "0");
    /// ```
    pub fn pop(&mut self) -> Result<(), CantBeNone> {
        match &mut self.0 {
            Cow::Borrowed(x) => *x = x.rsplit_once('&').ok_or(CantBeNone)?.0,
            Cow::Owned(x) => x.truncate(x.rsplit_once('&').ok_or(CantBeNone)?.0.len())
        }

        Ok(())
    }

    /// Removes the first segment.
    /// # Errors
    /// If there is only one segment, returns the error [`CantBeNone`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = BetterQuery::new("0&1&2&3");
    ///
    /// query.shift().unwrap    (); assert_eq!(query, "1&2&3");
    /// query.shift().unwrap    (); assert_eq!(query, "2&3");
    /// query.shift().unwrap    (); assert_eq!(query, "3");
    /// query.shift().unwrap_err(); assert_eq!(query, "3");
    /// ```
    pub fn shift(&mut self) -> Result<(), CantBeNone> {
        match &mut self.0 {
            Cow::Borrowed(x) => *x = x.split_once('&').ok_or(CantBeNone)?.1,
            Cow::Owned(x) => x.drain(..=x.split_once('&').ok_or(CantBeNone)?.0.len()).for_each(drop)
        }

        Ok(())
    }

    /// # Errors
    /// TODO
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = BetterQuery::new("a&b&b&a");
    ///
    /// query.remove(1).unwrap();
    ///
    /// assert_eq!(query, "a&b&a");
    ///
    /// query.remove(2).unwrap();
    ///
    /// assert_eq!(query, "a&b");
    /// ```
    pub fn remove(&mut self, index: isize) -> Result<(), RemoveError> {
        let (before, after) = self.0.split_around_substr(self.get(index).ok_or(SegmentNotFound)?.0);

        match (before.strip_suffix("&"), after.strip_prefix("&")) {
            (None        , None       ) => Err(CantBeNone)?,
            (None        , Some(after)) => self.0.retain_substr(after),
            (Some(before), None       ) => self.0.retain_substr(before),
            (Some(before), Some(after)) => self.0 = Cow::Owned(format!("{before}&{after}"))
        }

        Ok(())
    }

    /// # Errors
    /// TODO
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = BetterQuery::new("a&b&b&a");
    ///
    /// query.find_remove("b", 0).unwrap();
    ///
    /// assert_eq!(query, "a&b&a");
    ///
    /// query.find_remove("a", 1).unwrap();
    ///
    /// assert_eq!(query, "a&b");
    /// ```
    pub fn find_remove(&mut self, name: &str, index: isize) -> Result<(), RemoveError> {
        let (before, after) = self.0.split_around_substr(self.find(name, index).ok_or(SegmentNotFound)?.0);

        match (before.strip_suffix("&"), after.strip_prefix("&")) {
            (None        , None       ) => Err(CantBeNone)?,
            (None        , Some(after)) => self.0.retain_substr(after),
            (Some(before), None       ) => self.0.retain_substr(before),
            (Some(before), Some(after)) => self.0 = Cow::Owned(format!("{before}&{after}"))
        }

        Ok(())
    }
}

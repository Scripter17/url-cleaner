//! Filterers.

use std::ops::Range;

use crate::prelude::*;

impl BetterMaybeQuery<'_> {
    /// Filters segments in place, using the same allocation if possible.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e");
    /// query.filter(|segment| segment.lazy_name() != "a");
    /// assert_eq!(query, "b=3&c&d&e");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e");
    /// query.filter(|segment| segment.lazy_name() == "a");
    /// assert_eq!(query, "a=2&a=4");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e");
    /// query.filter(|segment| segment.lazy_name() == "c");
    /// assert_eq!(query, "c");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e");
    /// query.filter(|segment| segment.lazy_name() == "f");
    /// assert_eq!(query, None::<&str>);
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e".to_string());
    /// query.filter(|segment| segment.lazy_name() != "a");
    /// assert_eq!(query, "b=3&c&d&e");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e".to_string());
    /// query.filter(|segment| segment.lazy_name() == "a");
    /// assert_eq!(query, "a=2&a=4");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e".to_string());
    /// query.filter(|segment| segment.lazy_name() == "c");
    /// assert_eq!(query, "c");
    ///
    /// let mut query = BetterMaybeQuery::from("a=2&b=3&a=4&c&d&e".to_string());
    /// query.filter(|segment| segment.lazy_name() == "f");
    /// assert_eq!(query, None::<&str>);
    /// ```
    pub fn filter<F: FnMut(RawQuerySegment<'_>) -> bool>(&mut self, mut f: F) {
        let Some(ref mut query) = self.0 else {return;};

        let mut x = Vec::new();

        for segment in query.iter() {
            if f(segment) {
                let Range {start, end} = query.0.my_substr_range(segment.as_str());

                if let Some((_, last_end)) = x.last_mut() && *last_end == start - 1 {
                    *last_end = end;
                } else {
                    x.push((start, end));
                }
            }
        }

        match &*x {
            [] => *self = Self(None),
            &[(start, end)] => query.0.retain_range(start..end),
            _ => *self = Self::from_iter(x.into_iter().map(|(start, end)| RawQuerySegment(&query.0[start..end])))
        }
    }

    /// [`Self::filter`] but consumes and returns `self`.
    pub fn filtered<F: FnMut(RawQuerySegment<'_>) -> bool>(mut self, f: F) -> Self {
        self.filter(f);
        self
    }

    /// Keep only segments for which `f` returns [`true`].
    /// # Errors
    /// If any call to `f` returns an error, that error is returned.
    pub fn try_filter<F: FnMut(RawQuerySegment<'_>) -> Result<bool, E>, E>(&mut self, mut f: F) -> Result<(), E> {
        let Some(ref mut query) = self.0 else {return Ok(());};

        let mut x = Vec::new();

        for segment in query.iter() {
            if f(segment)? {
                let Range {start, end} = query.0.my_substr_range(segment.as_str());

                if let Some((_, last_end)) = x.last_mut() && *last_end == start - 1 {
                    *last_end = end;
                } else {
                    x.push((start, end));
                }
            }
        }

        match &*x {
            [] => *self = Self(None),
            &[(start, end)] => query.0.retain_range(start..end),
            _ => *self = Self::from_iter(x.into_iter().map(|(start, end)| RawQuerySegment(&query.0[start..end])))
        }

        Ok(())
    }

    /// [`Self::try_filter`] but consumes and returns `self`.
    /// # Errors
    /// If the call to [`Self::try_filter`] returns an error, that error is returned.
    pub fn try_filtered<F: FnMut(RawQuerySegment<'_>) -> Result<bool, E>, E>(mut self, f: F) -> Result<Self, E> {
        self.try_filter(f)?;
        Ok(self)
    }
}

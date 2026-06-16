//! Setters.

use crate::prelude::*;

impl FragmentQuery<'_> {
    /// Finds the `index`th segment whose [`FragmentQuerySegment::name`] is `name` and does stuff.
    ///
    /// - If found and `value` is `Some(Some(x))`, replaces its value with `x`.
    ///
    /// - If found and `value` is `Some(None)`, removes its value.
    ///
    /// - If found and `value` is `None`, removes it.
    ///
    /// - If we're one short, `index` is `0..`, and `value` is `Some(_)`, appends a new segment at the end.
    ///
    /// - If we're one short, `index` is `..0`, and `value` is `Some(_)`, prepends a new segment to the beginning.
    /// # Errors
    /// - If we're more than one short and `value` is `Some(_)`, returns the error [`InsertNotFound`].
    ///
    /// - If not found and `value` is `None`, returns the error [`SegmentNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = FragmentQuery::new("a=1&b=2&a=3");
    ///
    /// query.set(0, "a", Some(Some("2"))).unwrap();
    /// assert_eq!(query, "a=2&b=2&a=3");
    ///
    /// query.set(0, "c", Some(Some("4"))).unwrap();
    /// assert_eq!(query, "a=2&b=2&a=3&c=4");
    ///
    /// query.set(0, "c", Some(None)).unwrap();
    /// assert_eq!(query, "a=2&b=2&a=3&c");
    ///
    /// query.set(0, "c", None).unwrap();
    /// assert_eq!(query, "a=2&b=2&a=3");
    ///
    /// query.set(-1, "c", Some(None)).unwrap();
    /// assert_eq!(query, "c&a=2&b=2&a=3");
    /// ```
    pub fn set(&mut self, index: isize, name: &str, value: Option<Option<&str>>) -> Result<bool, SetQueryError> {
        let temp = self.find_iter(name).try_neg_nth(index);

        match value.map(|value| FragmentQuerySegment::from_pair(name, value)) {
            Some(new) => match temp {
                Ok(old) if old == new => return Ok(false),
                Ok(old) => self.0.replace_substr(old.as_str(), new.as_str()),
                Err(0) => match index {
                    0.. => self.0.to_mut().extend     (    ["&", new.as_str()]),
                    ..0 => self.0.to_mut().insert_with(0, &[new.as_str(), "&"]),
                },
                Err(_) => Err(InsertNotFound)?
            },
            None => {
                let Range {start, end} = self.as_str().my_substr_range(temp.map_err(|_| SegmentNotFound)?.as_str());
                match (start == 0, end == self.len()) {
                    (true , true ) => Err(CantBeNone)?,
                    (false, _    ) => self.0.replace_range(start - 1 ..  end, ""),
                    (true , false) => self.0.replace_range(start     ..= end, ""),
                }
            }
        }

        Ok(true)
    }
}

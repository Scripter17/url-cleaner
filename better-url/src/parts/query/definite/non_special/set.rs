//! Setters.

use crate::prelude::*;

impl NonSpecialQuery<'_> {
    /// Finds the `index`th segment whose [`NonSpecialQuerySegment::name`] is `name` and does stuff.
    ///
    /// - If found and `value` is `Some(Some(x))`, replaces its value with `x`.
    ///
    /// - If found and `value` is `Some(None)`, removes its value.
    ///
    /// - If found and `value` is `None`, removes it.
    ///
    /// - If we're one short, `index` is `0..`, and `value` is [`Some`], appends a new segment at the end.
    ///
    /// - If we're one short, `index` is `..0`, and `value` is [`Some`], prepends a new segment to the beginning.
    ///
    /// - If not found and `value` is [`None`], does nothing.
    /// # Errors
    /// - If we're more than one short and `value` is `Some(_)`, returns the error [`InsertNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut query = NonSpecialQuery::new("a=1&b=2&a=3");
    ///
    /// assert!( query.set("a",  1, None::<&str>      ).unwrap()); assert_eq!(query,     "a=1&b=2"      );
    /// assert!(!query.set("a",  1, None::<&str>      ).unwrap()); assert_eq!(query,     "a=1&b=2"      );
    /// assert!( query.set("b",  1, Some(None::<&str>)).unwrap()); assert_eq!(query,     "a=1&b=2&b"    );
    /// assert!( query.set("b",  2, Some(Some("3"))   ).unwrap()); assert_eq!(query,     "a=1&b=2&b&b=3");
    /// assert!( query.set("c", -1, Some(Some("4"))   ).unwrap()); assert_eq!(query, "c=4&a=1&b=2&b&b=3");
    /// assert!( query.set("c", -1, Some(None::<&str>)).unwrap()); assert_eq!(query,   "c&a=1&b=2&b&b=3");
    /// assert!( query.set("c", -1, None::<&str>      ).unwrap()); assert_eq!(query,     "a=1&b=2&b&b=3");
    /// ```
    pub fn set<'b, T: Into<MaybeNonSpecialQueryValue<'b>>>(&mut self, name: &str, index: isize, value: Option<T>) -> Result<bool, SetQueryError> {
        Ok(match value.map(|x| x.into().into_inner()) {
            Some(Some(value)) => {
                let temp = self.find_iter(name).try_neg_nth(index);

                match temp {
                    Ok(old) => match old.value().as_str() {
                        Some(old) => self.0.replace_substr(old                                           ,       &value ),
                        None      => self.0.insert_with   (old.as_str().end_addr() - self.as_str().addr(), ["=", &value]),
                    },
                    Err(0) => {
                        let name = NonSpecialQueryName::new(name).into_inner();

                        match index {
                            0.. => self.0.extend     (   ["&", &name, "=", &value     ]),
                            ..0 => self.0.insert_with(0, [     &name, "=", &value, "&"]),
                        }
                    },
                    Err(_) => Err(InsertNotFound)?
                }

                true
            },
            Some(None) => {
                let temp = self.find_iter(name).try_neg_nth(index);

                match temp {
                    Ok(old) => match old.value().as_str() {
                        Some(old) => {
                            self.0.replace_range(old.addr() - 1 - self.0.addr() .. old.end_addr() - self.0.addr(), "");

                            true
                        },
                        None => false
                    },
                    Err(0) => {
                        let (_, name) = encode_query_part(name);

                        match index {
                            0.. => self.0.extend     (   ["&", &name]),
                            ..0 => self.0.insert_with(0, [&name, "&"]),
                        }

                        true
                    },
                    Err(_) => Err(InsertNotFound)?
                }
            },
            None => {
                let temp = self.find(name, index);

                match temp {
                    Some(temp) => {
                        let Range {start, end} = self.as_str().my_substr_range(temp.as_str());

                        match (start == 0, end == self.len()) {
                            (true , true ) => Err(CantBeNone)?,
                            (false, _    ) => self.0.replace_range(start - 1 ..  end, ""),
                            (true , false) => self.0.replace_range(start     ..= end, ""),
                        }

                        true
                    },
                    None => false
                }
            }
        })
    }
}

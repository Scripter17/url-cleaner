//! Setters.

use crate::prelude::*;

impl FileSegmentedPath<'_> {
    /// Append `value`.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FileSegmentedPath::new("/c:" ); path.push(".."); assert_eq!(path, "/c:/");
    /// let mut path = FileSegmentedPath::new("/c:/"); path.push(".."); assert_eq!(path, "/c:/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi" ); path.push("123"); assert_eq!(path, "/abc/def/ghi/123" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi" ); path.push("."  ); assert_eq!(path, "/abc/def/ghi/"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi" ); path.push(".." ); assert_eq!(path, "/abc/def/"        );
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi/"); path.push("123"); assert_eq!(path, "/abc/def/ghi//123");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi/"); path.push("."  ); assert_eq!(path, "/abc/def/ghi//"   );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi/"); path.push(".." ); assert_eq!(path, "/abc/def/"        );
    ///
    /// let mut path = FileSegmentedPath::new("/abc" ); path.push("123"); assert_eq!(path, "/abc/123" );
    /// let mut path = FileSegmentedPath::new("/abc" ); path.push("."  ); assert_eq!(path, "/abc/"    );
    /// let mut path = FileSegmentedPath::new("/abc" ); path.push(".." ); assert_eq!(path, "/"        );
    ///
    /// let mut path = FileSegmentedPath::new("/abc/"); path.push("123"); assert_eq!(path, "/abc//123");
    /// let mut path = FileSegmentedPath::new("/abc/"); path.push("."  ); assert_eq!(path, "/abc//"   );
    /// let mut path = FileSegmentedPath::new("/abc/"); path.push(".." ); assert_eq!(path, "/"        );
    ///
    /// let mut path = FileSegmentedPath::new("/"); path.push("123"); assert_eq!(path, "//123");
    /// let mut path = FileSegmentedPath::new("/"); path.push("."  ); assert_eq!(path, "//"   );
    /// let mut path = FileSegmentedPath::new("/"); path.push(".." ); assert_eq!(path, "/"    );
    /// ```
    pub fn push<'a, T: Into<FilePathSegment<'a>>>(&mut self, value: T) -> bool {
        let value = value.into();

        if value.is_dot() {
            self.0.to_mut().push('/');
        } else if value.is_double_dot() {
            match self.0.as_bytes() {
                [b'/', b'a'..=b'z' | b'A'..=b'Z', b':', b'/'] => return false,
                [b'/', b'a'..=b'z' | b'A'..=b'Z', b':'      ] => self.0.to_mut().push('/'),
                _ => match self.0.my_trim_suffix("/").rfind('/') {
                    Some(x) => self.0.retain_range(..=x),
                    None    => return false
                }
            }
        } else {
            self.0.to_mut().extend(["/", value.as_str()]);
        }

        true
    }

    /// Prepend `value`.
    pub fn prepend<'a, T: Into<FilePathSegment<'a>>>(&mut self, value: T) -> bool {
        let value = value.into();

        if value.is_dot() || value.is_double_dot() {
            false
        } else {
            self.0.to_mut().insert_with(0, ["/", value.as_str()]);
            true
        }
    }

    /// Set, insert, or remove the `index`th segment.
    /// # Errors
    /// If `value` is [`Some`] and `index` is more than 1 out of bounds, returns the error [`InsertNotFound`].
    ///
    /// If `value` is [`None`] and `index` is out of bounds, returns the error [`SegmentNotFound`].
    ///
    /// If `value` is [`None`] and `index` is the only segment, returns the error [`CantBeNone`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(0, Some("c|")).unwrap(); assert_eq!(path, "/c:/def/ghi"  );
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(0, Some("." )).unwrap(); assert_eq!(path, "/def/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(1, Some("." )).unwrap(); assert_eq!(path, "/abc/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(2, Some("." )).unwrap(); assert_eq!(path, "/abc/def/"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(3, Some("." )).unwrap(); assert_eq!(path, "/abc/def/ghi/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(0, Some("..")).unwrap(); assert_eq!(path, "/def/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(1, Some("..")).unwrap(); assert_eq!(path, "/ghi"         );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(2, Some("..")).unwrap(); assert_eq!(path, "/abc/"        );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set(3, Some("..")).unwrap(); assert_eq!(path, "/abc/def/"    );
    ///
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(0, Some("." )).unwrap(); assert_eq!(path, "/def/ghi"     );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(1, Some("." )).unwrap(); assert_eq!(path, "/c:/ghi"      );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(2, Some("." )).unwrap(); assert_eq!(path, "/c:/def/"     );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(3, Some("." )).unwrap(); assert_eq!(path, "/c:/def/ghi/" );
    ///
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(0, Some("..")).unwrap(); assert_eq!(path, "/def/ghi"     );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(1, Some("..")).unwrap(); assert_eq!(path, "/c:/ghi"      );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(2, Some("..")).unwrap(); assert_eq!(path, "/c:/"         );
    /// let mut path = FileSegmentedPath::new("/c:/def/ghi") ; path.set(3, Some("..")).unwrap(); assert_eq!(path, "/c:/def/"     );
    /// ```
    pub fn set<'a, T: Into<FilePathSegment<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        let temp = self.iter().try_neg_nth(index);

        let mut changed = false;

        let insert = match temp {
            Ok(x) => {
                let mut range = self.0.my_substr_range(x.as_str());
                range.start -= 1;

                if range.len() == self.len() && value.is_none() {
                    Err(CantBeNone)?;
                }

                self.0.replace_range(range.clone(), "");
                changed = true;
                range.start
            },
            Err(0) => {
                if value.is_none() {
                    Err(SegmentNotFound)?
                }

                match index {
                    0.. => self.len(),
                    ..0 => 0
                }
            },
            Err(_) => Err(InsertNotFound)?
        };

        if let Some(value) = value {
            changed |= insert_path_segments(self.0.to_mut(), true, insert, [value.into()]);
        }

        Ok(changed)
    }

    /// Set the range of segments.
    /// # Errors
    /// If the call to [`Self::get_range`] returns [`None`], returns the error [`RangeNotFound`].
    ///
    /// If the iterator is empty and all segments are set to be replaced, returns the error [`CantBeEmpty`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// return;
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, ["123"              ]).unwrap(); assert_eq!(path, "/123/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, ["123"              ]).unwrap(); assert_eq!(path, "/abc/123/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, ["123"              ]).unwrap(); assert_eq!(path, "/abc/def/123");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, ["123", "456", "789"]).unwrap(); assert_eq!(path, "/123/456/789/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, ["123", "456", "789"]).unwrap(); assert_eq!(path, "/abc/123/456/789/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, ["123", "456", "789"]).unwrap(); assert_eq!(path, "/abc/def/123/456/789");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, ["."                ]).unwrap(); assert_eq!(path, "/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, ["."                ]).unwrap(); assert_eq!(path, "/abc/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, ["."                ]).unwrap(); assert_eq!(path, "/abc/def/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, [".."               ]).unwrap(); assert_eq!(path, "/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, [".."               ]).unwrap(); assert_eq!(path, "/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, [".."               ]).unwrap(); assert_eq!(path, "/abc/"    );
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, ["..", ".."         ]).unwrap(); assert_eq!(path, "/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, ["..", ".."         ]).unwrap(); assert_eq!(path, "/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, ["..", ".."         ]).unwrap(); assert_eq!(path, "/"        );
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(0..=0, ["..", "123", ".."  ]).unwrap(); assert_eq!(path, "/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(1..=1, ["..", "123", ".."  ]).unwrap(); assert_eq!(path, "/ghi"     );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.set_segments(2..=2, ["..", "123", ".."  ]).unwrap(); assert_eq!(path, "/abc/"    );
    /// ```
    pub fn set_segments<'a, T: Into<FilePathSegment<'a>>, I: IntoIterator<Item = T>, B: RangeBounds<isize>>(&mut self, range: B, iter: I) -> Result<bool, SetPathError> {
        let mut range = self.as_str().my_substr_range(self.get_range(range).ok_or(RangeNotFound)?.as_str());
        range.start -= 1;

        let mut iter = iter.into_iter().map(Into::into).peekable();

        if range.len() == self.len() && iter.peek().is_none() {
            Err(CantBeEmpty)?;
        }

        let path = self.0.to_mut();

        path.replace_range(range.clone(), "");

        insert_path_segments(path, true, range.start, iter);

        Ok(true)
    }

    /// Insert a new `index`th segment.
    /// # Errors
    /// If the insert isn't found, returns the error [`InsertNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FileSegmentedPath::new("/"); path.insert(0, "." ); assert_eq!(path, "/");
    /// let mut path = FileSegmentedPath::new("/"); path.insert(1, "." ); assert_eq!(path, "//");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(0, "." ); assert_eq!(path, "/abc/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(1, "." ); assert_eq!(path, "/abc/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(2, "." ); assert_eq!(path, "/abc/def/ghi" );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(3, "." ); assert_eq!(path, "/abc/def/ghi/");
    ///
    ///
    ///
    /// let mut path = FileSegmentedPath::new("/"); path.insert(0, ".." ); assert_eq!(path, "/");
    /// let mut path = FileSegmentedPath::new("/"); path.insert(1, ".." ); assert_eq!(path, "/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(0, ".."); assert_eq!(path, "/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(1, ".."); assert_eq!(path, "/def/ghi"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(2, ".."); assert_eq!(path, "/abc/ghi"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(3, ".."); assert_eq!(path, "/abc/def/"   );
    ///
    ///
    ///
    /// let mut path = FileSegmentedPath::new("/"); path.insert(-1, ".." ); assert_eq!(path, "/");
    /// let mut path = FileSegmentedPath::new("/"); path.insert(-2, ".." ); assert_eq!(path, "/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(-1, ".."); assert_eq!(path, "/abc/def/"   );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(-2, ".."); assert_eq!(path, "/abc/ghi"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(-3, ".."); assert_eq!(path, "/def/ghi"    );
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert(-4, ".."); assert_eq!(path, "/abc/def/ghi");
    /// ```
    pub fn insert<'a, T: Into<FilePathSegment<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        self.insert_segments(index, [value])
    }

    /// Insert new segments starting at `index`.
    /// # Errors
    /// If the insert isn't found, returns the error [`InsertNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(0, ["123", "456", "789"]); assert_eq!(path, "/123/456/789/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(1, ["123", "456", "789"]); assert_eq!(path, "/abc/123/456/789/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(2, ["123", "456", "789"]); assert_eq!(path, "/abc/def/123/456/789/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(3, ["123", "456", "789"]); assert_eq!(path, "/abc/def/ghi/123/456/789");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(0, ["."]); assert_eq!(path, "/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(1, ["."]); assert_eq!(path, "/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(2, ["."]); assert_eq!(path, "/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(3, ["."]); assert_eq!(path, "/abc/def/ghi/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(0, [".."]); assert_eq!(path, "/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(1, [".."]); assert_eq!(path, "/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(2, [".."]); assert_eq!(path, "/abc/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(3, [".."]); assert_eq!(path, "/abc/def/");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(0, ["123", ".", "789"]); assert_eq!(path, "/123/789/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(1, ["123", ".", "789"]); assert_eq!(path, "/abc/123/789/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(2, ["123", ".", "789"]); assert_eq!(path, "/abc/def/123/789/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(3, ["123", ".", "789"]); assert_eq!(path, "/abc/def/ghi/123/789");
    ///
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(0, ["123", "..", "789"]); assert_eq!(path, "/789/abc/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(1, ["123", "..", "789"]); assert_eq!(path, "/abc/789/def/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(2, ["123", "..", "789"]); assert_eq!(path, "/abc/def/789/ghi");
    /// let mut path = FileSegmentedPath::new("/abc/def/ghi"); path.insert_segments(3, ["123", "..", "789"]); assert_eq!(path, "/abc/def/ghi/789");
    /// ```
    pub fn insert_segments<'a, T: Into<FilePathSegment<'a>>, I: IntoIterator<Item = T>>(&mut self, index: isize, iter: I) -> Result<bool, SetPathError> {
        let path = self.0.to_mut();

        let mut changed = false;

        let insert = match (path[1..].split('/').try_neg_nth(index), index) {
            (Ok(x) , 0..) => x.addr    () - path.addr() - 1,
            (Ok(x) , ..0) => x.end_addr() - path.addr(),
            (Err(0), 0..) => path.len(),
            (Err(0), ..0) => 0,
            (Err(_), _  ) => Err(InsertNotFound)?
        };

        changed |= insert_path_segments(path, true, insert, iter.into_iter().map(Into::into));

        Ok(changed)
    }
}

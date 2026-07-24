//! Setters.

use crate::prelude::*;

impl FilePath<'_> {
    /// [`resolve_file_path_range`].
    fn resolve_range<B: RangeBounds<usize>>(&mut self, range: B) -> bool {
        let mut temp = "/".into();

        std::mem::swap(&mut self.0, &mut temp);

        let (changed, mut temp) = resolve_file_path_range(temp, range);

        std::mem::swap(&mut self.0, &mut temp);

        changed
    }

    /// Append `value`.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FilePath::new("/abc/def/ghi");
    ///
    /// path.push("123/456"); assert_eq!(path, "/abc/def/ghi/123/456");
    /// path.push(".."     ); assert_eq!(path, "/abc/def/ghi/123/"   );
    /// path.push(".."     ); assert_eq!(path, "/abc/def/ghi/123/"   );
    /// path.push("../.."  ); assert_eq!(path, "/abc/def/ghi/"       );
    /// path.push("."      ); assert_eq!(path, "/abc/def/ghi//"      );
    /// ```
    pub fn push<'a, T: Into<FilePathSegments<'a>>>(&mut self, value: T) -> bool {
        let start = self.len();

        self.0.extend(["/", value.into().as_str()]);

        let _ = self.resolve_range(start..);

        true
    }

    /// Prepend `value`.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FilePath::new("/abc/def/ghi");
    ///
    /// path.prepend("123"); assert_eq!(path, "/123/abc/def/ghi");
    /// path.prepend("."  ); assert_eq!(path, "/123/abc/def/ghi");
    /// path.prepend(".." ); assert_eq!(path, "/123/abc/def/ghi");
    /// ```
    pub fn prepend<'a, T: Into<FilePathSegments<'a>>>(&mut self, value: T) -> bool {
        let new = value.into();

        self.0.insert_with(0, ["/", new.as_str()]);

        let _ = self.resolve_range(.. 1 + new.len());

        true
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
    /// let mut path = FilePath::new("/c:/abc/def/ghi");
    ///
    /// path.set( 1, Some("../..")).unwrap(); assert_eq!(path, "/c:/def/ghi"    );
    /// path.set( 0, Some("e|"   )).unwrap(); assert_eq!(path, "/e:/def/ghi"    );
    /// path.set(-4, Some("123"  )).unwrap(); assert_eq!(path, "/123/e:/def/ghi");
    /// path.set( 2, Some(".."   )).unwrap(); assert_eq!(path, "/123/ghi"       );
    /// path.set( 2, Some("456"  )).unwrap(); assert_eq!(path, "/123/ghi/456"   );
    /// ```
    pub fn set<'a, T: Into<FilePathSegments<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        Ok(match value.map(Into::into) {
            Some(new) => match (self.iter_strs().try_neg_nth(index), index) {
                (Ok(old), _) => {
                    let start = old.addr() - self.as_str().addr() - 1;

                    self.0.replace_substr(old, new.as_str());

                    let _ = self.resolve_range(start .. start + 1 + new.len());

                    true
                },

                (Err(0), 0..) => self.push   (new),
                (Err(0), ..0) => self.prepend(new),

                (Err(_), _) => Err(InsertNotFound)?,
            },
            None => {
                let Range {start, end} = self.0.my_substr_range(self.iter_strs().neg_nth(index).ok_or(SegmentNotFound)?);

                self.0.replace_range(start - 1 .. end, "");

                true
            }
        })
    }

    /// Set the range of segments.
    /// # Errors
    /// If the call to [`Self::range`] returns [`None`], returns the error [`RangeNotFound`].
    ///
    /// If the iterator is empty and all segments are set to be replaced, returns the error [`CantBeEmpty`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FilePath::new("/c:/abc/def/ghi");
    ///
    /// path.set_range(1..2, Some(".././123/.")).unwrap(); assert_eq!(path, "/c:/123/def/ghi");
    /// ```
    pub fn set_range<'a, T: Into<FilePathSegments<'a>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetPathError> {
        Ok(match value.map(Into::into) {
            Some(new) => {
                let old = self.range_str(range).ok_or(RangeNotFound)?;

                let start = old.addr() - self.as_str().addr() - 1;

                self.0.replace_substr(old, new.as_str());

                let _ = self.resolve_range(start .. start + 1 + new.len());

                true
            },
            None => {
                let old = self.range_str(range).ok_or(RangeNotFound)?;

                let Range {start, end} = self.0.my_substr_range(old);

                self.0.replace_range(start - 1 .. end, "");

                true
            }
        })
    }

    /// Insert a new `index`th segment.
    /// # Errors
    /// If the insert isn't found, returns the error [`InsertNotFound`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut path = FilePath::new("/c:/abc/def/ghi");
    ///
    /// ```
    pub fn insert<'a, T: Into<FilePathSegments<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        let new = value.into();

        let i = match (self.iter_strs().try_neg_nth(index), index) {
            (Ok (x), _  ) => x.addr() - self.0.addr() - 1,
            (Err(0), 0..) => self.len(),
            (Err(0), ..0) => 0,
            (Err(_), _  ) => Err(InsertNotFound)?
        };

        self.0.insert_with(i, ["/", new.as_str()]);

        let _ = self.resolve_range(i .. i + 1 + new.len());

        Ok(true)
    }
}

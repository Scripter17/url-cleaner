//! Prefix stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// If it has a prefix.
    pub fn has_prefix(&self) -> bool {
        self.details.ms != 0
    }



    /// The [`Range::start`] of the prefix.
    fn prefix_start(&self) -> Option<usize> {
        match self.details.ms {
            0 => None,
            _ => Some(0)
        }
    }

    /// The [`Range::end`] of the prefix.
    fn prefix_after(&self) -> Option<usize> {
        match self.details.ms {
            0 => None,
            x => Some(x as usize - 1)
        }
    }

    /// The [`Range`] of the prefix.
    pub(crate) fn prefix_thing(&self) -> Option<Range<usize>> {
        Some(self.prefix_start()? .. self.prefix_after()?)
    }



    /// The prefix as a [`str`].
    pub fn prefix_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.prefix_thing()?)})
    }

    /// The prefix as a [`DomainSegments`].
    pub fn prefix(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments(self.prefix_str()?.into()))
    }



    /// The prefix segments as [`str`]s.
    pub fn prefix_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.prefix_str()?)))
    }

    /// The prefix segments as [`DomainSegment`]s.
    pub fn prefix_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.prefix_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th prefix segment as a [`str`].
    pub fn prefix_segment_str(&self, index: isize) -> Option<&str> {
        self.prefix_segment_strs()?.neg_nth(index)
    }

    /// The `index`th prefix segment as a [`DomainSegment`].
    pub fn prefix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.prefix_segments()?.neg_nth(index)
    }



    /// The range of prefix segments as a [`str`].
    pub fn prefix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.prefix_segments()?.range_str(range)
    }

    /// The range of prefix segments as a [`DomainSegments`].
    pub fn prefix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.prefix_segments()?.range(range)
    }





    /// Set the prefix.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_prefix<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match (self.prefix_str(), value.map(TryInto::try_into).transpose()?) {
            (None     , None     )               => return Ok(false),
            (Some(old), Some(new)) if old == new => return Ok(false),

            (None     , Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,
            (Some(old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,

            (None, Some(new)) => {
                self.host.insert_with(0, [new.as_str(), "."]);

                self.details.ms += new.len() as u32 + 1;
                self.details.ss += new.len() as u32 + 1;
                self.details.wp  = new == "www";
            },

            (Some(old), None) => {
                let ol = old.len();

                self.host.retain_range(ol + 1..);

                self.details.ms -= ol as u32 + 1;
                self.details.ss -= ol as u32 + 1;
                self.details.wp  = false;
            },

            (Some(old), Some(new)) => {
                let ol = old.len();

                self.host.replace_range(..ol, new.as_str());

                self.details.ms = self.details.ms - ol as u32 + new.len() as u32;
                self.details.ss = self.details.ss - ol as u32 + new.len() as u32;
                self.details.wp = new == "www";
            }
        }

        Ok(true)
    }

    /// Set or insert the `index`th prefix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.com").unwrap();
    ///
    /// domain.set_prefix_segment( 0, Some("www") ).unwrap(); assert_eq!(domain,         "www.example.com");
    /// domain.set_prefix_segment( 0, Some("abc") ).unwrap(); assert_eq!(domain,         "abc.example.com");
    /// domain.set_prefix_segment( 1, Some("def") ).unwrap(); assert_eq!(domain,     "abc.def.example.com");
    /// domain.set_prefix_segment(-3, Some("123") ).unwrap(); assert_eq!(domain, "123.abc.def.example.com");
    /// domain.set_prefix_segment(-3, None::<&str>).unwrap(); assert_eq!(domain,     "abc.def.example.com");
    /// domain.set_prefix_segment(-1, None::<&str>).unwrap(); assert_eq!(domain,         "abc.example.com");
    /// domain.set_prefix_segment(-1, None::<&str>).unwrap(); assert_eq!(domain,             "example.com");
    /// ```
    pub fn set_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let temp = self.prefix_segments().into_iter().flatten().try_neg_nth(index);

        match (temp, value.map(TryInto::try_into).transpose()?) {
            (Ok (old), Some(new)) if old == new => return Ok(false),
            (Err(0  ), None     )               => return Ok(false),

            (Ok (old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,
            (Err(0  ), Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (Err(1..), Some(_)) => Err(InsertNotFound)?,
            (Err(_  ), None   ) => Err(SegmentNotFound)?,

            (Ok(old), Some(new)) => {
                let range = self.as_str().my_substr_range(old.as_str());

                self.host.replace_range(range.clone(), new.as_str());

                self.details.ms = self.details.ms - range.len() as u32 + new.len() as u32;
                self.details.ss = self.details.ss - range.len() as u32 + new.len() as u32;
            },

            (Ok(old), None) => {
                let mut range = self.host.my_substr_range(old.as_str());
                range.end += 1;

                self.host.replace_range(range.clone(), "");

                self.details.ms -= range.len() as u32;
                self.details.ss -= range.len() as u32;
            },

            (Err(0), Some(new)) => {
                let ms = self.middle_start().ok_or(InsertNotFound)?;

                match index {
                    0.. => self.host.insert_with(ms, [new.as_str(), "."]),
                    ..0 => self.host.insert_with(0 , [new.as_str(), "."]),
                }

                self.details.ms = self.details.ms + new.len() as u32 + 1;
                self.details.ss = self.details.ss + new.len() as u32 + 1;
            },
        }

        self.details.wp = self.prefix_str() == Some("www");

        Ok(true)
    }

    /// Set the `range` prefix segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_prefix_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.prefix_range(range).ok_or(RangeNotFound)?;
        let new = value.map(TryInto::try_into).transpose()?;

        match new {
            Some(new) if old == new => return Ok(false),

            Some(new) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            Some(new) => {
                let range = self.as_str().my_substr_range(old.as_str());

                self.host.replace_range(range.clone(), new.as_str());

                self.details.ms = self.details.ms - range.len() as u32 + new.len() as u32;
                self.details.ss = self.details.ss - range.len() as u32 + new.len() as u32;
            },
            None => {
                let mut range = self.as_str().my_substr_range(old.as_str());
                range.end += 1;

                self.host.replace_range(range.clone(), "");

                self.details.ms -= range.len() as u32;
                self.details.ss -= range.len() as u32;
            }
        }

        self.details.wp = self.prefix_str() == Some("www");

        Ok(true)
    }

    /// Insert a new `index`th prefix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.com").unwrap();
    ///
    /// domain.insert_prefix_segment( 0, "www").unwrap(); assert_eq!(domain,                 "www.example.com");
    /// domain.insert_prefix_segment( 0, "abc").unwrap(); assert_eq!(domain,             "abc.www.example.com");
    /// domain.insert_prefix_segment( 2, "def").unwrap(); assert_eq!(domain,         "abc.www.def.example.com");
    /// domain.insert_prefix_segment(-4, "ghi").unwrap(); assert_eq!(domain,     "ghi.abc.www.def.example.com");
    /// domain.insert_prefix_segment(-4, "jkl").unwrap(); assert_eq!(domain, "ghi.jkl.abc.www.def.example.com");
    /// ```
    pub fn insert_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        let ms = self.middle_start().ok_or(InsertNotFound)?;

        let new = value.try_into()?;

        if self.len() + new.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let temp = self.prefix_segments().into_iter().flatten().try_neg_nth(index).map(|x| self.as_str().my_substr_range(x.as_str()));

        match (temp, index) {
            (Ok(Range {start, ..}), 0..) => self.host.insert_with(start, [new.as_str(), "."]),
            (Ok(Range {end  , ..}), ..0) => self.host.insert_with(end  , [".", new.as_str()]),
            (Err(0)               , 0..) => self.host.insert_with(ms   , [new.as_str(), "."]),
            (Err(0)               , ..0) => self.host.insert_with(0    , [new.as_str(), "."]),
            _ => Err(InsertNotFound)?
        }

        self.details.ms += new.len() as u32 + 1;
        self.details.ss += new.len() as u32 + 1;

        self.details.wp = unsafe {self.host.get_unchecked(..ms)} == "www.";

        Ok(())
    }
}

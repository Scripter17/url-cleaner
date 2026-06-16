//! Prefix stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainPartsDetails::has_prefix`].
    pub fn has_prefix(&self) -> bool {
        self.details.parts.has_prefix()
    }



    /// The prefix as a [`str`].
    pub fn prefix_str(&self) -> Option<&str> {
        self.details.parts.prefix_range().map(|r| &self.host[r])
    }

    /// The [`BidiDetailsIter`] for the prefix.
    pub fn prefix_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        self.details.bidi.urange(self.details.prefix_segments_urange()?)
    }

    /// The prefix as a [`DomainSegments`].
    pub fn prefix(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments {
            segments    : self.prefix_str         ()?.into(),
            bidi_details: self.prefix_bidi_details()?.into(),
        })
    }



    /// The prefix segments as [`str`]s.
    pub fn prefix_segment_strs(&self) -> Option<std::str::Split<'_, char>> {
        Some(self.prefix_str()?.split('.'))
    }

    /// The prefix segments as [`DomainSegment`]s.
    pub fn prefix_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        Some(DomainSegmentsIter {
            segments    : self.prefix_segment_strs()?,
            bidi_details: self.prefix_bidi_details()?,
        })
    }



    /// The `index`th prefix segment as a [`str`].
    pub fn prefix_segment_str(&self, index: isize) -> Option<&str> {
        self.prefix_segment_strs()?.neg_nth(index)
    }

    /// The `index`th [`BidiDetail`] of the prefix.
    pub fn prefix_segment_bidi_detail(&self, index: isize) -> Option<BidiDetail> {
        self.prefix_bidi_details()?.neg_nth(index)
    }

    /// The `index`th prefix segment as a [`DomainSegment`].
    pub fn prefix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.prefix_segments()?.neg_nth(index)
    }



    /// The range of prefix segments as a [`str`].
    pub fn prefix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        segments_range_thing(self.prefix_str()?, '.', range)
    }

    /// The [`BidiDetailsIter::subrange`] of the prefix.
    pub fn prefix_range_bidi_details<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.prefix_bidi_details()?.subrange(range)
    }

    /// The range of prefix segments as a [`DomainSegments`].
    pub fn prefix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments {
            segments    : self.prefix_range_str         (range)?.into(),
            bidi_details: self.prefix_range_bidi_details(range)?.into(),
        })
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
                self.host.to_mut().insert_with(0, &[new.as_str(), "."]);

                self.details.parts.ms += new.len() as u32 + 1;
                self.details.parts.ss += new.len() as u32 + 1;
                self.details.parts.sa += new.len() as u32 + 1;
                self.details.parts.wp  = new == "www";
            },

            (Some(old), None) => {
                let ol = old.len();

                self.host.retain_range(ol + 1..);

                self.details.parts.ms -= ol as u32 + 1;
                self.details.parts.ss -= ol as u32 + 1;
                self.details.parts.sa -= ol as u32 + 1;
                self.details.parts.wp  = false;
            },

            (Some(old), Some(new)) => {
                let ol = old.len();

                self.host.replace_range(..ol, new.as_str());

                self.details.parts.ms = self.details.parts.ms - ol as u32 + new.len() as u32;
                self.details.parts.ss = self.details.parts.ss - ol as u32 + new.len() as u32;
                self.details.parts.sa = self.details.parts.sa - ol as u32 + new.len() as u32;
                self.details.parts.wp = new == "www";
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
    /// domain.set_prefix_segment( 0, Some("1bc") ).unwrap(); assert_eq!(domain,         "1bc.example.com");
    /// domain.set_prefix_segment( 1, Some("def") ).unwrap(); assert_eq!(domain,     "1bc.def.example.com");
    /// domain.set_prefix_segment(-3, Some("123") ).unwrap(); assert_eq!(domain, "123.1bc.def.example.com");
    /// domain.set_prefix_segment(-3, None::<&str>).unwrap(); assert_eq!(domain,     "1bc.def.example.com");
    /// domain.set_prefix_segment(-1, None::<&str>).unwrap(); assert_eq!(domain,         "1bc.example.com");
    /// domain.set_prefix_segment(-1, None::<&str>).unwrap(); assert_eq!(domain,             "example.com");
    /// ```
    pub fn set_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let bidi_range = self.prefix_bidi_details().ok_or(InsertNotFound)?.set_urange(index).ok_or(RangeNotFound)?;
        let old = self.prefix_segment_strs().into_iter().flatten().try_neg_nth(index);
        let new = value.map(TryInto::try_into).transpose()?;

        match (old, new) {
            (Ok (old), Some(new)) if old == new => return Ok(false),
            (Err(0  ), None     )               => return Ok(false),

            (Ok (old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,
            (Err(0  ), Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (Err(1..), Some(_)) => Err(InsertNotFound)?,
            (Err(_  ), None   ) => Err(SegmentNotFound)?,

            (Ok(old), Some(new)) => {
                let range = self.as_str().my_substr_range(old);

                self.details.bidi.set_urange(bidi_range, &new.bidi_details)?;

                self.host.replace_range(range.clone(), new.as_str());

                self.details.parts.ms = self.details.parts.ms - range.len() as u32 + new.len() as u32;
                self.details.parts.ss = self.details.parts.ss - range.len() as u32 + new.len() as u32;
                self.details.parts.sa = self.details.parts.sa - range.len() as u32 + new.len() as u32;
            },

            (Ok(old), None) => {
                let mut range = self.host.my_substr_range(old);
                range.end += 1;

                self.details.bidi.set_urange(bidi_range, &Default::default())?;

                self.host.replace_range(range.clone(), "");

                self.details.parts.ms -= range.len() as u32;
                self.details.parts.ss -= range.len() as u32;
                self.details.parts.sa -= range.len() as u32;
            },

            (Err(0), Some(new)) => {
                let ms = self.details.parts.middle_start().ok_or(InsertNotFound)?;

                self.details.bidi.set_urange(bidi_range, &new.bidi_details)?;

                match index {
                    0.. => self.host.to_mut().insert_with(ms, &[new.as_str(), "."]),
                    ..0 => self.host.to_mut().insert_with(0 , &[new.as_str(), "."]),
                }

                self.details.parts.ms = self.details.parts.ms + new.len() as u32 + 1;
                self.details.parts.ss = self.details.parts.ss + new.len() as u32 + 1;
                self.details.parts.sa = self.details.parts.sa + new.len() as u32 + 1;
            },
        }

        self.details.parts.wp = self.prefix_str() == Some("www");

        Ok(true)
    }

    /// Set the `range` prefix segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_prefix_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        let old = self.prefix_range_str(range).ok_or(RangeNotFound)?;
        let new = value.map(TryInto::try_into).transpose()?;

        let bidi_range = self.prefix_bidi_details().ok_or(InsertNotFound)?.subrange(range).ok_or(InsertNotFound)?.range;

        match new {
            Some(new) if old == new => return Ok(false),

            Some(new) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            Some(new) => {
                let range = self.as_str().my_substr_range(old);

                self.details.bidi.set_urange(bidi_range, &new.bidi_details)?;

                self.host.replace_range(range.clone(), new.as_str());

                self.details.parts.ms = self.details.parts.ms - range.len() as u32 + new.len() as u32;
                self.details.parts.ss = self.details.parts.ss - range.len() as u32 + new.len() as u32;
                self.details.parts.sa = self.details.parts.sa - range.len() as u32 + new.len() as u32;
            },
            None => {
                let mut range = self.as_str().my_substr_range(old);
                range.end += 1;

                self.details.bidi.set_urange(bidi_range, &Default::default())?;

                self.host.replace_range(range.clone(), "");

                self.details.parts.ms -= range.len() as u32;
                self.details.parts.ss -= range.len() as u32;
                self.details.parts.sa -= range.len() as u32;
            }
        }

        self.details.parts.wp = self.prefix_str() == Some("www");

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
    ///
    /// assert_eq!(domain.bidi_details().iter().len(), 7);
    /// ```
    pub fn insert_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        let prefix_bidi = self.prefix_bidi_details().ok_or(InsertNotFound)?;
        let thing = thing2(index, prefix_bidi.len()).ok_or(InsertNotFound)?;

        let new = value.try_into()?;

        if self.len() + new.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let bidi_urange = prefix_bidi.insert_urange(index).ok_or(InsertNotFound)?;

        let i = match thing {
            Thing2::New | Thing2::Prepend => 0,
            Thing2::Append                => self.details.parts.ms as usize,
            Thing2::Insert(i)             => self.prefix_segment_strs().expect("???").nth(i).expect("???").addr() - self.host.addr()
        };

        self.details.bidi.set_urange(bidi_urange, &new.bidi_details)?;

        self.host.to_mut().insert_with(i, &[new.as_str(), "."]);

        self.details.parts.ms += new.len() as u32 + 1;
        self.details.parts.ss += new.len() as u32 + 1;
        self.details.parts.sa += new.len() as u32 + 1;

        self.details.parts.wp = &self.host[..self.details.parts.ms as usize] == "www.";

        Ok(())
    }
}

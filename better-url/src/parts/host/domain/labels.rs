//! Labels stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainPartsDetails::has_labels`].
    pub fn has_labels(&self) -> bool {
        self.details.parts.has_labels()
    }



    /// The labels as a [`str`].
    pub fn labels_str(&self) -> &str {
        &self.host[self.details.parts.labels_range()]
    }

    /// The [`BidiDetailsIter`] of the labels.
    pub fn labels_bidi_details(&self) -> BidiDetailsIter<'_> {
        self.details.labels_bidi_details()
    }

    /// The labels as a [`DomainSegments`].
    pub fn labels(&self) -> DomainSegments<'_> {
        DomainSegments {
            segments    : self.labels_str         ().into(),
            bidi_details: self.labels_bidi_details().into(),
        }
    }



    /// The labels segments as [`str`]s.
    pub fn labels_segment_strs(&self) -> std::str::Split<'_, char> {
        self.labels_str().split('.')
    }

    /// The labels segments as [`DomainSegment`]s.
    pub fn labels_segments(&self) -> DomainSegmentsIter<'_> {
        DomainSegmentsIter {
            segments    : self.labels_segment_strs(),
            bidi_details: self.labels_bidi_details(),
        }
    }



    /// The `index`th labels segment as a [`str`].
    pub fn labels_segment_str(&self, index: isize) -> Option<&str> {
        self.labels_segment_strs().neg_nth(index)
    }

    /// The `index`th [`BidiDetail`] of the labels.
    pub fn labels_segment_bidi_detail(&self, index: isize) -> Option<BidiDetail> {
        self.labels_bidi_details().neg_nth(index)
    }

    /// The `index`th labels segment as a [`DomainSegment`].
    pub fn labels_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.labels_segments().neg_nth(index)
    }



    /// The range of labels segments as a [`str`].
    pub fn labels_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        segments_range_thing(self.labels_str(), '.', range)
    }

    /// The [`BidiDetailsIter::subrange`] of the labels.
    pub fn labels_range_bidi_details<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.labels_bidi_details().subrange(range)
    }

    /// The range of labels segments as a [`DomainSegments`].
    pub fn labels_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments {
            segments    : self.labels_range_str         (range)?.into(),
            bidi_details: self.labels_range_bidi_details(range)?.into(),
        })
    }



    /// Set the labels.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_labels<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: T) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match (self.labels(), value.try_into()?) {
            (old, new) if old == new => return Ok(false),

            (old, new) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            (_  , new) if new.     is_empty() && !self.is_fqdn() => Err(CantBeEmpty)?,
            (_  , new) if new.last_is_empty() && !self.is_fqdn() => Err(NonFqdnCantEndInEmpty)?,

            (_  , new) if (self.is_fqdn() && new.last_is_a_number()) || (!self.is_fqdn() && new.ends_in_a_number()) => Err(CantEndInANumber)?,

            (old, new) => self.host.replace_substr(old.as_str(), new.as_str()),
        }

        self.details.parts = DomainPartsDetails::from_raw_unchecked(&self.host);

        Ok(true)
    }

    /// Set or insert the `index`th labels segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::new("www.example.com").unwrap();
    ///
    /// domain.set_labels_segment(0, None::<&str>).unwrap(); assert_eq!(domain, "example.com");
    /// domain.set_labels_segment(1, None::<&str>).unwrap(); assert_eq!(domain, "example"    );
    ///
    /// domain.set_labels_segment(-1, Some("")    ).unwrap_err();
    /// domain.set_labels_segment(-1, Some("abc.")).unwrap_err();
    ///
    ///
    /// DomainHost::new(  "com.").unwrap().set_labels_segment( 0, None::<&str>).unwrap_err();
    /// DomainHost::new( ".com" ).unwrap().set_labels_segment(-1, None::<&str>).unwrap_err();
    /// DomainHost::new("..com" ).unwrap().set_labels_segment(-1, None::<&str>).unwrap_err();
    /// ```
    pub fn set_labels_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.segments().try_neg_nth(index);
        let new = value.map(TryInto::try_into).transpose()?;

        match (old, new) {
            (Ok (old), Some(new)) if old == new => return Ok(false),

            (Ok (old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,
            (Err(0  ), Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (Err(1..), Some(_)) => Err(InsertNotFound )?,
            (Err(_  ), None   ) => Err(SegmentNotFound)?,

            (Ok(old), Some(new)) => match self.host.split_around_substr(old.as_str()) {
                ("", "" ) if new.is_empty()         => Err(CantBeEmpty)?,
                (_ , "" ) if new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
                (_ , "" ) if new.ends_in_a_number() => Err(CantEndInANumber)?,
                (_ , ".") if new.last_is_a_number() => Err(CantEndInANumber)?,

                _ => self.host.replace_substr(old.as_str(), new.as_str()),
            },

            (Ok(old), None) => match self.host.split_around_substr(old.as_str()) {
                ("", "" | ".")                        => Err(CantBeEmpty)?,
                (x , ""      ) if ends_in_empty   (x) => Err(NonFqdnCantEndInEmpty)?,
                (x , ""      ) if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , "."     ) if last_is_a_number(x) => Err(CantEndInANumber)?,

                (x , "" | ".") => self.host.replace_range(x.len() - 1 ..                     , ""),
                (x , _       ) => self.host.replace_range(x.len()     ..= x.len() + old.len(), ""),
            },

            (Err(0), Some(new)) => match index {
                0.. if (!self.is_fqdn() || !new.last_is_empty()) && new.ends_in_a_number() => Err(CantEndInANumber)?,

                0.. => self.host.to_mut().insert_with(self.details.parts.suffix_after(), &[".", new.as_str()]),
                ..0 => self.host.to_mut().insert_with(0                          , &[new.as_str(), "."]),
            },
        }

        self.details.parts = DomainPartsDetails::from_raw_unchecked(&self.host);

        Ok(true)
    }

    /// Set the `range` labels segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::new("www.example.com").unwrap();
    ///
    /// domain.set_labels_range(0..2, None::<&str>).unwrap    (); assert_eq!(domain, "com");
    /// domain.set_labels_range(0..2, Some("aaa") ).unwrap_err(); assert_eq!(domain, "com");
    /// domain.set_labels_range(0..1, None::<&str>).unwrap_err(); assert_eq!(domain, "com");
    ///
    /// DomainHost::new(  "com.").unwrap().set_labels_range( 0..= 0, None::<&str>).unwrap_err();
    /// DomainHost::new( ".com" ).unwrap().set_labels_range(-1..=-1, None::<&str>).unwrap_err();
    /// DomainHost::new("..com" ).unwrap().set_labels_range(-1..=-1, None::<&str>).unwrap_err();
    /// ```
    pub fn set_labels_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.labels_range(range).ok_or(RangeNotFound)?;
        let new = value.map(TryInto::try_into).transpose()?;

        match new {
            Some(new) if old == new => return Ok(false),

            Some(new) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            Some(new) => match self.host.split_around_substr(old.as_str()) {
                ("", "" ) if new.is_empty()         => Err(CantBeEmpty)?,
                (_ , "" ) if new.last_is_empty()    => Err(NonFqdnCantEndInEmpty)?,
                (_ , "" ) if new.ends_in_a_number() => Err(CantEndInANumber)?,
                (_ , ".") if new.last_is_a_number() => Err(CantEndInANumber)?,

                _ => self.host.replace_substr(old.as_str(), new.as_str())
            },
            None => match self.host.split_around_substr(old.as_str()) {
                ("", "" | ".")                        => Err(CantBeEmpty)?,
                (x , ""      ) if ends_in_empty   (x) => Err(NonFqdnCantEndInEmpty)?,
                (x , ""      ) if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , "."     ) if last_is_a_number(x) => Err(CantEndInANumber)?,

                (x , "" | ".") => self.host.replace_range(x.len() - 1 ..                     , ""),
                (x , _       ) => self.host.replace_range(x.len()     ..= x.len() + old.len(), ""),
            }
        }

        self.details.parts = DomainPartsDetails::from_raw_unchecked(&self.host);

        Ok(true)
    }

    /// Insert a new `index`th labels segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::new("example.com").unwrap();
    ///
    /// domain.insert_labels_segment( 0, "www").unwrap    (); assert_eq!(domain, "www.example.com");
    /// domain.insert_labels_segment(-1, "123").unwrap_err(); assert_eq!(domain, "www.example.com");
    /// domain.insert_labels_segment(-1, ""   ).unwrap_err(); assert_eq!(domain, "www.example.com");
    ///
    /// let mut domain = DomainHost::new("example.com.").unwrap();
    ///
    /// domain.insert_labels_segment( 0, "www").unwrap    (); assert_eq!(domain, "www.example.com." );
    /// domain.insert_labels_segment(-1, "123").unwrap_err(); assert_eq!(domain, "www.example.com." );
    /// domain.insert_labels_segment(-1, ""   ).unwrap    (); assert_eq!(domain, "www.example.com..");
    /// ```
    pub fn insert_labels_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        let new = value.try_into()?;

        if self.len() + new.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let temp = self.segments().try_neg_nth(index).map(|x| self.as_str().my_substr_range(x.as_str()));

        match (temp, index) {
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.last_is_a_number() => Err(CantEndInANumber)?,
            (Ok (_), -1) | (Err(0), 0..) if  self.is_fqdn() && new.ends_in_a_number() => Err(CantEndInANumber)?,

            (Err(1..), _) => Err(InsertNotFound)?,

            (Ok(Range {start, ..}), 0..) => self.host.to_mut().insert_with(start, &[new.as_str(), "."]),
            (Ok(Range {end  , ..}), ..0) => self.host.to_mut().insert_with(end  , &[".", new.as_str()]),

            (Err(0), 0..) => self.host.to_mut().insert_with(self.details.parts.suffix_after(), &[".", new.as_str()]),
            (Err(0), ..0) => self.host.to_mut().insert_with(0                          , &[new.as_str(), "."]),
        }

        self.details.parts = DomainPartsDetails::from_raw_unchecked(&self.host);

        Ok(())
    }
}

//! Suffix stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainDetails::has_suffix`].
    pub fn has_suffix(&self) -> bool {
        self.details().has_suffix()
    }



    /// The suffix as a [`str`].
    pub fn suffix_str(&self) -> &str {
        &self.host[self.details.suffix_range()]
    }

    /// The suffix as a [`DomainSegments`].
    pub fn suffix(&self) -> DomainSegments<'_> {
        DomainSegments(self.suffix_str().into())
    }



    /// The suffix segments as [`str`]s.
    pub fn suffix_segment_strs(&self) -> SplitDots<'_> {
        SplitDots(Some(self.suffix_str()))
    }

    /// The suffix segments as [`DomainSegment`]s.
    pub fn suffix_segments(&self) -> DomainSegmentsIter<'_> {
        DomainSegmentsIter(self.suffix_segment_strs())
    }



    /// The `index`th suffix segment as a [`str`].
    pub fn suffix_segment_str(&self, index: isize) -> Option<&str> {
        self.suffix_segment_strs().neg_nth(index)
    }

    /// The `index`th suffix segment as a [`DomainSegment`].
    pub fn suffix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.suffix_segments().neg_nth(index)
    }



    /// The range of suffix segments as a [`str`].
    pub fn suffix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        domain_range_thing(self.suffix_str(), range)
    }

    /// The range of suffix segments as a [`DomainSegments`].
    pub fn suffix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments(self.suffix_range_str(range)?.into()))
    }



    /// Set the suffix.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.com").unwrap();
    ///
    /// domain.set_suffix(Some("co.uk")).unwrap(); assert_eq!(domain, "example.co.uk");
    /// domain.set_suffix(Some("com"  )).unwrap(); assert_eq!(domain, "example.com"  );
    /// domain.set_suffix(None::<&str> ).unwrap(); assert_eq!(domain, "example"      );
    /// ```
    pub fn set_suffix<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.suffix_str();

        match value.map(TryInto::try_into).transpose()? {
            Some(new) if old == new => return Ok(false),

            Some(new) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            Some(new) => match self.host.split_around_substr(old) {
                ("", "" ) if new.is_empty        () => Err(CantBeEmpty)?,
                (_ , "" ) if new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
                (_ , "" ) if new.ends_in_a_number() => Err(CantEndInANumber)?,
                (_ , ".") if new.last_is_a_number() => Err(CantEndInANumber)?,

                _ => self.host.replace_substr(old, new.as_str())
            },

            None => match self.host.split_around_substr(old) {
                ("", "" | ".")                        => Err(CantBeEmpty)?,
                (x , ""      ) if ends_in_empty   (x) => Err(NonFqdnCantEndInEmpty)?,
                (x , ""      ) if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , "."     ) if last_is_a_number(x) => Err(CantEndInANumber)?,

                (x , "" | ".") => self.host.replace_range(x.len() - 1 ..                     , ""),
                (x , _       ) => self.host.replace_range(x.len()     ..= x.len() + old.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }

    /// Set or insert the `index`th suffix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.com").unwrap();
    ///
    /// domain.set_suffix_segment( 0, Some("co")  ).unwrap    ();  assert_eq!(domain.as_str(), "example.co"    );
    /// domain.set_suffix_segment( 1, Some("uk")  ).unwrap    ();  assert_eq!(domain.as_str(), "example.co.uk" );
    /// domain.set_suffix_segment( 0, None::<&str>).unwrap    ();  assert_eq!(domain.as_str(), "example.uk"    );
    /// domain.set_suffix_segment( 0, Some("123") ).unwrap_err();  assert_eq!(domain.as_str(), "example.uk"    );
    /// domain.set_suffix_segment(-2, Some("123") ).unwrap    ();  assert_eq!(domain.as_str(), "example.123.uk");
    /// domain.set_suffix_segment(-1, None::<&str>).unwrap_err();  assert_eq!(domain.as_str(), "example.123.uk");
    ///
    ///
    ///
    /// DomainHost::try_from("com.").unwrap().set_suffix_segment(0, None::<&str>).unwrap_err();
    /// ```
    pub fn set_suffix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let temp = self.suffix_segments().try_neg_nth(index);

        match (temp, value.map(TryInto::try_into).transpose()?) {
            (Ok (old), Some(new)) if old == new => return Ok(false),

            (Ok (old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,
            (Err(0  ), Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (Err(1..), Some(_)) => Err(InsertNotFound )?,
            (Err(_  ), None   ) => Err(SegmentNotFound)?,

            (Ok(old), Some(new)) => match self.host.split_around_substr(old.as_str()) {
                ("", "" ) if new.is_empty        () => Err(CantBeEmpty)?,
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
                0.. if !(self.details.is_fqdn() && new.last_is_empty()) && new.ends_in_a_number() => Err(CantEndInANumber)?,

                0.. => self.host.to_mut().insert_with(self.details.suffix_after(), &[".", new.as_str()]),
                ..0 => self.host.to_mut().insert_with(self.details.suffix_start(), &[new.as_str(), "."]),
            },
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }

    /// Set the `range` suffix segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_suffix_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.suffix_range(range).ok_or(RangeNotFound)?;
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

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }

    /// Insert a new `index`th suffix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.co").unwrap();
    ///
    /// domain.insert_suffix_segment( 1, "uk" ).unwrap    (); assert_eq!(domain, "example.co.uk"      );
    /// domain.insert_suffix_segment( 1, "aa" ).unwrap    (); assert_eq!(domain, "example.co.aa.uk"   );
    /// domain.insert_suffix_segment( 0, "co" ).unwrap    (); assert_eq!(domain, "example.co.aa.co.uk");
    /// domain.insert_suffix_segment(-1, "123").unwrap_err(); assert_eq!(domain, "example.co.aa.co.uk");
    ///
    /// let mut domain = DomainHost::try_from(".").unwrap();
    /// domain.insert_suffix_segment( 1, "123").unwrap_err();
    /// domain.insert_suffix_segment(-1, "123").unwrap_err();
    /// ```
    pub fn insert_suffix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        let new = value.try_into()?;

        if self.len() + new.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let temp = self.suffix_segments().try_neg_nth(index).map(|x| self.host.my_substr_range(x.as_str()));

        match (temp, index) {
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.ends_in_a_number() => Err(CantEndInANumber)?,
            (Ok (_), -1) | (Err(0), 0..) if  self.is_fqdn() && new.last_is_a_number() => Err(CantEndInANumber)?,

            (Err(1..), _) => Err(InsertNotFound)?,

            (Ok(Range {start, ..}), 0..) => self.host.to_mut().insert_with(start, &[new.as_str(), "."]),
            (Ok(Range {end  , ..}), ..0) => self.host.to_mut().insert_with(end  , &[".", new.as_str()]),

            (Err(0), 0..) => self.host.to_mut().insert_with(self.details.suffix_after(), &[".", new.as_str()]),
            (Err(0), ..0) => self.host.to_mut().insert_with(self.details.suffix_start(), &[new.as_str(), "."]),
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}

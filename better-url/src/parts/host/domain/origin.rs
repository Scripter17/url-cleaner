//! Origin stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainDetails::has_origin`].
    pub fn has_origin(&self) -> bool {
        self.details.has_origin()
    }



    /// The origin as a [`str`].
    pub fn origin_str(&self) -> Option<&str> {
        self.details.origin_range().map(|r| &self.host[r])
    }

    /// The origin as a [`DomainSegments`].
    pub fn origin(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments(self.origin_str()?.into()))
    }



    /// The origin segments as [`str`]s.
    pub fn origin_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.origin_str()?)))
    }

    /// The origin segments as [`DomainSegment`]s.
    pub fn origin_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.origin_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th origin segment as a [`str`].
    pub fn origin_segment_str(&self, index: isize) -> Option<&str> {
        self.origin_segment_strs()?.neg_nth(index)
    }

    /// The `index`th origin segment as a [`DomainSegment`].
    pub fn origin_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.origin_segments()?.neg_nth(index)
    }



    /// The range of origin segments as a [`str`].
    pub fn origin_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        domain_range_thing(self.origin_str()?, range)
    }

    /// The range of origin segments as a [`DomainSegments`].
    pub fn origin_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments(self.origin_range_str(range)?.into()))
    }



    /// Set the origin.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_origin<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let replace = self.origin_str().unwrap_or(self.suffix_str());

        match value.map(TryInto::try_into).transpose()? {
            Some(new) if replace == new => return Ok(false),

            Some(new) if self.len() - replace.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            Some(new) => match self.host.split_around_substr(replace) {
                ("", "" ) if new.is_empty        () => Err(CantBeEmpty)?,
                (_ , "" ) if new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
                (_ , "" ) if new.ends_in_a_number() => Err(CantEndInANumber)?,
                (_ , ".") if new.last_is_a_number() => Err(CantEndInANumber)?,

                _ => self.host.replace_substr(replace, new.as_str())
            },

            None => match self.host.split_around_substr(replace) {
                ("", "" | ".")                        => Err(CantBeEmpty)?,
                (x , ""      ) if ends_in_empty   (x) => Err(NonFqdnCantEndInEmpty)?,
                (x , ""      ) if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , "."     ) if last_is_a_number(x) => Err(CantEndInANumber)?,

                (x , "" | ".") => self.host.replace_range(x.len() - 1 ..                         , ""),
                (x , _       ) => self.host.replace_range(x.len()     ..= x.len() + replace.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }

    /// Set or insert the `index`th origin segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_origin_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let temp1 = self.origin_str().unwrap_or(self.suffix_str());

        let insert_start = temp1.addr    () - self.host.addr();
        let insert_after = temp1.end_addr() - self.host.addr();

        match (temp1.split('.').try_neg_nth(index), value.map(TryInto::try_into).transpose()?) {
            (Ok(old), Some(new)) if old == new => return Ok(false),

            (Ok (old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,
            (Err(_  ), Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (Err(1..), Some(_)) => Err(InsertNotFound )?,
            (Err(_  ), None   ) => Err(SegmentNotFound)?,

            (Ok(old), Some(new)) => match self.host.split_around_substr(old) {
                ("", "" ) if new.is_empty()         => Err(CantBeEmpty)?,
                (_ , "" ) if new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
                (_ , "" ) if new.ends_in_a_number() => Err(CantEndInANumber)?,
                (_ , ".") if new.last_is_a_number() => Err(CantEndInANumber)?,

                _ => self.host.replace_substr(old, new.as_str()),
            },

            (Ok(old), None) => match self.host.split_around_substr(old) {
                ("", "" | ".")                        => Err(CantBeEmpty)?,
                (x , ""      ) if ends_in_empty   (x) => Err(NonFqdnCantEndInEmpty)?,
                (x , ""      ) if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , "."     ) if last_is_a_number(x) => Err(CantEndInANumber)?,

                (x , "" | ".") => self.host.replace_range(x.len() - 1 ..                     , ""),
                (x , _       ) => self.host.replace_range(x.len()     ..= x.len() + old.len(), ""),
            },

            (Err(0), Some(new)) => match index {
                0.. if !(self.details.is_fqdn() && new.last_is_empty()) && new.ends_in_a_number() => Err(CantEndInANumber)?,

                0.. => self.host.to_mut().insert_with(insert_after, &[".", new.as_str()]),
                ..0 => self.host.to_mut().insert_with(insert_start, &[new.as_str(), "."]),
            },
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }

    /// Set the `range` origin segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_origin_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        let old = self.origin_range(range).ok_or(RangeNotFound)?;
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

    /// Insert a new `index`th origin segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn insert_origin_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        let new = value.try_into()?;

        if self.len() + new.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let temp1 = self.origin_str().unwrap_or(self.suffix_str());

        let insert_start = temp1.addr    () - self.host.addr();
        let insert_after = temp1.end_addr() - self.host.addr();

        let temp2 = temp1.split('.').map(|x| self.host.my_substr_range(x)).try_neg_nth(index);

        match (temp2, index) {
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.last_is_empty   () => Err(NonFqdnCantEndInEmpty)?,
            (Ok (_), -1) | (Err(0), 0..) if !self.is_fqdn() && new.ends_in_a_number() => Err(CantEndInANumber)?,
            (Ok (_), -1) | (Err(0), 0..) if  self.is_fqdn() && new.last_is_a_number() => Err(CantEndInANumber)?,

            (Err(1..), _) => Err(InsertNotFound)?,

            (Ok(Range {start, ..}), 0..) => self.host.to_mut().insert_with(start, &[new.as_str(), "."]),
            (Ok(Range {end  , ..}), ..0) => self.host.to_mut().insert_with(end  , &[".", new.as_str()]),

            (Err(0), 0..) => self.host.to_mut().insert_with(insert_after, &[".", new.as_str()]),
            (Err(0), ..0) => self.host.to_mut().insert_with(insert_start, &[new.as_str(), "."]),
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}

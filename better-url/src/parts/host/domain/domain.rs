//! Domain stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// Get the segments.
    pub fn segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.labels().split('.')
    }

    /// Get the `index`th segment.
    pub fn segment(&self, index: isize) -> Option<&str> {
        self.segments().neg_nth(index)
    }

    /// Get the `index`th segment or how many short we are.
    fn try_segment(&self, index: isize) -> Result<&str, usize> {
        self.segments().try_neg_nth(index)
    }

    /// Set or insert the `index`th segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => match self.try_segment(index) {
                Ok(segment) => match self.host.split_around_substr(segment) {
                    ("", "" ) if value.is_empty()                                    => Err(CantBeEmpty)?,
                    (_ , "" ) if ends_in_a_number(value)                             => Err(CantEndInANumber)?,
                    (_ , ".") if !value.ends_with(".") && ends_in_a_number(value)    => Err(CantEndInANumber)?,
                    (x , y  ) if x.len() + value.len() + y.len() > u32::MAX as usize => Err(TooLong)?,
                    _         if value.bytes().any(invalid_domain_byte)              => Err(InvalidDomainByte)?,
                    _                                                                => self.host.replace_substr(segment, value),
                },
                Err(0) => {
                    if self.len() + value.len() + 1 > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    match index {
                        0.. => {
                            if (!self.details.is_fqdn() || !value.ends_with(".")) && ends_in_a_number(value) {
                                Err(CantEndInANumber)?;
                            }

                            self.host.to_mut().insert_with(self.details.suffix_after(), [".", value]);
                        }
                        ..0 => self.host.to_mut().insert_with(0, [value, "."]),
                    }
                },
                Err(_) => Err(InsertNotFound)?
            },
            None => match self.host.split_around_substr(self.segment(index).ok_or(SegmentNotFound)?) {
                ("", ""      )                        => Err(CantBeEmpty)?,
                (x , "" | ".") if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , y       )                        => self.host.replace_range(x.len() - 1 .. x.len() - 1 + y.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }

    /// Replace the `index`th segment without inserting a new one.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn replace_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let segment = self.segment(index).ok_or(SegmentNotFound)?;

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => match self.host.split_around_substr(segment) {
                ("", "" ) if value.is_empty()                                    => Err(CantBeEmpty)?,
                (_ , "" ) if ends_in_a_number(value)                             => Err(CantEndInANumber)?,
                (_ , ".") if !value.ends_with(".") && ends_in_a_number(value)    => Err(CantEndInANumber)?,
                (x , y  ) if x.len() + y.len() + value.len() > u32::MAX as usize => Err(TooLong)?,
                _         if value.bytes().any(invalid_domain_byte)              => Err(InvalidDomainByte)?,
                _                                                                => self.host.replace_substr(segment, value),
            },
            None => match self.host.split_around_substr(segment) {
                ("", ""      ) => Err(CantBeEmpty)?,
                (x , "" | ".") if ends_in_a_number(x) => Err(CantEndInANumber)?,
                ("", y       ) => self.host.retain_substr(y),
                (x , y       ) => self.host.replace_range(x.len() - 1 .. x.len() - 1 + y.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }

    /// Insert a new `index`th segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn insert_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let value = &encode_domain(value.into());

        if self.len() + value.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        if value.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainByte)?;
        }

        match self.try_segment(index).map(|x| self.host.my_substr_range(x)) {
            Ok(Range {end, ..}) => match index {
                0.. => self.host.to_mut().insert_with(0, [value, "."]),
                ..0 => {
                    if end == self.details.suffix_after() && (!self.details.is_fqdn() || !value.ends_with(".")) && ends_in_a_number(value) {
                        Err(CantEndInANumber)?;
                    }

                    self.host.to_mut().insert_with(end, [".", value]);
                }
            },
            Err(0) => match index {
                0.. => {
                    if (!self.is_fqdn() || !value.ends_with(".")) && ends_in_a_number(value) {
                        Err(CantEndInANumber)?;
                    }

                    self.host.to_mut().insert_with(self.details.suffix_after(), [".", value]);
                },
                ..0 => self.host.to_mut().insert_with(0, [value, "."]),
            },
            Err(_) => Err(InsertNotFound)?
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}

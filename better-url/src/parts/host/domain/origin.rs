//! Origin stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// Returns [`true`] if the domain has a origin.
    pub fn has_origin(&self) -> bool {
        self.details.has_origin()
    }

    /// Get the origin.
    pub fn origin(&self) -> Option<&str> {
        self.details.origin_range().map(|r| &self.host[r])
    }

    /// Get the origin segments.
    pub fn origin_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.origin().into_iter().flat_map(|x| x.split('.'))
    }

    /// Get the `index`th origin segment.
    pub fn origin_segment(&self, index: isize) -> Option<&str> {
        self.origin_segments().neg_nth(index)
    }

    /// Set the origin.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_origin(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => {
                let replace = self.origin().unwrap_or(self.suffix());

                if replace == value {
                    return Ok(());
                }

                if self.len() - replace.len() + value.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                if ends_in_a_number(value) {
                    Err(CantEndInANumber)?;
                }

                self.host.replace_substr(replace, value);
            },
            None => if let Some(replace) = self.origin() {
                match self.host.split_around_substr(replace) {
                    ("", "") => Err(CantBeEmpty)?,
                    (x, "" | ".") if ends_in_a_number(x) => Err(CantEndInANumber)?,
                    _ => self.host.replace_substr(replace, "")
                }
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }

    /// Set or insert the `index`th origin or suffix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut x = self.origin().unwrap_or(self.suffix()).split('.');

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => match x.try_neg_nth(index) {
                Ok(segment) => {
                    if segment == value {
                        return Ok(());
                    }

                    match self.host.split_around_substr(segment) {
                        ("", "" ) if value.is_empty()                                    => Err(CantBeEmpty)?,
                        (_ , "" ) if ends_in_a_number(value)                             => Err(CantEndInANumber)?,
                        (_ , ".") if !value.ends_with(".") && ends_in_a_number(value)    => Err(CantEndInANumber)?,
                        (x , y  ) if x.len() + value.len() + y.len() > u32::MAX as usize => Err(TooLong)?,
                        _         if value.bytes().any(invalid_domain_byte)              => Err(InvalidDomainByte)?,
                        _                                                                => self.host.replace_substr(segment, value),
                    }
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

                            self.host.to_mut().insert_with(self.details.origin_start().ok_or(InsertNotFound)?, [".", value]);
                        }
                        ..0 => self.host.to_mut().insert_with(self.details.origin_after().ok_or(InsertNotFound)?, [value, "."]),
                    }
                },
                Err(_) => Err(InsertNotFound)?
            },
            None => match self.host.split_around_substr(x.neg_nth(index).ok_or(SegmentNotFound)?) {
                ("", ""      )                        => Err(CantBeEmpty)?,
                (x , "" | ".") if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , y       )                        => self.host.replace_range(x.len() - 1 .. x.len() - 1 + y.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }

    /// Set the `index`th origin or suffix segment without inserting a new one.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn replace_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let segment = self.origin().unwrap_or(self.suffix()).split('.').neg_nth(index).ok_or(SegmentNotFound)?;

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => {
                if segment == value {
                    return Ok(());
                }

                match self.host.split_around_substr(segment) {
                    ("", "" ) if value.is_empty()                                    => Err(CantBeEmpty)?,
                    (_ , "" ) if ends_in_a_number(value)                             => Err(CantEndInANumber)?,
                    (_ , ".") if !value.ends_with(".") && ends_in_a_number(value)    => Err(CantEndInANumber)?,
                    (x , y  ) if x.len() + y.len() + value.len() > u32::MAX as usize => Err(TooLong)?,
                    _         if value.bytes().any(invalid_domain_byte)              => Err(InvalidDomainByte)?,
                    _                                                                => self.host.replace_substr(segment, value),
                }
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

    /// Insert a new `index`th origin or suffix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn insert_origin_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let value = &encode_domain(value.into());

        if self.len() + value.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        if value.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainByte)?;
        }

        let mut x = self.origin().unwrap_or(self.suffix()).split('.');

        match x.try_neg_nth(index).map(|x| self.host.my_substr_range(x)) {
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

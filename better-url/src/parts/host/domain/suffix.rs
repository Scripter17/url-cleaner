//! Suffix stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// Get the suffix.
    pub fn suffix(&self) -> &str {
        &self.host[self.details.suffix_range()]
    }

    /// Get the suffix's segments.
    pub fn suffix_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.suffix().split('.')
    }

    /// Get the `index`th suffix segment.
    pub fn suffix_segment(&self, index: isize) -> Option<&str> {
        self.suffix_segments().neg_nth(index)
    }

    /// Get the `index`th suffix segment or how many short we are.
    fn try_suffix_segment(&self, index: isize) -> Result<&str, usize> {
        self.suffix_segments().try_neg_nth(index)
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
    /// domain.set_suffix(Some("co.uk")).unwrap();
    /// assert_eq!(domain.suffix(), "co.uk");
    ///
    /// domain.set_suffix(Some("com")).unwrap();
    /// assert_eq!(domain.suffix(), "com");
    /// ```
    pub fn set_suffix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let suffix = self.suffix();

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => {
                if suffix == value {
                    return Ok(())
                }
                
                if self.len() - self.suffix().len() + value.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                if !(self.details.is_fqdn() && value.ends_with(".")) && ends_in_a_number(value) {
                    Err(CantEndInANumber)?;
                }

                self.host.replace_substr(suffix, value);
            },
            None => match (self.details.middle_after(), self.is_fqdn()) {
                (None   , false) => Err(CantBeEmpty)?,
                (None   , true ) => self.host = ".".into(),
                (Some(x), false) => self.host.retain_range(..x),
                (Some(_), true ) => self.host.replace_substr(suffix, ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
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
    /// domain.set_suffix_segment(0, Some("co")).unwrap();
    /// assert_eq!(domain.as_str(), "example.co");
    ///
    /// domain.set_suffix_segment(1, Some("uk")).unwrap();
    /// assert_eq!(domain.as_str(), "example.co.uk");
    ///
    /// domain.set_suffix_segment(0, None).unwrap();
    /// assert_eq!(domain.as_str(), "example.uk");
    ///
    /// domain.set_suffix_segment(0, Some("123")).unwrap_err();
    /// assert_eq!(domain.as_str(), "example.uk");
    ///
    /// domain.set_suffix_segment(-2, Some("123")).unwrap();
    /// assert_eq!(domain.as_str(), "example.123.uk");
    ///
    /// domain.set_suffix_segment(-1, None).unwrap_err();
    /// assert_eq!(domain.as_str(), "example.123.uk");
    /// ```
    pub fn set_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => match self.try_suffix_segment(index) {
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

                            self.host.to_mut().insert_with(self.details.suffix_after(), [".", value]);
                        }
                        ..0 => self.host.to_mut().insert_with(self.details.suffix_start(), [value, "."]),
                    }
                },
                Err(_) => Err(InsertNotFound)?
            },
            None => match self.host.split_around_substr(self.suffix_segment(index).ok_or(SegmentNotFound)?) {
                ("", ""      )                        => Err(CantBeEmpty)?,
                (x , "" | ".") if ends_in_a_number(x) => Err(CantEndInANumber)?,
                (x , y       )                        => self.host.replace_range(x.len() - 1 .. x.len() - 1 + y.len(), ""),
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }

    /// Set the `index`th suffix segment without inserting a new one.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.co.uk").unwrap();
    ///
    /// domain.replace_suffix_segment(0, Some("123")).unwrap();
    /// assert_eq!(domain.as_str(), "example.123.uk");
    ///
    /// domain.replace_suffix_segment(-1, None).unwrap_err();
    /// assert_eq!(domain.as_str(), "example.123.uk");
    ///
    /// let mut domain = DomainHost::try_from(".").unwrap();
    /// domain.replace_suffix_segment( 0, Some("123")).unwrap_err();
    /// domain.replace_suffix_segment(-1, Some("123")).unwrap_err();
    /// ```
    pub fn replace_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let segment = self.suffix_segment(index).ok_or(SegmentNotFound)?;

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

    /// Insert a new `index`th suffix segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("example.co").unwrap();
    ///
    /// domain.insert_suffix_segment(1, "uk").unwrap();
    /// assert_eq!(domain.as_str(), "example.co.uk");
    ///
    /// domain.insert_suffix_segment(1, "aa").unwrap();
    /// assert_eq!(domain.as_str(), "example.co.aa.uk");
    ///
    /// domain.insert_suffix_segment(0, "co").unwrap();
    /// assert_eq!(domain.as_str(), "example.co.aa.co.uk");
    ///
    /// domain.insert_suffix_segment(-1, "123").unwrap_err();
    /// assert_eq!(domain.as_str(), "example.co.aa.co.uk");
    ///
    /// let mut domain = DomainHost::try_from(".").unwrap();
    /// domain.insert_suffix_segment( 1, "123").unwrap_err();
    /// domain.insert_suffix_segment(-1, "123").unwrap_err();
    /// ```
    pub fn insert_suffix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let value = &encode_domain(value.into());

        if self.len() + value.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        if value.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainByte)?;
        }

        match self.try_suffix_segment(index).map(|x| self.host.my_substr_range(x)) {
            Ok(Range {start, end}) => match index {
                0.. => self.host.to_mut().insert_with(start, [value, "."]),
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
                ..0 => self.host.to_mut().insert_with(self.details.suffix_start(), [value, "."]),
            },
            Err(_) => Err(InsertNotFound)?
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}

//! Prefix stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// Returns [`true`] if the domain has a prefix.
    pub fn has_prefix(&self) -> bool {
        self.details.has_prefix()
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.details.prefix_range().map(|r| &self.host[r])
    }

    /// Get the prefix segments.
    pub fn prefix_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.prefix().into_iter().flat_map(|x| x.split('.'))
    }

    /// Get the `index`th prefix segment.
    pub fn prefix_segment(&self, index: isize) -> Option<&str> {
        self.prefix_segments().neg_nth(index)
    }

    /// Get the `index`th prefix segment or how many short we are.
    fn try_prefix_segment(&self, index: isize) -> Result<&str, usize> {
        self.prefix_segments().try_neg_nth(index)
    }

    /// Set the prefix.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_prefix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        match (self.prefix(), value.map(|x| encode_domain(x.into())).as_deref()) {
            (None, None) => {},

            (None, Some(value)) => {
                if !self.details.has_middle() {
                    Err(InsertNotFound)?;
                }

                if self.len() + value.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.to_mut().insert_with(0, [value, "."]);

                self.details.ms += value.len() as u32 + 1;
                self.details.ss += value.len() as u32 + 1;
                self.details.sa += value.len() as u32 + 1;
            },

            (Some(prefix), None) => {
                let pl = prefix.len();

                self.host.replace_range(..=pl, "");

                self.details.ms -= pl as u32 + 1;
                self.details.ss -= pl as u32 + 1;
                self.details.sa -= pl as u32 + 1;
            },

            (Some(prefix), Some(value)) => {
                if prefix == value {
                    return Ok(());
                }
                
                if self.len() - prefix.len() + value.len() > u32::MAX as usize {
                    Err(TooLong)?
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                let pl = prefix.len();

                self.host.replace_substr(prefix, value);

                self.details.ms = self.details.ms - pl as u32 + value.len() as u32;
                self.details.ss = self.details.ss - pl as u32 + value.len() as u32;
                self.details.sa = self.details.sa - pl as u32 + value.len() as u32;
            }
        }

        self.details.www_prefix = &self.host[..self.details.ms as usize] == "www.";

        Ok(())
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
    /// domain.set_prefix_segment(0, Some("www")).unwrap();
    /// assert_eq!(domain.prefix(), Some("www"));
    ///
    /// domain.set_prefix_segment(0, Some("abc")).unwrap();
    /// assert_eq!(domain.prefix(), Some("abc"));
    ///
    /// domain.set_prefix_segment(1, Some("def")).unwrap();
    /// assert_eq!(domain.prefix(), Some("abc.def"));
    ///
    /// domain.set_prefix_segment(-3, Some("123")).unwrap();
    /// assert_eq!(domain.prefix(), Some("123.abc.def"));
    ///
    /// domain.set_prefix_segment(-3, None).unwrap();
    /// assert_eq!(domain.prefix(), Some("abc.def"));
    ///
    /// domain.set_prefix_segment(-1, None).unwrap();
    /// assert_eq!(domain.prefix(), Some("abc"));
    ///
    /// domain.set_prefix_segment(-1, None).unwrap();
    /// assert_eq!(domain.prefix(), None);
    /// ```
    pub fn set_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => match self.try_prefix_segment(index) {
                Ok(segment) => {
                    if segment == value {
                        return Ok(());
                    }
                    
                    if self.len() - segment.len() + value.len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    if value.bytes().any(invalid_domain_byte) {
                        Err(InvalidDomainByte)?
                    }

                    let sl = segment.len();

                    self.host.replace_substr(segment, value);

                    self.details.ms = self.details.ms - sl as u32 + value.len() as u32;
                    self.details.ss = self.details.ss - sl as u32 + value.len() as u32;
                    self.details.sa = self.details.sa - sl as u32 + value.len() as u32;
                },

                Err(0) => {
                    let ms = self.details.middle_start().ok_or(InsertNotFound)?;

                    if self.len() + value.len() + 1 > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    if value.bytes().any(invalid_domain_byte) {
                        Err(InvalidDomainByte)?;
                    }

                    match index {
                        0.. => self.host.to_mut().insert_with(ms, [value, "."]),
                        ..0 => self.host.to_mut().insert_with(0 , [value, "."]),
                    }

                    self.details.ms = self.details.ms + value.len() as u32 + 1;
                    self.details.ss = self.details.ss + value.len() as u32 + 1;
                    self.details.sa = self.details.sa + value.len() as u32 + 1;
                },

                Err(_) => Err(InsertNotFound)?
            },
            None => {
                let Range {start, end} = self.host.my_substr_range(self.prefix_segment(index).ok_or(SegmentNotFound)?);
                let sl = end - start;

                self.host.replace_range(start ..= end, "");

                self.details.ms = self.details.ms - sl as u32 - 1;
                self.details.ss = self.details.ss - sl as u32 - 1;
                self.details.sa = self.details.sa - sl as u32 - 1;
            }
        }

        self.details.www_prefix = &self.host[..self.details.ms as usize] == "www.";

        Ok(())
    }

    /// Set the `index`th prefix segment without inserting a new one.
    /// # Errors
    /// See [`Self`]'s documentation.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut domain = DomainHost::try_from("www.example.com").unwrap();
    ///
    /// domain.replace_prefix_segment(0, Some("abc")).unwrap();
    /// assert_eq!(domain.prefix(), Some("abc"));
    ///
    /// domain.replace_prefix_segment(0, None).unwrap();
    /// assert_eq!(domain.prefix(), None);
    ///
    /// domain.replace_prefix_segment(0, Some("www")).unwrap_err();
    /// ```
    pub fn replace_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let segment = self.prefix_segment(index).ok_or(SegmentNotFound)?;
        let Range {start, end} = self.host.my_substr_range(segment);
        let sl = end - start;

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => {
                if segment == value {
                    return Ok(());
                }
                
                if self.len() - sl + value.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.replace_range(start..end, value);

                self.details.ms = self.details.ms - sl as u32 + value.len() as u32;
                self.details.ss = self.details.ss - sl as u32 + value.len() as u32;
                self.details.sa = self.details.sa - sl as u32 + value.len() as u32;
            },
            None => {
                self.host.replace_range(start..=end, "");

                self.details.ms = self.details.ms - sl as u32 - 1;
                self.details.ss = self.details.ss - sl as u32 - 1;
                self.details.sa = self.details.sa - sl as u32 - 1;
            }
        }

        self.details.www_prefix = &self.host[..self.details.ms as usize] == "www.";

        Ok(())
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
    /// domain.insert_prefix_segment(0, "www").unwrap();
    /// assert_eq!(domain.as_str(), "www.example.com");
    ///
    /// domain.insert_prefix_segment(0, "abc").unwrap();
    /// assert_eq!(domain.as_str(), "abc.www.example.com");
    ///
    /// domain.insert_prefix_segment(2, "def").unwrap();
    /// assert_eq!(domain.as_str(), "abc.www.def.example.com");
    ///
    /// domain.insert_prefix_segment(-4, "ghi").unwrap();
    /// assert_eq!(domain.as_str(), "ghi.abc.www.def.example.com");
    /// ```
    pub fn insert_prefix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let value = &encode_domain(value.into());

        if self.len() + value.len() + 1 > u32::MAX as usize {
            Err(TooLong)?;
        }

        let ms = self.details.middle_start().ok_or(InsertNotFound)?;

        if value.bytes().any(invalid_domain_byte) {
            Err(InvalidDomainByte)?;
        }

        match (self.try_prefix_segment(index).map(|x| self.host.my_substr_range(x)), index) {
            (Ok(Range {start, ..}), 0..) => self.host.to_mut().insert_with(start, [value, "."]),
            (Ok(Range {end  , ..}), ..0) => self.host.to_mut().insert_with(end  , [value, "."]),
            (Err(0)               , 0..) => self.host.to_mut().insert_with(ms   , [value, "."]),
            (Err(0)               , ..0) => self.host.to_mut().insert_with(0    , [value, "."]),
            _ => Err(InsertNotFound)?
        }

        self.details.ms += value.len() as u32 + 1;
        self.details.ss += value.len() as u32 + 1;
        self.details.sa += value.len() as u32 + 1;

        self.details.www_prefix = &self.host[..self.details.ms as usize] == "www.";

        Ok(())
    }
}

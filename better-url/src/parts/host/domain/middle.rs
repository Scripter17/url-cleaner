//! Middle stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// If it has a middle..
    pub fn has_middle(&self) -> bool {
        self.details.ss != 0
    }



    /// The [`Range::start`] of the middle.
    pub(crate) fn middle_start(&self) -> Option<usize> {
        match self.details.ss {
            0 => None,
            _ => Some(self.details.ms as usize)
        }
    }

    /// The [`Range::end`] of the middle.
    fn middle_after(&self) -> Option<usize> {
        match self.details.ss {
            0 => None,
            x => Some(x as usize - 1)
        }
    }

    /// The [`Range`] of the middle.
    pub(crate) fn middle_thing(&self) -> Option<Range<usize>> {
        Some(self.middle_start()? .. self.middle_after()?)
    }



    /// The middle as a [`str`].
    pub fn middle_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.middle_thing()?)})
    }

    /// The middle as a [`DomainSegment`].
    pub fn middle(&self) -> Option<DomainSegment<'_>> {
        Some(DomainSegment(self.middle_str()?.into()))
    }



    /// Set the middle.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_middle<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match (self.middle(), value.map(TryInto::try_into).transpose()?) {
            (None     , None     )               => return Ok(false),
            (Some(old), Some(new)) if old == new => return Ok(false),

            (None     , Some(new)) if self.len()             + new.len() + 1 > u32::MAX as usize => Err(TooLong)?,
            (Some(old), Some(new)) if self.len() - old.len() + new.len()     > u32::MAX as usize => Err(TooLong)?,

            (Some(old), Some(new)) => self.host.replace_substr(old.as_str(), new.as_str()),

            (Some(old), None) => {
                let mut range = self.host.my_substr_range(old.as_str());
                range.end += 1;
                self.host.replace_range(range, "");
            },

            (None, Some(new)) => self.host.insert_with(0, [new.as_str(), "."]),
        }

        self.details = DomainHostDetails::parse_unchecked(&self.host);

        Ok(true)
    }
}

//! Middle stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainDetails::has_middle`].
    pub fn has_middle(&self) -> bool {
        self.details.has_middle()
    }



    /// The middle as a [`str`].
    pub fn middle_str(&self) -> Option<&str> {
        self.details.middle_range().map(|r| &self.host[r])
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

            (None, Some(new)) => self.host.to_mut().insert_with(0, &[new.as_str(), "."]),
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(true)
    }
}
